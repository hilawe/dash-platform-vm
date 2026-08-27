//! Shared GroveDB-backed CosmWasm host components for the prototype spikes.
//!
//! This crate holds the ONE storage backend and cost adapter the host bins share, so there is no
//! drifting copy per binary. The storage is an OVERLAY over GroveDB: contract writes go to an
//! in-memory overlay during a call. The overlay is durable ONLY when the caller commits it; on
//! failure the caller must `discard` (or drop the storage). Skipping `commit` is not by itself
//! rollback, because the overlay persists on a reused instance and a later `commit` would flush an
//! earlier failed call's writes. This crate provides the primitives (`commit`, `discard`, `atomic`),
//! but does NOT itself impose a per-call boundary; each caller decides. The EVM spike's `apply`
//! wraps every call as a real boundary (commit on success, discard on failure) and demonstrates the
//! rollback: a store-then-fail call leaves no durable write even after a later successful commit. The
//! e2e and write spikes are single-success-path demos that commit once and do not exercise the
//! discard side. `commit` clears the overlay so committed writes are neither re-applied nor re-read
//! as pending. Reads consult the overlay first (read-your-writes and tombstones), then GroveDB.
//! Errors are mapped honestly: only a genuine key/path-not-found reads as absence, and every other
//! GroveDB error is surfaced rather than silently turned into an empty result.

use cosmwasm_std::{Order, Record};
use cosmwasm_vm::{BackendError, BackendResult, GasInfo, Storage};
use grovedb::query_result_type::QueryResultType;
use grovedb::{Element, Error as GroveError, GroveDb, PathQuery, Query, SizedQuery};
use grovedb_costs::{CostContext, OperationCost};
use grovedb_version::version::GroveVersion;
use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

pub fn gv() -> &'static GroveVersion {
    GroveVersion::latest()
}

/// Gas per byte of storage/overlay work, shared by the GroveDB cost adapter and the in-memory overlay
/// charges so both price a byte the same way.
const GAS_PER_BYTE: u64 = 100;
/// Base gas for touching the in-memory overlay on a single-key read (independent of value size).
const OVERLAY_READ_BASE: u64 = 500;
/// Gas per overlay entry EXAMINED during a range merge, so a scan that must walk a large pending
/// overlay is charged for that O(N) work rather than a flat fee.
const OVERLAY_SCAN_PER_ENTRY: u64 = 50;

/// The cost-to-gas adapter: CosmWasm gas derived deterministically from GroveDB's measured
/// OperationCost (seeks, storage bytes, loaded bytes, hash-node calls), not a flat schedule. The
/// absolute weights are a policy calibration; the point is that gas tracks the real measured cost.
pub fn cost_to_gas(c: &OperationCost) -> u64 {
    const SEEK: u64 = 10_000;
    const HASH: u64 = 2_000;
    let removed = c.storage_cost.removed_bytes.total_removed_bytes() as u64;
    (c.seek_count as u64).saturating_mul(SEEK)
        .saturating_add(
            (c.storage_cost.added_bytes as u64)
                .saturating_add(c.storage_cost.replaced_bytes as u64)
                .saturating_add(removed)
                .saturating_add(c.storage_loaded_bytes)
                .saturating_mul(GAS_PER_BYTE),
        )
        .saturating_add((c.hash_node_calls as u64).saturating_mul(HASH))
}

/// Deterministic gas for the in-memory overlay work a range merge performs: one per-entry charge for
/// every overlay entry EXAMINED, plus a per-byte charge for every key byte compared against the
/// bounds (charged for every entry, including out-of-range entries and tombstones) and every value
/// byte materialized into the merged result. This is what stops an abandoned scan over a large
/// pending overlay from being O(N) work for a flat fee. Saturating so it never wraps below a real cost.
fn overlay_scan_gas(entries_examined: u64, bytes_touched: u64) -> u64 {
    entries_examined
        .saturating_mul(OVERLAY_SCAN_PER_ENTRY)
        .saturating_add(bytes_touched.saturating_mul(GAS_PER_BYTE))
}

/// Deterministic gas for handling a raw query REQUEST of `len` bytes: the caller-controlled request is
/// deserialized (and, on error, an error is constructed) before any read, so a querier must charge for
/// that parsing/allocation on EVERY return path, not return free gas for a malformed or unsupported
/// request. Saturating per-byte.
pub fn request_gas(len: usize) -> u64 {
    (len as u64).saturating_mul(GAS_PER_BYTE)
}

/// True for the GroveDB errors that genuinely mean "this key/path is not present", as opposed to
/// corruption or an operational failure, which must NOT be conflated with absence.
pub fn is_not_found(e: &GroveError) -> bool {
    matches!(
        e,
        GroveError::PathKeyNotFound(_)
            | GroveError::PathNotFound(_)
            | GroveError::PathParentLayerNotFound(_)
    )
}

/// Read one key directly from GroveDB (no overlay), mapping only genuine not-found to `None` and
/// surfacing every other error.
pub fn read_item(
    db: &GroveDb,
    subtree: &[&[u8]],
    key: &[u8],
) -> Result<Option<Vec<u8>>, String> {
    let cc: CostContext<Result<Element, GroveError>> = db.get(subtree, key, None, gv());
    match cc.value {
        Ok(Element::Item(bytes, _)) => Ok(Some(bytes)),
        Ok(other) => Err(format!("unexpected element type at key: {other:?}")),
        Err(ref e) if is_not_found(e) => Ok(None),
        Err(e) => Err(format!("grovedb get error: {e}")),
    }
}

/// A `cosmwasm_vm::Storage` over a GroveDB subtree with an in-memory write overlay for rollback.
/// Owns an `Arc<GroveDb>` so it satisfies the VM's `S: Storage + 'static` bound.
pub struct OverlayGroveStorage {
    db: Arc<GroveDb>,
    path: Vec<Vec<u8>>,
    /// Pending writes for this call. `Some(v)` is a set, `None` is a tombstone (remove).
    overlay: BTreeMap<Vec<u8>, Option<Vec<u8>>>,
    iterators: HashMap<u32, std::vec::IntoIter<Record>>,
    next_iterator_id: u32,
}

impl OverlayGroveStorage {
    pub fn new(db: Arc<GroveDb>, path: Vec<Vec<u8>>) -> Self {
        OverlayGroveStorage {
            db,
            path,
            overlay: BTreeMap::new(),
            iterators: HashMap::new(),
            next_iterator_id: 1,
        }
    }

    pub fn db(&self) -> Arc<GroveDb> {
        self.db.clone()
    }

