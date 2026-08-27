//! Phase C of the metering prototype: per-class SYNTHETIC record fixtures and their terminalization
//! cost, run against the REAL GroveDB store at the frozen candidate (v5.0.0, 9b98a356).
//!
//! SCOPE AND CLAIM WIDTH (important): this does NOT invoke the platform's real disposition-class
//! lifecycle code or serialize real Platform record types. For each disposition class it builds a
//! SYNTHETIC representative record shape (a plausible byte layout plus the secondary indexes such a
//! record would carry) and runs a hand-written sequence of GroveDB operations that stands in for
//! that class's terminalization, then captures the cumulative OperationCost per dimension. So the
//! measured numbers are the real cost of REPRESENTATIVE GroveDB operations, not of the platform's
//! actual per-class lifecycle. That is the right altitude for a cost fixture, but the per-class
//! attribution is a modelling choice, not a measurement of the platform's own class code. A later
//! step that wants class-exact numbers would serialize the real record types and drive rs-drive's
//! own lifecycle operations.
//!
//! No VM, no consensus, no networking. This is the store plus synthetic lifecycle operations. The
//! storage dimension (added/removed bytes) is measured directly; per Phase A the worst-case
//! estimator predicts added_bytes exactly and over-predicts propagation, so the measured
//! propagation here is the honest figure to denominate that dimension with.

use grovedb::{Element, GroveDb};
use grovedb_costs::OperationCost;
use grovedb_version::version::GroveVersion;

fn gv() -> &'static GroveVersion {
    GroveVersion::latest()
}

/// The D15 disposition classes, each modelled here as a SYNTHETIC record fixture with a distinct
/// synthetic terminalization sequence (not the platform's real class lifecycle code).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Class {
    /// Cleanup only: the balance discharge is owner-paid, so terminal work is record and index
    /// reclamation.
    SingleOwner,
    /// Cleanup plus protocol-driven distribution to `fan_out` recipients.
    Autonomous,
    /// A synthetic fixed-recipe settlement (a single extra write standing in for consuming a
    /// pre-materialized finalization record, which this fixture does not actually build).
    IrrevocableRequest,
    /// A scheduled task: record plus a schedule-height index.
    DeferredTask,
    /// A hook binding: record plus a trigger index.
    HookBinding,
    /// A lease-and-exit record: reclamation leaves a tombstone.
    LeaseExit,
}

impl Class {
    fn name(self) -> &'static str {
        match self {
            Class::SingleOwner => "single_owner",
            Class::Autonomous => "autonomous",
            Class::IrrevocableRequest => "irrevocable_request",
            Class::DeferredTask => "deferred_task",
            Class::HookBinding => "hook_binding",
            Class::LeaseExit => "lease_exit",
        }
    }
}

/// A summed cost vector across a sequence of operations. Mirrors OperationCost's dimensions.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Vec6 {
    added: u64,
    replaced: u64,
    removed: u64,
    seek: u64,
    loaded: u64,
    hash: u64,
}

impl Vec6 {
    fn add_cost(&mut self, c: &OperationCost) {
        self.added += c.storage_cost.added_bytes as u64;
        self.replaced += c.storage_cost.replaced_bytes as u64;
        self.removed += c.storage_cost.removed_bytes.total_removed_bytes() as u64;
        self.seek += c.seek_count as u64;
        self.loaded += c.storage_loaded_bytes;
        self.hash += c.hash_node_calls as u64;
    }
}

/// The root subtree path, typed so the byte element `B` is inferable at the root insert sites.
const ROOT: &[&[u8]] = &[];

/// Subtree names.
const POSITIONS: &[u8] = b"positions";
const OWNER_IDX: &[u8] = b"owner_idx";
const SUBJECT_IDX: &[u8] = b"subject_idx";
const AUX_IDX: &[u8] = b"aux_idx"; // schedule/trigger/lease index, class-dependent
const RECIPIENTS: &[u8] = b"recipients"; // distribution targets for autonomous class

/// Create the scaffolding subtrees at the root.
fn scaffold(db: &GroveDb) {
    for name in [POSITIONS, OWNER_IDX, SUBJECT_IDX, AUX_IDX, RECIPIENTS] {
        db.insert(ROOT, name, Element::empty_tree(), None, None, gv())
            .unwrap()
            .expect("scaffold subtree");
    }
}

/// Populate a subtree with `count` filler entries so index operations run at a realistic density.
fn populate(db: &GroveDb, subtree: &[u8], count: u32, payload_len: usize) {
    let payload = vec![0u8; payload_len];
    for i in 0..count {
        let key = i.to_be_bytes();
        db.insert(
            [subtree].as_ref(),
            &key,
            Element::new_item(payload.clone()),
            None,
            None,
            gv(),
        )
        .unwrap()
        .expect("populate");
    }
}

