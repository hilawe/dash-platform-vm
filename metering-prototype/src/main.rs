//! Phase A calibration harness for the Dash Platform execution-layer metering prototype.
//!
//! Purpose (see docs/METERING_PROTOTYPE_SPEC.md, phase A): calibrate GroveDB's shipped
//! worst-case cost estimator against the MEASURED cost of the same operation run for real,
//! so the terminal-work unit can be anchored to the estimators the design already names and
//! their error is measured rather than assumed. This binary links GroveDB at the frozen
//! candidate (v5.0.0, 9b98a356) as a library. It contains no VM, no consensus, no networking.
//!
//! Method. For each (operation, tree-density) cell the harness:
//!   1. builds a GroveDB on a fresh temp RocksDB and pre-populates a subtree to the target
//!      density (number of sibling elements), which is what the estimator's propagation term
//!      is sensitive to, and which the round-8 findings turned on;
//!   2. runs the real operation and captures its OperationCost (the five-field storage vector
//!      plus seeks, loaded bytes, and hash-node calls);
//!   3. computes the worst-case estimator's prediction for the same shape;
//!   4. reports estimator-versus-actual per cost dimension.
//!
//! Determinism note. GroveDB's OperationCost is a deterministic function of the operation and
//! tree shape, not a wall-clock timing, so a single measured run per cell is reproducible; the
//! harness asserts that by running each measured cell twice and requiring identical costs. The
//! wall-clock timing that DOES vary is not the unit here and is not used.

use grovedb::batch::key_info::KeyInfo;
use grovedb::{Element, GroveDb, TreeType, WorstCaseLayerInformation};
use grovedb_costs::OperationCost;
use grovedb_version::version::GroveVersion;
use std::time::Instant;

/// Densities at which every operation is calibrated. The spec requires three; propagation is
/// logarithmic in sibling count, so the densities span three orders of magnitude to expose the
/// level-growth term.
const DENSITIES: &[u32] = &[16, 256, 4096];

/// Item payload sizes (bytes) exercised, spanning a minimal obligation record's plausible range
/// (the estimates document carries 100 to 500 durable bytes) plus a small and a large anchor.
const ITEM_SIZES: &[usize] = &[32, 128, 512];

fn gv() -> &'static GroveVersion {
    GroveVersion::latest()
}

/// The root subtree path, typed so the byte element `B` is inferable at every call site.
const ROOT: &[&[u8]] = &[];

/// Insert `count` distinct items into the root tree so the target subtree has that many siblings.
/// Keys are 8-byte big-endian so ordering and length are uniform across densities.
fn populate_root(db: &GroveDb, count: u32, item_len: usize) {
    let payload = vec![0u8; item_len];
    for i in 0..count {
        let key = i.to_be_bytes();
        db.insert(
            ROOT,
            &key,
            Element::new_item(payload.clone()),
            None,
            None,
            gv(),
        )
        .unwrap()
        .expect("populate insert");
    }
}

/// A measured cost plus the wall-clock it took, for the record (wall-clock is not the unit).
struct Measured {
    cost: OperationCost,
    wall_nanos: u128,
}

/// Run a fresh DB to `density` siblings, then measure a single INSERT of a new item at the root.
fn measure_insert_item(density: u32, item_len: usize) -> Measured {
    let tmp = tempfile::TempDir::new().unwrap();
    let db = GroveDb::open(tmp.path()).unwrap();
    populate_root(&db, density, item_len);

    let key = u32::MAX.to_be_bytes(); // a key past the populated range, so it is a genuine insert
    let payload = vec![7u8; item_len];

    let start = Instant::now();
    let cost = db
        .insert(
            ROOT,
            &key,
            Element::new_item(payload),
            None,
            None,
            gv(),
        )
        .cost;
    let wall = start.elapsed().as_nanos();
    // The operation must have succeeded for the cost to be meaningful.
    db.get(ROOT, &key, None, gv())
        .unwrap()
        .expect("inserted item is present");
    Measured {
        cost,
        wall_nanos: wall,
    }
}