    fn subtree(&self) -> Vec<&[u8]> {
        self.path.iter().map(|v| v.as_slice()).collect()
    }

    /// The effective value for a key: overlay first (including tombstones), then GroveDB.
    fn read(&self, key: &[u8]) -> Result<Option<Vec<u8>>, String> {
        if let Some(pending) = self.overlay.get(key) {
            return Ok(pending.clone());
        }
        read_item(self.db.as_ref(), self.subtree().as_slice(), key)
    }

    /// The effective (key, value) records in `[start, end)`, GroveDB merged with the overlay, in the
    /// requested order, together with the REAL `OperationCost` of the underlying GroveDB range query
    /// AND a deterministic gas charge for the in-memory overlay work (entries examined + bytes
    /// materialized). Both are returned even on the error path (they reflect work already performed),
    /// so the caller can charge gas for everything a scan forces regardless of outcome.
    fn effective_range(
        &self,
        start: Option<&[u8]>,
        end: Option<&[u8]>,
        order: Order,
    ) -> (Result<Vec<Record>, String>, OperationCost, u64) {
        // Charge for the range-bound copies (start/end are `to_vec()`'d into the query below); this is
        // added to the overlay gas on every return path so an empty-store scan cannot materialize
        // unbounded bound input for a bounded query charge.
        let bound_bytes =
            start.map(|s| s.len()).unwrap_or(0) as u64 + end.map(|e| e.len()).unwrap_or(0) as u64;
        let bound_gas = bound_bytes.saturating_mul(GAS_PER_BYTE);
        let mut q = Query::new_with_direction(true);
        match (start, end) {
            (Some(s), Some(e)) => q.insert_range(s.to_vec()..e.to_vec()),
            (Some(s), None) => q.insert_range_from(s.to_vec()..),
            (None, Some(e)) => q.insert_range_to(..e.to_vec()),
            (None, None) => q.insert_all(),
        }
        let pq = PathQuery {
            path: self.path.clone(),
            query: SizedQuery {
                query: q,
                limit: None,
                offset: None,
            },
        };
        let cc = self.db.query(
            &pq,
            false,
            false,
            true,
            QueryResultType::QueryKeyElementPairResultType,
            None,
            gv(),
        );
        // Partial-move the cost and value out of the CostContext so the measured cost is available on
        // both the success and error paths.
        let cost = cc.cost;
        let (elements, _) = match cc.value {
            Ok(v) => v,
            Err(e) => return (Err(format!("grovedb range query error: {e}")), cost, bound_gas),
        };
        let mut map: BTreeMap<Vec<u8>, Vec<u8>> = BTreeMap::new();
        for (k, el) in elements.to_key_elements() {
            match el {
                Element::Item(bytes, _) => {
                    map.insert(k, bytes);
                }
                // A non-item element in a contract's flat key-value subtree is unexpected; surface it
                // rather than silently dropping it, so corruption cannot masquerade as a short range.
                other => {
                    return (
                        Err(format!("range: unexpected non-item element at key {k:?}: {other:?}")),
                        cost,
                        bound_gas,
                    )
                }
            }
        }
        // Apply the overlay within the range bounds, charging for every entry EXAMINED and every byte
        // TOUCHED, so walking a large pending overlay is not free O(N) work per scan. EVERY overlay
        // key is compared against the bounds (so its bytes are charged, even for an out-of-range entry
        // or a tombstone), and a materialized value's bytes are charged on top.
        let in_range = |k: &[u8]| -> bool {
            start.map(|s| k >= s).unwrap_or(true) && end.map(|e| k < e).unwrap_or(true)
        };
        let mut entries_examined: u64 = 0;
        let mut bytes_touched: u64 = 0;
        for (k, pending) in &self.overlay {
            entries_examined = entries_examined.saturating_add(1);
            // Every key is compared against the range bounds, regardless of whether it falls in range.
            bytes_touched = bytes_touched.saturating_add(k.len() as u64);
            if !in_range(k) {
                continue;
            }
            match pending {
                Some(v) => {
                    bytes_touched = bytes_touched.saturating_add(v.len() as u64);
                    map.insert(k.clone(), v.clone());
                }
                None => {
                    map.remove(k);
                }
            }
        }
        let overlay_gas = overlay_scan_gas(entries_examined, bytes_touched);
        let mut records: Vec<Record> = map.into_iter().collect();
        if matches!(order, Order::Descending) {
            records.reverse();
        }
        (Ok(records), cost, overlay_gas.saturating_add(bound_gas))
    }

    /// The number of pending (uncommitted) overlay writes. Zero after a `commit` or `discard`.
    pub fn pending_writes(&self) -> usize {
        self.overlay.len()
    }

    /// The number of live iterators. Zero after a `commit` or `discard` (a call boundary), so an
    /// abandoned scan cannot leave a range snapshot resident across calls.
    pub fn iterator_count(&self) -> usize {
        self.iterators.len()
    }

    /// Deterministic gas for ONE probe of the pending overlay (a `BTreeMap`): a base plus the
    /// worst-case key-comparison work, which is O(log N) comparisons of up to `key_len` bytes each.
    /// Charged on every overlay touch (hit AND miss, set, remove), so a large pending overlay does not
    /// make each point operation's overlay work free.
    fn overlay_lookup_gas(&self, key_len: usize) -> u64 {
        // depth ~= ceil(log2(len+1)): the number of comparisons a balanced-tree probe performs.
        let depth = (u64::BITS - (self.overlay.len() as u64).leading_zeros()) as u64;
        OVERLAY_READ_BASE.saturating_add(
            depth
                .saturating_mul(key_len as u64)
                .saturating_mul(GAS_PER_BYTE),
        )
    }

    /// Discard every pending write, rolling the overlay back to the last committed state. This is the
    /// explicit rollback a failed call takes instead of `commit`. It is also a call boundary, so it
    /// drops any iterators opened during the call (an abandoned scan must not leave a range snapshot
    /// resident across calls).
    pub fn discard(&mut self) {
        self.iterators.clear();
        self.overlay.clear();
    }

    /// Run `f` as an atomic unit against the overlay: if it returns `Err`, every overlay write it
    /// made is rolled back to the pre-call state, so a failed unit leaves no partial writes in the
    /// overlay. This is the per-call rollback primitive when several units share one storage before a
    /// single `commit`.
    pub fn atomic<T, E>(&mut self, f: impl FnOnce(&mut Self) -> Result<T, E>) -> Result<T, E> {
        let checkpoint = self.overlay.clone();
        match f(self) {
            Ok(v) => Ok(v),
            Err(e) => {
                self.overlay = checkpoint;
                Err(e)
            }
        }
    }