/// The minimal position record payload: owner (32) + subject ref (32) + class descriptor (1) +
/// terminal-work vector (6 * 8) + funding state (8). This is the ~121-byte minimal record the
/// estimates document reasoned about, now built concretely.
fn record_payload() -> Vec<u8> {
    let mut v = Vec::new();
    v.extend_from_slice(&[1u8; 32]); // owner
    v.extend_from_slice(&[2u8; 32]); // subject ref
    v.push(0u8); // class descriptor
    v.extend_from_slice(&[0u8; 48]); // terminal-work vector (6 x u64)
    v.extend_from_slice(&[0u8; 8]); // funding state
    v
}

/// Build a minimal record for a class: the position item plus its secondary indexes. Returns the
/// deposit-time storage cost (the added-bytes footprint the depositor pays for).
fn build_record(db: &GroveDb, class: Class, id: u32) -> Vec6 {
    let mut deposit = Vec6::default();
    let key = id.to_be_bytes();

    // Position record.
    let c = db
        .insert(
            [POSITIONS].as_ref(),
            &key,
            Element::new_item(record_payload()),
            None,
            None,
            gv(),
        );
    deposit.add_cost(&c.cost);
    c.value.expect("insert position");

    // Owner index and subject index (all classes).
    for idx in [OWNER_IDX, SUBJECT_IDX] {
        let c = db
            .insert(
                [idx].as_ref(),
                &key,
                Element::new_item(id.to_be_bytes().to_vec()),
                None,
                None,
                gv(),
            );
        deposit.add_cost(&c.cost);
        c.value.expect("insert index");
    }

    // Class-dependent auxiliary index (schedule height, trigger, or lease marker).
    if matches!(class, Class::DeferredTask | Class::HookBinding | Class::LeaseExit) {
        let c = db
            .insert(
                [AUX_IDX].as_ref(),
                &key,
                Element::new_item(id.to_be_bytes().to_vec()),
                None,
                None,
                gv(),
            );
        deposit.add_cost(&c.cost);
        c.value.expect("insert aux index");
    }

    deposit
}

/// Run the class's terminalization and return the cumulative cost per dimension.
fn terminalize(db: &GroveDb, class: Class, id: u32, fan_out: u32) -> Vec6 {
    let mut cost = Vec6::default();
    let key = id.to_be_bytes();

    // Every class reclaims the position record and the owner and subject indexes.
    for (subtree, k) in [(POSITIONS, key), (OWNER_IDX, key), (SUBJECT_IDX, key)] {
        let c = db
            .delete([subtree].as_ref(), &k, None, None, gv());
        cost.add_cost(&c.cost);
        c.value.expect("delete during terminalization");
    }

    match class {
        Class::SingleOwner => {
            // Cleanup only; nothing further.
        }
        Class::Autonomous => {
            // Protocol-driven distribution to fan_out recipients: a credit write each.
            for r in 0..fan_out {
                let rk = (1_000_000 + r).to_be_bytes();
                let c = db
                    .insert(
                        [RECIPIENTS].as_ref(),
                        &rk,
                        Element::new_item(vec![9u8; 16]),
                        None,
                        None,
                        gv(),
                    );
                cost.add_cost(&c.cost);
                c.value.expect("distribution credit");
            }
        }
        Class::IrrevocableRequest => {
            // A synthetic fixed-recipe settlement (a single extra write). This does NOT build or
            // consume a real pre-materialized finalization record; it stands in for one.
            let c = db
                .insert(
                    [RECIPIENTS].as_ref(),
                    &key,
                    Element::new_item(vec![5u8; 24]),
                    None,
                    None,
                    gv(),
                );
            cost.add_cost(&c.cost);
            c.value.expect("settlement write");
        }
        Class::DeferredTask | Class::HookBinding => {
            // Reclaim the auxiliary index entry.
            let c = db.delete([AUX_IDX].as_ref(), &key, None, None, gv());
            cost.add_cost(&c.cost);
            c.value.expect("delete aux index");
        }
        Class::LeaseExit => {
            // Reclaim the auxiliary index and leave a tombstone in its place.
            let c = db.delete([AUX_IDX].as_ref(), &key, None, None, gv());
            cost.add_cost(&c.cost);
            c.value.expect("delete aux index");
            let c = db
                .insert(
                    [AUX_IDX].as_ref(),
                    &key,
                    Element::new_item(vec![0xFFu8; 1]),
                    None,
                    None,
                    gv(),
                );
            cost.add_cost(&c.cost);
            c.value.expect("tombstone write");
        }
    }

    cost
}

