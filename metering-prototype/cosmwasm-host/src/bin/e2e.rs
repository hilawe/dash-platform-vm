//! End-to-end: a compiled CosmWasm contract run through the real VM, backed by GroveDB.
//!
//! A real compiled contract (hackatom, the canonical cosmwasm-vm test contract, Apache-2.0, copied
//! from the cosmwasm-vm 1.5 test data) is instantiated and queried through the real cosmwasm-vm
//! (wasmer, singlepass), backed by the shared overlay storage over GroveDB. The contract's writes
//! land in an overlay during the calls and are flushed to GroveDB with a single `commit` after both
//! calls SUCCEED. This is a single-success-path demonstration: it exercises only the commit side, not
//! a per-call commit/discard boundary (the discard/rollback side is demonstrated in the EVM spike).
//! The committed state is then proven with GroveDB `prove_query`, bound to exact content.

use cosmwasm_host::{gv, OverlayGroveStorage};
use cosmwasm_std::Empty;
use cosmwasm_vm::testing::{mock_env, mock_info, MockApi, MockQuerier};
use cosmwasm_vm::{call_instantiate, call_query, Backend, Instance, InstanceOptions};
use grovedb::{Element, GroveDb, PathQuery, Query, SizedQuery};
use std::sync::Arc;

const ROOT: &[&[u8]] = &[];
const CONTRACTS: &[u8] = b"contracts";
const CONTRACT_ID: &[u8] = b"demo_contract";
const HACKATOM: &[u8] = include_bytes!("../../testdata/hackatom.wasm");

fn contract_path() -> Vec<Vec<u8>> {
    vec![CONTRACTS.to_vec(), CONTRACT_ID.to_vec()]
}

fn main() {
    println!("# End-to-end: a compiled CosmWasm contract (hackatom) run through cosmwasm-vm (wasmer),");
    println!("# backed by the overlay storage over GroveDB, with the result proven by GroveDB.\n");

    let tmp = tempfile::TempDir::new().unwrap();
    let db = Arc::new(GroveDb::open(tmp.path()).unwrap());
    db.insert(ROOT, CONTRACTS, Element::empty_tree(), None, None, gv())
        .unwrap()
        .expect("contracts tree");
    db.insert([CONTRACTS].as_ref(), CONTRACT_ID, Element::empty_tree(), None, None, gv())
        .unwrap()
        .expect("contract subtree");

    let storage = OverlayGroveStorage::new(db.clone(), contract_path());
    let backend = Backend {
        api: MockApi::default(),
        storage,
        querier: MockQuerier::<Empty>::new(&[]),
    };
    let mut instance = Instance::from_code(
        HACKATOM,
        backend,
        InstanceOptions {
            gas_limit: u64::MAX,
            print_debug: false,
        },
        None,
    )
    .expect("compile and instantiate wasm");
    println!("## the contract compiled and loaded into the VM (wasmer, singlepass)");

    let env = mock_env();
    let info = mock_info("creator", &[]);

    let imsg = br#"{"verifier": "verifies", "beneficiary": "benefits"}"#;
    call_instantiate::<_, _, _, Empty>(&mut instance, &env, &info, imsg)
        .expect("call_instantiate")
        .into_result()
        .expect("the contract's instantiate entry point succeeded");
    println!("## instantiate ran through the VM and wrote contract state into the overlay");

    let qmsg = br#"{"verifier":{}}"#;
    let qres = call_query(&mut instance, &env, qmsg).expect("call_query");
    let qbin = qres.into_result().expect("query returned Ok");
    assert_eq!(
        qbin.as_slice(),
        br#"{"verifier":"verifies"}"#,
        "the query read the stored verifier back through the VM"
    );
    println!("## query read the stored verifier back through the VM: {}",
        String::from_utf8_lossy(qbin.as_slice()));

    // Both calls succeeded, so commit the overlay to GroveDB. This is the COMMIT side only: this demo
    // has no failing call and no discard path. The discard/rollback side (a failed call leaves no
    // durable write) is demonstrated in the EVM spike, not here.
    instance
        .with_storage::<_, ()>(|s| {
            // Route the success path through the ENFORCING commit (a generous storage-gas budget here,
            // in the adapter's own units), so this demo uses the same budget-checked path a metered
            // caller would, not the unrestricted commit.
            s.commit_within_budget(100_000_000).expect("flush overlay within budget");
            Ok(())
        })
        .expect("with_storage");
    println!("## the successful calls' writes were committed to GroveDB (single-success-path demo; the");
    println!("## discard/rollback side is demonstrated in the EVM spike)");

    // Prove the contract's committed state, bound to exact content: hackatom writes exactly one
    // config entry, and its stored value must contain the verifier we set.
    let mut q = Query::new();
    q.insert_all();
    let pq = PathQuery {
        path: contract_path(),
        query: SizedQuery { query: q, limit: None, offset: None },
    };
    let root = db.root_hash(None, gv()).unwrap().expect("root");
    let proof = db.prove_query(&pq, None, gv()).unwrap().expect("prove");
    let (vroot, results) = GroveDb::verify_query(&proof, &pq, gv()).expect("verify");
    assert_eq!(vroot, root, "the proof verifies against the live committed root");
    // Require the proof to contain EXACTLY one entry, that it is an item (a non-item is an error, not
    // silently skipped), and bind its key and both stored fields.
    assert_eq!(results.len(), 1, "hackatom writes exactly one config entry");
    let (_, key, el) = &results[0];
    let config = match el {
        Some(Element::Item(v, _)) => v,
        other => panic!("the single proven entry is not an item: {other:?}"),
    };
    assert_eq!(key.as_slice(), b"config", "the single entry is stored under the `config` key");
    // Bind the EXACT value: decode into a strict schema that rejects unknown or trailing content
    // (from_json rejects trailing bytes; deny_unknown_fields rejects extra keys), assert every field,
    // and require the canonical reserialization to equal the proven bytes. A malformed blob that
    // merely contains the substrings "verifies"/"benefits" fails to decode and is rejected.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    struct HackatomConfig {
        verifier: String,
        beneficiary: String,
        funder: String,
    }
    let cfg: HackatomConfig =
        cosmwasm_std::from_json(config).expect("the proven config decodes as the exact hackatom schema");
    assert_eq!(cfg.verifier, "verifies", "proven config binds the verifier we set");
    assert_eq!(cfg.beneficiary, "benefits", "proven config binds the beneficiary we set");
    assert_eq!(cfg.funder, "creator", "proven config binds the funder (the instantiate sender)");
    let canonical = cosmwasm_std::to_json_vec(&cfg).expect("reserialize config");
    assert_eq!(
        canonical.as_slice(),
        config.as_slice(),
        "the proven bytes are exactly the canonical serialization of the config (no extra/trailing content)"
    );
    println!("## the contract's stored config is provable in GroveDB and binds the exact serialized value");
    println!("  committed root: {}", hex(&root));

    println!("\n# End-to-end complete. A real compiled CosmWasm contract instantiated and queried");
    println!("# through cosmwasm-vm over the overlay-backed GroveDB store, its writes committed on");
    println!("# success, and the exact stored config proven with GroveDB prove_query.");
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
