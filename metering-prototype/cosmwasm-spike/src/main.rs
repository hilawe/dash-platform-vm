//! The CosmWasm-on-GroveDB spike.
//!
//! This is the "recommended next artifact" from docs/COSMWASM_STORAGE_ASSESSMENT.md: a running
//! end-to-end slice that turns the interface-level assessment into execution-produced evidence. It
//! implements the REAL `cosmwasm_std::Storage` trait over a GroveDB subtree, drives it through the
//! REAL `cw-storage-plus` library (the storage layer every CosmWasm contract uses), exercising
//! get, set, remove, and ordered range scans in both directions with bounds, all inside a GroveDB
//! transaction, and then proves a piece of the resulting contract state with GroveDB `prove_query`
//! and verifies the proof against the live root hash.
//!
//! Scope. This deliberately does not boot the wasmer VM or run a compiled Wasm contract, because
//! that would exercise cosmwasm-vm's contract-calling path, which every Cosmos chain already
//! proves, rather than the GroveDB-backing question, which is the uncertain part. What it does
//! exercise is the exact `cosmwasm_std::Storage` trait (so the trait shape is compiler-verified
//! against the real crate) and real cw-storage-plus `Map` operations over GroveDB.

use cosmwasm_std::{Order, Record, Storage};
use cw_storage_plus::{Bound, Map};
use grovedb::query_result_type::QueryResultType;
use grovedb::{Element, GroveDb, PathQuery, Query, SizedQuery, Transaction};
use grovedb_version::version::GroveVersion;

fn gv() -> &'static GroveVersion {
    GroveVersion::latest()
}

/// True for the GroveDB errors that genuinely mean the key/path is absent, as opposed to corruption
/// or operational failure (which must not be conflated with absence).
fn is_not_found(e: &grovedb::Error) -> bool {
    matches!(
        e,
        grovedb::Error::PathKeyNotFound(_)
            | grovedb::Error::PathNotFound(_)
            | grovedb::Error::PathParentLayerNotFound(_)
    )
}

/// The root subtree path, typed so the byte element is inferable at root insert sites.
const ROOT: &[&[u8]] = &[];
const CONTRACTS: &[u8] = b"contracts";
const CONTRACT_ID: &[u8] = b"demo_contract";

/// A `cosmwasm_std::Storage` implementation backed by a GroveDB subtree within a transaction.
/// Each contract's key-value space is one GroveDB subtree (here `[contracts, demo_contract]`),
/// which gives per-contract isolation for free.
struct GroveStorage<'db: 'a, 'a> {
    db: &'a GroveDb,
    tx: &'a Transaction<'db>,
    path: Vec<Vec<u8>>,
}

impl<'db: 'a, 'a> GroveStorage<'db, 'a> {
    fn subtree(&self) -> Vec<&[u8]> {
        self.path.iter().map(|v| v.as_slice()).collect()
    }
}