/// Measure one class end to end on a fresh store at a given density and fan-out.
fn measure_class(class: Class, density: u32, fan_out: u32) -> (Vec6, Vec6) {
    let tmp = tempfile::TempDir::new().unwrap();
    let db = GroveDb::open(tmp.path()).unwrap();
    scaffold(&db);
    // Populate the position tree and indexes to the target density for realistic propagation.
    populate(&db, POSITIONS, density, 121);
    populate(&db, OWNER_IDX, density, 4);
    populate(&db, SUBJECT_IDX, density, 4);
    populate(&db, AUX_IDX, density, 4);

    let id = density + 1; // a fresh id past the filler range
    let deposit = build_record(&db, class, id);
    let terminal = terminalize(&db, class, id, fan_out);
    (deposit, terminal)
}

fn print_vec(label: &str, class: &str, fan_out: u32, v: &Vec6) {
    println!(
        "{label:<9} {class:<20} fan_out={fan_out:<3} \
         added={:<6} replaced={:<7} removed={:<6} seek={:<4} loaded={:<8} hash={:<4}",
        v.added, v.replaced, v.removed, v.seek, v.loaded, v.hash
    );
}

fn main() {
    println!("# Phase C: per-class terminal-work vectors, measured against real GroveDB 9b98a356");
    println!("# density = 256 filler siblings per subtree. deposit = build-time added-bytes cost;");
    println!("# terminal = cumulative cost of the class's full terminalization, per dimension.\n");

    let density = 256u32;
    let cases: &[(Class, u32)] = &[
        (Class::SingleOwner, 0),
        (Class::Autonomous, 1),
        (Class::Autonomous, 8),
        (Class::Autonomous, 64),
        (Class::IrrevocableRequest, 0),
        (Class::DeferredTask, 0),
        (Class::HookBinding, 0),
        (Class::LeaseExit, 0),
    ];

    let mut results: Vec<(&str, u32, Vec6, Vec6)> = Vec::new();
    for &(class, fan_out) in cases {
        // Determinism: measure twice and require the WHOLE cost vectors to match, not just the
        // added/removed columns.
        let (d1, t1) = measure_class(class, density, fan_out);
        let (d2, t2) = measure_class(class, density, fan_out);
        assert_eq!(d1, d2, "deposit cost vector must be deterministic for {} fan_out={}", class.name(), fan_out);
        assert_eq!(t1, t2, "terminal cost vector must be deterministic for {} fan_out={}", class.name(), fan_out);
        print_vec("deposit", class.name(), fan_out, &d1);
        print_vec("terminal", class.name(), fan_out, &t1);
        println!();
        results.push((class.name(), fan_out, d1, t1));
    }

    // Assert the properties the results document promotes, so they are checked rather than printed.
    // (1) Reclamation conservation: terminalization reclaims exactly the durable bytes the deposit
    // added, for every class.
    for (name, fan_out, dep, term) in &results {
        assert_eq!(
            term.removed, dep.added,
            "reclamation conservation for {name} fan_out={fan_out}: removed {} != deposited {}",
            term.removed, dep.added
        );
    }
    // (2) Single-owner terminalization adds no durable bytes (cleanup only).
    let single = results.iter().find(|r| r.0 == "single_owner").expect("single_owner case");
    assert_eq!(single.3.added, 0, "single-owner terminalization adds no durable bytes");
    // (3) Autonomous distribution scales linearly with fan-out and proportionally through the
    // origin (no hardcoded slope: the two measured slopes must agree, and the fan-out-1 point must
    // equal that slope).
    let auto_added = |n: u32| -> u64 {
        results.iter().find(|r| r.0 == "autonomous" && r.1 == n).expect("autonomous case").3.added
    };
    let (a1, a8, a64) = (auto_added(1), auto_added(8), auto_added(64));
    // Exact linearity requires the two spans to be evenly divisible (no remainder), so a point that
    // is off the line but happens to share the quotient cannot pass.
    assert_eq!((a8 - a1) % (8 - 1), 0, "fan-out 1..8 span is exactly divisible (a point off the line fails)");
    assert_eq!((a64 - a8) % (64 - 8), 0, "fan-out 8..64 span is exactly divisible (a point off the line fails)");
    let slope_lo = (a8 - a1) / (8 - 1);
    let slope_hi = (a64 - a8) / (64 - 8);
    assert_eq!(slope_lo, slope_hi, "autonomous distribution linear in fan-out: slopes {slope_lo} vs {slope_hi}");
    assert_eq!(a1, slope_lo, "autonomous distribution proportional through the origin (per-recipient {slope_lo})");
    println!("# asserted: reclamation conservation (removed == deposited) for every class,");
    println!("# single-owner adds zero durable bytes at terminalization, and autonomous distribution");
    println!("# is linear in fan-out at {slope_lo} durable bytes per recipient.");

    println!("# Phase C complete: each class's deposit and terminalization cost measured per");
    println!("# dimension. The added-bytes column is the durable footprint; removed-bytes is what");
    println!("# terminalization reclaims; the autonomous rows show distribution scaling with fan_out.");
}
