# Metering prototype results, Phase A calibration

Dated 2026-08-11. This document records the first execution-produced numbers from the metering
prototype specified in `docs/METERING_PROTOTYPE_SPEC.md`. Phase A calibrates GroveDB's shipped
worst-case cost estimator against the measured cost of the same operation run for real, on the frozen
candidate (GroveDB v5.0.0, commit 9b98a356), linked as a library with no VM, consensus, or networking.
The harness source is `metering-prototype/`, the raw output is
`metering-prototype/results/phase_a_output.txt`, and every number below is reproducible by rebuilding
and running that binary.

## Claim width, stated first

These are HARNESS-MEASURED numbers, meaning measurements of real GroveDB operations and the store's own
estimator on the frozen candidate. They are NOT platform-measured (they do not include the Layer 2 state
transition path, consensus, fee conversion to credits, or networking), and they do not touch DESIGN.md,
which remains review-complete under the standing rule that any substantive change needs a fresh
adversarial round. They upgrade the affected VERIFY items in `docs/PHASE0_VERIFY_ESTIMATES.md` from
ESTIMATE to HARNESS-MEASURED, and no further.

## Method

For each operation (insert, replace, delete), tree density (16, 256, 4096 sibling elements), and item
size (32, 128, 512 bytes), the harness builds a fresh RocksDB-backed GroveDB, populates the root subtree
to the target density, runs one real operation, and captures its `OperationCost`. GroveDB's
`OperationCost` is a deterministic function of the operation and tree shape rather than a wall-clock
timing, so the harness runs each measured cell twice and asserts the two costs are identical (the run
passes this assertion). The cost dimensions are the platform's storage-cost vector (added, replaced, and
removed bytes) plus seek count, loaded bytes, and hash-node calls. Wall-clock is recorded for reference
only and is not the unit.

For calibration, the worst-case estimator `worst_case_merk_insert_element` is evaluated for the same
item size at the balanced-tree level count for each density (Merk is AVL-balanced, height at most about
1.44 log2(n+2)), and the harness asserts the estimator never under-predicts the durable-storage
dimension.

## The two findings that matter to the design

### 1. The durable-storage dimension is exactly predicted, and density-independent

`added_bytes`, the permanent on-disk footprint an operation adds, depends ONLY on the record size, not
on tree density, and the worst-case estimator predicts it exactly (headroom 1.00x at every cell):

| Item bytes | Measured added_bytes | Worst-case estimator | Headroom |
| --- | --- | --- | --- |
| 32 | 178 | 178 | 1.00x |
| 128 | 275 | 275 | 1.00x |
| 512 | 661 | 661 | 1.00x |

This holds identically at densities 16, 256, and 4096. The consequence for the terminal-work meter is
direct, because the permanent-storage component of a per-class terminal-work vector can be charged at
the
worst-case estimate with ZERO over-reservation, because for this dimension the worst case is the true
cost. It also confirms the estimates document's classification A (per-class storage vectors are
boundable now) and replaces its 100-to-500-byte assumption with measured points, since a native record
is
about 178 durable bytes at a 32-byte payload and 275 at 128 bytes.

### 2. The propagation dimension is over-predicted by two to three orders of magnitude

`replaced_bytes`, the cost of rewriting parent node hashes up the tree path, is where the worst-case
estimator is extremely conservative. The estimator assumes every node on the path is at the maximum
node size (65,535 bytes), so it predicts 590K to 1.3M bytes against a measured 700 to 8,100:

| Density | Item bytes | Measured replaced | Worst-case estimator | Headroom |
| --- | --- | --- | --- | --- |
| 16 | 32 | 705 | 589,815 | 837x |
| 256 | 128 | 2,142 | 917,490 | 428x |
| 4096 | 512 | 8,112 | 1,310,700 | 162x |

The measured propagation cost grows slowly and predictably with density (the seek count rises exactly 8
per 16x density step, tracking tree-level growth), while the worst-case bound is dominated by the
maximum-node-size assumption. The consequence for the design is a specification-level choice made
concrete. If the terminal-work meter charges the WORST-CASE estimator for the propagation dimension, it
over-reserves by 160x to 840x, which would make the drain rate R and the per-class vectors
enormously pessimistic. The propagation dimension should instead be denominated by the average-case
estimator or a tighter path-size bound. This is a measured input to how R is set, exactly the kind of
number Phase A existed to produce, and it does not change any design decision, only the future
calibration of a dial the design left open.