    /// Insert/delete overlay entries into `tx`, accumulating the REAL GroveDB gas, and STOP as soon as
    /// the accumulated cost exceeds `budget`, so a tiny budget cannot force the whole density-dependent
    /// flush before rejection. On any error (a real GroveDB failure OR budget exhaustion) returns
    /// `Err((message, consumed_gas))` so the caller can debit the gas the partial flush actually cost;
    /// `Ok(total)` on a full flush. Does not commit `tx`.
    fn flush_overlay_into(&self, tx: &grovedb::Transaction, budget: u64) -> Result<u64, (String, u64)> {
        let path = self.subtree();
        let mut durable_gas: u64 = 0;
        for (k, pending) in &self.overlay {
            match pending {
                Some(v) => {
                    let cc = self.db.insert(
                        path.as_slice(),
                        k,
                        Element::new_item(v.clone()),
                        None,
                        Some(tx),
                        gv(),
                    );
                    // Checked: a true total that overflows u64 must REJECT, not saturate to u64::MAX
                    // (which would then never exceed a u64::MAX budget and let the loop continue).
                    durable_gas = match durable_gas.checked_add(cost_to_gas(&cc.cost)) {
                        Some(g) => g,
                        None => return Err(("durable write cost overflows u64".to_string(), u64::MAX)),
                    };
                    if let Err(e) = cc.value {
                        return Err((format!("insert during commit: {e}"), durable_gas));
                    }
                }
                None => {
                    let cc = self.db.delete(path.as_slice(), k, None, Some(tx), gv());
                    durable_gas = match durable_gas.checked_add(cost_to_gas(&cc.cost)) {
                        Some(g) => g,
                        None => return Err(("durable write cost overflows u64".to_string(), u64::MAX)),
                    };
                    // A tombstone for a key that never existed is a no-op, so ignore not-found.
                    if let Err(e) = cc.value {
                        if !is_not_found(&e) {
                            return Err((format!("delete during commit: {e}"), durable_gas));
                        }
                    }
                }
            }
            // Stop the moment the budget is exhausted, before doing more density-dependent work.
            if durable_gas > budget {
                return Err((
                    format!("durable write cost exceeds budget {budget} (consumed {durable_gas} so far)"),
                    durable_gas,
                ));
            }
        }
        Ok(durable_gas)
    }

    /// Flush the overlay to GroveDB inside one transaction, committing only if every write applies,
    /// then CLEAR the overlay so the now-durable writes are not re-applied or re-read as pending by a
    /// later call. Call this ONCE after a successful contract call. Returns the TOTAL measured
    /// GroveDB gas of the durable inserts/deletes plus the transaction commit.
    ///
    /// This MEASURES and returns the durable cost; it does NOT itself enforce a budget. Use
    /// `commit_within_budget` to reject a commit whose durable cost exceeds a remaining gas budget
    /// (rolling it back before it becomes durable). Skipping `commit` does NOT by itself roll back:
    /// the overlay stays PENDING until `discard`, a drop of the storage, or a later successful
    /// `commit`. A failed call must therefore `discard` (or drop the storage) to roll back.
    pub fn commit(&mut self) -> Result<u64, String> {
        // A commit is a call boundary: drop any iterators opened during the call so an abandoned scan
        // cannot leave a range snapshot resident across calls (cleared up front so every exit path,
        // including an error return below, drops them).
        self.iterators.clear();
        let tx = self.db.start_transaction();
        let mut durable_gas = self.flush_overlay_into(&tx, u64::MAX).map_err(|(m, _)| m)?;
        let cc = self.db.commit_transaction(tx);
        durable_gas = durable_gas.saturating_add(cost_to_gas(&cc.cost));
        cc.value.map_err(|e| format!("commit transaction: {e}"))?;
        // The overlay is now durable in GroveDB; clear it so subsequent reads come from the store and
        // a subsequent commit does not re-apply these writes.
        self.overlay.clear();
        Ok(durable_gas)
    }

    /// Commit ONLY if the durable insert/delete cost fits `budget`, ENFORCING the gas rather than just
    /// measuring it. The overlay is flushed into an uncommitted transaction that STOPS the moment the
    /// budget is exhausted (so a tiny budget cannot force the whole density-dependent flush); on
    /// exhaustion the transaction is dropped (rolled back) and the overlay discarded, and an error
    /// carrying the CONSUMED gas is returned, so an unaffordable set of writes never becomes durable
    /// and the caller can still debit the partial work. Otherwise the transaction is committed and the
    /// total durable gas is returned. (The `commit_transaction` cost cannot be measured without
    /// committing, so enforcement is on the dominant insert/delete cost.)
    pub fn commit_within_budget(&mut self, budget: u64) -> Result<u64, (String, u64)> {
        self.iterators.clear();
        let tx = self.db.start_transaction();
        let durable_gas = match self.flush_overlay_into(&tx, budget) {
            Ok(g) => g,
            Err(e) => {
                drop(tx);
                self.overlay.clear();
                return Err(e);
            }
        };
        let cc = self.db.commit_transaction(tx);
        let total = durable_gas.saturating_add(cost_to_gas(&cc.cost));
        if let Err(e) = cc.value {
            self.overlay.clear();
            return Err((format!("commit transaction: {e}"), durable_gas));
        }
        self.overlay.clear();
        Ok(total)
    }
}

impl Storage for OverlayGroveStorage {
    fn get(&self, key: &[u8]) -> BackendResult<Option<Vec<u8>>> {
        // A read probes the pending overlay first (an O(log N) key comparison over the map) and, on a
        // hit, clones the pending value. Charge the overlay LOOKUP on both a hit and a miss (a miss
        // still probed the map), plus the value clone on a hit; a miss additionally pays the real
        // GroveDB read cost. No path is a flat fee independent of overlay size or value length.
        let lookup_gas = self.overlay_lookup_gas(key.len());
        if let Some(pending) = self.overlay.get(key) {
            let value_bytes = pending.as_ref().map(|v| v.len()).unwrap_or(0) as u64;
            let gas = lookup_gas.saturating_add(value_bytes.saturating_mul(GAS_PER_BYTE));
            return (Ok(pending.clone()), GasInfo::with_cost(gas));
        }
        let cc = self.db.get(self.subtree().as_slice(), key, None, gv());
        let gas = cost_to_gas(&cc.cost).saturating_add(lookup_gas);
        let result = match cc.value {
            Ok(Element::Item(bytes, _)) => Ok(Some(bytes)),
            Ok(other) => Err(BackendError::unknown(format!(
                "unexpected element type: {other:?}"
            ))),
            Err(ref e) if is_not_found(e) => Ok(None),
            Err(e) => Err(BackendError::unknown(format!("grovedb get: {e}"))),
        };
        (result, GasInfo::with_cost(gas))
    }

