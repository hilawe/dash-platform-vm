# Metering prototype specification

Dated 2026-08-10. This is the specification for the artifact `docs/PHASE0_VERIFY_ESTIMATES.md` names as
the recommended next step, a harness that converts the design's remaining VERIFY numbers from estimates
to measurements. It is written to be buildable by a session that has read nothing else, so it restates
the constraints it depends on. Written before any code, per the write-time discipline, so that the
invariants are enumerated before implementation and every test is specified before it exists.

## 1. Purpose, and the one-sentence scope rule

The prototype defines the TERMINAL-WORK UNIT per resource dimension over real GroveDB operations on the
frozen candidate, implements the D3b meter's admission arithmetic exactly as the review-complete record
states it, and exercises the scenarios the twelve review rounds certified, under synthetic load, so that
the drain rate R, the per-class terminal-work vectors, the flow ceiling, the cleanup partition point,
and the creation-budget bounds become measured quantities. The scope rule is that the prototype is a
HARNESS, NOT A VM. It contains no WebAssembly execution, no consensus, no networking, and no Tenderdash.
Anything that would require those is out of scope and is listed in section 9 with its own path.

## 2. What it answers (the measurement matrix)

Each row names a VERIFY item from the design record, the experiment that produces it, and the output.

| Item | Experiment | Output |
| --- | --- | --- |
| Terminal-work unit per dimension | Calibrate GroveDB worst-case estimators against measured operation costs (section 5, phase A) | A unit definition per dimension, with estimator error bars |
| Per-class worst-case vectors | Build each D15 class's minimal record shape, run its full terminalization, measure per dimension | A vector per class, storage dimensions measured, compute dimension bounded (section 9) |
| Drain rate R candidates | Sweep the drain at candidate R values under backlog, measure per-block service cost | The service cost of a block's worth of drain at each candidate R |
| Flow ceiling matched to R | Drive creation flow at, below, and above R; observe backlog trajectories | Confirmation that flow at or below R bounds the backlog, and the starvation margin |
| known_due share throughput | Fill dated buckets with clustered deadlines at candidate share sizes | Irrevocability admissions per block per share size, and the refusal behaviour at the gate |
| Cleanup partition point | Sweep the split of C_total across workload mixes | Throughput surfaces over the split, per mix |
| Creation-budget bounds | Adversarial creation at the per-block byte budget | Measured cost wall and state-growth rate, refining the estimates document |
| Minimal obligation record size | Serialize each class's minimal record with platform conventions | Exact durable bytes per class, replacing the 100-to-500-byte estimate |

## 3. Environment and dependencies

- The frozen candidate governs everything: GroveDB v5.0.0 at 9b98a356, and the Platform v4.0.0 workspace
  at 9f9092cc for record conventions, fee constants, and the five-field storage cost vector (the crates
  of interest are rs-drive, rs-dpp, and rs-platform-version, used as libraries).
- The harness is a Rust workspace that links GroveDB directly as a library against a local RocksDB
  store. No node daemon runs, so the standing host constraint on long-running self-built daemons does
  not bite; a short-lived test binary is acceptable natively, and the containerized stack (colima plus
  the existing dev container) is the fallback if the host interferes with anything. Build and first run
  should be attempted natively and moved to the container only on evidence of interference.
- Determinism note for the harness itself: measurements of wall-clock cost vary run to run, so every
  timed experiment reports a distribution over repeated runs (minimum, median, and a high percentile),
  and unit definitions are anchored to operation counts and estimator outputs first, with wall-clock as
  the calibration check, not the unit.

## 4. Components

- **The unit layer.** Defines the terminal-work unit per resource dimension. The dimensions follow the
  platform's existing storage cost vector plus processing, namely storage bytes added, storage bytes
  replaced,
  storage bytes removed, loaded bytes, hash node computations, and processing. For each GroveDB
  operation the harness records the shipped worst-case estimator's prediction, the shipped average-case
  prediction, the actual operation counts, and wall-clock, so the unit is grounded in the estimators the
  design already names and their error is measured rather than assumed.
- **The class catalog.** One minimal record shape per D15 disposition class, built with rs-dpp
  serialization conventions, covering single-owner custody (with the v12 lane-attributed vector,
  cleanup lane only), autonomous custody (parameterized by declared payout fan-out), the irrevocable
  request with its
  pre-materialized finalization record, the deferred task, the hook binding, and the lease and exit
  records. Each class implements its full terminalization path as the disposition table states it.
- **The meter core.** The D3b arithmetic exactly as written in DESIGN.md v12: the class-dependent,
  lane-attributed worst-case vector as an authenticated value on every object; dated known_due buckets;
  the rate-R drain; the overdue reserve; the flow condition (per-block admitted positive deltas at or
  below R, per dimension, destination-lane attributed); the partition equation known_due plus the R
  share plus overdue at or below C_total; and admission refusal paths. Counters use the native CountTree
  and SumTree primitives so the authenticated-counter claim is exercised, not simulated.
- **The lifecycle driver.** Implements every transition the review record certified: creation with
  admission, footprint-growing mutation with marginal charge, in-place reclassification in both
  directions, discharge-and-recreate transfer with the accounting handoff, the logical pull with cleanup
  enqueue, the retirement generation marker with phased fan-out, drain processing under the one-fifth
  bound per batch, and early completion from idle capacity only.