## Cross-operation observations

- Delete reclaims exactly what insert added: `removed_bytes` on delete equals `added_bytes` on insert
  for the same item size (178, 275, 661). The durable footprint is symmetric, which supports the
  cleanup accounting model where a queued cleanup item reclaims the record's own admitted storage.
- Replace adds zero durable bytes (`added_bytes` = 0) and only rewrites, so an in-place mutation that
  does not grow the record imposes no new permanent-storage terminal work, consistent with the meter
  charging only positive marginal growth.
- Hash-node calls scale with tree level (8 to 36 across the range) and are modest, so the hashing
  dimension of terminal work is small relative to storage on these shapes.

## What this leaves for the next phases

Phase A has calibrated the storage and propagation dimensions of single operations. It does not yet
define the terminal-work unit for compute (phase E), build the per-class record catalog and run each
class's full terminalization (phase C), or exercise the meter arithmetic and the certified scenarios
under load (phases B and D). The immediate next step is phase B, the meter core plus the invariant
assertion suite, each invariant watched failing before passing. The calibration here supplies phase B
its unit for the two storage dimensions and the key finding that the two dimensions must be charged
differently, worst-case for permanent storage and a tighter bound for propagation.

# Phase B results, meter core and invariant suite

Dated 2026-08-11. Phase B implements the D3b terminal-work meter arithmetic as an in-memory model
(`metering-prototype/meter-core/`, a zero-dependency Rust crate) and makes the ten certified invariants
(spec section 6) executable. It has no store, no consensus, and no networking, because the meter core is
pure deterministic bookkeeping. The storage-dimension unit comes from Phase A; the capacity numbers in
the model are illustrative values chosen to exercise the arithmetic, not platform claims. Raw test
output is `metering-prototype/results/phase_b_test_output.txt`.

## What was built

A `Meter` over a block loop carries the certified lifecycle transitions. They are create (admission
against the flow condition), grow (marginal charge), bidirectional reclassify (positive-delta charge
only), pull (logical
exit enqueuing prepaid cleanup), transfer (discharge-and-recreate with an accounting handoff and a gross
positive delta), the deadline-free drain at rate R under a per-block budget, dated known_due settlement,
the irrevocability-burst gate, and mass retirement. After every block a single `check_invariants`
routine evaluates the continuous invariants (accounting, flow, partition, conservation, release timing,
single disposition).

## The mutation-check discipline, satisfied

Each of the ten invariants has a deliberately broken variant (the `Fault` enum) built to violate exactly
that invariant. Every test asserts BOTH directions, so the invariant holds on the clean path and is
detected as violated under its fault. This makes the check self-enforcing, because a fault that failed
to
trip its invariant fails the test. That is not hypothetical here, because the first run of the inv9 test
FAILED,
because the fault as first written (crediting the negative delta on a downward reclassification) did not
actually break the accounting, since accounting legitimately drops to the new smaller worst case. The
fault was corrected to model the real round-11 hazard, an upward reclassification that raises the worst
case but skips the charge, leaving accounted below worst case, and the test then passed in both
directions. A test that had never been watched failing would not have caught the modelling error.

The twelve tests, all passing:

- inv1 accounting: accounted(o) at or above worst_case_work(o) for live and queued objects.
- inv2 flow: admitted positive drain-lane deltas per block at or below R.
- inv3 partition: known_due + R + overdue at or below C_total.
- inv4 conservation: a transfer debits once and credits once.
- inv5 release timing: funding released at reclamation, neither before (queued) nor never (terminal).
- inv6 refusal: a refused transition leaves state bit-identical.
- inv7 deadline: dated work due at a height is completed by the drain at that height.
- inv8 gate: an irrevocability burst beyond the known_due share fails atomically with no artifact.
- inv9 reclassification: an upward class change that skips its charge is caught (accounted below worst
  case).
- inv10 disposition: mass retirement drives each object to exactly one terminal disposition.
- Two certified scenarios: a ten-hop transfer chain stays bounded and accounted across blocks, and the
  round-12 equal-aggregate different-lane reclassification admits positive flow rather than reading as
  zero-growth.

## Claim width