/// Measure a single DELETE of an existing item at a given density.
fn measure_delete_item(density: u32, item_len: usize) -> Measured {
    let tmp = tempfile::TempDir::new().unwrap();
    let db = GroveDb::open(tmp.path()).unwrap();
    populate_root(&db, density, item_len);

    // Delete a key in the middle of the populated range.
    let key = (density / 2).to_be_bytes();
    let start = Instant::now();
    let cost = db.delete(ROOT, &key, None, None, gv()).cost;
    let wall = start.elapsed().as_nanos();
    Measured {
        cost,
        wall_nanos: wall,
    }
}

/// Measure a single REPLACE (insert over an existing key) at a given density.
fn measure_replace_item(density: u32, item_len: usize) -> Measured {
    let tmp = tempfile::TempDir::new().unwrap();
    let db = GroveDb::open(tmp.path()).unwrap();
    populate_root(&db, density, item_len);

    let key = (density / 2).to_be_bytes();
    let payload = vec![9u8; item_len];
    let start = Instant::now();
    let cost = db
        .insert(
            ROOT,
            &key,
            Element::new_item(payload),
            None,
            None,
            gv(),
        )
        .cost;
    let wall = start.elapsed().as_nanos();
    Measured {
        cost,
        wall_nanos: wall,
    }
}

/// Extract the storage-cost vector components as a printable tuple.
/// Returns (added, replaced, removed, seek, loaded, hash_calls).
fn dims(c: &OperationCost) -> (u32, u32, u32, u32, u64, u32) {
    let removed = c.storage_cost.removed_bytes.total_removed_bytes();
    (
        c.storage_cost.added_bytes,
        c.storage_cost.replaced_bytes,
        removed,
        c.seek_count,
        c.storage_loaded_bytes,
        c.hash_node_calls,
    )
}

/// The worst-case estimator's predicted cost for inserting an item of `item_len` bytes with a
/// 4-byte key, at a supplied number of tree levels. In real Platform use the level count comes
/// from EstimatedLayerInformation metadata; here we supply the balanced-tree level estimate for
/// the density so the estimator and the measured run describe the same tree shape.
fn estimate_insert(item_len: usize, levels: u32) -> OperationCost {
    let key = KeyInfo::KnownKey(vec![0u8; 4]);
    let value = Element::new_item(vec![0u8; item_len]);
    let layer = WorstCaseLayerInformation::NumberOfLevels(levels);
    GroveDb::worst_case_merk_insert_element(
        &key,
        &value,
        TreeType::NormalTree,
        Some(&layer),
        gv(),
    )
    .cost
}

/// Balanced-tree level count supplied to the estimator for a given sibling density. A Merk tree
/// is AVL-balanced, whose height is at most about 1.44*log2(n+2), so this is the conservative
/// level input a caller would derive for the worst case.
fn levels_for(density: u32) -> u32 {
    let n = (density.max(1)) as f64;
    (1.44 * (n + 2.0).log2()).ceil() as u32
}

fn print_row(op: &str, density: u32, item_len: usize, m: &Measured) {
    let (a, r, rm, sk, ld, hc) = dims(&m.cost);
    println!(
        "{op:<8} density={density:<6} item={item_len:<5} \
         added={a:<6} replaced={r:<6} removed={rm:<6} \
         seek={sk:<4} loaded={ld:<7} hash_calls={hc:<4} wall_ns={}",
        m.wall_nanos
    );
}

