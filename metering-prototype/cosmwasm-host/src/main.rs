//! CosmWasm host-backend demonstration over GroveDB (the shared overlay storage).
//!
//! Drives the real `cosmwasm_vm::Storage` host trait directly (no VM instance) to demonstrate the
//! two things the host backend adds over the contract-facing trait: gas accounting and iterator-id
//! management. Reads charge gas derived from the REAL GroveDB read cost; writes are charged
//! proportional to bytes written (the durable cost is realized at commit); and scan/next manage
//! several concurrent iterators by id. The backend is the ONE shared OverlayGroveStorage from the
//! library, so there is no per-binary copy to drift.

use cosmwasm_host::{gv, OverlayGroveStorage};
use cosmwasm_std::Order;
use cosmwasm_vm::{BackendError, Storage};
use grovedb::{Element, GroveDb};
use std::sync::Arc;

const ROOT: &[&[u8]] = &[];
const CONTRACTS: &[u8] = b"contracts";
const CONTRACT_ID: &[u8] = b"demo_contract";

fn contract_path() -> Vec<Vec<u8>> {
    vec![CONTRACTS.to_vec(), CONTRACT_ID.to_vec()]
}

/// Set up a store with some already-committed keys, so a `get` of one of them reads from GroveDB and
/// its gas reflects the real read cost.
fn seeded_store(db: &Arc<GroveDb>) -> OverlayGroveStorage {
    db.insert(ROOT, CONTRACTS, Element::empty_tree(), None, None, gv())
        .unwrap()
        .expect("contracts");
    db.insert([CONTRACTS].as_ref(), CONTRACT_ID, Element::empty_tree(), None, None, gv())
        .unwrap()
        .expect("subtree");
    let path: Vec<&[u8]> = vec![CONTRACTS, CONTRACT_ID];
    for (k, v) in [(&b"alice"[..], &b"100"[..]), (b"bob", b"50"), (b"dave", b"25")] {
        db.insert(path.as_slice(), k, Element::new_item(v.to_vec()), None, None, gv())
            .unwrap()
            .expect("seed");
    }
    OverlayGroveStorage::new(db.clone(), contract_path())
}

struct RunResult {
    read_gas: u64,
    write_gas: u64,
    interleaved: Vec<(u32, String)>,
    root_hash: [u8; 32],
}

fn run_once(verbose: bool) -> RunResult {
    let tmp = tempfile::TempDir::new().unwrap();
    let db = Arc::new(GroveDb::open(tmp.path()).unwrap());
    let mut store = seeded_store(&db);

    // get a committed key: gas reflects the real GroveDB read cost.
    let (r, g) = store.get(b"alice");
    assert_eq!(r.expect("get alice"), Some(b"100".to_vec()), "committed value read back");
    let read_gas = g.cost;
    assert!(read_gas > 0, "read gas derives from the real GroveDB read cost");

    // set a new key: gas is proportional to the bytes written.
    let (r, g) = store.set(b"carol", b"75");
    r.expect("set carol");
    let write_gas = g.cost;
    // read-your-writes: the new key is visible via the overlay before commit.
    assert_eq!(store.get(b"carol").0.expect("get carol"), Some(b"75".to_vec()), "read-your-writes");

    if verbose {
        println!("## gas accounting");
        println!("  get alice (committed): gas={read_gas} (from the real GroveDB read cost)");
        println!("  set carol (new): gas={write_gas} (proportional to bytes written)");
    }

    // remove dave, then confirm absence via the overlay tombstone.
    store.remove(b"dave").0.expect("remove dave");
    assert!(store.get(b"dave").0.expect("get dave").is_none(), "removed key absent via tombstone");

    // Two concurrent iterators over the effective state (committed + overlay), interleaved by id.
    let (asc_id, _) = store.scan(None, None, Order::Ascending);
    let asc_id = asc_id.expect("scan asc");
    let (desc_id, _) = store.scan(None, None, Order::Descending);
    let desc_id = desc_id.expect("scan desc");
    assert_ne!(asc_id, desc_id, "each scan gets a distinct id");
    let mut interleaved = Vec::new();
    loop {
        let a = store.next(asc_id).0.expect("next asc");
        let d = store.next(desc_id).0.expect("next desc");
        if a.is_none() && d.is_none() {
            break;
        }
        if let Some((k, _)) = a {
            interleaved.push((asc_id, String::from_utf8(k).unwrap()));
        }
        if let Some((k, _)) = d {
            interleaved.push((desc_id, String::from_utf8(k).unwrap()));
        }
    }
    // An unknown id reports IteratorDoesNotExist.
    let (bad, _) = store.next(999);
    assert!(
        matches!(bad, Err(BackendError::IteratorDoesNotExist { id: 999 })),
        "unknown iterator id is reported"
    );
    // An exhausted iterator is dropped, so re-using its id also reports absence.
    let (after_exhaust, _) = store.next(asc_id);
    assert!(
        matches!(after_exhaust, Err(BackendError::IteratorDoesNotExist { .. })),
        "an exhausted iterator is dropped, not leaked"
    );

    // Commit the overlay and take the root hash.
    // Route the success path through the ENFORCING commit (generous storage-gas budget in the
    // adapter's own units), the same budget-checked path a metered caller would use.
    store.commit_within_budget(100_000_000).expect("commit overlay within budget");
    let root_hash = db.root_hash(None, gv()).unwrap().expect("root");

    RunResult { read_gas, write_gas, interleaved, root_hash }
}

fn main() {
    println!("# CosmWasm host backend over GroveDB: the real cosmwasm_vm::Storage trait, with gas");
    println!("# from the real read cost, size-based write gas, and iterator-id management.\n");

    let r1 = run_once(true);

    println!("\n## two concurrent iterators, interleaved next() by id");
    for (id, key) in &r1.interleaved {
        println!("  iterator {id} -> {key}");
    }
    let ids: Vec<u32> = r1.interleaved.iter().map(|(id, _)| *id).collect();
    assert!(ids.windows(2).all(|w| w[0] != w[1]), "interleaved next() alternates ids");
    // The effective state after removing dave is {alice, bob, carol}; ascending order confirms it.
    let asc: Vec<&str> = r1.interleaved.iter().filter(|(id, _)| *id == ids[0]).map(|(_, k)| k.as_str()).collect();
    assert_eq!(asc, vec!["alice", "bob", "carol"], "effective ascending order (dave removed, carol added)");

    println!("\n## determinism");
    let r2 = run_once(false);
    assert_eq!(r1.read_gas, r2.read_gas, "read gas is deterministic");
    assert_eq!(r1.write_gas, r2.write_gas, "write gas is deterministic");
    assert_eq!(r1.root_hash, r2.root_hash, "committed root hash is deterministic");
    println!("  read gas, write gas, and committed root hash are identical across two runs");
    println!("  committed root: {}", hex(&r1.root_hash));

    println!("\n# Host backend complete. Reads carry gas from the real GroveDB read cost, writes are");
    println!("# size-priced with the durable cost realized at commit, scan/next manage concurrent");
    println!("# iterators by id (exhausted ones dropped, unknown ids reported), and the result is");
    println!("# deterministic and committed atomically.");
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