These are model-level results. They demonstrate that the meter arithmetic as specified UPHOLDS the
certified invariants and that the invariants are strong enough to CATCH the specific defects each fault
injects. They are not a proof of the design and not platform-measured; the model uses Phase A's storage
unit but its own illustrative capacities. Phase B does not price compute (phase E) or run against the
real store. What it establishes is that the D3b meter, translated from prose to executable arithmetic,
is internally consistent and that its ten invariants are individually load-bearing.

## Next

Phase C builds the per-class record catalog against the real store and runs each class's full
terminalization, and phase D drives the certified scenarios under synthetic load. The meter core here is
the arithmetic those phases will exercise with measured, rather than illustrative, vectors.

# Phase C results, per-class terminal-work vectors measured against the real store

Dated 2026-08-11. Phase C builds a minimal native record for each D15 disposition class in a REAL
GroveDB at the frozen candidate (v5.0.0, 9b98a356) and runs each class's full terminalization, capturing
the cumulative OperationCost per dimension. This replaces Phase B's illustrative per-class vectors with
measured ones. The binary is `metering-prototype/src/phase_c.rs`; raw output is
`metering-prototype/results/phase_c_output.txt`. Density is 256 filler siblings per subtree. No VM, no
consensus, no networking, just the store and the lifecycle operations.

## Measured vectors (added and removed bytes are the durable dimensions)

| Class | fan_out | deposit added | terminal removed | terminal added | terminal replaced |
| --- | --- | --- | --- | --- | --- |
| single_owner | 0 | 568 | 568 | 0 | 4073 |
| autonomous | 1 | 568 | 568 | 162 | 4325 |
| autonomous | 8 | 568 | 568 | 1296 | 8339 |
| autonomous | 64 | 568 | 568 | 10368 | 63576 |
| irrevocable_request | 0 | 568 | 568 | 170 | 4325 |
| deferred_task | 0 | 718 | 718 | 0 | 5224 |
| hook_binding | 0 | 718 | 718 | 0 | 5224 |
| lease_exit | 0 | 718 | 718 | 147 | 6375 |

The record is a ~121-byte payload (owner, subject reference, class descriptor, the six-component vector,
funding state) plus its secondary indexes. Classes with an auxiliary index (deferred task, hook binding,
lease exit) carry a larger deposit (718 vs 568 durable bytes).

## Two measured properties worth stating

1. RECLAMATION CONSERVATION. For every class, the bytes removed at terminalization exactly equal the
   bytes added at deposit (568 = 568 for the three simple classes, 718 = 718 for the three indexed
   ones). What a record durably adds when it is created is exactly what its terminalization reclaims.
   This is a measured cross-check on the accounting premise the meter rests on, that terminal work is a
   real, recoverable quantity rather than an estimate.
2. DISTRIBUTION IS LINEAR IN FAN-OUT. The autonomous class's terminal added-bytes scale linearly with
   the declared payout fan-out at 162 durable bytes per recipient (162 at fan-out 1, 1296 at 8, 10368 at
   64, each within rounding of 162 times the count). The design's statement that autonomous distribution
   scales with declared fan-out is now a measured slope. The single-owner class, by contrast, adds ZERO
   durable bytes at terminalization (it is cleanup only), confirming the v12 lane attribution that a
   single-owner balance discharge is owner-paid and not drain-lane work.

## What this settles and what it does not

Phase C settles the per-class STORAGE-dimension vectors (VERIFY item 2) as measured, not estimated, and
gives the meter concrete class vectors to carry instead of Phase B's illustrative ones. It does not
price the VM-execution (compute) dimension, which needs phase E, and its propagation figures (the
replaced-bytes column) are real measured costs but should be read with Phase A's caution that the
worst-case estimator over-predicts propagation, so denominating that dimension for admission wants the
average-case estimator or a measured bound rather than the worst case. These are harness-measured
numbers on the frozen store, not platform-execution-path numbers, and DESIGN.md is untouched.

## Next

Phase D drives the certified scenarios (transfer chains, reclassification, mass retirement, clustered
irrevocability) under synthetic load, using these measured class vectors in the Phase B meter
arithmetic,
to produce the drain-rate and throughput surfaces (VERIFY items 1, 4, 6). Phase E, optional, bounds the
compute dimension with an interpreter microbenchmark.