    fn scan(
        &mut self,
        start: Option<&[u8]>,
        end: Option<&[u8]>,
        order: Order,
    ) -> BackendResult<u32> {
        // `scan` eagerly runs the GroveDB range query AND merges the pending overlay, so it must
        // charge for BOTH the real database work and the overlay traversal/materialization, not a flat
        // fee: otherwise a contract could force a full-subtree read or a large-overlay merge for a
        // constant cost without ever consuming the iterator. Gas is the measured query cost plus the
        // deterministic overlay charge (both charged on the error path too, since the work was done).
        let (result, cost, overlay_gas) = self.effective_range(start, end, order);
        let gas = cost_to_gas(&cost).saturating_add(overlay_gas);
        let records = match result {
            Ok(r) => r,
            Err(e) => return (Err(BackendError::unknown(e)), GasInfo::with_cost(gas)),
        };
        // Allocate an id with checked wraparound so a long-running instance cannot silently reuse an
        // id that collides with a live iterator.
        let id = self.next_iterator_id;
        self.next_iterator_id = self
            .next_iterator_id
            .checked_add(1)
            .expect("iterator id space exhausted");
        self.iterators.insert(id, records.into_iter());
        (Ok(id), GasInfo::with_cost(gas))
    }

    fn next(&mut self, iterator_id: u32) -> BackendResult<Option<Record>> {
        match self.iterators.get_mut(&iterator_id) {
            Some(iter) => {
                let item = iter.next();
                if item.is_none() {
                    // Drop the exhausted iterator rather than leaking it in the map.
                    self.iterators.remove(&iterator_id);
                }
                (Ok(item), GasInfo::with_cost(500))
            }
            None => (
                Err(BackendError::iterator_does_not_exist(iterator_id)),
                GasInfo::free(),
            ),
        }
    }

    fn set(&mut self, key: &[u8], value: &[u8]) -> BackendResult<()> {
        // Provisional write-time charge (saturating): the O(log N) overlay lookup (which is a base
        // when the overlay is empty, so it does NOT cover the key), PLUS an unconditional per-byte fee
        // on the key AND value MATERIALIZED (the `key.to_vec()` / value copy always happens, even into
        // an empty overlay). This is NOT the durable cost, which is density-dependent and is realized
        // (and returned/enforced) by `commit` / `commit_within_budget`.
        let gas = self
            .overlay_lookup_gas(key.len())
            .saturating_add((key.len() as u64).saturating_mul(GAS_PER_BYTE))
            .saturating_add((value.len() as u64).saturating_mul(GAS_PER_BYTE));
        self.overlay.insert(key.to_vec(), Some(value.to_vec()));
        (Ok(()), GasInfo::with_cost(gas))
    }

    fn remove(&mut self, key: &[u8]) -> BackendResult<()> {
        // The O(log N) overlay lookup plus an UNCONDITIONAL per-byte fee on the key materialized (the
        // `key.to_vec()` copy always happens, even into an empty overlay where the lookup base alone
        // would otherwise be flat). The durable delete cost is realized at `commit`.
        let gas = self
            .overlay_lookup_gas(key.len())
            .saturating_add((key.len() as u64).saturating_mul(GAS_PER_BYTE));
        self.overlay.insert(key.to_vec(), None);
        (Ok(()), GasInfo::with_cost(gas))
    }
}

/// A minimal token modelled as balances in a GroveDB `bank` subtree, shared by the querier and
/// write-path spikes so there is ONE codec and ONE router (no per-binary copy to drift). Balances
/// are keyed `denom:address` and stored as decimal-string items. The router applies a transfer as a
/// net-delta map committed in a single GroveDB transaction, so it cannot mint on a self-transfer and
/// cannot leave a partial application if one write fails.
pub mod bank {
    use super::{gv, is_not_found, read_item};
    use grovedb::{Element, GroveDb};
    use std::collections::BTreeMap;

    pub const BANK: &[u8] = b"bank";
    pub const DENOM: &str = "udash";

    pub fn bank_key(address: &str) -> Vec<u8> {
        format!("{DENOM}:{address}").into_bytes()
    }

    /// Decode a stored balance, treating a non-UTF-8 or non-integer value as corruption (an error),
    /// never as zero.
    pub fn decode_balance(bytes: &[u8]) -> Result<u128, String> {
        let s = std::str::from_utf8(bytes).map_err(|_| "balance is not valid UTF-8 (corrupt)".to_string())?;
        s.parse::<u128>()
            .map_err(|_| format!("balance {s:?} is not a valid integer (corrupt)"))
    }

    /// Read a balance, surfacing real errors and treating only genuine absence as zero.
    pub fn read_balance(db: &GroveDb, address: &str) -> Result<u128, String> {
        match read_item(db, [BANK].as_ref(), &bank_key(address))? {
            Some(bytes) => decode_balance(&bytes),
            None => Ok(0),
        }
    }

    /// Read a balance together with its real GroveDB read cost (for gas), surfacing corruption as an
    /// error rather than a false zero. The measured gas is returned on BOTH the success and error
    /// paths, so an error answer still carries the cost of the read it performed.
    pub fn read_balance_costed(db: &GroveDb, address: &str) -> (Result<u128, String>, u64) {
        let cc = db.get([BANK].as_ref(), &bank_key(address), None, gv());
        let gas = super::cost_to_gas(&cc.cost);
        let amount = match cc.value {
            Ok(Element::Item(bytes, _)) => decode_balance(&bytes),
            Ok(other) => Err(format!("unexpected element type: {other:?}")),
            Err(ref e) if is_not_found(e) => Ok(0),
            Err(e) => Err(format!("grovedb get: {e}")),
        };
        (amount, gas)
    }

    /// The maximum number of coins one send may carry, so the per-coin loop (string clones, notes) is
    /// bounded work, not unbounded for a constant read/write set.
    pub const MAX_COINS_PER_SEND: usize = 1024;

    /// Read a balance and its cost WITHIN a transaction, so the read reflects the transaction's view
    /// and the transfer's reads and writes are all against one transaction.
    fn read_balance_costed_tx(
        db: &GroveDb,
        tx: &grovedb::Transaction,
        address: &str,
    ) -> (Result<u128, String>, u64) {
        let cc = db.get([BANK].as_ref(), &bank_key(address), Some(tx), gv());
        let gas = super::cost_to_gas(&cc.cost);
        let amount = match cc.value {
            Ok(Element::Item(bytes, _)) => decode_balance(&bytes),
            Ok(other) => Err(format!("unexpected element type: {other:?}")),
            Err(ref e) if is_not_found(e) => Ok(0),
            Err(e) => Err(format!("grovedb get: {e}")),
        };
        (amount, gas)
    }

