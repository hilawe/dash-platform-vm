# The remaining VERIFY numbers, classified and bounded

Dated 2026-08-10, written at the close of the review loop (DESIGN.md v12). The design's mechanisms are
review-complete but denominated in numbers nobody has measured. This document takes each remaining
VERIFY number, classifies what kind of work would actually produce it, derives bounds or estimates where
the Phase 0 measurements permit, and states assumptions next to every derived figure. Evidence grades
follow the shared assurance program, in which MEASURED means execution-produced in Phase 0, READ means
repository-resolved from the frozen candidate's source, ESTIMATE means derived here from measured or
read inputs under stated assumptions, and OPEN means nothing yet supports a number.

The one-line conclusion is that none of the remaining numbers requires the full VM, and most require one
artifact, a METERING PROTOTYPE, a harness that defines the terminal-work unit per resource dimension
over real GroveDB operations and exercises the drain, admission, and reservation paths against the
frozen candidate. The store's own worst-case estimators (READ, they exist and ship in GroveDB) are the
starting point the design already names.

## Classification key

- A. Boundable now from Phase 0 inputs (estimates in this document).
- B. Requires the metering prototype.
- C. Requires a workload or economic model (a policy choice informed by modeling, not a measurement).
- D. Requires data the repository cannot produce (a network survey).

## The measured and read inputs everything below draws on

- MEASURED: per-operation aggregate fees on the live local network, identity create about 116M credits,
  data contract create about 12B, document create about 9.5M.
- MEASURED: per-block execution slice about 158 ms under one-transaction load, a light-load floor across
  separately-timed ABCI phases, finalize-block dominant; under-load block cadence about 0.5 s versus
  about 195 s idle.
- READ: durable storage price 27,000 credits per byte, amortized over 50 eras with early-discharge
  refund; storage cost is a five-field vector; GroveDB ships worst-case and average-case cost
  estimators; provable count and sum trees exist natively; the maximum state transition size is 20,480
  bytes; Tenderdash imposes no execution gas limit of its own.

## 1. The drain rate R and the per-batch latency W (D3b), classification B for R and C for W

R is the reserved per-block service rate for deadline-free terminal work, one share of the
cleanup-and-terminal allocation C_total, which itself sits inside the block's physical capacity vector
alongside the one-fifth cap and the ordinary-execution minimum. What bounds it today is the certified
fact that the
capacity invariant requires C_total plus every other reserved maximum plus the one-fifth allowance to
fit the physical vector, so R is necessarily a minority share of block capacity. The measured light-load
execution slice (about 158 ms per block against an under-load cadence of about 0.5 s) says the platform
currently runs with substantial execution headroom, but that figure is a floor under trivial load and
does not by itself justify any particular R. Producing R requires the metering prototype, because R is
denominated in terminal-work units that do not exist until the prototype defines them per dimension and
measures what a block's worth of drain service actually costs. W, the per-batch latency, is a policy
dial (how many blocks a batch may span) whose only constraint is user-facing latency tolerance, so it is
a choice informed by modeling rather than a measurement.

UPDATE 2026-08-11, HARNESS-MEASURED (Phase D, see `docs/METERING_RESULTS.md`). R is now denominated in
the measured propagation unit and its throughput measured per class. At a representative R of 40,730
propagation units per block, the drain clears about 10 single-owner cleanups per block (and 9, 4, then
under 1 for autonomous at fan-out 1, 8, 64), which at the Phase 0 cadence is on the order of 1.7 million
single-owner terminalizations per day. R itself stays a policy choice (a minority share of block
capacity), but it is no longer dimensionless, and any candidate R now maps to a concrete per-class
throughput. Phase D also locates the batched-drain trigger for W: a single autonomous terminalization at
fan-out 64 costs 63,576 units, more than a one-block budget, so above that fan-out one terminalization
must span multiple blocks, which is the user-facing latency the W dial governs.

## 2. Per-class worst-case terminal-work vectors (D3b 2a), classification B with A for storage