# Phase D results, certified scenarios under synthetic load

Dated 2026-08-11. Phase D drives the Phase B meter with the Phase C MEASURED per-class vectors under
synthetic load, turning the drain rate R and the flow ceiling from policy dials into measured throughput
and backlog curves (VERIFY items 1, 4, 6). The binary is `metering-prototype/meter-core/src/bin/
phase_d.rs`; raw output is `metering-prototype/results/phase_d_output.txt`. The Phase B invariant suite
still passes 12 of 12 after the two methods Phase D added (create-with-measured-vector and
direct-enqueue), so the load driver rests on the same certified arithmetic.

Denomination. Drain-service cost is denominated in the measured PROPAGATION dimension (GroveDB
replaced_bytes from Phase C's terminal rows), which Phase A and C identify as the dominant, binding cost
of a cleanup. R is a per-block budget in those units. The representative budget is R = 40,730
propagation units per block, about ten single-owner cleanups per block. The multi-dimensional budget is
a
later refinement.

## D1, the flow ceiling bounds the backlog (VERIFY 4)

Offered load was applied for 200 blocks, once through the governed meter (admission enforced against R)
and once ungoverned (admission bypassed), at offered rates from half of R to 1.5 times R.

| Offered load | Governed max backlog | Ungoverned max backlog |
| --- | --- | --- |
| 50% of R | 0 | 0 |
| 90% of R | 0 | 0 |
| 100% of R | 0 | 0 |
| 150% of R | 0 | 1000 |

At 1.5 times R the ungoverned queue grows without bound (1000 items after 200 blocks, the expected 5 per
block times 200), while the governed meter throttles admission to R and holds the backlog at zero. The
flow ceiling is what keeps the backlog bounded, measured as a contrast rather than asserted.

## D2, drain throughput per class (VERIFY 1)

At R = 40,730 propagation units per block, and the Phase 0 under-load cadence of about 0.5 s per block
(about 172,800 blocks per day):

| Class | W (prop units) | Throughput per block | Per day |
| --- | --- | --- | --- |
| single_owner | 4073 | 10 | ~1.73M |
| autonomous (fan=1) | 4325 | 9 | ~1.56M |
| autonomous (fan=8) | 8339 | 4 | ~691k |
| autonomous (fan=64) | 63576 | 0 | 0 |
| irrevocable_request | 4325 | 9 | ~1.56M |
| deferred_task | 5224 | 7 | ~1.21M |
| hook_binding | 5224 | 7 | ~1.21M |
| lease_exit | 6375 | 6 | ~1.04M |

One real finding. A single autonomous terminalization at fan-out 64 costs 63,576 propagation units, MORE
than the entire per-block drain budget of 40,730, so it cannot be serviced in one block at this R (shown
as 0 per block, meaning fewer than one). This is not a defect but a constraint the design already
anticipates, since high-fan-out distribution work must either be serviced across blocks by the batched
drain
(the per-batch latency W dial from VERIFY item 1) or provisioned against a larger R. The measurement
locates the exact fan-out at which a single terminalization exceeds a one-block budget, which is the
kind
of dial-setting input the prototype was built to produce.

## D3, the partition point (VERIFY 6)

C_total was held fixed and split across the deadline-free R share, the dated known_due reserve, and the
overdue reserve. Raising R buys deadline-free throughput at the cost of dated capacity:

| R share of C_total | Deadline-free per block | Dated capacity per block |
| --- | --- | --- |
| 20% | 5 | 18 |
| 40% | 10 | 14 |
| 60% | 15 | 9 |
| 80% | 20 | 4 |

The invariant holds at every split by construction; the split only tunes the tradeoff. The measured
surface is what a workload model (VERIFY item 3, the arrival and deadline-clustering of real requests)
would index into to pick the split.

## D4, mass-retirement load test

A large backlog was enqueued and drained at R, with all ten invariants checked at every block boundary:

| Objects retired | Drained in (blocks) | Predicted | Invariants |
| --- | --- | --- | --- |
| 1,000 | 100 | 100 | held every block |
| 10,000 | 1,000 | 1,000 | held every block |
| 100,000 | 10,000 | 10,000 | held every block |

The drain clears the backlog in exactly the predicted number of blocks (count times W over R, rounded
up), and no invariant is violated at any block across a 100,000-object, 10,000-block run. This is the
mass-retirement scenario the review rounds worried about, run at scale against the measured arithmetic.

## What Phase D settles and what it does not

Phase D converts VERIFY items 1 (drain-rate throughput), 4 (the flow ceiling bounds the backlog), and 6
(the partition tradeoff) from open to harness-measured, denominated in the Phase C propagation unit. It
also locates a concrete design-relevant threshold, the fan-out at which one terminalization exceeds a
one-block budget. What it does not do: it uses the single binding (propagation) dimension rather than
the
full GroveDB cost vector, it does not model a realistic application workload mix (VERIFY item 3, a
modeling task), and it does not price the VM-execution dimension (phase E). These are model-and-store
results, not platform-execution numbers, and DESIGN.md is untouched.

## The prototype track, complete through Phase D

Phase A calibrated the cost unit against GroveDB's shipped estimators, Phase B proved the meter
arithmetic upholds ten invariants each watched failing first, Phase C measured the per-class vectors on
the real store, and Phase D drove those vectors under load to produce the drain-rate, flow-ceiling, and
partition curves. The remaining prototype work is the optional phase E compute bound and the workload
model (VERIFY item 3), neither of which blocks the dial-setting the design listed as open.

# Phase E results, the compute dimension bounded

Dated 2026-08-11. Phase E bounds the COMPUTE dimension of terminal work, the one dimension Phases A, C,
and D could not price without a VM. It runs a fixed integer workload (an LCG-style 64-bit mixing loop,
standing in for the deterministic integer settlement math a terminalization runs) through an off-the-
shelf deterministic WebAssembly interpreter (wasmi) with FUEL metering on, and reports the fuel
consumed.
Fuel is a deterministic count of executed operations, not a wall-clock time, so it is a consensus-safe
unit, which is what a metered compute dimension requires. The crate is
`metering-prototype/compute-bench/` (dependencies wasmi and wat only, no store, no consensus, no
networking); raw output is `metering-prototype/results/phase_e_output.txt`.

## Measured fuel per workload size

| Iterations | Fuel | Fuel per iteration |
| --- | --- | --- |
| 0 | 21 | (fixed overhead) |
| 1 | 37 | 37.00 |
| 8 | 149 | 18.62 |
| 64 | 1045 | 16.33 |
| 256 | 4117 | 16.08 |
| 1024 | 16405 | 16.02 |
| 8192 | 131093 | 16.00 |

Each size was run twice and required identical fuel, so the unit is reproducible by assertion, not just
in practice.

## The compute unit

The marginal fuel per loop iteration, taken between the two largest sizes where the fixed per-call
overhead is negligible, is 16.00 fuel per integer settlement step. There is a fixed per-call overhead of
about 21 fuel (the zero-iteration case) that amortizes away as the workload grows. So the compute
dimension has exactly the denominated, boundable form the storage dimension has, in that a class's
compute terminal-work component is (the number of settlement steps it runs) times about 16 fuel, for
example an
autonomous distribution over N recipients costs on the order of N times 16 fuel. This closes the compute
column of the per-class terminal-work vector that Phase C left open.

## What Phase E settles and what it does not

Phase E settles that the compute dimension has a deterministic, reproducible, linear unit (fuel), so the
terminal-work vector's compute component is measurable and boundable rather than an open unknown. It
does
NOT claim an absolute VM throughput (fuel-to-time depends on the production runtime, the host, and the
credit-to-fuel schedule, none of which exists yet), and it uses a single representative integer kernel
rather than a catalog of real settlement programs. It is a unit and its scaling law, measured against a
real deterministic Wasm interpreter, not a performance benchmark of a VM that has not been built.

## The prototype track, complete through Phase E

All four resource dimensions the design's terminal-work meter needs now have measured units. Phase A
calibrated the storage unit against GroveDB's shipped estimators, Phase C measured the per-class storage
vectors on the real store, Phase D drove those vectors under load for the drain-rate, flow-ceiling, and
partition curves, and Phase E bounds the compute dimension in deterministic fuel. The remaining open
item
is not a measurement but a modeling task, the application-workload mix (VERIFY item 3), which indexes
into the Phase D partition surface, plus the network survey (VERIFY item 7) that no repository work can
produce.
