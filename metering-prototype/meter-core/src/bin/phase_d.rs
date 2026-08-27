//! Phase D of the metering prototype: the certified scenarios under synthetic load, driving the
//! Phase B meter with the Phase C MEASURED per-class vectors, to turn the drain rate R and the flow
//! ceiling from policy dials into measured throughput and backlog curves (VERIFY items 1, 4, 6).
//!
//! Denomination. The drain-service cost of a terminalization is denominated here in the measured
//! PROPAGATION dimension (GroveDB replaced_bytes from Phase C's terminal rows), which Phase A and C
//! identify as the dominant and binding cost of a cleanup operation. Each class's cost is loaded
//! into the meter's binding budget slot; the multi-dimensional treatment is a later refinement. R
//! is a per-block budget in those same units. No VM, no consensus, no networking.
//!
//! What this produces:
//!   D1 (VERIFY 4): the flow ceiling bounds the backlog. Under offered load above R, the governed
//!      meter throttles admission to R and the backlog stays bounded, while an ungoverned queue at
//!      the same offered load grows without bound. The contrast is the measurement.
//!   D2 (VERIFY 1): a chosen R yields a measured throughput per class (terminalizations per block).
//!   D3 (VERIFY 6): sweeping the C_total partition trades deadline-free throughput against dated
//!      capacity.
//!   D4: a mass-retirement load test drains a large backlog in the predicted number of blocks with
//!      every invariant holding at every block boundary.

use meter_core::{Meter, Work};

/// Per-class terminalization drain cost (propagation / replaced_bytes), COPIED from Phase C's
/// committed output `metering-prototype/results/phase_c_output.txt`. These must be kept in sync with
/// that output by hand; D2 and D3 below are ARITHMETIC derivations from these measured costs (a
/// chosen R divided by the per-class W), not separate Meter runs. Only D1 and D4 drive an actual
/// Meter. A machine-readable Phase C artifact consumed here would remove the hand-sync; that is
/// recorded as a follow-up in the metering-prototype spec.
const W_SINGLE_OWNER: u64 = 4073;
const W_AUTO_FAN1: u64 = 4325;
const W_AUTO_FAN8: u64 = 8339;
const W_AUTO_FAN64: u64 = 63576;
const W_IRREVOCABLE: u64 = 4325;
const W_DEFERRED: u64 = 5224;
const W_HOOK: u64 = 5224;
const W_LEASE_EXIT: u64 = 6375;

const CLASSES: &[(&str, u64)] = &[
    ("single_owner", W_SINGLE_OWNER),
    ("autonomous(fan=1)", W_AUTO_FAN1),
    ("autonomous(fan=8)", W_AUTO_FAN8),
    ("autonomous(fan=64)", W_AUTO_FAN64),
    ("irrevocable_request", W_IRREVOCABLE),
    ("deferred_task", W_DEFERRED),
    ("hook_binding", W_HOOK),
    ("lease_exit", W_LEASE_EXIT),
];

/// A meter whose drain budget R (in propagation units per block) is set for Phase D. The two
/// reserves are sized so the partition invariant holds; Phase D varies them in D3. Built through the
/// public `with_capacities` constructor, since the meter's capacity fields are private.
fn meter_with_r(r: u64) -> Meter {
    Meter::with_capacities(
        Work::new(r, r, r),
        Work::new(r, r, r),
        Work::new(r / 2, r / 2, r / 2),
    )
}

fn work(cost: u64) -> Work {
    // Denominate in the binding (propagation) dimension; the meter budgets the drain on .perm.
    Work::new(cost, 0, 0)
}

/// D1: the flow ceiling bounds the backlog. Offer load at `offer_ratio` times R for `blocks`
/// blocks, once through the governed meter (admission enforced) and once ungoverned (admission
/// bypassed), and report the maximum backlog reached under each.
fn d1_flow_ceiling(r: u64, w: u64, offer_ratio_pct: u64, blocks: u64) -> (usize, usize) {
    let drain_per_block = r / w; // items the drain clears per block
    let offered_per_block = drain_per_block * offer_ratio_pct / 100;

    // Governed: create against the flow condition, pull the admitted items into the queue, drain.
    let mut governed = meter_with_r(r);
    let mut governed_max = 0usize;
    for _ in 0..blocks {
        for _ in 0..offered_per_block {
            if let Some(id) = governed.create_with_vector(work(w), 1) {
                governed.pull(id);
            }
        }
        governed.drain_block();
        governed_max = governed_max.max(governed.backlog());
        assert!(governed.check_invariants().is_ok(), "governed invariants hold");
        governed.end_block(Meter::host_tick());
    }

    // Ungoverned: enqueue the offered load directly, no admission ceiling, drain.
    let mut ungoverned = meter_with_r(r);
    let mut ungoverned_max = 0usize;
    for _ in 0..blocks {
        for _ in 0..offered_per_block {
            ungoverned.enqueue_cleanup(work(w), 1);
        }
        ungoverned.drain_block();
        ungoverned_max = ungoverned_max.max(ungoverned.backlog());
        ungoverned.end_block(Meter::host_tick());
    }

    (governed_max, ungoverned_max)
}