    /// Apply a bank send of `coins` (denom, amount) from `sender` to `to_address`, ENFORCING a
    /// storage-gas `budget` and returning the notes plus the total gas charged; on any error path it
    /// returns `Err((message, consumed_gas))` so the caller can still debit the work performed.
    ///
    /// Bounds: at most `MAX_COINS_PER_SEND` coins, and each coin is charged (per-coin) BEFORE its
    /// strings are materialized, with the budget checked as it accumulates, so an arbitrarily long
    /// coin list cannot do unbounded work for a constant charge. Semantics: each debited account must
    /// cover its GROSS debit from its starting balance (so an over-balance send is rejected even when
    /// it is a self-transfer that would net to zero), the transfer is reduced to a NET delta per
    /// address (so a within-balance self-transfer cannot mint), and ALL reads and writes are performed
    /// against ONE GroveDB transaction, committed at the end: the reads reflect the transaction's view
    /// and two concurrent sends touching the same balance keys conflict at commit (optimistic
    /// concurrency), so one fails rather than both committing a stale-read overwrite. Only `DENOM`.
    pub fn route_bank_send(
        db: &GroveDb,
        sender: &str,
        to_address: &str,
        coins: &[(String, u128)],
        budget: u64,
    ) -> Result<(String, u64), (String, u64)> {
        let mut gas: u64 = 0;
        // Bound the coin count so the loop below is bounded work.
        if coins.len() > MAX_COINS_PER_SEND {
            return Err((
                format!("too many coins in one send: {} > {MAX_COINS_PER_SEND}", coins.len()),
                gas,
            ));
        }
        // 1. Accumulate total debits and credits per address (u128, no i128 cast), charging PER COIN
        //    before materializing its strings, and checking the budget as gas accumulates.
        let mut debit: BTreeMap<String, u128> = BTreeMap::new();
        let mut credit: BTreeMap<String, u128> = BTreeMap::new();
        let mut notes = Vec::new();
        for (denom, amt) in coins {
            gas = gas.saturating_add(super::request_gas(denom.len()).saturating_add(super::GAS_PER_BYTE));
            if gas > budget {
                return Err((format!("bank transfer cost exceeds budget {budget}"), gas));
            }
            if denom != DENOM {
                return Err((format!("router only handles denom {DENOM}, got {denom}"), gas));
            }
            let d = debit.entry(sender.to_string()).or_default();
            *d = match d.checked_add(*amt) {
                Some(v) => v,
                None => return Err(("sender debit overflow".to_string(), gas)),
            };
            let c = credit.entry(to_address.to_string()).or_default();
            *c = match c.checked_add(*amt) {
                Some(v) => v,
                None => return Err(("recipient credit overflow".to_string(), gas)),
            };
            notes.push(format!("transfer {amt} {denom} from {sender} to {to_address}"));
        }

        // All reads and writes happen against ONE transaction (for conflict detection and a consistent
        // view). Any early return below drops the transaction, rolling it back.
        let tx = db.start_transaction();

        // 2. GROSS-debit check against the STARTING balance, read within the transaction and charged.
        for (addr, gross_debit) in &debit {
            let (res, g) = read_balance_costed_tx(db, &tx, addr);
            gas = gas.saturating_add(g);
            let old = match res {
                Ok(v) => v,
                Err(e) => return Err((e, gas)),
            };
            if *gross_debit > old {
                return Err((
                    format!("insufficient funds: {addr} cannot debit {gross_debit} from balance {old}"),
                    gas,
                ));
            }
        }

        // 3. Compute every resulting balance (reads within the transaction, charged). With the
        //    gross-debit check passed, a self-transfer nets to zero and a normal transfer nets by the
        //    smaller side; neither underflows or (barring a supply near u128::MAX) overflows.
        let mut affected: std::collections::BTreeSet<&String> = std::collections::BTreeSet::new();
        affected.extend(debit.keys());
        affected.extend(credit.keys());
        let mut new_balances: Vec<(String, u128)> = Vec::new();
        for addr in affected {
            let (res, g) = read_balance_costed_tx(db, &tx, addr);
            gas = gas.saturating_add(g);
            let old = match res {
                Ok(v) => v,
                Err(e) => return Err((e, gas)),
            };
            let d = debit.get(addr).copied().unwrap_or(0);
            let c = credit.get(addr).copied().unwrap_or(0);
            let new = if c >= d {
                match old.checked_add(c - d) {
                    Some(v) => v,
                    None => return Err((format!("balance overflow for {addr}"), gas)),
                }
            } else {
                match old.checked_sub(d - c) {
                    Some(v) => v,
                    None => return Err((format!("insufficient funds: {addr} balance would go negative"), gas)),
                }
            };
            new_balances.push((addr.clone(), new));
        }

        // 4. Stage the resulting balances into the transaction, accumulating the REAL write cost and
        //    stopping the moment the budget is exhausted (bounded work), then commit.
        for (addr, bal) in &new_balances {
            let cc = db.insert(
                [BANK].as_ref(),
                &bank_key(addr),
                Element::new_item(bal.to_string().into_bytes()),
                None,
                Some(&tx),
                gv(),
            );
            gas = gas.saturating_add(super::cost_to_gas(&cc.cost));
            if let Err(e) = cc.value {
                return Err((format!("insert balance: {e}"), gas));
            }
            if gas > budget {
                return Err((format!("bank transfer cost {gas} exceeds budget {budget}; rolled back"), gas));
            }
        }
        let cc = db.commit_transaction(tx);
        gas = gas.saturating_add(super::cost_to_gas(&cc.cost));
        if let Err(e) = cc.value {
            // A commit conflict (a concurrent send touched the same keys) or other commit error.
            return Err((format!("commit bank transfer: {e}"), gas));
        }
        Ok((notes.join("; "), gas))
    }
}

#[cfg(test)]
mod tests {
    use super::bank;
    use super::*;
    use grovedb::{Element, GroveDb};

    fn open() -> (tempfile::TempDir, Arc<GroveDb>) {
        let tmp = tempfile::TempDir::new().unwrap();
        let db = Arc::new(GroveDb::open(tmp.path()).unwrap());
        (tmp, db)
    }