impl<'db: 'a, 'a> Storage for GroveStorage<'db, 'a> {
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        let path = self.subtree();
        // The contract-facing trait is infallible (returns Option), so a genuine not-found reads as
        // None, but any other GroveDB error (corruption, wrong element type) is surfaced by a panic
        // rather than silently masquerading as an empty result.
        match self.db.get(path.as_slice(), key, Some(self.tx), gv()).unwrap() {
            Ok(Element::Item(bytes, _)) => Some(bytes),
            Ok(other) => panic!("unexpected element type in contract storage: {other:?}"),
            Err(ref e) if is_not_found(e) => None,
            Err(e) => panic!("grovedb get error (not absence): {e}"),
        }
    }

    fn set(&mut self, key: &[u8], value: &[u8]) {
        let path = self.subtree();
        self.db
            .insert(
                path.as_slice(),
                key,
                Element::new_item(value.to_vec()),
                None,
                Some(self.tx),
                gv(),
            )
            .unwrap()
            .expect("grovedb insert");
    }

    fn remove(&mut self, key: &[u8]) {
        let path = self.subtree();
        // A remove of a missing key is a no-op for CosmWasm, so ignore ONLY a genuine not-found; any
        // other GroveDB error (corruption, operational failure) is surfaced rather than swallowed.
        match self.db.delete(path.as_slice(), key, None, Some(self.tx), gv()).unwrap() {
            Ok(()) => {}
            Err(ref e) if is_not_found(e) => {}
            Err(e) => panic!("grovedb delete error (not absence): {e}"),
        }
    }

    fn range<'b>(
        &'b self,
        start: Option<&[u8]>,
        end: Option<&[u8]>,
        order: Order,
    ) -> Box<dyn Iterator<Item = Record> + 'b> {
        // Query GroveDB in ascending key order and reverse in the adapter for a descending
        // request. This yields correct CosmWasm order (start inclusive, end exclusive) without
        // depending on GroveDB's native descending traversal of a full range, and it is correct
        // here because the adapter uses no query limit, so direction does not change the set.
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
        let (elements, _) = self
            .db
            .query(
                &pq,
                false,
                false,
                true,
                QueryResultType::QueryKeyElementPairResultType,
                Some(self.tx),
                gv(),
            )
            .unwrap()
            .expect("grovedb range query");
        let mut records: Vec<Record> = elements
            .to_key_elements()
            .into_iter()
            .map(|(k, el)| match el {
                Element::Item(bytes, _) => (k, bytes),
                // A non-item element in a flat contract subtree is unexpected; surface it rather than
                // silently dropping it, so corruption cannot masquerade as a short range.
                other => panic!("unexpected non-item element in contract storage range: {other:?}"),
            })
            .collect();
        if matches!(order, Order::Descending) {
            records.reverse();
        }
        Box::new(records.into_iter())
    }
}

/// The contract-level state we exercise: a balances map keyed by an account name. This is a real
/// cw-storage-plus Map, so every read and write below goes through the same code a CosmWasm
/// contract would run.
const BALANCES: Map<&str, u64> = Map::new("balances");

struct RunResult {
    root_hash: [u8; 32],
    ascending: Vec<(String, u64)>,
    descending: Vec<(String, u64)>,
    bounded: Vec<(String, u64)>,
    proven_count: usize,
    proven_alice: u64,
}

fn contract_path() -> Vec<Vec<u8>> {
    vec![CONTRACTS.to_vec(), CONTRACT_ID.to_vec()]
}

fn run_once() -> RunResult {
    let tmp = tempfile::TempDir::new().unwrap();
    let db = GroveDb::open(tmp.path()).unwrap();
    let tx = db.start_transaction();

    // Create the contracts tree and this contract's subtree inside the transaction.
    db.insert(ROOT, CONTRACTS, Element::empty_tree(), None, Some(&tx), gv())
        .unwrap()
        .expect("create contracts tree");
    db.insert(
        [CONTRACTS].as_ref(),
        CONTRACT_ID,
        Element::empty_tree(),
        None,
        Some(&tx),
        gv(),
    )
    .unwrap()
    .expect("create contract subtree");

    let ascending;
    let descending;
    let bounded;
    {
        let mut store = GroveStorage {
            db: &db,
            tx: &tx,
            path: contract_path(),
        };

        // set (through cw-storage-plus save).
        BALANCES.save(&mut store, "alice", &100u64).expect("save alice");
        BALANCES.save(&mut store, "bob", &50u64).expect("save bob");
        BALANCES.save(&mut store, "carol", &75u64).expect("save carol");
        BALANCES.save(&mut store, "dave", &25u64).expect("save dave");

        // get (read-your-writes inside the transaction).
        let alice = BALANCES.load(&store, "alice").expect("load alice");
        assert_eq!(alice, 100, "read-your-writes inside the transaction");

        // remove carol, then confirm it is gone within the same transaction.
        BALANCES.remove(&mut store, "carol");
        assert!(
            BALANCES.may_load(&store, "carol").expect("may_load carol").is_none(),
            "removed key is absent within the transaction"
        );

        // ordered range, both directions, reflecting the uncommitted transaction state.
        ascending = BALANCES
            .range(&store, None, None, Order::Ascending)
            .collect::<Result<Vec<_>, _>>()
            .expect("ascending range");
        descending = BALANCES
            .range(&store, None, None, Order::Descending)
            .collect::<Result<Vec<_>, _>>()
            .expect("descending range");
        // bounded range: [bob, dave). Should yield only bob (carol was removed, dave is excluded).
        bounded = BALANCES
            .range(
                &store,
                Some(Bound::inclusive("bob")),
                Some(Bound::exclusive("dave")),
                Order::Ascending,
            )
            .collect::<Result<Vec<_>, _>>()
            .expect("bounded range");
    }

    // Commit, then prove the committed contract state.
    db.commit_transaction(tx).unwrap().expect("commit");
    let root_hash = db.root_hash(None, gv()).unwrap().expect("root hash");

    // Prove the whole contract subtree and verify the proof against the live root.
    let mut q = Query::new();
    q.insert_all();
    let pq = PathQuery {
        path: contract_path(),
        query: SizedQuery {
            query: q,
            limit: None,
            offset: None,
        },
    };
    let proof = db.prove_query(&pq, None, gv()).unwrap().expect("prove");
    let (verified_root, results) = GroveDb::verify_query(&proof, &pq, gv()).expect("verify");
    assert_eq!(
        verified_root, root_hash,
        "the proof must verify against the live committed root hash"
    );

    // Decode alice's balance out of the proven results, so the proof is shown to carry real
    // contract data, not just a matching root.
    let alice_key = BALANCES.key("alice").to_vec();
    let mut proven_alice = 0u64;
    let mut proven_count = 0usize;
    for (_path, key, maybe_el) in &results {
        if let Some(Element::Item(bytes, _)) = maybe_el {
            proven_count += 1;
            if *key == alice_key {
                proven_alice = cosmwasm_std::from_json::<u64>(bytes).expect("decode alice");
            }
        }
    }

    RunResult {
        root_hash,
        ascending,
        descending,
        bounded,
        proven_count,
        proven_alice,
    }
}