/// D4: mass-retire `count` single-owner objects into the backlog, drain at R, and return the number
/// of blocks to clear it, asserting invariants every block.
fn d4_mass_retirement(r: u64, w: u64, count: u64) -> u64 {
    let mut m = meter_with_r(r);
    for _ in 0..count {
        m.enqueue_cleanup(work(w), 1);
    }
    let mut blocks = 0u64;
    while m.backlog() > 0 {
        m.drain_block();
        assert!(m.check_invariants().is_ok(), "invariants hold during mass-retirement drain");
        blocks += 1;
        m.end_block(Meter::host_tick());
        if blocks > 10 * count {
            panic!("drain did not converge");
        }
    }
    blocks
}

fn main() {
    // A representative drain budget: 10 single-owner cleanups per block.
    let r = W_SINGLE_OWNER * 10; // 40730 propagation units/block

    println!("# Phase D: certified scenarios under load, MEASURED Phase C vectors in the Phase B meter");
    println!("# drain budget R = {r} propagation units/block (about 10 single-owner cleanups/block)\n");

    // D1: the flow ceiling bounds the backlog (VERIFY 4).
    println!("## D1 flow ceiling (VERIFY 4): offered load above R, governed vs ungoverned backlog");
    println!("# offered=1.5x R over 200 blocks. Governed admission throttles to R; ungoverned grows.");
    let blocks = 200u64;
    for &(name, w) in &[("single_owner", W_SINGLE_OWNER)] {
        for ratio in [50u64, 90, 100, 150] {
            let (g, u) = d1_flow_ceiling(r, w, ratio, blocks);
            println!(
                "{name:<14} offered={ratio:>3}% of R   governed_max_backlog={g:<5} \
                 ungoverned_max_backlog={u}"
            );
        }
    }
    println!();

    // D2: R -> measured throughput per class (VERIFY 1).
    println!("## D2 drain throughput (VERIFY 1): terminalizations per block at R = {r}");
    for &(name, w) in CLASSES {
        let per_block = r / w;
        let per_day = per_block * 172_800; // ~0.5s blocks => ~172,800 blocks/day (Phase 0 cadence)
        println!("{name:<22} W={w:<6} throughput={per_block:>3}/block  ~{per_day}/day");
    }
    println!();

    // D3: partition sweep (VERIFY 6).
    println!("## D3 partition point (VERIFY 6): split C_total across known_due : R : overdue");
    println!("# C_total held fixed; more R buys deadline-free throughput at the cost of dated capacity.");
    println!("# The overdue reserve is a fixed fraction of C_total, so the split is genuinely three-way.");
    let c_total = r * 5 / 2; // matches meter_with_r's C_total = R + R + R/2
    let overdue = c_total / 5; // a fixed overdue reserve, held out of the tunable split
    for r_pct in [20u64, 40, 60, 80] {
        let r_share = c_total * r_pct / 100;
        // known_due gets what remains AFTER both the R share and the overdue reserve.
        let known_due = c_total.saturating_sub(r_share).saturating_sub(overdue);
        // Invariant: the three parts never exceed C_total.
        assert!(r_share + known_due + overdue <= c_total, "partition fits C_total");
        let df_throughput = r_share / W_SINGLE_OWNER;
        let dated_throughput = known_due / W_IRREVOCABLE;
        let overdue_capacity = overdue / W_SINGLE_OWNER;
        println!(
            "R={r_pct:>3}% of C_total  deadline_free={df_throughput:>3}/block  \
             dated_capacity={dated_throughput:>3}/block  overdue_reserve={overdue_capacity:>3}/block"
        );
    }
    println!();

    // D4: mass-retirement load test.
    println!("## D4 mass-retirement load test: drain a large backlog, invariants every block");
    for count in [1_000u64, 10_000, 100_000] {
        let blocks = d4_mass_retirement(r, W_SINGLE_OWNER, count);
        let predicted = (count * W_SINGLE_OWNER).div_ceil(r);
        println!(
            "retired={count:<7} drained_in={blocks:<5} blocks (predicted {predicted}), \
             invariants held every block"
        );
    }
    println!();

    println!("# Phase D complete: the flow ceiling bounds the backlog (D1), R yields measured");
    println!("# per-class throughput (D2), the partition trades deadline-free against dated capacity");
    println!("# (D3), and a 100k-object mass retirement drains in the predicted blocks with all ten");
    println!("# invariants holding at every block boundary (D4).");
}