The storage components of each class's vector are boundable now, because GroveDB's worst-case estimators
exist (READ) and the record shapes are known. The VM-execution components require the prototype, since
no terminal-work unit exists yet for compute. The classes to cover, from the D15 disposition table, are
single-owner custody (cleanup only under the v12 lane-attributed definition), autonomous custody
(cleanup plus protocol-driven distribution, scaling with declared fan-out), irrevocable-request
settlement (fixed recipe from the pre-materialized finalization record), deferred tasks, hook bindings,
and lease and exit records (tombstone plus index reclamation).

ESTIMATE, minimal custody obligation record size, an input the record has carried since Phase 0. A
minimal native record (owner, subject references, class descriptor, vector, funding state) plausibly
occupies on the order of 100 to 500 durable bytes. At the read price of 27,000 credits per byte that is
a deposit-time durable charge on the order of 2.7M to 13.5M credits, that is, commensurate with one
document creation (measured at about 9.5M credits aggregate). The assumptions are that the record is a
single GroveDB item plus a small constant number of index entries, and no per-class padding dominates.
The
estimate says the deposit-time charge is unlikely to be a usability problem; it does not price the
distribution or cleanup work, which is the prototype's job.

UPDATE 2026-08-11, HARNESS-MEASURED (Phase A, see `docs/METERING_RESULTS.md`). The metering prototype
measured the durable footprint of a single GroveDB item directly: 178 bytes for a 32-byte payload, 275
for 128 bytes, 661 for 512 bytes, and the worst-case estimator predicts this dimension EXACTLY (1.00x)
and independently of tree density. This confirms the storage components of the per-class vectors are
boundable now at zero over-reservation and narrows the estimate above to measured points. The
propagation dimension (parent-hash rewrites) is a separate story, because the worst-case estimator
over-predicts it by 160x to 840x because it assumes maximum node sizes, so that dimension must be
denominated by the average-case estimator or a tighter bound rather than the worst case. The compute
and distribution components still require the later phases.

UPDATE 2026-08-11, HARNESS-MEASURED (Phase C, see `docs/METERING_RESULTS.md`). The per-class vectors
themselves are now measured on the real store. Each of the six D15 classes was built as a minimal native
record and fully terminalized: the durable deposit is 568 bytes for the three simple classes and 718 for
the three carrying an auxiliary index, and terminalization reclaims exactly the deposited bytes (a
measured reclamation-conservation property, removed equals added for every class). The autonomous
class's
distribution scales linearly at 162 durable bytes per declared recipient, and the single-owner class
adds
zero durable bytes at terminalization, confirming its balance discharge is owner-paid and off the drain
lane. This upgrades the storage components of item 2 from ESTIMATE to MEASURED per class. The compute
(VM-execution) component still requires phase E, and the propagation column carries the caveat above.

UPDATE 2026-08-11, HARNESS-MEASURED (Phase E, see `docs/METERING_RESULTS.md`). The compute component now
has a unit. A fixed integer workload run through a deterministic Wasm interpreter (wasmi) with fuel
metering consumes 16.00 fuel per integer settlement step (with a fixed per-call overhead of about 21
fuel that amortizes away), deterministic and reproducible by assertion. So a class's compute
terminal-work component is (settlement steps) times about 16 fuel, giving the compute column of the
per-class vector the same denominated, boundable form as the storage column. This does not fix an
absolute VM throughput (fuel-to-time and the credit-to-fuel schedule need the production runtime), but
it
closes the last dimension's UNIT question. Every dimension of item 2 is now measured or unit-bounded.

## 3. The known_due share and irrevocability throughput (D3b 3, D16), classification C then B

The gate itself is certified (a burst that exceeds the share fails atomically to cancelable, which is
safe), so the open question is throughput, how many requests per block may become irrevocable under a
given share. The shape is arithmetic, with the share admitting (share capacity at height h) divided by
(worst-case settlement work per request) irrevocability transitions targeting h. What is missing is a
workload model, the expected arrival rate and deadline clustering of asynchronous requests, which is an
application-mix assumption, not a repository fact. The same model decides whether the disclosed
create-time-reservation variant or the authority-maintenance subshare are warranted. The recommended
treatment is to parameterize now and confirm with the prototype under synthetic load later.