- **The load generator.** Deterministic synthetic workloads keyed by seed: steady mixed creation, bursty
  creation at the flow ceiling, adversarial patterns taken from the review record (transfer chains,
  reclassification alternation, principal-split creation floods, clustered irrevocability, mass
  retirement into a full backlog).
- **The invariant checker.** Every certified invariant as a runtime assertion evaluated after every
  block, per the standing rule that load-bearing design properties become assertions. The assertion list
  is section 6, and a violation halts the run and dumps the trace.

## 5. Phases, each with a done-condition

- **Phase A, calibration.** Link GroveDB, replay a corpus of representative operations (inserts,
  deletes, index maintenance, proofs, count and sum tree updates) at several tree densities, and record
  estimator-versus-actual for each dimension. Done when the unit definition is written down with
  measured estimator error at three densities, because density sensitivity is what the round-8 findings
  turned on.
- **Phase B, meter core plus invariants.** Implement the meter arithmetic and the assertion suite over
  an in-memory block loop. Done when every assertion in section 6 has been watched FAILING against a
  deliberately broken variant before passing against the correct one, per the mutation-check rule that a
  test does not exist until it has failed.
- **Phase C, classes and lifecycle.** Implement the class catalog and the lifecycle driver. Done when
  the certified model-checking scenarios (section 7) all run green and each class's measured
  terminalization cost is recorded per dimension.
- **Phase D, load and measurement.** Run the measurement matrix (section 2) under the load generator.
  Done when every matrix row has a measured output with distributions, committed to a results document.
- **Phase E, optional compute bound.** A minimal interpreter microbenchmark (an off-the-shelf
  WebAssembly interpreter running a fixed integer workload) to bound the processing dimension's unit for
  terminal-work purposes only. Optional because every phase-A-through-D output is meaningful without it,
  and flagged so its absence is never silently treated as coverage.

## 6. The invariant list (the checker's contract)

1. accounted(o) at or above worst_case_work(o) for every live object AND every queued cleanup item, per
   dimension, at every block boundary.
2. Per block and per dimension, the sum of admitted positive destination-lane deltas is at or below R
   for the deadline-free lane.
3. known_due(h) plus the R share plus overdue(h) at or below C_total at every height, and the floor is
   satisfied whenever due work exists.
4. Conservation on transfer: the balance is debited once and credited once, the old position ends
   discharged, its cleanup item exists with its funding, and no interleaving or injected failure leaves
   the balance in both places or neither.
5. No accounting release before the corresponding physical reclamation completes, including across
   partial drain batches under the one-fifth bound.
6. Every refused transition leaves state bit-identical to its pre-state (checked by root hash).
7. Deadline-bearing settlement admitted to a dated bucket completes by its height under every drain
   ordering the driver can produce, and early completion consumes only capacity idle after the floor.
8. A burst of irrevocability transitions exceeding the known_due share fails atomically to cancelable,
   with escrow intact and no artifact produced.
9. Reclassification in any direction preserves invariant 1, charges only positive destination-lane
   components, and creates no transferable credit from negative components.
10. After a mass retirement, every frozen object reaches exactly one terminal state, and one user's exit
    never changes another's execution rights (the D15 model check).

## 7. The certified scenarios (regression suite from the review record)

Each scenario is drawn from a reviewer's own confirmation recipe and must pass with all assertions on:
the two-users-one-program-one-environment-one-request retirement enumeration (round 4); the
thousand-transfer chain with cleanup accounting summed across abandoned dependency sets (round 10); the
transfer-while-cleanup-queued and transfer-interleaved-with-pull interleavings (round 11);
reclassification alternation with vector increases and decreases (rounds 9 and 11); the equal-aggregate,
different-lane reclassification (round 12); mass retirement with backlog exceeding R times the terminal
timeout, verifying no deadline-bearing loss (round 8); clustered irrevocability at a full share (round
9); and the saturated-flow reclassification of a zero-growth change (round 9).

## 8. Outputs and evidence handling

The results land in `docs/METERING_RESULTS.md` with the same evidence discipline as Phase 0: every
number is execution-produced BY THE HARNESS, and the claim width is stated on every table, namely that
these are measurements of the meter arithmetic and real store operations on the frozen candidate, not of
the platform's own execution path, so they upgrade the design's VERIFY items from estimate to
harness-measured, not to platform-measured. The corresponding VERIFY entries in DESIGN.md and the status
table in `docs/PHASE0_FINDINGS.md` are updated only after the results document exists, and per the
standing rule, folding any resulting change into the design record itself would require a fresh
adversarial round.

## 9. Out of scope, stated so nothing silently drops

- Real program execution costs (needs the VM; phase E bounds the unit, nothing more).
- Consensus timing, block propagation, and the true block cadence under network load (needs a network).
- Masternode hardware distribution (needs a survey; classification D in the estimates document).
- The workload and economic models (the known_due share sizing against real application mixes and the
  surcharge curve remain modeling choices that the harness can evaluate but not decide).
- Any change to DESIGN.md v12 (review-complete; the fresh-round rule applies).
