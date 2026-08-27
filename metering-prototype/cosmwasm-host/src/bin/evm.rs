//! EVM as a guest over GroveDB-backed CosmWasm storage.
//!
//! A minimal EVM interpreter compiled to a CosmWasm contract (source in
//! `metering-prototype/evm-guest-contract/`) runs inside the real cosmwasm-vm (wasmer) backed by the
//! shared overlay storage over GroveDB. EVM bytecode that does an `SSTORE` writes to the overlay,
//! which is flushed to GroveDB on success, and the exact stored slot is then proven.
//!
//! Artifact integrity, and its limit. At startup the embedded `evm_guest.wasm` is checked against
//! the committed digest in `testdata/evm_guest.wasm.sha256`. This is an INTEGRITY check that the two
//! committed artifacts (the wasm bytes and the recorded digest) agree; the build recipe regenerates
//! the digest from the freshly built wasm, so a rebuild updates both together and a wasm swapped
//! without updating its digest (or vice versa) is caught. It does NOT by itself prove the wasm was
//! compiled from the current interpreter source: if neither the wasm nor the digest is regenerated
//! after a source edit, both go stale together and the check still passes. Rebuilding the contract
//! (which rewrites both files) is what ties the artifact to the source.

use cosmwasm_host::{gv, OverlayGroveStorage};
use cosmwasm_std::Empty;
use cosmwasm_vm::testing::{mock_env, mock_info, MockApi, MockQuerier};
use cosmwasm_vm::{call_execute, call_instantiate, call_query, Backend, Instance, InstanceOptions, Storage};
use grovedb::{Element, GroveDb, PathQuery, Query, SizedQuery};
use std::sync::Arc;

const ROOT: &[&[u8]] = &[];
const CONTRACTS: &[u8] = b"contracts";
const CONTRACT_ID: &[u8] = b"evm_guest";

const EVM_GUEST: &[u8] = include_bytes!("../../testdata/evm_guest.wasm");
const EVM_GUEST_SHA256: &str = include_str!("../../testdata/evm_guest.wasm.sha256");

/// A generous per-call storage-gas budget (in the storage adapter's own gas units, distinct from VM
/// gas) so ordinary small writes commit; the enforcement path (commit_within_budget) rejects a write
/// whose durable cost exceeds this. Regression (5) drives a tight budget to watch the rejection.
const STORAGE_BUDGET: u64 = 100_000_000;

#[derive(serde::Deserialize)]
struct SlotResponse {
    value: String,
}

fn contract_path() -> Vec<Vec<u8>> {
    vec![CONTRACTS.to_vec(), CONTRACT_ID.to_vec()]
}