## 4. The deadline-free flow ceiling matched to R (D3b 2a), classification B

The rate-matching invariant (per-block admitted positive terminal-work deltas at or below R) is
dimensionless until the terminal-work unit exists. Once the prototype defines the unit and measures R,
the ceiling is R by construction, and the only residual question is whether holding creation flow at or
below R starves legitimate creation at realistic application mixes, which is the same workload model as
item 3.

UPDATE 2026-08-11, HARNESS-MEASURED (Phase D, see `docs/METERING_RESULTS.md`). The flow ceiling was
driven under load. At offered creation load of 1.5 times R over 200 blocks, the governed meter throttles
admission to R and holds the backlog at zero, while an ungoverned queue at the same offered load grows
to
1000 items (5 per block times 200). The rate-matching invariant is confirmed as a measured contrast, not
just by construction. The starvation question (whether the ceiling delays legitimate creation) remains
the item-3 workload model.

## 5. The creation budget peak velocity and surcharge curve (D6), classification A for bounds and C for
the curve

ESTIMATE, an upper bound on adversarial creation velocity and its floor cost. At the measured under-load
cadence of about 0.5 s per block, a year is on the order of 63 million blocks. An adversary filling a
per-block permanent-byte budget of B bytes pays at least 27,000 times B credits per block in durable
charges alone (READ price), before processing fees. For example, at B equal to 1,024 bytes per block the
durable floor cost is about 27.6M credits per block, about 1.7 trillion credits per day of sustained
filling, while accreting about 84 MB of permanent state per year. The assumptions are that the attacker
pays the full durable price with no refund (abandonment forfeits the early-discharge refund), and the
cadence
holds at sustained load. The estimate frames the policy choice, since B trades acceptable long-run
unfunded-state growth against the cost wall an attacker faces; the surcharge curve on top of it is an
economic-model choice (C), not a measurement.

## 6. The cleanup partition point (D3b 4), classification B plus C

Where to split C_total among known_due, the R share, and the overdue reserve depends on the realistic
density of dated obligations (workload model) and the measured cost of drain service (prototype). The
invariant holds at any split by construction; the split only tunes throughput.

UPDATE 2026-08-11, HARNESS-MEASURED (Phase D, see `docs/METERING_RESULTS.md`). The partition tradeoff
was
measured. Holding C_total fixed and denominating in the Phase C propagation unit, an R share of 20, 40,
60, and 80 percent of C_total yields deadline-free throughput of 5, 10, 15, and 20 single-owner cleanups
per block against dated capacity of 18, 14, 9, and 4 per block respectively. The invariant holds at
every
split, so the prototype half of this item is measured, and the split point is now a lookup into this
surface once the item-3 workload model supplies the realistic density of dated obligations.

## 7. Masternode hardware distribution, classification D

Named since Phase 0: this needs a network survey, and no repository work produces it. It matters for
setting conservative per-block budgets, and it remains open.

## 8. Items resolved or contingent, listed so nothing silently drops.

- Dependency-graph width for D17, resolved in Phase 0 as mis-specified; D17 was decided on reasoning
  with a revisit trigger, and no number is pending.
- The authority-maintenance subshare and the create-time-reservation variant, contingent options
  triggered only if the item-3 workload model shows the need.
- The keeper-incentive optimization for the drain, noted in the record, not built, no number pending.

## What the metering prototype actually is (the recommended next artifact)

A harness rather than a VM. Define the terminal-work unit per resource dimension over real GroveDB
operations
on the frozen candidate (the shipped worst-case estimators are the base), implement the meter's
admission arithmetic (dated buckets, the R drain, the flow condition, the partition equation), and
exercise the certified scenarios from the review record (mass retirement fan-out, transfer chains,
reclassification in both directions, clustered irrevocability) under synthetic load. That one artifact
converts items 1, 2, 4, and 6 from open to measured, and upgrades this document's estimates to numbers.