fn main() {
    println!("# Phase A calibration, GroveDB frozen candidate 9b98a356 (v5.0.0)");
    println!("# measured OperationCost per operation and tree density. Cost is deterministic;");
    println!("# each measured cell is run twice and required identical (see assertions).\n");

    let ops: &[(&str, fn(u32, usize) -> Measured)] = &[
        ("insert", measure_insert_item),
        ("replace", measure_replace_item),
        ("delete", measure_delete_item),
    ];

    for (name, f) in ops {
        for &density in DENSITIES {
            for &item_len in ITEM_SIZES {
                let m1 = f(density, item_len);
                let m2 = f(density, item_len);
                assert_eq!(
                    m1.cost, m2.cost,
                    "cost must be deterministic for {name} density={density} item={item_len}"
                );
                print_row(name, density, item_len, &m1);
            }
        }
        println!();
    }

    // Phase A step 2: calibrate the WORST-CASE estimator against the measured insert cost.
    // The estimator is the charge the design levies; the measured run is a real case. The ratio
    // measured/estimator is the conservatism headroom, and the estimator must never under-predict
    // the durable-storage dimension (added_bytes), which the harness asserts.
    println!("# Phase A step 2: worst-case estimator vs measured, INSERT of an item.");
    println!("# columns: est_added/meas_added est_replaced/meas_replaced (bytes), headroom = est/meas");
    for &density in DENSITIES {
        let levels = levels_for(density);
        for &item_len in ITEM_SIZES {
            let meas = measure_insert_item(density, item_len);
            let est = estimate_insert(item_len, levels);
            let (ma, mr, _, _, _, _) = dims(&meas.cost);
            let (ea, er, _, _, _, _) = dims(&est);
            assert!(
                ea >= ma,
                "worst-case estimator must not under-predict added_bytes: est={ea} < meas={ma} \
                 (density={density}, item={item_len})"
            );
            let add_headroom = ea as f64 / ma.max(1) as f64;
            let rep_headroom = er as f64 / mr.max(1) as f64;
            println!(
                "estimate density={density:<6} item={item_len:<5} levels={levels:<3} \
                 added={ea:<6}/{ma:<6} replaced={er:<6}/{mr:<6} \
                 add_headroom={add_headroom:.2}x rep_headroom={rep_headroom:.2}x"
            );
        }
    }
    println!();

    // Phase A step 3: ASSERT the empirical properties that the results document and commit message
    // promote, so a regression (a drifting estimator, a store whose durable footprint changes)
    // fails the harness rather than letting the binary print a false headline. Each assertion below
    // corresponds to a claimed property.
    println!("# Phase A step 3: asserting the promoted properties.");
    for &item_len in ITEM_SIZES {
        let mut added_by_density = Vec::new();
        for &density in DENSITIES {
            let (ma, mr, _, _, _, _) = dims(&measure_insert_item(density, item_len).cost);
            let (ea, er, _, _, _, _) = dims(&estimate_insert(item_len, levels_for(density)));
            // (a) the worst-case estimator predicts added_bytes EXACTLY.
            assert_eq!(
                ea, ma,
                "estimator predicts added_bytes exactly (item={item_len}, density={density})"
            );
            // (c) it over-predicts the propagation dimension by a large factor.
            assert!(
                er > mr && (er as f64 / mr.max(1) as f64) > 100.0,
                "estimator over-predicts propagation by >100x (item={item_len}, density={density}): \
                 est={er} meas={mr}"
            );
            added_by_density.push(ma);
        }
        // (b) added_bytes is independent of tree density for a fixed item size.
        assert!(
            added_by_density.windows(2).all(|w| w[0] == w[1]),
            "added_bytes is density-independent for item={item_len}: {added_by_density:?}"
        );
    }
    for &item_len in ITEM_SIZES {
        for &density in DENSITIES {
            let (ins_added, _, _, _, _, _) = dims(&measure_insert_item(density, item_len).cost);
            let (rep_added, _, _, _, _, _) = dims(&measure_replace_item(density, item_len).cost);
            let (_, _, del_removed, _, _, _) = dims(&measure_delete_item(density, item_len).cost);
            // (d) replacement adds zero durable bytes.
            assert_eq!(
                rep_added, 0,
                "replacement adds no durable bytes (item={item_len}, density={density})"
            );
            // (e) deletion reclaims exactly the bytes the insert deposited.
            assert_eq!(
                del_removed, ins_added,
                "deletion reclaims exactly the inserted added_bytes (item={item_len}, density={density})"
            );
        }
    }
    println!("# all promoted Phase A properties asserted: exact added prediction, density");
    println!("# independence, propagation over-prediction >100x, zero durable growth on replace,");
    println!("# and delete conservation (removed == inserted added_bytes).");

    println!("\n# Phase A complete: measured costs captured for insert, replace, delete across");
    println!("# three densities and three item sizes, determinism asserted, and every promoted");
    println!("# empirical property asserted rather than merely printed.");
}