fn main() {
    println!("# EVM as a guest over GroveDB. A minimal EVM interpreter, compiled as a CosmWasm");
    println!("# contract, runs EVM bytecode whose SSTORE lands in GroveDB, then it is proven.\n");

    // Integrity check (not a build-from-source proof, see the module doc): the embedded wasm must
    // match its committed digest, so a wasm swapped without updating the digest is caught.
    let actual = sha256::digest_hex(EVM_GUEST);
    let expected = EVM_GUEST_SHA256.trim();
    assert_eq!(
        actual, expected,
        "embedded evm_guest.wasm does not match its committed digest; rebuild the contract (which rewrites both)"
    );
    println!("## the embedded EVM-guest wasm matches its committed digest {} (integrity check)", &expected[..16]);

    let tmp = tempfile::TempDir::new().unwrap();
    let db = Arc::new(GroveDb::open(tmp.path()).unwrap());
    db.insert(ROOT, CONTRACTS, Element::empty_tree(), None, None, gv())
        .unwrap()
        .expect("contracts");
    db.insert([CONTRACTS].as_ref(), CONTRACT_ID, Element::empty_tree(), None, None, gv())
        .unwrap()
        .expect("subtree");

    let storage = OverlayGroveStorage::new(db.clone(), contract_path());
    let backend = Backend {
        api: MockApi::default(),
        storage,
        querier: MockQuerier::<Empty>::new(&[]),
    };
    let mut instance = Instance::from_code(
        EVM_GUEST,
        backend,
        InstanceOptions { gas_limit: u64::MAX, print_debug: false },
        None,
    )
    .expect("compile and load the EVM-guest contract");
    println!("## the EVM-guest contract compiled and loaded into the VM (wasmer, singlepass)");

    let env = mock_env();
    call_instantiate::<_, _, _, Empty>(&mut instance, &env, &mock_info("deployer", &[]), br#"{}"#)
        .expect("instantiate")
        .into_result()
        .expect("instantiate ok");

    // EVM bytecode: PUSH1 0x2a (42), PUSH1 0x01 (slot 1), SSTORE, STOP. Stores 42 at slot 1.
    let bytecode = "602a60015500";
    let exec_msg = format!(r#"{{"run":{{"code":"{bytecode}"}}}}"#);
    let exec = call_execute::<_, _, _, Empty>(
        &mut instance,
        &env,
        &mock_info("caller", &[]),
        exec_msg.as_bytes(),
    )
    .expect("call_execute")
    .into_result()
    .expect("execute ok");
    let opcodes = exec
        .attributes
        .iter()
        .find(|a| a.key == "opcodes")
        .map(|a| a.value.clone())
        .unwrap_or_default();
    println!("## the contract executed EVM bytecode {bytecode} ({opcodes} opcodes): SSTORE 42 -> slot 1");

    let q = call_query(&mut instance, &env, br#"{"slot":{"key":1}}"#)
        .expect("call_query")
        .into_result()
        .expect("query ok");
    let slot_value: SlotResponse = cosmwasm_std::from_json(&q).expect("decode slot");
    println!("## queried slot 1 through the VM, value = 0x{}", slot_value.value);
    // Bind the read to the exact 32-byte word 42.
    let expected_word_hex = format!("{:0>64}", "2a");
    assert_eq!(slot_value.value, expected_word_hex, "slot 1 holds exactly the 32-byte word 42");

    // Flush the overlay to GroveDB on success.
    instance
        .with_storage::<_, ()>(|s| {
            s.commit_within_budget(STORAGE_BUDGET).expect("flush overlay within budget");
            Ok(())
        })
        .expect("with_storage");

    // Prove the EVM guest's storage, bound to the EXACT slot key and value: key = [0;31]+[1], value
    // = [0;31]+[0x2a], and exactly one entry.
    let mut allq = Query::new();
    allq.insert_all();
    let pq = PathQuery {
        path: contract_path(),
        query: SizedQuery { query: allq, limit: None, offset: None },
    };
    let root = db.root_hash(None, gv()).unwrap().expect("root");
    let proof = db.prove_query(&pq, None, gv()).unwrap().expect("prove");
    let (vroot, results) = GroveDb::verify_query(&proof, &pq, gv()).expect("verify");
    assert_eq!(vroot, root, "the proof verifies against the live root");
    // Require EXACTLY one proven entry and that it is an item (a non-item element is an error, not
    // silently filtered out), then bind its key and value to the exact 32-byte words.
    assert_eq!(results.len(), 1, "exactly one EVM storage slot is present");
    let (_, key, el) = &results[0];
    let val = match el {
        Some(Element::Item(v, _)) => v,
        other => panic!("the single proven slot is not an item: {other:?}"),
    };
    let mut expected_key = [0u8; 32];
    expected_key[31] = 1;
    let mut expected_val = [0u8; 32];
    expected_val[31] = 0x2a;
    assert_eq!(key.as_slice(), &expected_key, "the proven key is exactly the 32-byte slot 1");
    assert_eq!(val.as_slice(), &expected_val, "the proven value is exactly the 32-byte word 42");
    let stored = [(key.clone(), val.clone())];
    println!("## the EVM guest's storage is provable in GroveDB, bound to the exact slot and value:");
    println!("  slot 0x{} = 0x{}", hex(&stored[0].0), hex(&stored[0].1));

    // Exercise the two Ethereum-semantics behaviours the interpreter enforces, so they are watched
    // rather than merely coded.
    exercise_semantics();

    println!("\n# EVM-as-guest complete. Real EVM bytecode ran inside a Wasm guest under cosmwasm-vm,");
    println!("# its SSTORE landed in GroveDB on success, the exact stored slot is provable, a FAILED");
    println!("# call is rolled back (its write never becomes durable), and the stack limit and");
    println!("# zero-store-clears-slot semantics are exercised.");
}

/// Build a fresh EVM-guest instance over its own GroveDB, ready for instantiate/execute/query.
fn fresh_instance(db: &Arc<GroveDb>) -> Instance<MockApi, OverlayGroveStorage, MockQuerier<Empty>> {
    db.insert(ROOT, CONTRACTS, Element::empty_tree(), None, None, gv())
        .unwrap()
        .expect("contracts");
    db.insert([CONTRACTS].as_ref(), CONTRACT_ID, Element::empty_tree(), None, None, gv())
        .unwrap()
        .expect("subtree");
    let backend = Backend {
        api: MockApi::default(),
        storage: OverlayGroveStorage::new(db.clone(), contract_path()),
        querier: MockQuerier::<Empty>::new(&[]),
    };
    let mut instance = Instance::from_code(
        EVM_GUEST,
        backend,
        InstanceOptions { gas_limit: u64::MAX, print_debug: false },
        None,
    )
    .expect("load contract");
    call_instantiate::<_, _, _, Empty>(&mut instance, &mock_env(), &mock_info("d", &[]), br#"{}"#)
        .expect("instantiate")
        .into_result()
        .expect("ok");
    instance
}

/// Run one EVM program through the VM as a real CALL BOUNDARY: the overlay is committed to GroveDB
/// only on FULL success, and DISCARDED on ANY failure, so a failed call leaves no durable write.
/// Returns whether the call fully succeeded. "Failure" spans all three ways a call can fail: an outer
/// VM error or trap (`call_execute` returns `Err`), an inner contract error (`ContractResult::Err`,
/// e.g. the invalid opcode), AND a GroveDB `commit` error on the success path. All three reach
/// `discard`; none panics past the boundary. Skipping commit is not rollback, because the overlay
/// persists on the instance and a later commit would flush a failed call's writes; discarding on
/// every failure is what makes the "a failed call rolls back" claim true.
fn apply(
    instance: &mut Instance<MockApi, OverlayGroveStorage, MockQuerier<Empty>>,
    code: &str,
    storage_budget: u64,
) -> bool {
    let msg = format!(r#"{{"run":{{"code":"{code}"}}}}"#);
    let outcome = call_execute::<_, _, _, Empty>(instance, &mock_env(), &mock_info("c", &[]), msg.as_bytes());
    // Full success requires an outer Ok AND an inner Ok. An outer Err (VM trap, gas, host error) is a
    // failed call too, and must reach discard rather than panicking.
    let call_ok = match outcome {
        Ok(contract_result) => contract_result.into_result().is_ok(),
        Err(_vm_error) => false,
    };
    // The boundary returns true only on FULL success: the call succeeded AND the durable writes fit
    // the storage budget. The success path goes through `commit_within_budget`, which ENFORCES the
    // budget (rolling back if the durable cost exceeds it) rather than the unrestricted `commit`, so
    // density-expensive writes cannot become durable past the budget. Any failure (inner/outer call
    // error, over-budget, or commit error) discards rather than panicking, and reports failure.
    instance
        .with_storage::<_, bool>(|s| {
            Ok(if call_ok {
                match s.commit_within_budget(storage_budget) {
                    Ok(_durable_gas) => true,
                    Err(_) => {
                        s.discard();
                        false
                    }
                }
            } else {
                s.discard();
                false
            })
        })
        .expect("with_storage accessor")
}

fn exercise_semantics() {
    println!("\n## semantics exercises");

    // (1) Storing zero clears the slot. Store 42 at slot 1, then store 0 at slot 1, and confirm the
    // slot is absent (query reads all-zero and the proof shows no entries).
    let tmp = tempfile::TempDir::new().unwrap();
    let db = Arc::new(GroveDb::open(tmp.path()).unwrap());
    let mut inst = fresh_instance(&db);
    assert!(apply(&mut inst, "602a60015500", STORAGE_BUDGET), "store 42 at slot 1 succeeds");
    assert!(apply(&mut inst, "600060015500", STORAGE_BUDGET), "store 0 at slot 1 succeeds (clears it)");
    let q = call_query(&mut inst, &mock_env(), br#"{"slot":{"key":1}}"#)
        .expect("query")
        .into_result()
        .expect("ok");
    let sv: SlotResponse = cosmwasm_std::from_json(&q).expect("decode");
    assert_eq!(sv.value, format!("{:0>64}", ""), "a cleared slot reads as zero");
    let mut allq = Query::new();
    allq.insert_all();
    let pq = PathQuery { path: contract_path(), query: SizedQuery { query: allq, limit: None, offset: None } };
    let root = db.root_hash(None, gv()).unwrap().expect("root");
    let proof = db.prove_query(&pq, None, gv()).unwrap().expect("prove");
    let (vroot, results) = GroveDb::verify_query(&proof, &pq, gv()).expect("verify");
    // Bind the proof to the LIVE committed root, so "no element" is proven against the real state and
    // not merely a self-consistent proof.
    assert_eq!(vroot, root, "the zero-store proof verifies against the live committed root");
    // Require NO present element of any kind (an item-only count would let a stray non-item slip
    // through and still read as "no slot").
    assert!(
        !results.iter().any(|(_, _, el)| el.is_some()),
        "a cleared slot leaves no present element of any kind in GroveDB (zero-store deletes)"
    );
    println!("  storing zero clears the slot: after store-42 then store-0, GroveDB holds no slot 1");

    // (2) The 1024-element stack limit is enforced: 1025 pushes must fail the execution.
    let tmp2 = tempfile::TempDir::new().unwrap();
    let db2 = Arc::new(GroveDb::open(tmp2.path()).unwrap());
    let mut inst2 = fresh_instance(&db2);
    let overflow = "6000".repeat(1025); // 1025 * PUSH1 0
    assert!(!apply(&mut inst2, &overflow, STORAGE_BUDGET), "1025 pushes exceed the 1024 stack limit and fail");
    println!("  the 1024-element stack limit is enforced: 1025 pushes fail the execution");

    // (3) A FAILED call rolls back. A program that SSTOREs slot 5 then hits an invalid opcode fails
    // AFTER the store; the call boundary discards its overlay, so a later successful call that
    // commits leaves slot 5 absent. This is the actual rollback, watched, not just "skip commit".
    let tmp3 = tempfile::TempDir::new().unwrap();
    let db3 = Arc::new(GroveDb::open(tmp3.path()).unwrap());
    let mut inst3 = fresh_instance(&db3);
    assert!(apply(&mut inst3, "602a60015500", STORAGE_BUDGET), "a first store to slot 1 succeeds and commits");
    // PUSH1 0xbb, PUSH1 0x05, SSTORE, INVALID(0xfe): stores slot 5 = 0xbb, then fails.
    assert!(!apply(&mut inst3, "60bb600555fe", STORAGE_BUDGET), "the call that stores then hits 0xfe is reported failed");
    // A later, unrelated successful store that commits. If the failed write were still pending, this
    // commit would flush it too.
    assert!(apply(&mut inst3, "603360025500", STORAGE_BUDGET), "a later unrelated store to slot 2 succeeds and commits");
    let mut allq3 = Query::new();
    allq3.insert_all();
    let pq3 = PathQuery { path: contract_path(), query: SizedQuery { query: allq3, limit: None, offset: None } };
    let root3 = db3.root_hash(None, gv()).unwrap().expect("root");
    let proof3 = db3.prove_query(&pq3, None, gv()).unwrap().expect("prove");
    let (vroot3, results3) = GroveDb::verify_query(&proof3, &pq3, gv()).expect("verify");
    assert_eq!(vroot3, root3, "the proof verifies against the live root");
    // Collect the FULL 32-byte keys and values (not just a trailing byte, which would collapse
    // distinct slots and ignore high bytes), requiring every proven element to be an item.
    let mut present: std::collections::BTreeMap<Vec<u8>, Vec<u8>> = Default::default();
    for (_, k, el) in &results3 {
        let val = match el {
            Some(Element::Item(v, _)) => v.clone(),
            other => panic!("proven EVM slot is not an item: {other:?}"),
        };
        assert!(present.insert(k.clone(), val).is_none(), "no duplicate proven key");
    }
    // Build the exact expected state: slot 1 = 42 and slot 2 = 51 as full 32-byte words, slot 5 (the
    // failed write) absent, and NOTHING else.
    let word = |slot: u8, v: u8| {
        let mut k = [0u8; 32];
        k[31] = slot;
        let mut val = [0u8; 32];
        val[31] = v;
        (k.to_vec(), val.to_vec())
    };
    let expected: std::collections::BTreeMap<Vec<u8>, Vec<u8>> =
        [word(1, 0x2a), word(2, 0x33)].into_iter().collect();
    assert_eq!(
        present, expected,
        "exactly slots 1=42 and 2=51 are durable; slot 5 (the failed call's write) is absent"
    );
    println!("  a failed call rolls back: slot 5 was written then the call failed, and it is absent");
    println!("  in GroveDB even after a later successful commit (slots 1 and 2 are durable)");

    // (4) An OUTER VM error (gas depletion) after a write is rolled back BY THE BOUNDARY. A program
    // SSTOREs slot 7 then runs a PUSH/POP tail that exhausts gas, so `call_execute` returns an OUTER
    // Err AFTER the write. Two checks on EQUIVALENT fresh gas-limited instances over the same
    // unchanged GroveDB (each instance has its own overlay; they share only db4):
    //   (a) the real `apply` boundary reports the gas-depleting call as failed and leaves no pending
    //       writes (its outer-error branch discarded), and
    //   (b) stepped through on instance b's single overlay: the SSTORE wrote slot 7 to the overlay
    //       BEFORE the trap (inspected directly), the boundary's discard clears it, and a later commit
    //       through THAT SAME storage makes slot 7 provably absent while a witness write (slot 9) lands.
    // Gas budget sized from a measured probe: after instantiate (~7.7e9) ~14.3e9 remains, the SSTORE
    // costs ~9.9e9 (so it runs, leaving ~4.4e9), and each ~1.95e8 PUSH/POP pair then eats the rest.
    // A 40-pair tail lands deterministically in the measured window [25,50] where the call traps with
    // an OUTER error while the slot-7 write is STILL pending in the overlay (so it can be inspected
    // before the boundary discards it). The assertions below bind that exact observed state, so any
    // future gas drift fails loudly rather than passing vacuously.
    const GAS_LIMIT: u64 = 22_000_000_000;
    let bomb = format!("60cc600755{}", "600050".repeat(40)); // SSTORE slot 7 = 0xcc, then the tail
    let bomb_msg = format!(r#"{{"run":{{"code":"{bomb}"}}}}"#);
    let slot7 = {
        let mut k = [0u8; 32];
        k[31] = 7;
        k
    };

    let tmp4 = tempfile::TempDir::new().unwrap();
    let db4 = Arc::new(GroveDb::open(tmp4.path()).unwrap());
    db4.insert(ROOT, CONTRACTS, Element::empty_tree(), None, None, gv()).unwrap().expect("contracts");
    db4.insert([CONTRACTS].as_ref(), CONTRACT_ID, Element::empty_tree(), None, None, gv()).unwrap().expect("subtree");
    let gas_limited_instance = |db: &Arc<GroveDb>| {
        let backend = Backend {
            api: MockApi::default(),
            storage: OverlayGroveStorage::new(db.clone(), contract_path()),
            querier: MockQuerier::<Empty>::new(&[]),
        };
        let mut inst = Instance::from_code(
            EVM_GUEST,
            backend,
            InstanceOptions { gas_limit: GAS_LIMIT, print_debug: false },
            None,
        )
        .expect("load gas-limited instance");
        call_instantiate::<_, _, _, Empty>(&mut inst, &mock_env(), &mock_info("d", &[]), br#"{}"#)
            .expect("instantiate")
            .into_result()
            .expect("instantiate ok");
        inst
    };

    // (a) The real boundary: apply reports the outer-error call as failed and discards.
    let mut b_apply = gas_limited_instance(&db4);
    assert!(!apply(&mut b_apply, &bomb, STORAGE_BUDGET), "the real apply boundary reports the gas-depleting call as failed");
    let pending_after_apply = b_apply.with_storage::<_, usize>(|s| Ok(s.pending_writes())).expect("with_storage");
    assert_eq!(pending_after_apply, 0, "apply's outer-error branch discarded the failed write (none pending)");

    // (b) Stepped through on one storage: write lands, discard clears, later same-storage commit omits it.
    let mut b = gas_limited_instance(&db4);
    let outer = call_execute::<_, _, _, Empty>(&mut b, &mock_env(), &mock_info("c", &[]), bomb_msg.as_bytes());
    assert!(outer.is_err(), "the gas-depleting call returns an OUTER VM error (not an inner contract error)");
    // The SSTORE wrote slot 7 to the overlay BEFORE gas depletion (inspected directly, so a trap that
    // happened before the write would fail this assertion rather than pass vacuously).
    let wrote_slot7 = b.with_storage::<_, bool>(|s| Ok(s.get(&slot7).0.expect("get").is_some())).expect("with_storage");
    assert!(wrote_slot7, "the SSTORE wrote slot 7 to the overlay before gas depletion");
    let pending_before_discard = b.with_storage::<_, usize>(|s| Ok(s.pending_writes())).expect("with_storage");
    assert_eq!(pending_before_discard, 1, "exactly the slot-7 write is pending after the outer error (before discard)");
    // The boundary's outer-error action is discard; it clears the failed write.
    b.with_storage::<_, ()>(|s| { s.discard(); Ok(()) }).expect("with_storage");
    let pending_after_discard = b.with_storage::<_, usize>(|s| Ok(s.pending_writes())).expect("with_storage");
    assert_eq!(pending_after_discard, 0, "discard cleared the failed slot-7 write from the same storage");
    // A later successful commit through THAT SAME storage lands a witness (slot 9) but never slot 7.
    let slot9 = {
        let mut k = [0u8; 32];
        k[31] = 9;
        k
    };
    let mut val9 = [0u8; 32];
    val9[31] = 0x11;
    b.with_storage::<_, ()>(|s| {
        s.set(&slot9, &val9).0.expect("set witness");
        // Even this fixture witness routes through the budget-enforcing commit, so no durable success
        // path in the spike uses the unrestricted `commit`.
        s.commit_within_budget(STORAGE_BUDGET).expect("commit witness within budget");
        Ok(())
    })
    .expect("with_storage");
    let mut allq4 = Query::new();
    allq4.insert_all();
    let pq4 = PathQuery { path: contract_path(), query: SizedQuery { query: allq4, limit: None, offset: None } };
    let root4 = db4.root_hash(None, gv()).unwrap().expect("root");
    let proof4 = db4.prove_query(&pq4, None, gv()).unwrap().expect("prove");
    let (vroot4, results4) = GroveDb::verify_query(&proof4, &pq4, gv()).expect("verify");
    assert_eq!(vroot4, root4, "the proof verifies against the live root");
    let mut present4: std::collections::BTreeMap<Vec<u8>, Vec<u8>> = Default::default();
    for (_, k, el) in &results4 {
        let val = match el {
            Some(Element::Item(v, _)) => v.clone(),
            other => panic!("proven EVM slot is not an item: {other:?}"),
        };
        assert!(present4.insert(k.clone(), val).is_none(), "no duplicate proven key");
    }
    let expected4: std::collections::BTreeMap<Vec<u8>, Vec<u8>> =
        [(slot9.to_vec(), val9.to_vec())].into_iter().collect();
    assert_eq!(
        present4, expected4,
        "after the OUTER error: slot 7 is absent; only the later witness slot 9 committed through the same storage is durable"
    );
    println!("  an OUTER VM error (gas depletion) after a write rolls back through the boundary: apply");
    println!("  reports failure and discards; slot 7 reached the overlay then was dropped, and a later");
    println!("  commit through the same storage keeps slot 7 absent (only the witness slot 9 is durable)");

    // (5) Storage-budget enforcement is WIRED INTO THE BOUNDARY, not just unit-tested: a successful
    // store whose durable cost exceeds a TIGHT budget is rejected by `apply` (rolled back), so nothing
    // becomes durable; the same store within a generous budget commits.
    let tmp5 = tempfile::TempDir::new().unwrap();
    let db5 = Arc::new(GroveDb::open(tmp5.path()).unwrap());
    let mut inst5 = fresh_instance(&db5);
    assert!(
        !apply(&mut inst5, "602a60015500", 1),
        "a store whose durable cost exceeds a tight storage budget is rejected by the boundary"
    );
    {
        let mut allq5 = Query::new();
        allq5.insert_all();
        let pq5 = PathQuery { path: contract_path(), query: SizedQuery { query: allq5, limit: None, offset: None } };
        let proof5 = db5.prove_query(&pq5, None, gv()).unwrap().expect("prove");
        let (_, results5) = GroveDb::verify_query(&proof5, &pq5, gv()).expect("verify");
        assert!(
            !results5.iter().any(|(_, _, el)| el.is_some()),
            "the budget-rejected write is not durable (nothing committed)"
        );
    }
    assert!(
        apply(&mut inst5, "602a60015500", STORAGE_BUDGET),
        "the same store within a generous budget commits"
    );
    println!("  storage-budget enforcement is wired into the boundary: a store over a tight budget is");
    println!("  rejected and left non-durable, while the same store within budget commits");
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// A minimal, self-contained SHA-256 (FIPS 180-4), used only to compare the embedded wasm against
/// its committed artifact digest (an integrity check, not a build-from-source proof; see the module
/// doc). Not performance-sensitive.
mod sha256 {
    pub fn digest_hex(data: &[u8]) -> String {
        let h = digest(data);
        h.iter().map(|b| format!("{b:02x}")).collect()
    }

    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
        0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
        0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
        0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
        0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
        0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
        0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
    ];

    pub fn digest(data: &[u8]) -> [u8; 32] {
        let mut h: [u32; 8] = [
            0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
        ];
        let mut msg = data.to_vec();
        let bitlen = (data.len() as u64) * 8;
        msg.push(0x80);
        while msg.len() % 64 != 56 {
            msg.push(0);
        }
        msg.extend_from_slice(&bitlen.to_be_bytes());
        for chunk in msg.chunks_exact(64) {
            let mut w = [0u32; 64];
            for i in 0..16 {
                w[i] = u32::from_be_bytes([chunk[i * 4], chunk[i * 4 + 1], chunk[i * 4 + 2], chunk[i * 4 + 3]]);
            }
            for i in 16..64 {
                let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
                let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
                w[i] = w[i - 16].wrapping_add(s0).wrapping_add(w[i - 7]).wrapping_add(s1);
            }
            let (mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut hh) =
                (h[0], h[1], h[2], h[3], h[4], h[5], h[6], h[7]);
            for i in 0..64 {
                let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
                let ch = (e & f) ^ ((!e) & g);
                let t1 = hh.wrapping_add(s1).wrapping_add(ch).wrapping_add(K[i]).wrapping_add(w[i]);
                let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
                let maj = (a & b) ^ (a & c) ^ (b & c);
                let t2 = s0.wrapping_add(maj);
                hh = g; g = f; f = e; e = d.wrapping_add(t1); d = c; c = b; b = a; a = t1.wrapping_add(t2);
            }
            h[0] = h[0].wrapping_add(a); h[1] = h[1].wrapping_add(b); h[2] = h[2].wrapping_add(c);
            h[3] = h[3].wrapping_add(d); h[4] = h[4].wrapping_add(e); h[5] = h[5].wrapping_add(f);
            h[6] = h[6].wrapping_add(g); h[7] = h[7].wrapping_add(hh);
        }
        let mut out = [0u8; 32];
        for i in 0..8 {
            out[i * 4..i * 4 + 4].copy_from_slice(&h[i].to_be_bytes());
        }
        out
    }
}