    fn seed_bank(db: &GroveDb) {
        db.insert(&([] as [&[u8]; 0]), bank::BANK, Element::empty_tree(), None, None, gv())
            .unwrap()
            .expect("bank tree");
    }

    #[test]
    fn commit_makes_writes_durable_and_clears_the_overlay() {
        let (_t, db) = open();
        db.insert(&([] as [&[u8]; 0]), b"c", Element::empty_tree(), None, None, gv())
            .unwrap()
            .expect("subtree");
        let mut s = OverlayGroveStorage::new(db.clone(), vec![b"c".to_vec()]);
        s.set(b"k", b"v").0.expect("set");
        assert_eq!(s.pending_writes(), 1, "the write is pending in the overlay");
        s.commit().expect("commit");
        assert_eq!(s.pending_writes(), 0, "commit clears the overlay");
        // The value is now durable and read from GroveDB (not the overlay).
        assert_eq!(s.get(b"k").0.expect("get"), Some(b"v".to_vec()), "committed value reads back");
        // A second commit with nothing pending is a no-op and leaves the root unchanged.
        let root_before = db.root_hash(None, gv()).unwrap().expect("root");
        s.commit().expect("empty commit");
        assert_eq!(db.root_hash(None, gv()).unwrap().expect("root"), root_before, "empty commit is a no-op");
    }

    #[test]
    fn uncommitted_overlay_rolls_back_on_discard_and_drop() {
        let (_t, db) = open();
        db.insert(&([] as [&[u8]; 0]), b"c", Element::empty_tree(), None, None, gv())
            .unwrap()
            .expect("subtree");
        // A committed baseline.
        let mut s = OverlayGroveStorage::new(db.clone(), vec![b"c".to_vec()]);
        s.set(b"k", b"1").0.expect("set");
        s.commit().expect("commit");
        // A new write that is discarded, not committed, leaves the baseline untouched.
        s.set(b"k", b"2").0.expect("set");
        s.discard();
        assert_eq!(s.pending_writes(), 0, "discard clears pending writes");
        assert_eq!(s.get(b"k").0.expect("get"), Some(b"1".to_vec()), "discard rolled back to the committed value");
    }

    #[test]
    fn atomic_unit_rolls_back_its_writes_on_error() {
        let (_t, db) = open();
        db.insert(&([] as [&[u8]; 0]), b"c", Element::empty_tree(), None, None, gv())
            .unwrap()
            .expect("subtree");
        let mut s = OverlayGroveStorage::new(db.clone(), vec![b"c".to_vec()]);
        let r: Result<(), &str> = s.atomic(|s| {
            s.set(b"x", b"1").0.expect("set");
            Err("unit failed after a partial write")
        });
        assert!(r.is_err(), "the atomic unit reports its error");
        assert!(s.get(b"x").0.expect("get").is_none(), "the failed unit's write was rolled back");
        assert_eq!(s.pending_writes(), 0, "no pending writes remain after rollback");
    }

    #[test]
    fn self_transfer_does_not_mint() {
        let (_t, db) = open();
        seed_bank(&db);
        let tx = db.start_transaction();
        db.insert([bank::BANK].as_ref(), &bank::bank_key("alice"), Element::new_item(b"100".to_vec()), None, Some(&tx), gv())
            .unwrap()
            .expect("seed alice");
        db.commit_transaction(tx).unwrap().expect("commit seed");
        // A self-transfer: alice sends 40 to alice. The net delta is zero, so the balance is unchanged
        // and nothing is minted.
        let (note, _gas) = bank::route_bank_send(&db, "alice", "alice", &[(bank::DENOM.to_string(), 40)], u64::MAX).expect("route");
        assert!(note.contains("transfer 40"), "the note records the attempted transfer");
        assert_eq!(bank::read_balance(&db, "alice").unwrap(), 100, "a self-transfer mints nothing");
    }

    #[test]
    fn transfer_conserves_supply_and_rejects_overspend() {
        let (_t, db) = open();
        seed_bank(&db);
        let tx = db.start_transaction();
        db.insert([bank::BANK].as_ref(), &bank::bank_key("alice"), Element::new_item(b"100".to_vec()), None, Some(&tx), gv())
            .unwrap()
            .expect("seed alice");
        db.commit_transaction(tx).unwrap().expect("commit seed");
        // A normal transfer conserves total supply.
        bank::route_bank_send(&db, "alice", "bob", &[(bank::DENOM.to_string(), 30)], u64::MAX).expect("route");
        assert_eq!(bank::read_balance(&db, "alice").unwrap(), 70);
        assert_eq!(bank::read_balance(&db, "bob").unwrap(), 30);
        assert_eq!(
            bank::read_balance(&db, "alice").unwrap() + bank::read_balance(&db, "bob").unwrap(),
            100,
            "total supply is conserved"
        );
        // An overspend is refused and applies nothing (atomic).
        let before_alice = bank::read_balance(&db, "alice").unwrap();
        let before_bob = bank::read_balance(&db, "bob").unwrap();
        assert!(bank::route_bank_send(&db, "alice", "bob", &[(bank::DENOM.to_string(), 999)], u64::MAX).is_err());
        assert_eq!(bank::read_balance(&db, "alice").unwrap(), before_alice, "a refused transfer changes nothing");
        assert_eq!(bank::read_balance(&db, "bob").unwrap(), before_bob, "a refused transfer changes nothing");
    }

    #[test]
    fn large_balances_are_not_corrupted_by_the_router() {
        let (_t, db) = open();
        seed_bank(&db);
        // A recipient balance above i128::MAX, which an `as i128` cast would misread as negative and
        // silently zero on the next credit.
        let big = i128::MAX as u128 + 1000;
        let tx = db.start_transaction();
        db.insert([bank::BANK].as_ref(), &bank::bank_key("whale"), Element::new_item(big.to_string().into_bytes()), None, Some(&tx), gv())
            .unwrap()
            .expect("seed whale");
        db.insert([bank::BANK].as_ref(), &bank::bank_key("alice"), Element::new_item(b"5".to_vec()), None, Some(&tx), gv())
            .unwrap()
            .expect("seed alice");
        db.commit_transaction(tx).unwrap().expect("commit seed");
        bank::route_bank_send(&db, "alice", "whale", &[(bank::DENOM.to_string(), 5)], u64::MAX).expect("route");
        assert_eq!(
            bank::read_balance(&db, "whale").unwrap(),
            big + 5,
            "a balance above i128::MAX grows by exactly the transferred amount, not corrupted to zero"
        );
        assert_eq!(bank::read_balance(&db, "alice").unwrap(), 0);
    }