fn main() {
    println!("# CosmWasm-on-GroveDB spike: real cosmwasm_std::Storage over a GroveDB subtree,");
    println!("# driven through real cw-storage-plus, proven with GroveDB prove_query.\n");

    let r1 = run_once();
    let r2 = run_once();

    // Determinism: the whole run is a deterministic function of its inputs.
    assert_eq!(
        r1.root_hash, r2.root_hash,
        "the committed root hash must be deterministic across runs"
    );

    println!("## get / set / remove");
    println!("alice loaded as 100 (read-your-writes inside the transaction): OK");
    println!("carol removed and absent within the transaction: OK\n");

    println!("## ordered range (ascending), reflecting uncommitted transaction state");
    for (k, v) in &r1.ascending {
        println!("  {k} = {v}");
    }
    let asc_keys: Vec<&str> = r1.ascending.iter().map(|(k, _)| k.as_str()).collect();
    assert_eq!(asc_keys, vec!["alice", "bob", "dave"], "ascending order, carol removed");

    println!("\n## ordered range (descending)");
    for (k, v) in &r1.descending {
        println!("  {k} = {v}");
    }
    let desc_keys: Vec<&str> = r1.descending.iter().map(|(k, _)| k.as_str()).collect();
    assert_eq!(desc_keys, vec!["dave", "bob", "alice"], "descending is the reverse");

    println!("\n## bounded range [bob, dave)");
    for (k, v) in &r1.bounded {
        println!("  {k} = {v}");
    }
    let bounded_keys: Vec<&str> = r1.bounded.iter().map(|(k, _)| k.as_str()).collect();
    assert_eq!(bounded_keys, vec!["bob"], "start inclusive, end exclusive, carol removed");

    println!("\n## provability");
    println!("  committed root hash: {}", hex(&r1.root_hash));
    println!("  proof verified against the live root: OK");
    println!("  proven contract entries: {}", r1.proven_count);
    println!("  alice's balance decoded from the proof: {}", r1.proven_alice);
    assert_eq!(r1.proven_count, 3, "three entries remain after removing carol");
    assert_eq!(r1.proven_alice, 100, "the proof carries alice's real balance");

    println!("\n# Spike complete. Every open item from the assessment is now execution-produced:");
    println!("# get/set/remove and ordered range (both directions, with bounds) run through real");
    println!("# cw-storage-plus over GroveDB inside a transaction with read-your-writes, the result");
    println!("# is deterministic across runs, and the committed contract state is provable with");
    println!("# GroveDB prove_query, carrying real contract data that verifies against the root.");
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