    #[test]
    fn abandoned_scan_gas_tracks_range_size() {
        let (_t, db) = open();
        db.insert(&([] as [&[u8]; 0]), b"c", Element::empty_tree(), None, None, gv())
            .unwrap()
            .expect("subtree");
        let mut s = OverlayGroveStorage::new(db.clone(), vec![b"c".to_vec()]);
        for i in 0..60u8 {
            s.set(&[i], &[i; 8]).0.expect("set");
        }
        s.commit().expect("commit");
        // scan charges the REAL GroveDB query cost, so a larger range costs more gas than a smaller
        // one EVEN IF the iterator is never consumed (the query already ran). A flat per-scan fee
        // would make these equal and let a contract force a full read cheaply.
        let (_small_id, g_small) = s.scan(Some(&[0]), Some(&[5]), Order::Ascending);
        let (_large_id, g_large) = s.scan(Some(&[0]), Some(&[60]), Order::Ascending);
        assert!(g_small.cost > 0, "even a small scan charges the real (nonzero) query cost");
        assert!(
            g_large.cost > g_small.cost,
            "a larger abandoned scan costs more gas than a smaller one (got {} vs {})",
            g_large.cost,
            g_small.cost
        );
    }

    #[test]
    fn scan_gas_charges_uncommitted_overlay_work() {
        let (_t, db) = open();
        db.insert(&([] as [&[u8]; 0]), b"c", Element::empty_tree(), None, None, gv())
            .unwrap()
            .expect("subtree");
        let mut s = OverlayGroveStorage::new(db.clone(), vec![b"c".to_vec()]);
        // The committed store is empty, so the GroveDB query cost is ~constant; the gas difference is
        // the overlay merge. A larger UNCOMMITTED overlay must cost more, even if the scan is never
        // consumed.
        for i in 0..5u8 {
            s.set(&[i], &[i; 8]).0.expect("set");
        }
        let (_small_id, g_small) = s.scan(None, None, Order::Ascending);
        for i in 5..40u8 {
            s.set(&[i], &[i; 8]).0.expect("set");
        }
        let (_large_id, g_large) = s.scan(None, None, Order::Ascending);
        assert!(
            g_large.cost > g_small.cost,
            "a scan over a larger uncommitted overlay costs more gas (got {} vs {})",
            g_large.cost,
            g_small.cost
        );
    }

    #[test]
    fn overlay_read_gas_tracks_value_size() {
        let (_t, db) = open();
        db.insert(&([] as [&[u8]; 0]), b"c", Element::empty_tree(), None, None, gv())
            .unwrap()
            .expect("subtree");
        let mut s = OverlayGroveStorage::new(db.clone(), vec![b"c".to_vec()]);
        s.set(b"small", &[0u8; 4]).0.expect("set small");
        s.set(b"big", &[0u8; 4000]).0.expect("set big");
        let (_r1, g_small) = s.get(b"small");
        let (_r2, g_big) = s.get(b"big");
        assert!(
            g_big.cost > g_small.cost,
            "reading a larger pending overlay value costs more gas (got {} vs {})",
            g_big.cost,
            g_small.cost
        );
    }

    #[test]
    fn commit_gas_reflects_the_durable_grovedb_cost() {
        // Committing a write into a LARGER existing tree costs more durable gas than into a small one,
        // because commit now charges the real density-dependent GroveDB insert cost, not a flat fee.
        let commit_one_more = |seed: u64| -> u64 {
            let (_t, db) = open();
            db.insert(&([] as [&[u8]; 0]), b"c", Element::empty_tree(), None, None, gv())
                .unwrap()
                .expect("subtree");
            {
                let mut s = OverlayGroveStorage::new(db.clone(), vec![b"c".to_vec()]);
                for i in 0..seed {
                    s.set(&i.to_be_bytes(), b"v").0.expect("seed set");
                }
                s.commit().expect("seed commit");
            }
            let mut s = OverlayGroveStorage::new(db.clone(), vec![b"c".to_vec()]);
            s.set(b"zzzzzzzz", b"v").0.expect("set");
            s.commit().expect("commit")
        };
        let g_small = commit_one_more(2);
        let g_large = commit_one_more(500);
        assert!(g_small > 0, "commit charges a nonzero durable GroveDB cost");
        assert!(
            g_large > g_small,
            "committing into a larger tree costs more durable gas (got {g_large} vs {g_small})"
        );
    }

    #[test]
    fn scan_charges_out_of_range_overlay_keys() {
        let (_t, db) = open();
        db.insert(&([] as [&[u8]; 0]), b"c", Element::empty_tree(), None, None, gv())
            .unwrap()
            .expect("subtree");
        let mut s = OverlayGroveStorage::new(db.clone(), vec![b"c".to_vec()]);
        // A narrow scan range [0,1), but a pending overlay of keys all OUTSIDE it (>= 10). None are
        // materialized, yet every key is compared against the bounds, so more of them costs more gas.
        for i in 10u8..15 {
            s.set(&[i], &[i]).0.expect("set");
        }
        let (_small_id, g_small) = s.scan(Some(&[0]), Some(&[1]), Order::Ascending);
        for i in 15u8..60 {
            s.set(&[i], &[i]).0.expect("set");
        }
        let (_large_id, g_large) = s.scan(Some(&[0]), Some(&[1]), Order::Ascending);
        assert!(
            g_large.cost > g_small.cost,
            "a scan over more out-of-range overlay keys costs more gas (got {} vs {})",
            g_large.cost,
            g_small.cost
        );
    }

    #[test]
    fn abandoned_iterators_are_cleared_at_call_boundaries() {
        let (_t, db) = open();
        db.insert(&([] as [&[u8]; 0]), b"c", Element::empty_tree(), None, None, gv())
            .unwrap()
            .expect("subtree");
        let mut s = OverlayGroveStorage::new(db.clone(), vec![b"c".to_vec()]);
        s.set(b"a", b"1").0.expect("set");
        s.commit().expect("commit");
        // Open several scans without consuming them: they stay resident within the call.
        for _ in 0..5 {
            let _ = s.scan(None, None, Order::Ascending);
        }
        assert_eq!(s.iterator_count(), 5, "abandoned scans leave iterators resident within a call");
        s.commit().expect("commit");
        assert_eq!(s.iterator_count(), 0, "commit (a call boundary) drops abandoned iterators");
        let _ = s.scan(None, None, Order::Ascending);
        assert_eq!(s.iterator_count(), 1);
        s.discard();
        assert_eq!(s.iterator_count(), 0, "discard (a call boundary) drops abandoned iterators");
    }

    #[test]
    fn over_balance_self_transfer_is_rejected() {
        let (_t, db) = open();
        seed_bank(&db);
        let tx = db.start_transaction();
        db.insert([bank::BANK].as_ref(), &bank::bank_key("alice"), Element::new_item(b"100".to_vec()), None, Some(&tx), gv())
            .unwrap()
            .expect("seed alice");
        db.commit_transaction(tx).unwrap().expect("commit seed");
        // A self-send of 101 from a balance of 100 must be REJECTED (you must hold funds you send,
        // even to yourself), leaving the balance unchanged, NOT accepted because the net delta is zero.
        assert!(
            bank::route_bank_send(&db, "alice", "alice", &[(bank::DENOM.to_string(), 101)], u64::MAX).is_err(),
            "an over-balance self-transfer is rejected"
        );
        assert_eq!(bank::read_balance(&db, "alice").unwrap(), 100, "the rejected self-transfer left the balance unchanged");
        // A within-balance self-transfer still succeeds and mints nothing.
        assert!(bank::route_bank_send(&db, "alice", "alice", &[(bank::DENOM.to_string(), 40)], u64::MAX).is_ok());
        assert_eq!(bank::read_balance(&db, "alice").unwrap(), 100, "a within-balance self-transfer mints nothing");
    }

    #[test]
    fn commit_within_budget_enforces_and_rolls_back() {
        let (_t, db) = open();
        db.insert(&([] as [&[u8]; 0]), b"c", Element::empty_tree(), None, None, gv())
            .unwrap()
            .expect("subtree");
        // A budget of 0 cannot afford any durable write: it is rejected and rolled back.
        let mut s = OverlayGroveStorage::new(db.clone(), vec![b"c".to_vec()]);
        s.set(b"k", b"v").0.expect("set");
        assert!(s.commit_within_budget(0).is_err(), "an unaffordable commit is rejected");
        assert_eq!(s.pending_writes(), 0, "the rejected commit discarded the overlay (rolled back)");
        assert!(s.get(b"k").0.expect("get").is_none(), "the rejected write never became durable");
        // With a generous budget the same write commits and returns the durable gas charged.
        let mut s = OverlayGroveStorage::new(db.clone(), vec![b"c".to_vec()]);
        s.set(b"k", b"v").0.expect("set");
        let gas = s.commit_within_budget(u64::MAX).expect("affordable commit succeeds");
        assert!(gas > 0, "commit_within_budget returns the durable gas charged");
        assert_eq!(s.get(b"k").0.expect("get"), Some(b"v".to_vec()), "the affordable write is durable");
    }

    #[test]
    fn overlay_miss_charges_a_lookup_that_grows_with_overlay_size() {
        let (_t, db) = open();
        db.insert(&([] as [&[u8]; 0]), b"c", Element::empty_tree(), None, None, gv())
            .unwrap()
            .expect("subtree");
        let miss_key = b"zzzzzzzzzzzzzzzz"; // absent from overlay AND the (empty) committed tree
        let mut small = OverlayGroveStorage::new(db.clone(), vec![b"c".to_vec()]);
        small.set(b"a", b"1").0.expect("set");
        let (_r, g_small) = small.get(miss_key);
        let mut large = OverlayGroveStorage::new(db.clone(), vec![b"c".to_vec()]);
        for i in 0..2000u16 {
            large.set(&i.to_be_bytes(), b"1").0.expect("set");
        }
        let (_r2, g_large) = large.get(miss_key);
        assert!(
            g_large.cost > g_small.cost,
            "an overlay MISS over a larger pending overlay costs more (O(log N) probe) (got {} vs {})",
            g_large.cost,
            g_small.cost
        );
    }

    #[test]
    fn bank_router_enforces_its_own_budget() {
        let (_t, db) = open();
        seed_bank(&db);
        let tx = db.start_transaction();
        db.insert([bank::BANK].as_ref(), &bank::bank_key("alice"), Element::new_item(b"100".to_vec()), None, Some(&tx), gv())
            .unwrap()
            .expect("seed alice");
        db.commit_transaction(tx).unwrap().expect("commit seed");
        // A budget of 0 cannot afford the transfer's own reads+writes: rejected, and rolled back.
        assert!(
            bank::route_bank_send(&db, "alice", "bob", &[(bank::DENOM.to_string(), 30)], 0).is_err(),
            "an over-budget bank transfer is rejected by the router"
        );
        assert_eq!(bank::read_balance(&db, "alice").unwrap(), 100, "the rejected transfer left balances unchanged");
        assert_eq!(bank::read_balance(&db, "bob").unwrap(), 0);
        // A generous budget commits and returns the gas the router charged.
        let (_note, gas) =
            bank::route_bank_send(&db, "alice", "bob", &[(bank::DENOM.to_string(), 30)], u64::MAX).expect("route");
        assert!(gas > 0, "the router returns the storage gas it charged");
        assert_eq!(bank::read_balance(&db, "bob").unwrap(), 30, "the within-budget transfer applied");
    }

    #[test]
    fn bank_router_bounds_coins_and_charges_per_coin() {
        let (_t, db) = open();
        seed_bank(&db);
        let tx = db.start_transaction();
        db.insert([bank::BANK].as_ref(), &bank::bank_key("alice"), Element::new_item(b"1000000".to_vec()), None, Some(&tx), gv())
            .unwrap()
            .expect("seed alice");
        db.commit_transaction(tx).unwrap().expect("commit seed");
        // A coin list over the bound is rejected outright, before any per-coin work.
        let too_many: Vec<(String, u128)> =
            (0..bank::MAX_COINS_PER_SEND + 1).map(|_| (bank::DENOM.to_string(), 0)).collect();
        assert!(
            bank::route_bank_send(&db, "alice", "bob", &too_many, u64::MAX).is_err(),
            "an over-long coin list is rejected"
        );
        // A long list of zero-value coins (constant reads/writes) is charged PER COIN, so a tiny
        // budget rejects it before doing the whole loop.
        let long: Vec<(String, u128)> = (0..200).map(|_| (bank::DENOM.to_string(), 0)).collect();
        assert!(
            bank::route_bank_send(&db, "alice", "bob", &long, 1).is_err(),
            "a per-coin charge over a tiny budget is rejected"
        );
    }

    #[test]
    fn wrong_denom_is_refused() {
        let (_t, db) = open();
        seed_bank(&db);
        assert!(
            bank::route_bank_send(&db, "alice", "bob", &[("bitcoin".to_string(), 1)], u64::MAX).is_err(),
            "the router refuses an unknown denom"
        );
    }
}
