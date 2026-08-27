# Execution layer for Dash Platform, design synthesis (v12, round-11 folded)

v12 synthesized 2026-08-10. Lineage: v1 came from a clean-room round of five independent designs, v2
folded review round 1, v3 folded round 2, v4 folded round 3, v5 folded the architecture-level items of
round 4, v6 folded round 5, v7 folded round 6, v8 folded round 7, v9 folded round 8, v10 folded round 9,
v11 folded round 10, and v12 folds round 11. Each review round drew reviewers from three independent review sources,
with the verbatim
output of every round
retained in the project's review record. FORCED marks clean-room consensus. CHOSEN marks deliberate picks. VERIFY
marks facts to confirm against the real codebase. Purpose is exploration, so nothing is a commitment.

**What v12 folds (round 11, four samples across three sources, two sources returning a clean
approval, with the adjudication retained in the project's review record).** Round 11 certified every v11 edit in detail
across
sources (the accounting handoff, the bidirectional reclassification rule, the narrowed concentration
claim) and produced exactly one finding, adjudicated as follows: the claimed lane-attribution mechanism
defect was REFUTED (the marginal rule's own distribution-fan-out example shows the vector is
class-dependent, so work a reclassification moves into the rate-R lane is charged by construction, a
reading three source voices certified), but a TEXT AMBIGUITY was verified, since the class dependence
and
lane attribution were implicit in one example. v12 is a CLARITY-ONLY fold of two edits: the
terminal-work
vector's definition in the meter (2a) now states explicitly that it is CLASS-DEPENDENT and
LANE-ATTRIBUTED, recording per dimension the work the current ownership class imposes on each service
lane, with reclassification recomputing the vector for the new class; and the reclassification bullet's
zero-growth clause is tightened to zero growth in the destination lanes' attributed vector. No mechanism
changed. This is NOT review-complete, because a fold is not a fresh pass, so v12 needs one closing
confirmation round over one clarified definition and one tightened phrase. The numbers are unchanged,
all Phase 0 measurements listed in VERIFY.

**A Phase 0 correction layer is applied on top of v5, and it is not a v6.** Phase 0 measures the real
codebase rather than reviewing the design, so its results are a different kind of input and are marked
in place with PREMISE CORRECTED, CONFIRMED, or ANSWERED rather than folded as a new synthesis version.
The first measurement session (2026-08-08) corrected one premise in D11 without changing its decision,
CONFIRMED the atomicity assumption D16 rests on and recorded its mechanism plus two rules as D16a,
partially answered the D6 obligation horizon, and confirmed D3's multidimensional metering against the
real cost model. Measurement added one decision (D16a) and changed none. Evidence for every Phase 0
claim, with file and line, is in `docs/PHASE0_FINDINGS.md`, which is
authoritative for measurement status. No design decision was changed by measurement alone; where a
measurement bears on a decision, it went to the owner.

Evidence status for the whole document, stated once. Every decision here rests on design reasoning
and independent review, not on measurement or execution. There is no implementation to run, so no
claim below has been tested against a working system, and every quantitative rule has at least one
unmeasured denominator. Phase 0 of `TODO.md` is what would change that. Findings folded from a
review round are resolved in the design record only, which is a weaker status than resolved in a
system, and the distinction is kept explicit wherever it matters.

Review findings are attributed by REVIEW SOURCE rather than by reviewer count throughout, because two
samples from one source agreeing is one voice. Where sources disagreed about a finding, the
disagreement and its resolution are recorded rather than smoothed away.

## What round 4 changed, and what v5 leaves open (read this first)

Round 4 was the fresh full pass v4 asked for. Four reviewers across three sources returned two
accept-with-fixes and two block, and because both blocks came from the two samples of a single
source, the split is two sources accepting with fixes against one source blocking.

Round 3's convergence signal held. No reviewer opened a new architectural front, and both samples of
the blocking source published "areas that survived" sections certifying five things: the adapter cut
as semantically cleaner than the v3 adapter it replaced, exact environment binding as sound when
"exact" means equality at inclusion, the pre-materialized finalization record as removing the need to
call retired code, the absolute one-fifth allowance as fixing the earlier load-dependent cap, and the
logical expiry view with its expiry-aware index.

What round 4 attacked successfully was not the shape of the design but the completeness of two things
v4 introduced and one thing it removed. Retirement named a subject without establishing that the
subject is unique. Exit evidence was promised protection by an event that occurs later than the
threat it protects against. And cutting the adapter removed a call path without stating what becomes
of a program whose dependency cannot follow it to a new environment.

v5 folds six architecture-level items and no others. Support is stated by source, since that is what
carries weight.

- **Retirement subjects made unique and exhaustive (D15).** Raised by one source in both samples, as a
  blocker, with a concrete two-user reproduction. A second source had explicitly cleared this ground
  as free of ownership races. The disagreement is resolved in favor of folding, because the finding
  supplies a specific scenario (one program version holding balances for two users) while the
  clearance was a general assessment that did not address shared custody.
- **Exit evidence becomes a non-expiring obligation from deposit (D6, D15). HORIZON HALF SUPERSEDED by
  Phase 0, 2026-08-08.** Two sources contributed opposite halves of one problem. One found that
  evidence promised a retirement pin can expire and be physically deleted before retirement ever
  happens, which locks an asset or destroys replay protection. The other found that once pinned, an
  abandoned record's finite prepayment cannot fund unbounded storage. v5 folded both, pairing a
  non-expiring obligation with a finite absolute horizon. The non-expiring half STANDS. The finite
  horizon is now REMOVED, because measurement showed the platform answers the second objection at the
  payment layer rather than with a deadline, and because every horizon that actually fires must either
  destroy a user's asset or route it through governance. See D6's perpetual-obligation rule below.
- **The signed dependency closure is bounded by manifest allowlists (D13).** Raised by two sources.
  A third cleared it conditionally, and its stated condition amounts to the same fix.
- **The migration source stays authoritative and materialized (D7).** Raised by one source in one
  sample only, which makes it the least corroborated of the five. It is folded because the mechanism
  it describes is checkable by reading v4's own text rather than by measurement, and v4's text does
  permit the state it describes.
- **Migration granularity becomes the dependency component, with admission gating (D17, new).** All
  three sources reached the adapter-cut consequence independently, two as findings and one as a
  recommendation. This is the best-corroborated finding of the round.
- **Terminal work is reserved at creation, not merely prepaid (D3b).** Raised by two sources, which
  makes it the second-best-corroborated finding. Prepayment buys work and does not reserve the block
  capacity to perform it, so retirement disposition queues and migration abort cleanup could be
  deferred indefinitely by ordinary demand. The mechanism needed already existed in D6 for leased keys
  and was simply not applied to the other obligation classes. The structural rule is folded; its
  numbers are Phase 0 measurements.

### The round-4 open item, now FOLDED in v6

v5 left one round-4 finding open: the cleanup service floor could exceed its own allocation. Round 5
established (all three sources) that this was a structural hole, not a missing number, and that the v5
D3b reservation fold made it reachable under ordinary admission, so v6 folds the structural fix. It is
no longer open.

- **The cleanup service floor can exceed its own allocation. FOLDED (round 5 blocker, v6).** Let C be
  the maximum cleanup-and-terminal allocation in a resource dimension. In v5, admission could fill a
  bucket to C, so with any positive overdue-feedback term the required floor exceeded C and no block
  could satisfy both the floor and the allocation, an internally unsatisfiable admitted state. v6
  partitions C into a due-work budget and an overdue reserve whose sum never exceeds C: admission
  reserves only against the due-work budget, and the overdue-feedback term is bounded by the overdue
  reserve. The invariant floor(h) <= C then holds at every height by construction. A bootstrap rule
  covers backlog present when the mechanism activates. The partition point between the two sub-budgets
  is
  a Phase 0 measurement; the invariant is not. This is the same fix recorded in D3b, where the terminal
  service-curve liability and this partition are one mechanism.

### (Round 3 context retained for the record)

## What round 3 changed, and the convergence signal

Round 3 is the first round that showed the process converging. Two independent signals:

- **Two reviewers explicitly CLEARED ground.** Both samples from one source published an "areas that
  survived" section certifying the escrow ordering (D16), the immutable content-addressed verifiers
  (D11), the lease logical-views and expiry index (D6), the allocation-ledger model (D3b), and the
  incremental migration cutover (D7). No prior round cleared anything.
- **No reviewer opened a new architectural front.** Every finding fell inside the five seams the
  charter named. The design is no longer springing contradictions across its whole surface.

The remaining work concentrated on two things, both largely of the synthesizer's own making. The
cross-environment ADAPTER was named in a round-2 one-liner and never designed, and all four reviewers
caught that it relocated the mixed-environment problem into an unspecified box. And RETIREMENT (D15)
was under-specified: it gave a terminal fate only to custody programs and irrevocable requests, and
left every other pending object class, plus the very SUBJECT of retirement (environment vs program
vs family), undefined.

v4 resolutions:
- **Adapter: CUT for v1 (owner decision).** Cross-environment program-to-program calls are removed;
  the canonical adapter exists only at the program-to-native-host boundary. This collapses the
  largest finding cluster and is provably sound. The async-message boundary one reviewer proposed is
  documented as the sanctioned future extension, additive and non-destabilizing, if a concrete
  cross-version use case ever appears. Rationale: an extension point built before two concrete uses
  exist is a guess, and there are zero real cross-version-call use cases on the table (D13).
- **Compatibility floor becomes an EXACT semantic binding (D13).** A reviewer showed the floor's
  justification was hollow: it was justified by avoiding mempool invalidation on repricing, but D2
  already says repricing is not an environment change, so exact binding does not invalidate the
  mempool. Bind the exact semantic environment (credit prices stay outside it, per D2).
- **Retirement fully specified (D14/D15).** Separate monotonic state machines for an execution
  environment and a program version; a user terminal exit retires only its own custody program,
  never a broader subject; an exhaustive per-object-class disposition table; irrevocable requests
  settled from a pre-materialized native finalization record; and leased state may never be the sole
  evidence for custody, since it can expire mid-retirement.
- **Allocation and cleanup tightened (D3b/D6):** the one-fifth cap is measured against the TOTAL
  physical dimension, and reserved-but-matured cleanup work sets a guaranteed service floor.
- **Migration TTL abort is two-phase (D7):** a fixed-cost logical abort needing no program or expired
  grant, then prepaid batched physical cleanup.
- **Non-custody async artefacts get environment-bound receipts (D16).**

The forced core still stands. Two reviewers clearing ground plus a decided adapter means v4 is
plausibly close to a clean pass, which round 4 will test.

### (Round 2 context retained for the record)

## What round 2 changed (read this first)

Round 2 found that folding round-1's fixes created new cross-decision contradictions. Three findings
were confirmed by three independent reviewers each, which is the strongest signal this process
produces:

- **Execution environment not bound to the signature (D7 x D13).** v2 bound the program version but
  not the environment, so an environment activation between signing and inclusion executes semantics
  the user never consented to. This is the SAME signed-intent-versus-consensus-resolution class as
  the round-1 alias blocker, re-created one level up. FIXED in D13, with a second source's nuance
  that the naive fix (sign the exact environment) would invalidate the whole mempool on any
  repricing, so the binding is a compatibility floor plus epoch-boundary activation, not an
  exact-match.
- **Cleanup-debt ceiling is a weaponizable global DoS (D6).** The hard global ceiling v2 added to
  bound backlog lets an attacker deny ALL users the ability to create expirable state at mostly
  refundable cost. FIXED by per-principal and per-family scoping, future-expiry-bucket reservation at
  creation, and an adaptive surcharge replacing the hard cutoff.
- **Reserved lanes plus the one-fifth rule oversubscribe the block (D3 x D3a x D6).** The reserved
  cleanup fraction and the one-fifth cap were never required to be chosen jointly, so a legal
  transaction can fail to fit or ordinary calls can starve. FIXED by a single per-dimension
  allocation ledger with an explicit capacity invariant.

Two more blockers, each confirmed by two reviewers, both at decision seams:

- **Terminal-exit versus bounded-pause desync (D14 x D15).** A bounded pause auto-resumes, but the
  resumed runtime is oblivious to asset withdrawals users made through the terminal-exit path during
  the pause, so it operates on phantom balances and permits double-spends. FIXED by a permanent
  defunct state (D15/D14).
- **Burn-at-request versus cancellation (D5 x D16).** Burning credits when a request is created
  collides with the request's own cancellation path. FIXED by escrow-through-every-cancelable-state
  (D16). This is the third time this exact ordering hazard has appeared across projects.

Plus two capability gaps round 1's fix left open (intrinsic authority bypassing caller-path checks;
the immutable family policy being unable to name future versions), the migration-cutover cost against
the one-fifth bound, and two lease proof-completeness gaps. All folded below.

The fourth sample arrived after the first v3 fold and confirmed seven of the folds independently
(strong validation), while adding three genuinely NEW majors the other three reviewers missed, folded
here as round-2 ADDENDUM items: cross-program calls between programs on different execution
environments had undefined semantics (D13), migration was never composed with lease expiry so a lease
expiring mid-migration could corrupt the destination (D6/D7), and the generic proof verifier was
replaceable by governance so a captured registry could make a deployed program accept forged
authorization (D11). It also sharpened D6, since a fully refundable cleanup deposit never actually
pays for the deletion work. This is the fourth-sample value the standing memory predicted.

### (Round 1 context retained for the record)

## What round 1 changed (read this first)

Two blocker-class defects, both self-inflicted while folding the fifth design into v1:

- **Family-principal authority leak (D10 vs D7).** v1 gave each program family one principal derived
  from the immutable family identifier, while claiming a hostile upgrade inherits only migration
  rights. Those cannot both hold: a shared principal hands a hostile new version the old version's
  custody outright. Two reviewers independently caught it. FIXED by version-scoped principals (D10).
- **Alias resolution at admission (D7).** v1 said an alias resolves at transaction admission and the
  resolved version is recorded in the signed transaction. Nothing can be written into already-signed
  bytes without voiding the signature, and local mempool resolution across a concurrent upgrade is a
  chain split. Three reviewers caught it. FIXED by client-side resolution with the exact version in
  the signed bytes (D7).

The heaviest rework is the state-lease and expiry model (D6), which four reviewers attacked from six
different angles, and the governance trust account (D14), which was under-rated as availability-only
when verifier registration is an integrity power. Several of the synthesizer's own confidence claims
were walked back: the interpreter does not remove divergence "by construction" (D1), the typed-quorum
fix was incomplete for attestations (D4), the repricing rule contradicted the benchmark-repricing
idea (D2), and the native privacy service was marked chosen while its architecture was deferred (D11).

## The forced core (survived round 1 unchanged)

1. WebAssembly as the consensus execution format, embedded in the L2 state transition function as a
   new transaction family. One state machine, one state root, one result.
2. Host calls are the program's only contact with the world. No network, filesystem, OS, ambient
   time, ambient randomness, threads, or hardware access.
3. All durable program state lives in the existing authenticated store, under program namespaces,
   with the same membership and non-membership proofs as native state.
4. The dominant contract virtual machine is rejected; its account and storage model would destroy
   uniform provability or demand wrappers around every native feature.
5. Integer-only execution, enforced at deployment by a static validator.
6. Canonical encoding and ordering at the host boundary; database layout, hash-map order, cache
   state, and memory addresses are never observable.
7. Metering in the existing credit unit against a hard per-block bound.
8. Existing data contracts, documents, identities, groups, tokens, credits, and bridge records keep
   working unchanged and become callable. No migration of existing applications.
9. Phased deployment, governance-gated first, permissionless later.
10. No new mandatory trusted party.
11. Determinism is the assumption whose failure kills the design; validated first, cross-architecture.
12. Rust first, other languages only after their toolchains provably emit the accepted subset.

## Deliberate choices (current through v12)

**D1. Interpreter for consensus; determinism is a trust assumption, not an elimination. REVISED.**
The consensus reference interpreter stands (no just-in-time compilation on the consensus path,
optimized engines only for mempool simulation whose output is never accepted). Round 1's correction:
the interpreter does NOT eliminate divergence "by construction," because it is itself compiled native
Rust running on real processors and crypto libraries, subject to compiler, microcode, unsafe-code,
decoder, and library divergence. v2 replaces the elimination claim with an explicit consensus trust
assumption plus obligations: an implementation-independent executable semantics for the accepted Wasm
subset, the host boundary, traps, and metering; proof or exhaustive validation of the small
consensus-critical core (decoder, static validator, integer semantics, memory bounds, meter
placement); the second interpreter treated as a release alarm and diagnostic, not an oracle that can
name the correct result; restricted unsafe Rust and platform-specific dependencies in the consensus
core; and pinned compiler and library versions in the release artifact hash. Permissionless
activation requires coverage and mutation-testing thresholds, not merely a clean replay period.

**D2. Multidimensional accounting, with a hard separation of work units from price. REVISED.** The
six meters stand (instruction, cryptographic, state reads and bytes, state writes with index updates
and bytes, peak memory with page duration, proof or commitment work). Round 1 exposed a contradiction
in v1: the rule "a schedule change that flips a call from success to failure is a new runtime version"
collided with the adopted idea that governance may reprice cryptographic host calls without a fork.
Repricing changes consumed units and can exhaust a declared vector, flipping the outcome. v2 resolves
it: **fixed consensus work units are separate from credit pricing.** Governance may change the credit
price or base-fee conversion for a fixed unit freely. Any change to resource units, meter placement,
or per-operation work consumption is a new execution-environment version (D13), never a price update.

**D3. Block bound by declared reservations, with two additions round 1 forced. REVISED.** A block is
valid only if the sum of declared limits fits the block resource vector, and no transaction may
exceed one fifth of any dimension. Two corrections:
- **Metering must bound REAL work, not just counters.** Every metered dimension must be a proven
  upper bound (with a stated safety margin) on the corresponding CPU, IO, and memory cost on minimum
  hardware, and the meter must cover the COMPLETE deterministic transaction lifecycle: canonical
  decoding, signature checks, code load and hashing, module validation, host argument conversion,
  schema validation, index derivation, journaling, rollback, final native invariant checks, and
  commit preparation. Prechargeability from canonical input shape is an admission requirement for
  every host operation; a host function whose worst case cannot be bounded from input and
  authenticated metadata is rejected. Cache hits never reduce consensus charges. Under-metering is a
  consensus-critical defect of the same class as a nondeterministic host call.
- **The non-refundable reservation charge must price the reservation, not be a flat fee.** v1 let a
  caller reserve a whole dimension, run a trivial path, reclaim most escrow, and deny block capacity
  for almost nothing. v2 makes the non-refundable portion a deterministic function of the declared
  vector and the exclusion it imposes, refunding only the part tied to actual execution cost.

**D3a. The one-fifth cap is a correctness gate, not just anti-monopoly. NEW.** Any indivisible
operation (an atomic proof verification, bridge transition, group recovery, shielded spend) whose
worst-case valid reservation exceeds one fifth of a block dimension can never execute. v2 states the
required inequality explicitly: every indivisible protocol operation must fit below the
per-transaction fraction in every dimension, or be exempted through a reserved lane with its own hard
limit. The fraction and the largest indivisible operation are chosen together, after measurement, not
independently.

**D3b. One allocation ledger per resource dimension. NEW (round 2).** Round 2 found that the reserved
cleanup lane (D6), other reserved protocol lanes, and the one-fifth cap (D3a) were specified
independently, so their limits can sum above the physical block vector (a legal one-fifth transaction
then fails to fit) or, if partitioned conservatively, leave ordinary calls with almost no capacity
even when reserved lanes sit idle. v3 requires a single allocation ledger per dimension from which all
ordinary transactions, legacy native operations, cleanup, and protocol exemptions draw, with total
hard limits no greater than the physical vector. The capacity invariant is explicit: the sum of every
reserved fraction plus the one-fifth allowance must not exceed the dimension, a positive minimum is
guaranteed for ordinary execution, and idle lane capacity is reclaimed by ordinary transactions under
a deterministic rule. REVISED (round 3): v3 measured the one-fifth cap against the NON-reserved
portion, which lets the adaptive cleanup lane (D6) shrink the absolute cap as backlog grows, so an
indivisible operation that fits at zero backlog traps under load and an attacker can bloat backlog to
deny large atomic operations. v4 measures the one-fifth cap against the TOTAL physical dimension, and
the capacity invariant is that the sum of every reserved fraction's MAXIMUM plus the absolute
one-fifth allowance is at most the dimension. Two further rules the reviewers required: reserved
capacity is a service GUARANTEE, not just a ceiling, so a block is invalid if it reclaims cleanup
capacity as idle while due cleanup exists (the guaranteed cleanup service at height h is at least the
validly-reserved work maturing at h plus a bounded overdue-feedback term, and ordinary transactions
reclaim only what remains after that floor); and the adaptive fraction carries hysteresis or a bounded
rate of change so it cannot limit-cycle between floor and ceiling under alternating dense and empty
expiry buckets.

REVISED (round 4): PREPAYMENT IS NOT A RESERVATION, and v4 conflated them for every obligation except
leased-key cleanup. Two sources found this independently, which makes it the second-best-corroborated
finding of the round. v4 says retirement disposition queues are prepaid and incrementally processed,
and that migration abort cleanup is prepaid, idempotent, and ledger-funded. Neither statement reserves
block capacity at the height where the work comes due. D6 does reserve future cleanup capacity for
leased keys at their known expiry heights, so the mechanism already exists and was simply not applied
to the other obligation classes.

The failure this permits does not require a defect anywhere else. Retirement markers and logical aborts
are constant-cost, so they always succeed. Afterwards, an adversary who keeps the ordinary lane
saturated at ordinary cost leaves the disposition queues to drain only out of whatever capacity happens
to be left. Every block remains valid, because the service floor covers only validly RESERVED work and
none of this work is reserved. Escrow refunds, irrevocable settlements, task tombstones, destination
trees, journals, verifier state, and exit pins can then sit for an unbounded number of heights.
Migration makes it worse in one specific way, because many principals can start large migrations whose
time-to-live values coincide, so a single height can mature an arbitrary volume of abort cleanup that
was never scheduled. The result is that lazy terminalization becomes a practical denial of the exit
guarantee D14 and D15 exist to provide, and pinned bytes grow with no protocol drain bound. Funds are
not lost, since the native terminal path still governs the outcome, but the exit is deferred, and a
deferred exit is what the guarantee was supposed to rule out.

Rounds 1 through 7 revised this decision. v5 applied the D6 pattern uniformly (reserve terminal work at
creation, not at maturity), round 5 split it by maturity-height knowledge for v6 because perpetual and
event-triggered work has no dated bucket to reserve into, round 6 found the v6 liability bound was not
global and that custody's perpetual class forced a finite ceiling on user balances (v7 lifted it with
the
hybrid), round 7 found the v7 hybrid too broad (it did not account the cleanup work terminalized objects
generate and it assumed every custody position has a single owner able to pull, so v8 narrowed it), and
round 8 found the v8 accounting metered in the wrong unit, uncharged on mutation, deadline-blind,
forcing
synchronous cleanup, and classifying ownership only once (v9 unified it as the terminal-work meter), and
round 9 certified that meter across sources while finding three edge seams at its boundaries with D15,
the flow ceiling, and the deadline lanes (v10 closed them), round 10 certified the v10 edits except one
accounting defect inside the transfer bullet plus two wording gaps (v11 closed them), and round 11
certified every v11 edit with one text ambiguity remaining, the implicit class dependence of the
terminal-work vector (v12 states it explicitly, clarity only). The settled mechanism is restated cleanly
below (round-11 folded), with the round 1 through 4 derivation above and this note as the record.

**v12 CONSOLIDATION (round 11 folded, clarity only), the current normative D3b terminal mechanism.** The
v9 core is
certified across sources and unchanged: only SINGLE-OWNER custody leaves the liability, the
TERMINAL-WORK METER (subsection 2a) carries every object's worst-case terminal-work vector in work units
maintained on mutation, admission splits deadline-bearing work (dated reservation, hard guarantee) from
deadline-free cleanup (rate-R drain), and the single-owner pull is a logical exit that enqueues prepaid
cleanup. The v10 edge closures stand (discharge-and-recreate for transfers, the marginal charge,
idle-capacity early completion), and v11 completes them: the transfer carries an ATOMIC ACCOUNTING
HANDOFF (the old position's funding and vector ride its drain-queue cleanup item, the new vector is
admitted as a gross positive delta), the reclassification rule is BIDIRECTIONAL over every class pair,
and the concentration-limit claim is narrowed to anti-targeting. The cleanup partition equation and the
block-capacity invariant are unchanged and certified.

**1. Reservation by maturity-height knowledge (unchanged from v6).**
- Known-height expiry (leased-key cleanup) reserves into the dated future-capacity bucket at its fixed
  expiry height.
- A migration reserves worst-case abort cleanup at admission against its known TTL height, releasing the
  reservation on successful cutover.
- Event-triggered terminal work, whose maturity height is unknown at creation, is governed by (2) and
  (3).

**2a. The terminal-work meter (NEW in v9; vector definition made explicit in v12).** Every object that
can generate terminal or cleanup work (single-owner and autonomous custody, deferred tasks, hook
bindings, irrevocable requests, lease and exit records) carries an authenticated WORST-CASE
TERMINAL-WORK VECTOR over the resource dimensions, recording the maximum work its terminalization and
physical reclamation can require. This vector is measured in TERMINAL-WORK UNITS per dimension, never in
permanent bytes, so the D6 byte budget stays a separate storage control and does not double as the
terminal-work meter. The vector is CLASS-DEPENDENT and LANE-ATTRIBUTED (stated explicitly in v12, since
one example previously carried it): it records, per resource dimension, the work the object's CURRENT
ownership class will impose on each service lane, so owner-paid ordinary-lane discharge (a single-owner
pull) is not drain work, while protocol-driven distribution and physical cleanup are. A reclassification
recomputes the vector FOR THE NEW CLASS and charges every positive component of the delta, which by
construction includes any work the class change moves INTO the rate-R lane.

- **The vector is a lifecycle invariant.** The accounting invariant `accounted(o) >= worst_case_work(o)`
  holds at every height. It is established at creation and maintained on every mutation. Any transition
  that increases an object's footprint (adding a beneficiary, an index entry, a dependency edge)
  atomically pays the marginal delta and admits it (below) before the mutation commits, and a decrease
  may
  release accounting under deterministic rules. Where a class can use a density-independent reclamation
  layout (a tombstone plus a fixed-cost index update) its vector is a constant and needs no mutation
  charge, and classes that cannot are metered on mutation.
- **Admission splits by whether the work has a deadline.** Deadline-bearing terminal work reserves into
  the dated `known_due` bucket at its deadline height, exactly as section 1's known-height expiry does.
  Deadline-free terminal work is admitted against the rate-R drain by a FLOW condition, that the sum of
  positive worst-case terminal-work deltas admitted per block, per dimension, across every class, stays
  at
  or below the drain rate R. This is the corrected rate-matching invariant, now metered in terminal-work
  units by this meter rather than by the D6 byte budget. A block-level property is that no admitted
  transition lets the deadline-free flow exceed R in any dimension.

**2. Custody terminalization is split by OWNERSHIP TYPE, with DYNAMIC classification (REVISED in v9).**
The v8 bifurcation stands, and only single-owner custody leaves the liability while autonomous or joint
custody stays in it. v9 makes the classification dynamic and changes how the single-owner exit runs.

- **Single-owner custody leaves the service-curve liability.** A position whose terminal discharge is
  authorized by a single external principal that can sign a native exit WITHOUT executing program code
  is
  a user-owned balance. It accrues no drain flow for its balance discharge and its live count is not
  capped. On retirement the position is logically frozen at once by the constant-cost generation marker
  (D15). The retention-not-funding wording and the native-exit visibility exemption are unchanged from
  v8,
  both certified in round 8: the frozen position is RETAINED (charged on the same 50-era amortization as
  any durable record, carried under the disclosed inherited subsidy past era 50, not funded
  indefinitely), and its native exit path and light-client membership proof are EXEMPT from D6 logical
  exclusion, reading the pinned native obligation record directly, so a frozen position is invisible to
  live-state queries but always locatable by its owner.
- **The single-owner pull is a LOGICAL exit that ENQUEUES cleanup, never synchronous.** The pull is a
  constant-cost transaction. It transitions the object's one canonical lifecycle owner by atomic
  compare-and-set (so a pull racing a governance or environment retirement or another pull yields one
  transition and one loser observing the completed state, D15), releases the balance to the owner, and
  enqueues the position's prepaid physical cleanup into the deadline-free drain. It does NOT perform the
  physical cleanup synchronously, so a dense position can never make its own exit exceed the one-fifth
  transaction cap and the owner is never locked out. The cleanup was funded through the meter (2a) at
  creation and on every mutation, so the enqueued drain work is already paid. All physical cleanup flows
  through the rate-R drain, and there is no synchronous cleanup path.
- **Classification is bound to an authenticated exit-authority descriptor, is DYNAMIC, and respects the
  immutable dependency set (REVISED in v10).** The single-owner class is recorded as a native
  exit-authority descriptor naming the single external principal, decidable and unforgeable from
  authenticated state. Any transition that changes discharge authority is handled by exactly one of
  three rules, split by what it does to the position's D15 dependency set, which is fixed at creation
  and NEVER mutated.
  - **In-place change, same dependency set.** A change that keeps the position under the same program
    version and environment (key rotation to another external key, a change of the named principal) and
    preserves the property "dischargeable by one external principal via a native exit that runs no
    program code" commits in place. No reclassification, no meter charge.
  - **In-place reclassification, same dependency set, BIDIRECTIONAL, marginal charge only (REVISED in
    v11).** A change that keeps the same program version and environment but changes the ownership
    CLASS, in either direction, reclassifies the position in place: single-owner to autonomous or joint
    (a multi-party upgrade held under the same program, delegation to a co-signer set), autonomous or
    joint to single-owner (a co-signer set collapsing to one external principal with a native, code-free
    exit), and changes among autonomous or joint forms. The dependency set is unchanged, so D15 subject
    indexing still reaches it, and the exit-authority descriptor, the liability class, and the lane
    assignment are updated in the same atomic batch. The position's worst-case terminal-work vector V
    was admitted through the meter at creation and on every footprint-increasing mutation, and
    reclassification performs no new physical work, it changes which lane will eventually drain work
    that is already accounted. So the reclassification admits only the positive MARGINAL increase in the
    worst-case vector that the class change itself produces, if any (for example a wider distribution
    fan-out for a multi-party payout), releasing negative components only under the existing
    deterministic release rules. If the class change leaves the vector unchanged or decreases it, the
    flow check is a no-op and the transition cannot be refused for flow reasons. Re-admitting the full
    prior V would double-count work the meter already carries, and it is what allowed a saturated flow
    to wedge legitimate authority upgrades. Reclassifications with zero growth in the destination
    lanes' attributed vector (per the class-dependent, lane-attributed definition in 2a) cannot be
    blocked by the flow ceiling under any load. Positive marginal increases compete under ordinary
    global admission and may
    be delayed by congestion, and the per-principal and per-family concentration limits (section 6) are
    anti-targeting controls, not an actor-level fairness guarantee, since D6 already records that
    per-principal limits are defeated by principal-splitting. If Phase 0 shows that authority-upgrade
    liveness needs a stronger guarantee, the named option is a bounded authority-maintenance subshare
    inside R reserved for positive marginal reclassifications, ordered deterministically by age, one
    pending request per position, reclaimed when idle, noting that actor-level fairness would
    additionally need a protocol-defined non-splittable resource.
  - **Transfer to a NEW program or environment is DISCHARGE-AND-RECREATE, with an accounting handoff
    (REVISED in v11).** A transfer that would place discharge authority under a different program
    version
    or environment (into another durable record, a programmatic treasury, a different app) can never
    mutate the existing position, because its D15 dependency set is immutable and a mutated set would
    either strand the position (retirement of the new subject never reaches it through the subject
    indexes) or violate the certified fixed-at-creation rule. Instead the transition performs an atomic
    DISCHARGE with an ACCOUNTING HANDOFF, then a CREATION, all in one batch (D16a):
    - The old position's canonical lifecycle owner transitions to discharged by compare-and-set and its
      exit evidence closes, remaining as spent evidence.
    - The old position's residual physical-cleanup work (subject indexes, position storage, evidence) is
      enqueued as its drain-queue cleanup item, and the old position's prepaid funding and accounted
      terminal-work vector TRANSFER to that item. Nothing is released at discharge, because the
      certified no-synchronous-cleanup rule means this work still runs through the rate-R drain, and the
      transferred accounting is released only when the physical reclamation completes. The lifecycle
      invariant accounted(o) >= worst_case_work(o) therefore holds over live positions AND queued
      cleanup items at every height.
    - The new position is created with its own immutable dependency set naming the new program and
      environment, its own subject indexes, and full D17 composition admission, and its complete
      terminal-work vector is admitted through the meter as a GROSS positive delta under the current
      block's flow condition, never netted against the old position's accounting. A transfer therefore
      always consumes flow equal to the new vector, which is what prevents repeated transfers from
      recycling one admission into an unbounded trail of queued cleanup.
    - The balance is debited from the old position and credited to the new exactly once, inside the
      same batch.
    The new position is admitted under the CURRENT retirement generation, and the transition is REFUSED
    while the target program or environment is retired or retiring (D15), or when the new position's
    admission fails. If the transition is refused, the old position is unchanged and still
    dischargeable.
  - Mere loss of the sole private key is undetectable by the protocol and remains the owner's key-loss
    risk, requiring no reclassification.
- **Autonomous or joint custody STAYS in the liability**, exactly as v8. A position whose terminal
  discharge has no single external puller (shared among principals, controlled by program logic, or held
  by a programmatic treasury or pool) remains a subject in the event-triggered liability of section 3,
  so
  its terminalization is protocol-driven, and on retirement the native disposition table (D15)
  distributes
  or refunds it without executing program code. Its worst-case terminal work is carried by the meter
  (2a)
  and maintained on mutation like every other class. The ownership type is DECLARED and ENFORCED before
  the first deposit, extending the existing D15 rule that single-user custody must be stated and
  enforced
  before any deposit is accepted, and it is re-checked on every authority change per the dynamic rule
  above.

**3. The two terminal-work lanes and the phased fan-out (REVISED in v9, fold of the round-8 deadline and
mutation findings).** With the meter (2a) accounting every class in work units and maintaining the
invariant on mutation, section 3 states the two lanes and the retirement behaviour.

- **Deadline-bearing work uses the dated reservation and has a HARD completion guarantee (REVISED in v10
  on the early-completion lane and the explicit gate).** When an asynchronous request becomes
  irrevocable, its terminal height (the D16 terminal timeout) is known, so its worst-case settlement
  work
  is reserved into the dated `known_due` bucket at that height, admitted against the `known_due` share
  of
  C_total. This guarantees the settlement completes by its deadline. A retirement may complete an
  irrevocable settlement EARLY, but never at the expense of either lane's guarantee. Early completion
  runs only from capacity that is genuinely idle after the current block's known_due(h), the full
  drain-rate share R, and the overdue service are all satisfied, or by atomically moving the
  settlement's
  future dated reservation into the current block's `known_due` bucket when that bucket has room. It is
  never charged against R while deadline-free work is waiting, so it cannot eat the drain's guaranteed
  service, and it never runs in addition to the full allocation, so it cannot exceed the cleanup
  equation. On early completion the future dated reservation is released. A settlement that cannot
  complete early simply waits for its reserved height, where its completion is already guaranteed, so it
  can never be lost to queue position. Absolute timeouts remain only for the pre-irrevocability
  share-production phase, and once irrevocable, settlement is deadline-guaranteed rather than
  deadline-lost. This removes the v8 collision between drain latency and the D16 terminal timeout.
  Stated explicitly, because a review misread this boundary. The transition that makes a request
  irrevocable is an ON-CHAIN state transition that precedes any quorum share (D16), so it is gateable
  like any other admission. It reserves the settlement work in the `known_due` bucket at the terminal
  height, and it FAILS ATOMICALLY when that bucket's share is full, leaving the request pre-irrevocable,
  cancelable, with escrow intact and no external artifact produced, which is safe. A burst of requests
  reaching irrevocability together is therefore throttled by the share, never admitted past it. The
  share's sizing affects throughput of irrevocability, not the hard completion guarantee of anything
  admitted. A stronger variant remains open as a deliberate option, reserving `known_due` at request
  CREATION under a worst-case assumption, which would guarantee that a request which reaches
  irrevocability
  always finds room, at the cost of reserving capacity for requests that may never become irrevocable.
  The design does not adopt it now, and names it in case Phase 0 shows irrevocability throughput
  matters.
-  **Deadline-free work uses the rate-R drain, safety-immediate and stock-uncapped.** Physical cleanup
  and
  reclamation of terminalized objects, and autonomous-custody distribution that carries no deadline, are
  admitted against the deadline-free flow condition (2a) and drained at rate R. An environment
  retirement
  writes only the O(1) generation marker (safety-immediate), then the deadline-free work drains at rate
  R
  over ceil(backlog / R) blocks. The live object stock is NOT capped, because the bound is the flow
  condition (deadline-free terminal-work flow at or below R), not a stock sum. The instantaneous
  committed
  drain load is at most R per block, at most W times R in flight for any single admitted batch of
  latency
  W.
- **Domain-union closure is unchanged and trivial.** The drain rate R and the flow condition are global
  and not domain-scoped, so a D17 domain union or split cannot change them and no per-union reaccounting
  is required. The flat global rate is the single normative rule.
- **Disclosed costs, stated plainly.** Two liveness weakenings, both disclosed. Deadline-free
  terminalization latency scales with backlog (ceil(backlog / R) blocks), unchanged from v8. And a
  single-owner exit is now two-step, in that the owner receives their balance immediately at the logical
  pull but the position's storage is reclaimed later through the drain rather than at the pull. Safety
  is
  immediate in both cases. Deadline-bearing settlement, by contrast, gains a HARD completion guarantee
  it
  lacked in v8. The unbounded growth of live stock over time is the same perpetual-state accumulation
  the
  platform already accepts for all durable records (D6 disclosed subsidy).

**4. The cleanup and terminal allocation is one explicit equation (fold of the round-6 wording
finding).** Per dimension, the cleanup-and-terminal allocation C_total is partitioned into exactly three
shares whose sum never exceeds it:

    known_due(h) + event_terminal + overdue(h) <= C_total

where known_due(h) is the dated work maturing at height h, event_terminal is a dedicated share equal to
the terminal service rate R, and overdue(h) is the bounded overdue-feedback reserve. R is a partition of
C_total, not a term added on top of it. The guaranteed cleanup service floor at height h is known_due(h)
plus the R share plus the bounded overdue term, and ordinary transactions reclaim only what remains
after that floor. Because the three shares sum to at most C_total, the floor satisfies floor(h) <=
C_total at every height by construction. A bootstrap rule covers any backlog present when the mechanism
activates.

**5. The block capacity invariant (restated to remove the v6 ambiguity).** Per dimension, the sum of
C_total (already inclusive of its known_due, R, and overdue shares), every other reserved fraction's
maximum, and the absolute one-fifth allowance is at most the physical dimension. R is counted once,
inside C_total. A positive minimum for ordinary execution is guaranteed, and idle reserved capacity is
reclaimed by ordinary transactions under the deterministic rule, except that the cleanup floor is not
reclaimed while due cleanup exists.

**6. Concentration limits and terminal timeouts (unchanged).** Concentration limits apply across
principal, family, and retirement cohort, and native finalization that waits on quorum work carries a
terminal timeout selecting completion or the declared loss path.

**7. Finality signal (unchanged).** Per-subject per-class counters are exposed to light clients as
membership and non-membership proofs that every disposition of a subject under a given retirement
generation has been processed.

WHAT IS FOLDED AND WHAT IS NOT. The mechanism is structural and folded. Its NUMBERS remain Phase 0
measurements: the drain rate R, the per-batch latency W, the worst-case terminal-work vector per object
class (from GroveDB's worst-case estimators for the storage dimensions, with the VM work vector an
implementation obligation), the per-block deadline-free flow ceiling matched to R, and the `known_due`
share sizing for deadline-bearing reservations. The single coupled question is whether the terminal-work
meter and the deadline-free flow ceiling can be sized so legitimate creation is not starved while the
drain keeps pace. A keeper incentive funded by the D6 prepaid cleanup charge, converting part of the
deadline-free drain into keeper-pull work, is noted as a possible optimization and is not built.

**D4. Typed quorum adapters, and attestations must be predicates, not free-form. REVISED.** Domain
separation stands and prevents a signature from being replayed across domains. Round 1's correction:
within the "application attestation" domain, a program could still get the quorum to sign a
semantically FALSE statement (a false price, eligibility, or ownership claim) that an external party
treats as Dash-validated. Domain separation stops replay, not lying. v2 requires every attestation
type to be a deterministic predicate over committed state, with the adapter deriving the signed
payload itself and binding chain identifier, state root, request identifier, quorum identifier,
expiry, and exact predicate version. A domain that cannot be reduced to such a predicate is signed
only as an opaque request acknowledgment with no protocol branding.

**D5. Asynchronous quorum production, now with a request state machine and randomness liveness.
REVISED.** Verification is a synchronous metered call; production is asynchronous by receipt.
Randomness still binds a future height. Two additions:
- **A consensus request state machine** with explicit authorization, share-production,
  irrevocability, completion, cancellation, expiry, and pause semantics per domain (feeds D15 and
  D16).
- **Randomness liveness is stated separately from signature uniqueness.** A unique future-height
  signature does not stop an aggregator who learns the value first from withholding it when
  unfavorable, if the application may retry against a fresh height. v2 requires reliable
  dissemination of shares or results and binds retry and timeout so an absent result cannot become a
  new draw chosen by the beneficiary. (Minor, but real.)

**D6. Leased state, reworked into a consensus-bounded lifecycle. HEAVILY REVISED.** This was the most
attacked decision. The lease idea survives; the naive mechanism did not. v2:
- **Cleanup is a metered, reserved consensus lane, funded at creation, with backpressure that cannot
  be weaponized. REVISED (round 2).** Expiry deletion is not a free background task outside the block
  vector. Every leased-key write pays a refundable cleanup deposit returned only on successful
  deletion, and the cleanup lane draws from the D3b allocation ledger (an ADAPTIVE fraction, zero when
  backlog is zero and rising toward a documented maximum as backlog grows, so there is no permanent
  capacity tax). v2's hard GLOBAL cleanup-debt ceiling is REMOVED: three reviewers showed it lets an
  attacker synchronize mass expiry to deny every user the ability to create expirable state at mostly
  refundable cost. v3 instead reserves capacity in future expiry buckets at creation time (so a
  schedule that would overwhelm a future block's cleanup lane is rejected when the key is made, not
  when it expires), enforces per-principal and per-family concentration limits on scheduled cleanup
  obligations, and applies a progressive creation surcharge that rises with current backlog. The
  design must state a bound on the longest global creation outage any valid expiry schedule can
  cause, and that bound must be short. ADDENDUM (round 2, fourth sample): a fully refundable deposit
  does not actually pay validators for the deletion and index-maintenance work, contradicting "funded at
  creation." v3 splits the payment into a NON-REFUNDABLE prepaid cleanup charge priced for worst-case
  deletion and index maintenance (which pays the lane's real work) and a separate REFUNDABLE
  congestion bond (which supplies the backpressure), and defines how a later price change affects
  outstanding deposits.
- **Logical liveness is defined independently of physical presence.** Expired records are excluded
  from every point, range, and secondary-index view at the same height, whether or not the bounded
  queue has physically deleted them yet. A raw store proof of physical presence is never the logical
  answer.
- **Deletion evidence is a bounded accumulator, with the supported proof claim and its horizon stated
  explicitly. REVISED (round 2).** v1's per-key receipts either lived forever (recreating unbounded
  state) or were pruned (breaking the promised proof of deletion). v2 aggregated deletion evidence
  into authenticated epoch accumulators. Round 2 asked the sharper question: which statement does a
  light client actually get to prove, and for how long. v3 answers it as a choice the design must make
  and name, not leave implicit. Either the mechanism promises only CURRENT logical absence, in which
  case it must not claim durable deletion evidence and the accumulator witnesses may be finite, or
  deletion events must stay provable, in which case authenticated epoch roots are retained permanently
  and the design names who stores or reconstructs the witnesses, for how long, and which claims become
  unprovable past that horizon. Retaining historical witnesses "somewhere else" indefinitely is not a
  bounded mechanism; it is unbounded state relocated.
- **Range and index reads are bounded by LIVE entries, not by tombstone density. NEW (round 2).** An
  attacker can order many expirable keys immediately before a victim's live records and expire them
  together. If logical iteration must physically traverse or prove around them until cleanup catches
  up, a query for the first live result can exceed the transaction cap or the proof-response limit,
  so the logical answer exists but neither execution nor a light client can obtain a bounded proof of
  it. v3 requires an authenticated expiry-aware index (or an equivalent layout) whose work is bounded
  by returned live entries plus fixed metadata, with provable forward progress across expired entries
  and contract interfaces that tolerate continuation across transactions.
- **Paying rent is sponsorship, not authority.** Anyone may fund bytes, but funding alone must never
  revive a logically expired object or extend semantic validity. Reactivation requires the
  collection's declared authority and mints a new epoch or version so old signatures and capabilities
  cannot silently become valid again. Application schemas must not use lease height as an
  authorization expiry, and the SDK must enforce that separation.

**D7. Immutable code, client-resolved version binding, and a real migration lifecycle. REVISED.**
Code versions are immutable and a family declares its upgrade policy at creation. The two changes:
- **Version binding is client-side.** The client resolves the alias against committed state and signs
  the exact version identifier. To preserve alias intent, it may also sign the alias, the expected
  alias mapping, and a height or state bound as preconditions. Consensus validates only signed bytes
  and committed state and never mutates a transaction at admission or decides a consensus-visible
  version locally.
- **Migration is a defined lifecycle, not a batch loop.** v2 requires a consensus-visible migration
  epoch and a source snapshot root, a stated rule for concurrent writes (range lock, or journal for
  deterministic replay), calls naming the migration epoch they expect, an atomic cutover that
  verifies the complete destination invariant before the family routes to the new version, and
  explicit abort, retry, timeout, and rent-responsibility behavior for a migration that stalls. The
  old version's exit path stays available until cutover or a defined recovery state. REVISED
  (round 2): "verify the complete destination invariant" cannot mean scanning the whole destination
  at cutover, because for a large family that scan is an indivisible operation whose worst-case cost
  exceeds the one-fifth bound (D3a) and a reserved lane cannot rescue an unbounded operation.
  Invariant verification must be INCREMENTAL and authenticated: each bounded migration batch advances
  a committed verification state, and cutover performs only a fixed-cost check over the final
  commitment (or a consensus limit on migratable state proves the final operation always fits). Every
  migration epoch carries a maximum time-to-live, so a migration whose authority stalls or loses its
  keys reaches a deterministic aborted-and-cleaned state. REVISED (round 3): that abort cannot be one
  atomic operation for a large family (deleting the destination, unwinding the journal, releasing
  source pins, settling rent, and removing indexes exceeds the one-fifth bound), and it cannot use the
  migration grant that expired at the same height. v4 splits it in two: at TTL a fixed-cost native
  transition that needs no program code and no live migration grant marks the epoch ABORTED, restores
  the single authoritative source route, rejects every destination call for that epoch, and freezes an
  authenticated cleanup manifest; then prepaid, idempotent, ledger-funded batches remove the
  destination, journal, verifier state, and pins, reaching ABORTED_CLEAN. Application-level write
  restrictions lift at the logical abort via a generation marker, so the family is not held in
  concurrent-write limbo while physical cleanup drains.

  REVISED (round 4), because the fixed-cost abort cannot discard an unapplied journal safely. v4 kept
  two permitted mechanisms for concurrent writes during migration, a range lock or a journal, and it
  also made lease expiry and renewal ordered journal events. Under the range-lock variant a long
  migration can accumulate renewals and expiries that were never materialized into the source
  snapshot. At the time-to-live boundary the fixed-cost abort restores the source route and lifts write
  restrictions, and it cannot replay that journal without exceeding the one-fifth bound. Both outcomes
  are wrong. If calls resume against the source immediately, they read stale lease status, so a valid
  renewal can be lost or an expired record used. If calls keep reading through the journal, the journal
  is still authoritative, the advertised cleanup cannot remove it, deleting it corrupts state, and
  retaining it leaves the migration's pinned bytes unreclaimable. This was raised by one source in one
  sample, the least corroborated of the folded five, and it is folded because v4's own text permits the
  state and the check needs no measurement. v5 resolves it by making the source authoritative
  throughout:
  - The SOURCE remains the fully materialized authoritative state for the entire migration. Every
    write, renewal, expiry, and spent-status change applies to the source in bounded consensus work.
  - A journal, where used, is a replay FEED for the destination only. It is never the authoritative
    record of source state, so discarding it at abort cannot lose a committed change.
  - If a future variant retains an overlay as authoritative, abort must preserve that overlay as
    authoritative and compact it through a separately metered, guaranteed process before any journal
    cleanup runs. The epoch is not called clean and the affected pins are not released until compaction
    completes.
  - The abort remains fixed-cost, because restoring a route to already-materialized state is a constant
    operation regardless of how much changed during the migration.
- **Migration composes with lease expiry as ordered journal events. ADDENDUM (round 2, fourth sample).**
  A migration snapshots a source tree, but a record's lease can expire DURING the migration, so under
  a range lock the record changes logical status without a write, under a journal rule a renewal can
  race the expiry, and the cleanup lane can physically delete the source before deterministic replay
  or abort needs it, leaving the destination with an expired record, a lost valid renewal, or a source
  that cannot be reconstructed. v3 pins all source bytes and authenticated metadata needed for replay
  until cutover or abort, charges that retention to the migration, treats expiry and renewal as
  ordered consensus events in the migration journal, and binds the single logical evaluation height
  the destination invariant is computed against.
- **Leased state may never be the SOLE evidence for custody, conservation, replay protection, or a
  terminal exit. NEW (round 3, blocker).** A custody program that recorded a user's balance or
  withdrawal entitlement only in a leased collection, then got retired while that record was in grace,
  would have its evidence excluded by the logical-view rule and physically deleted by cleanup, leaving
  the terminal exit unable to prove ownership, amount, or spent status, so the asset locks forever or
  a double-withdrawal becomes possible. v4 requires that at deposit time the native layer commit
  permanent or retirement-pinned exit evidence (owner, amount, asset, program version, spent status),
  and that a retirement transition atomically pin or reclassify every remaining exit record before
  logical lease expiry can remove it, with that storage and its eventual cleanup prepaid.

  REVISED (round 4), because v4's protection arrived later than the threat. v4 allowed exit evidence
  to be "permanent OR retirement-pinned", and the pin is applied by the retirement transition, which
  happens only when retirement begins. A record that is merely pin-ELIGIBLE is still an ordinary
  leased record until then, so its lease can expire at height h, the logical-view rule excludes it at
  h, physical cleanup can delete it, and a retirement at h+1 can pin only what remains. A retirement
  generation marker cannot restore an absent record, and the constant-cost transition rules out
  enumeration as a rescue. One source found this in both samples and rated it a blocker. A second
  source had cleared the pin as atomic, which is true of the pin itself and does not address the
  window before it exists. v5 removes the eligible-but-unprotected state:
  - Custody evidence enters a NON-EXPIRING native obligation ledger at deposit acceptance, and stays
    there until the obligation is spent, withdrawn, or transferred into another equally durable
    record. Durability begins at deposit, never at retirement.
  - Retirement may change the record's accounting class, access mode, or cleanup destination. It is
    never the event that first protects the record from expiry.
  - Every expiry and cleanup path consults the native obligation state before excluding a record from
    a logical view or deleting it physically. An open obligation refuses both.
  - Where compact commitments replace full records, the permanent commitment carries enough
    authenticated information to prove owner, amount, asset, program or account scope, and spent
    status without reading any leased state.
  - The same-height case is defined rather than left to implementation order. Expiry, cleanup, and
    retirement occurring at one height resolve in a fixed protocol order with the obligation check
    first.

  THE OBLIGATION IS PERPETUAL AND CARRIES NO HORIZON. DECIDED BY OWNER 2026-08-08 ON PHASE 0 EVIDENCE,
  REPLACING v5's FINITE-HORIZON RULE. A second source found the opposite failure in v4: if a user
  deposits, the program is retired, and the user then loses their keys and never exits, an obligation
  that never expires is unbounded storage funded by a finite prepayment. v5 answered that with a maximum
  absolute lifetime plus a named terminal disposition for abandoned records. Measurement showed the
  platform answers the same objection WITHOUT a deadline, and its answer is adopted here instead:

  - **State is perpetual and the CHARGE is amortized, not the lifetime.** A durable record is written
    once, never expires, and its one-time fee is distributed to validators over 50 eras, one era being
    one year on mainnet, on a front-loaded declining curve (5.0% in the first era falling to 0.125% in
    the fiftieth, summing to exactly 1.0). The relevant constant is named for perpetual storage. So the
    objection is not that a prepayment cannot fund forever, it is that the WRONG prepayment was assumed.
    A custody obligation is charged at deposit on this schedule rather than on a lease term.
  - **Early discharge refunds the undistributed remainder.** The platform already refunds the
    unconsumed portion when a record is removed before the schedule completes. That makes the economics
    two-way and supplies exactly the incentive an abandonment problem wants, since a user who exits
    promptly recovers the remainder while one who never exits simply leaves it consumed.
  - **No code path destroys or reassigns a user's asset on a timer.** This is the reason to prefer this
    over v5's rule rather than merely a cheaper way to get there. Every horizon that actually fires must
    burn the asset, deplete it to pay rent, or transfer it to a governance recovery pool. The first two
    take a user's funds for missing a deadline they may never have known about, and the third widens the
    D14 governance surface that three rounds were spent narrowing. Removing the horizon removes all
    three.
  - **The residual subsidy is DISCLOSED, not resolved.** After the fiftieth era the fee is exhausted
    while the record persists, so validators carry it unfunded from then on. That is a real cost and the
    reviewer's hundred-year trace lands inside it. It is also a cost the platform already accepts for
    every durable record it holds, so a custody obligation is not a special case. The design states this
    rather than claiming a boundedness it does not have.

  What this leaves for measurement is only the AMOUNT: the byte size of a minimal obligation record,
  priced at the known 27,000 credits per durable byte. The policy shape no longer waits on it.

  REVISED (round 5, two sources, folded in v6). The framing above overstated: the 50-era schedule makes
  perpetual storage CONSISTENT with existing platform policy, it does not make it perpetually FUNDED.
  Past era 50 the record is unfunded, and an attacker abandoning minimum-value positions grows unfunded
  permanent state that the early-refund incentive does not deter. The perpetual shape STANDS (a
  confiscating timer is worse), and v6 makes two changes. First, the record no longer claims the
  schedule
  fully internalises the cost: the post-era-50 subsidy is stated as explicit inherited policy, the same
  subsidy the platform already carries for every durable record. Second, a custody obligation is subject
  to the SAME creation-rate and congestion controls the platform applies to other permanent state, and a
  minimal obligation record's attack cost and long-run burden are compared against an ordinary permanent
  document (the comparison itself is a Phase 0 measurement). An optional, owner-consented,
  delay-protected recovery path remains a narrow D14 alternative only if the burden proves intolerable,
  never a forced timer.

  REVISED (round 6, two sources, folded in v7). The v6 "SAME creation-rate and congestion controls"
  sentence above is made concrete and is now larger in scope, and this block supersedes that sentence.
  Two sources found "the same controls" underspecified in two ways. A per-principal rate limit is
  defeated by principal-splitting, because an adversary mints fresh identities and resets the limit, and
  a program running in a tight loop creates obligations far faster than a human creating documents, so a
  control calibrated to human document creation is the wrong scale. There is also a v7-specific reason
  the control must be strong. Under the D3b hybrid, a custody obligation no longer accrues a D3b
  service-curve liability, so the D3b admission bound that previously backstopped the stock of live
  custody obligations no longer applies to them. That backstop was removed deliberately to lift the
  ceiling on user balances, and the consequence is that this creation budget and the perpetual storage
  pricing are now the only controls on the rate and the stock of live custody obligations. This is a
  disclosed shift of the control point, not a gap. The concrete budget:

  - **Byte-weighted and system-wide, not per-principal.** Admission charges each new perpetual
    obligation against a global per-block budget measured in permanent bytes created, not in object
    count, so many small obligations and one large obligation are limited by the quantity they actually
    consume. The budget is global rather than per-identity, so splitting creation across fresh
    identities does not multiply it. The per-principal, per-family, and per-cohort concentration limits
    in D3b
    remain as anti-targeting controls, but they are not the capacity bound.
  - **No more favorable per permanent byte than an ordinary document.** A perpetual obligation byte is
    priced at least at the durable-byte schedule an ordinary document byte pays (the measured 27,000
    credits per durable byte input), on the same 50-era amortization, so the VM offers no cheaper path
    to permanent state than the platform already exposes. Underpricing here would make the VM a discount
    channel for exactly the unfunded permanent state this decision already discloses as a residual cost.
  - **Velocity ceiling set for automated creation, above the human-document baseline.** Because a
    program mints obligations at machine rate, the per-block byte budget for VM-created perpetual
    obligations is derived from the VM's realistic peak creation velocity, not from the
    document-creation rate a human client sustains, and a progressive surcharge rises with the current
    stock of unfunded
    (post-schedule) permanent state so that sustained automated minting becomes monotonically more
    expensive rather than flat. The peak-velocity figure and the surcharge curve are Phase 0
    measurements.

**D8. Proof construction outside consensus execution. UNCHANGED.** Praised by three reviewers.
Validators execute against the committed snapshot; proof serving is a query-layer activity funded by
credit-backed vouchers with response-size limits. In-consensus tree and index update hashing remains
its own metered dimension.

**D9. Program collections plus mediated native access, with hooks constrained. REVISED.** Programs
write only into manifest-declared collections; shared native objects are reached through typed host
calls where existing checks run. Round 1 found the adopted data-contract hook (v1's "cleanest legacy
path") is a native confused deputy: a hook could inherit the user's authorization or the contract's
authority, mutate unrelated state, or re-enter through native frames outside D12's guard, observing a
half-validated object. v2 constrains hooks: they run under a distinct hook principal with an explicit
capability envelope; a validation hook sees a read-only snapshot and returns a deterministic decision
or proposed computed values and may not mutate the contract that invoked it; native validation and
index updates apply only after the hook result is checked; and native-to-program entry uses the same
unified call graph, depth, re-entry, meter, and capability machinery as program calls (D12).

**D10. Version-scoped principals and non-delegable capabilities. REVISED (was the blocker).** Each
program VERSION, not family, gets a principal, exercised only while that exact version executes.
Capabilities carry an audience, a permitted call path, and a delegation flag, are non-delegable by
default, and narrowing must intersect both operation scope AND authorized audience, so passing an
allowance to a second program is treated as authority expansion even when every numeric limit
decreases. A delegated capability carries an authenticated chain from signer through each caller that
native host calls check against the current frame.

REVISED (round 2), two gaps the version-scoped principal left open:
- **Intrinsic authority must be checked against the full caller path, not just the current frame.**
  If version A holds an intrinsic family-custody grant for a withdrawal method, a hostile version B
  can call an entry point on A that reaches that method, and a native policy that sees only A as the
  current frame approves it even though B never received a custody capability. v3 binds every custody
  grant to permitted origins and complete authenticated caller paths, and native custody calls
  validate the whole chain. Entry through a hook, migration worker, recovery code, or another program
  must not reactivate ambient intrinsic authority absent an explicit path grant. (The authority-graph
  checker enumerates calls into every custody method from an unauthorized program or hook and must
  find no intrinsic-policy edge without an authorized origin.)
- **Family custody is an immutable META-policy plus per-version grants, not a fixed version list.**
  An immutable policy naming exact versions cannot authorize a successor whose code hash is unknown
  at family creation, so v2 was either unable to upgrade custody-bearing families or silently mutable.
  v3 separates an immutable meta-policy (who may propose a successor and the MAXIMUM authority that
  may ever transfer) from per-transition grants that each authorize one exact successor, exact
  methods, assets, caller paths, migration epoch, and expiry, and that never confer general
  family-principal authority. An alias still confers no authority.

**D11. Generic proof verification is CHOSEN; the native shielded-note service is DEFERRED. REVISED
(was overstated). PREMISE CORRECTED by Phase 0 measurement, 2026-08-08, decision UNCHANGED.**

> ROUND 5 CLARIFICATION (one source): `verify_proof` returns cryptographic predicate validity ONLY and
> confers NO authority over the native shielded pool. It does not check anchor eligibility, insert
> nullifiers, append commitments, or update value accounting, so a true result must never be treated as
> authority to move an asset (the same proof could be replayed through another path). "Interoperation"
> in
> the text below overstates it. v6 states that a deferred shielded VM interface means VM programs cannot
> read or mutate native shielded balances, notes, anchors, or nullifiers, and any future access must
> atomically combine verification with anchor check, nullifier insertion, commitment update, and value
> accounting. Note also that the deferral's RETENTION rationale is weaker now that D6 accepts permanent
> state, so scope and governance exposure are the sound grounds for deferring, not retention.

A
bounded, metered `verify_proof` host call for governance-registered systems, circuits, and keys with
fixed parser rules, input encoding, and size limits is chosen for v1. The native shielded-note service
(commitments, nullifiers, tree roots, note accounting) is NOT marked chosen for the execution-layer
release. Round 1 showed v1 both added it to the trust and cost model AND called it a separate project,
which leaves builders unable to tell whether note semantics are v1 host behavior. It also has no
retention model compatible with D6, because an unspent commitment cannot expire without burning an
asset and a spent nullifier cannot expire without permitting a double spend. It is additionally the
main lever behind the governance integrity finding (D14).

PHASE 0 CORRECTION. Earlier versions of this record stated that a native shielded subsystem does not
exist and that Dash privacy is coordinated mixing rather than a cryptographic shielded pool. That is
true of the Dash CORE chain and FALSE of Platform. Platform v4.0.0 ships a live shielded pool with
note and nullifier insertion, shielded transfer and withdrawal, identity creation from the shielded
pool, an Orchard and Halo 2 proving path, a snapshot and query surface, and a commitment-tree crate in
GroveDB whose anchor hashing is metered as its own cost dimension. Full evidence is in
`docs/PHASE0_FINDINGS.md`.

The DEFERRAL STANDS, on the two grounds that survive measurement rather than on the premise that was
wrong. The retention objection is now CONFIRMED rather than merely argued. The real implementation
answers retention with a three-way split: nullifiers are permanent and never pruned, because pruning
one permits a double spend; notes and commitments are permanent in an append-only commitment tree,
because pruning one burns an asset; and only anchors are pruned, on a 1000-block retention window that
is stable across protocol versions v2 through v8. So the platform did not solve the incompatibility
with D6's lease model, it accepted unbounded permanent growth for the two classes where expiry is
unsafe and bounded only the third. The D14 governance-integrity lever is untouched by this finding.

WHAT CHANGES IS THE FRAMING, NOT THE SCOPE. The chosen host call is no longer preparation for a
hypothetical verifier, it is interoperation with a subsystem that exists, is actively developed, and
has a known size model (about 2,681 bytes per action plus about 2,930 fixed, so the 20 KiB state
transition cap allows six actions and the nominal 16-action limit is not the binding constraint). That
is a stronger position than the record previously claimed, and it means the immutable
content-addressed verifier registry below has a concrete first counterparty rather than a speculative
one. v2 splits it into its own
architecture record with its own threat model, asset accounting, verifier lifecycle, recovery, and
metadata review. The base record reserves extension points and distinguishes generic bounded proof
verification from native note semantics. ADDENDUM (round 2, fourth sample): the generic verifier path
needs the same protection D14 gives the deferred note service. If a deployed program authorizes an
asset action when "verifier P accepts a proof" and governance can later replace P's key, circuit,
parser, or parameters, captured governance makes that unchanged program accept forged authorization
without touching its code or environment identifier. v3 makes every verifier entry immutable and
content-addressed, binds the exact verifier and parameter hashes into the call and the capability
preview, and permits governance only to ADD entries: changing which verifier a deployed program uses
requires a new program version or an application-policy transition with the owner opt-in, delay, and
exit window of D14.

**D12. One unified call graph, re-entry denied by default. REVISED.** Program-to-program AND
native-to-program (hook) calls live in a single call graph with one set of depth, cycle, re-entry,
meter, and capability-consumption rules. Re-entry is denied unless a manifest opts a named method in,
with limits still binding. Deferred work is only an explicit task object invoked by a later signed
transaction. Depth cap chosen after benchmarking (proposals 8 to 32).

**D13. Execution-environment versioning. NEW.** A program version freezes its code but not the native
services it calls. A change to token, identity, document, group, index, cryptographic, or schema
semantics can alter results without changing the program identifier, so validators on different host
releases can disagree. v2 defines an execution-environment identifier committing to the interpreter,
module validator, host ABI, canonical encodings, native validation semantics, cryptographic rules,
metering schedule, and store traversal rules. A program version plus a transaction resolves to exactly
one environment identifier, and any consensus-relevant host change creates a new environment version.
This is the umbrella that makes D2's "new runtime version" and D7's version binding well-defined.

REVISED (round 2): v2 defined the environment but never bound it to the signer, re-creating the
round-1 signed-intent mismatch one level up, since an environment activation between signing and
inclusion would execute semantics the client never observed. v3 tried a COMPATIBILITY FLOOR to avoid
invalidating the mempool on a repricing. REVISED AGAIN (round 3): that justification was hollow. D2
already separates credit prices from fixed work units and says a repricing is NOT a new execution
environment, so binding the exact environment does not invalidate the mempool merely because a price
changed, and a signer-selected list of "important properties" cannot be trusted to enumerate every
semantic dependency, since a native authorization rule the signer never listed can still change under
a newer environment and move assets under semantics the signer never approved. v4 binds the EXACT
semantic environment identifier in the signed transaction, with the credit-price schedule kept outside
that identifier per D2. Compatibility is a machine-checked relation over the transitive dependency
closure (the exact program, every reachable callee, every host operation, every verifier), computed by
the wallet and enforced by consensus, not a governance assertion that E2 is newer than E1. Environment
activations still occur only at epoch boundaries so clients sign predictably. The resolved environment
is persisted into asynchronous request objects (D16), deferred tasks, and migration epochs (D7). An
environment may not be retired while custody or irrevocable requests still depend on it unless D15's
terminal path covers them.

REVISED (round 4): the transitive dependency closure v4 requires the wallet to compute is not yet a
bounded consensus object, so the binding rule as written is not implementable. Two sources found this
independently, and a third cleared it only by assuming the signed capability preview is authoritative
over callees, which is the same fix stated as an assumption. The problem is that v4 imposes no finite
allowlist on call targets or verifier identifiers. A program may read a callee identifier from
authenticated state, take one from transaction input, or walk a registry whose contents change after
signing, and the call-depth cap bounds one execution trace rather than the static reachable set. A
wallet therefore cannot know or bound the closure it is required to sign. Neither available reading
works. If consensus validates only the realized trace, the signed preview is not a closure at all and a
state change can steer execution to a different callee between signing and inclusion. If consensus
tries to enumerate every possible target, admission cost is unbounded. Banning runtime-resolved calls
outright would remove polymorphism, which is a real cost the reviewers named. v5 bounds the closure
instead of banning dispatch:

- Every deployed program version commits, in its manifest, to FINITE allowlists of the exact program
  versions, host operations, and immutable content-addressed verifier hashes it may reach. The
  allowlist is part of the immutable version, so it cannot widen after deployment.
- Dynamic dispatch stays available and may select only from within those declared sets. A runtime
  target outside the allowlist traps rather than executing.
- Closure SIZE and closure-COMPUTATION work must fit explicit deployment-time and transaction-time
  limits, so both the wallet and consensus do bounded work and reach the same answer.
- The signed transaction binds a commitment to the closure AND the exact authenticated state
  preconditions that affect dispatch, so a change to a dispatch-selecting state key between signing and
  inclusion causes deterministic rejection before execution and before charging, never a silent
  substitution.
- At an epoch boundary a stale exact environment causes deterministic rejection. It is never upgraded
  through the compatibility relation, which is the property both samples from the blocking source
  certified and which this rule preserves.

The declared allowlist also gives D9's hooks, D11's verifiers, and D12's unified call graph one shared
notion of reachability, so the authority-graph checker and the signed capability preview operate over
the same bounded set rather than three different approximations of it.

ADDENDUM (round 2) SUPERSEDED (round 3, owner decision): v2/v3 introduced a "versioned canonical
adapter" to let a program on environment E1 call a program frozen on E2. All four round-3 reviewers
found it relocated the mixed-environment problem into an unspecified component that never inherited the
D1 semantics, D3 metering, trap, or D10 caller-path obligations, and that it contradicts the
single-signed-environment rule (executing the callee under E1 is the coercion the rule forbids;
executing it under E2 runs two interpreters in one transaction). v4 CUTS cross-environment
program-to-program calls entirely. Program-to-program calls are valid only when both programs share
the exact environment; a program that wants to call one frozen on a different environment must first
migrate to a shared environment (migration is the sanctioned mechanism for moving between
environments). The canonical adapter exists ONLY at the program-to-native-host boundary, never between
two program interpreters. If a concrete cross-version composition need ever appears, the sanctioned
extension is an ASYNCHRONOUS canonical-message boundary (the E1 source transaction commits a
content-addressed message binding the adapter version and both endpoint environments; a separate E2
transaction consumes it exactly once), which preserves single-environment-per-transaction and is
purely additive. It is documented, not built, because there are zero concrete use cases today and an
extension point built before two real uses exist is a guess.

**D14. Governance as a power-by-phase matrix, not one trusted-party row. NEW (resolves a blind
spot).** v1 treated governance as a pre-existing trusted party and framed the deployment gate as an
availability power that disappears when the gate opens. Round 1 showed that adding an execution layer,
asset custody, and verifier registration raises the value and the KIND of governance power:
registering a conservation-broken circuit lets captured governance authorize counterfeit shielded
value while every honest validator faithfully enforces the malicious rule, which is an INTEGRITY
attack, not an availability one. v2 replaces the single row with a matrix over deployment admission,
runtime activation, runtime pause, verifier registration, family upgrade authority, and emergency
recovery. For each power it states whether it affects availability, integrity, confidentiality, or
custody, whether it expires, and its maximum value at risk. Requirements that follow: a hard sunset
height after which the deployment gate permanently opens; verifier registration gated by delay,
independent review, and asset-owner opt-in, never altering the verifier of existing notes; runtime
pause bounded by a cumulative maximum that cannot be reset through repeated proposals, with a
mandatory challenge window; and delays plus user-exit windows wherever an action can change integrity
or custody. REVISED (round 2): the bounded pause is now paired with permanent RETIREMENT (D15),
because a runtime found unsound after its pause budget is exhausted must be retired to a defunct state
rather than force-resumed. And the deployment-gate sunset must NOT open the gate unconditionally:
permissionless activation stays fail-closed under an objective consensus predicate (D1's coverage and
mutation-testing thresholds) even at the sunset height, so an attacker cannot plan inputs for whichever
safety condition is unmet at a known block. Missing the predicate leaves the feature inactive without
extending governance discretion. ADDENDUM (round 3): RETIREMENT is added as its own matrix row,
distinct from bounded pause, because it permanently destroys execution availability, activates custody
exits, terminates migrations, and changes the value of pending objects. Its row names the exact scope
(see D15's subject distinction), authorizer, proposal, challenge, and delay rules, the treatment of a
depleted pause budget (retire, never force-resume), the value at risk, and the sunset behavior. The
retirement transition freezes new deposits and object creation atomically BEFORE it checks or lazily
terminalizes existing dependencies, so it can neither be blocked by an adversary racing new objects in
nor left partially applied.

**D15. Terminal-state exit, with a permanent defunct state separating retirement from bounded pause.
REVISED (round 2).** A runtime pause forbids substitute execution, but if a program holds user assets
and withdrawal requires executing its now-paused code, the assets are locked, and a time limit cannot
help because auto-resuming known-bad code risks a split. v2 required every custody-bearing program to
declare an immutable terminal-state exit policy BEFORE it accepts deposits (pre-signed conditions,
time locks, or an exact-version recovery grant not requiring the paused runtime), with terminal-state
drills as a release gate. Round 2 found the fatal seam: D14's pause is bounded and auto-resumes, but
the resumed runtime is oblivious to the asset withdrawals users made through the terminal-exit path
during the pause, so it operates on phantom balances and permits double-spends. v3 separates two
governance states that v2 conflated. A bounded PAUSE is temporary, preserves state, and may resume,
and while paused NO terminal exit may execute. Permanent RETIREMENT is a distinct, irreversible state
entered when a runtime is found unsound: it disables new calls and deposits forever, permits ONLY
native terminal exits and the drainage of already-irrevocable requests, and can never resume. The
moment any terminal exit executes, the affected runtime is permanently defunct, and consensus
resume-from-pause must check for and deny execution if a terminal exit was invoked. This closes the
double-spend and also resolves the D14-versus-D15 deadlock (a runtime found unsound after its pause
budget is exhausted is retired, not force-resumed). It pairs with a D14 fix: governance's
permissionless-activation gate must stay fail-closed under an objective consensus predicate even at
the sunset height, so missing the predicate leaves the feature inactive rather than extending
governance discretion.

REVISED (round 3), three gaps all four reviewers converged on:
- **The retirement SUBJECT must be named precisely.** v3's "runtime" blurred an execution environment,
  a program version, and a family, so a single user's terminal exit could read as retiring the whole
  shared environment and disabling unrelated programs, while the environment-retirement case D13 needs
  went unserved. v4 defines separate monotonic state machines for an execution environment and for a
  program version, and every request, hook, task, migration, and exit record names its exact
  retirement subject. A user terminal exit retires ONLY that user's own custody program, never a
  broader subject. Environment retirement is a governance action under the D14 matrix.
- **Every pending object class gets a native terminal disposition.** v3 named only custody programs
  and already-irrevocable requests. v4 adds an exhaustive table, none of whose paths call retired
  code: a cancelable request is natively canceled and its escrow refunded (safe because D16 forbids
  any share before irrevocability); an irrevocable request is settled from the finalization record
  below; a deferred task is deterministically tombstoned under a declared fee rule; a migration takes
  the D7 fixed-cost logical abort followed by funded batched cleanup; a hook binding follows an
  immutable retirement behavior chosen when the hook was registered (a native transaction whose hook
  is unavailable fails before any mutation rather than becoming permanently stuck); and lease records
  needed for an exit are pinned per D6. Retirement is refused while any object lacks a defined native
  terminal path, but the transition itself is constant-cost (a retirement generation marker plus
  indexed, incrementally processed queues, never an enumeration of all objects in one transaction, so
  it cannot exceed the one-fifth bound or be blocked by an adversary creating many objects first).
- **Irrevocable requests settle from a pre-materialized native finalization record.** Before the first
  quorum share is produced, the request materializes an immutable native record carrying the exact
  external payload, domain, chain identifier, amount, destination, request identifier, cryptographic
  suite, quorum rule, and environment hash. Retirement settlement only finishes that fixed recipe
  through a separately versioned native finalization kernel, so drainage never re-derives payload
  bytes from the retired environment. If the finalization kernel or the fixed recipe is itself the
  reason for retirement, terminal recovery closes the request without producing more shares and never
  restores the consumed credits, and the design states that loss case openly.

REVISED (round 4). v4 named a retirement subject per object but never established that the subject is
UNIQUE, and uniqueness is the property the whole disposition table rests on. One source found this in
both samples and rated it a blocker, with two concrete reproductions. A second source had cleared the
subject split as free of ownership races, and its own account shows that clearance was reached without
considering shared custody. The reproductions are specific enough to settle the disagreement.

The first reproduction is multi-user custody. v4 says a user terminal exit retires only that user's own
custody program, while the only program-level state machine it defines is program-version-wide. If one
program version holds balances for two users, Alice's exit either retires the whole version, which
destroys Bob's execution availability with no policy authorizing that, or it retires something narrower
than a version, for which v4 defines no state machine at all.

The second is overlapping subjects. Several object classes depend on more than one subject at once. A
migration depends on both programs and both environments. An asynchronous request depends on its
program version and on the environment frozen into its finalization record. Indexed under only one
subject, retirement of the other misses it and strands escrow or pinned state. Indexed and terminalized
under both, two retirement queues can race to refund, tombstone, or settle the same object.

v5 resolves both with one ownership rule.

- **A CUSTODY POSITION is its own retirement subject**, with its own state machine, distinct from the
  program version holding it. A user terminal exit retires that user's position and nothing wider, and
  never alters another user's execution rights. Where a deployment intends single-user custody programs
  only, that restriction is stated and ENFORCED before any deposit is accepted rather than assumed.
- **Every pending object carries an immutable DEPENDENCY SET** naming every program version and
  environment whose retirement affects it, fixed when the object is created.
- **Exactly one canonical native lifecycle owner per object**, holding one terminal state word. Every
  subject index is a pointer into that single state machine.
- **Retirement markers may ENQUEUE references to an object and acquire no independent disposition
  authority over it.** Only the canonical owner transitions the object.
- **Every disposition is an atomic compare-and-set from the object's current state**, so two queues
  reaching one object produce one transition and one loser that observes the completed state.
- **Precedence is defined for overlapping retirement**, so an environment and a program version
  retiring together produce an outcome determined by rule rather than by queue arrival order.
- **A terminal exit retires no subject beyond the position it discharges.** Whether an exit may occur
  before program retirement, and what it implies for the program, is stated rather than inferred.

The completeness test belongs in the Phase 0 model checker rather than in prose, and the reviewers
supplied it. Model two users, one custody program, one environment, and one irrevocable request, then
enumerate both retirement orders against every queue interleaving. Every asset and request must have
exactly one terminal owner throughout, must reach exactly one terminal state, and one user's exit must
not change another user's execution rights unless an explicit policy allows it.

**D16a. How atomicity is obtained, stated rather than inherited. NEW (Phase 0, 2026-08-08).** Several
decisions here assume that a state transition either applies wholly or not at all, D16 and the round-4
compare-and-set dispositions most of all. Phase 0 CONFIRMED that assumption against the real code, and
recorded the mechanism so a future implementation cannot lose it by accident. Atomicity comes from three
things working together, none of which is rollback: state transitions are validated to completion and
produce an execution event BEFORE anything is written, so a failure never starts rather than being
unwound; writes are accumulated and applied through the store's atomic batch path; and the whole block
proposal runs inside one rollback-able store transaction. Evidence is in `docs/PHASE0_FINDINGS.md`.

ROUND 5 TIGHTENING (one source; the other two certified D16a sound under exactly the single-batch
assumption this makes explicit). The atomicity claim holds only if EVERY effect is in one
transaction-wide batch. A nested native host op that applies its OWN atomic batch mid-execution would
survive a later trap while other effects roll back, so each batch is atomic but the outer transition is
not. So the batch rule below is strengthened: there is ONE transaction-wide effect plan covering the
complete authenticated call graph (nested programs, hooks, native token and escrow operations,
reservations, counters, task creation), execution reads through a transaction-local overlay for
read-your-writes, NO nested host op may call any store apply path, and the single final batch is applied
only after the top-level call completes and every invariant passes.

Two rules follow, and they are rules rather than observations because both are currently free properties
that a reasonable implementation could discard without noticing:

- **A program's state effects are accumulated and applied as ONE atomic batch.** Never
  operation-at-a-time during execution, and never through a nested host op's own apply path (see the
  round-5 tightening above). The store offers an unbatched apply path that is documented for
  testing only, and whose own documentation warns that a mid-list failure leaves earlier operations
  applied and that resulting root hashes differ from the batch path because batching propagates hashes
  in a single pass. That path is PROHIBITED for the execution layer. This is an escape hatch in the
  D-series sense, so it is named, its prohibition is explicit, and what it would bypass (atomicity and
  root-hash agreement, therefore consensus) is stated.
- **The execution layer does not adopt the multi-inner-transition pattern.** The platform's
  documents-batch transition carries multiple inner document operations and is capped at one because
  that path applies earlier successful inner operations when a later one errors, with undefined
  nonce-bump semantics for mixed outcomes (upstream issue 2867). A program's operations are inner
  effects of ONE transition, resolved by the batch rule above, not a sequence of independently applied
  sub-transitions. The design does not depend on the upstream path and does not wait on its repair.

**D16. Escrow-through-every-cancelable-state for asynchronous requests. REVISED (round 2).** A program
could submit a bridge release request, then spend the same L2 credits before the quorum result
arrives, or submit several pending requests against one balance, because the later L1 release is
outside the original atomic call tree and post-execution native checks cannot catch it. v2 said the
request could atomically "burn or escrow" the credits. Round 2 showed burn-at-request collides with
the request's own cancellation and expiry (D5): burning first means a later cancel either loses user
funds or restores spendable credits while a threshold signature may still release L1 value. v3 removes
the burn option: credits are ESCROWED (not burned) through every cancelable state; a finalized
transition atomically makes the request irrevocable AND consumes the escrow BEFORE any quorum share is
produced; only pre-irrevocable requests may cancel; and an irrevocable request either drains or enters
a terminal recovery path that can never restore spendable L2 credits while an externally valid
signature might exist. The request binds one destination and amount, a globally unique identifier, the
resolved execution environment (D13), and is idempotent on completion. This burn-versus-reserve
ordering is the same hazard the DashDollar redemption review kept surfacing, and it is now the third
project where escrow-before-irrevocability is the answer, so treat it as a standing Platform-bridge
rule, not a per-project quirk. ADDENDUM (round 3): the conservation guarantee was written only for L2
credits and L1 signatures, but a NON-custody async result (randomness, an attestation) can be drained
under one environment and consumed under another after an epoch change or retirement, so a usable
external artefact can exist while L2 has already treated the request as resolved. v4 lifts the
invariant from "no spendable credits while a signature might exist" to "no usable external artefact
while the corresponding L2 obligation has been cleared": every async domain's terminal path emits a
domain-separated, environment-bound, content-addressed receipt (parallel to the D11 verifier entries),
and any later consumer must verify that receipt against the originally bound environment and reject
any use that cannot.

**D17. Migration granularity is the dependency component, gated at composition admission. NEW (round
4, owner decision).** Cutting the cross-environment adapter (D13) made migration the only way to move
between environments, and v4 never stated the availability consequence. All three sources reached it
independently, two as findings and one as a recommendation, which makes it the best-corroborated
finding of the round. It has three distinct shapes, and a fix that handles only one is not a fix.

A fourth shape, REVISED (round 5, one source; the other two certified D17 sound, and this makes explicit
the condition they assumed): a SHARED stateful library. Independent programs A and B both call mutable
library L but not each other, so their strongly connected components are separate and the "component
plus
its dependencies" cohorts {A,L} and {B,L} OVERLAP, and migrating one strands the other on L. v6 defines
migration over DISJOINT DOMAINS, not overlapping outgoing closures. Any stateful, non-replicable
dependency edge joins both endpoints and all reverse dependents into one domain; immutable stateless
dependencies may be copied only under a separate environment-lift rule; admission verifies the whole
domain has the authority and size for one cutover, or classifies the composition terminal-only and
discloses the outage. The native terminal path remains the safety floor, so this is availability, not
asset loss. NOTE (v8): a review asked whether a domain union here can invalidate D3b's
terminal-liability
accounting (two domains merging to exceed a bound). It cannot, because v8 makes the D3b bound a global
rate-matching condition (creation flow at or below the drain rate R) rather than a per-domain or global
liability SUM, and a rate is not domain-scoped, so a union changes nothing and this admission check
needs
no separate liability re-check. See D3b, section 3.

- **The abandoned dependency.** Environment E1 holds a widely used immutable library L and a dependent
  program A. Governance retires E1. A must migrate to E2 to survive, but L's author has lost their keys
  or declines to move. A cannot stay on E1 because E1 is retired, and it cannot call L across
  environments because that path is cut, so A is permanently uncallable.
- **The mutual stateful pair.** A and B call each other on E1 and both must move. If B cuts over first,
  active A on E1 cannot call B on E2. If A cuts over first, the symmetric failure occurs. Keeping both
  versions live works only when B's state and authority can safely be active in two environments, which
  the single-authoritative-route and version-scoped-custody rules do not generally permit.
- **The frozen component.** A dependency frozen by an immutable upgrade policy, or governed
  independently by a party that declines to move, pins every caller to E1, so retiring E1 makes the
  whole connected component uncallable.

The mitigation already present must be stated before the fix, because it changes what the fix is for.
The native terminal path (D15) means no asset is lost in any of these shapes. Every custody position
exits through native machinery that does not call program code. So retirement of a stranded component
is an AVAILABILITY and LIVENESS event, not a loss-of-funds event, and the question D17 answers is how
much mechanism is justified to preserve availability given that safety already holds.

The owner decision is to make the migration unit match the dependency graph and to refuse unsafe
compositions at admission, rather than to reinstate any cross-environment call path.

1. **The migration unit is a disjoint dependency DOMAIN, not the program and not an SCC.** REVISED
   (round 5/6): an earlier draft defined the unit as the strongly connected component plus its
   dependencies, but that produces OVERLAPPING cohorts for a shared stateful library (independent
   callers A and B of one mutable library L give cohorts {A,L} and {B,L}, and migrating one strands the
   other). The unit is instead the TRANSITIVE EQUIVALENCE CLOSURE of mutable, non-replicable stateful
   dependency edges, including reverse dependents, so every program is in exactly one domain and the
   shared library pulls all its callers into the same domain. ONE authenticated cutover commitment
   covers every stateful member that cannot tolerate a mixed interval. Immutable stateless dependencies
   belong only to the separate environment-lift rule, never to a domain. Any graph change that would
   merge domains must atomically recertify authority, size, liability, and migration state, or be
   rejected. This resolves the mutual stateful pair, the movable library, and the shared library, since
   a whole domain migrates as a unit.
2. **Composition admission gates the graph before any obligation exists.** Before a program may accept
   custody or any other non-abandonable obligation, its dependency component must have either a viable
   cohort-migration path to a shared future environment or a complete native terminal path for every
   obligation. A graph with neither is REFUSED at admission. Each deployment declares, per dependency,
   whether it is migratable, replaceable, or permanently environment-pinning, so the pinning case is
   visible rather than discovered at retirement.
3. **The migration component is surfaced before users commit anything.** The component, including
   immutable and independently governed members, appears in the signed capability preview and in wallet
   and deployment tooling, so a permanently pinned dependency is visible before a deposit rather than
   after. Migration logic may not depend on a peer being live during cutover.
4. **The native terminal path is the stated safety floor.** Where availability cannot be preserved, the
   design says so plainly and names the bound: the component becomes uncallable and every obligation
   discharges natively. Retirement is a liveness event with a guaranteed exit.
5. **The asynchronous cross-environment message boundary stays where v4 put it**, documented as the
   sanctioned future extension and not built, for the reason v4 gave. There are still zero concrete
   cross-version composition use cases, and an extension point built before two real uses exist is a
   guess.

Rule 2 is invariant class 9, refusal, in the shared assurance vocabulary. Admission rejects a state
that settlement would later be unable to honor, which is the same discipline D6 applies to expiry
schedules and D3 applies to reservation vectors.

WHAT MEASUREMENT COULD SIMPLIFY. Two sources noted that the burden of this rule depends on real
dependency-graph width, which nobody has measured. If Phase 0 shows components are almost always small
and shallow, a lighter combination becomes defensible: an environment-lift mechanism that lets an active
program carry an immutable, stateless dependency into a new environment after that dependency's code
passes the new environment's static validator, plus an accepted and documented outage for the rest.
Environment-lift alone was proposed by one source and is NOT adopted as the primary rule, because it
does not address the mutual stateful pair or a stateful dependency with lost keys. It is recorded as the
Phase-0-gated simplification, and dependency-graph width is added to the VERIFY list below.

## Novel ideas retained, plus round-1 additions

Retained from the clean-room round: reserved protocol lanes, reproducible builds with deployed-code
hash verification, two independent interpreters (now explicitly a diagnostic, not an oracle, per D1),
shadow-mode replay across heterogeneous machines, signed amount-bound fee sponsorship, state
preconditions in calls (now required to be scoped to specific keys rather than the state root, so
shared-state applications are not capped at one transaction per block), a refundable deployment bond,
a local reference-semantics simulator, deterministic divergence-replay tooling, and validator startup
refusal on a missing interpreter or wrong conformance hash.

Added from round 1: a machine-readable consensus cost-coverage map from every validity-path operation
to a resource dimension, an upper-bound rule, and an adversarial benchmark, failing the build if any
operation has no owner. An authority-graph checker that generates the effective authority graph for a
signed call (signer, exact version, family policy, principal, allowances, delegated capabilities,
hooks, nested calls, async requests, migration grants) and rejects any edge that widens audience or
survives an upgrade without an explicit grant, which makes the capability findings testable as one
invariant. Signed capability previews, where a wallet obtains a deterministic preview of exact
versions, possible native operations, transitive callees, maximum asset movement, expiry, and fee
vector, and the user signs the preview hash so consensus can verify execution never exceeds it.

Added from round 2: an adaptive cleanup fraction (zero at zero backlog, rising toward a documented
maximum) folded into D6, removing the permanent capacity tax while still guaranteeing progress. An
ephemeral state class for high-frequency non-critical data (temporary oracle price feeds) that lives
only in a rolling window or accumulator and drops off without metered physical deletion, bypassing the
D6 cleanup lane entirely. A pre-flight compatibility check extending the signed capability preview:
the node simulates a pending transaction against the upcoming epoch-boundary environment activation
and rejects it before charging if a scheduled D13 change would invalidate it, saving the user the
non-refundable reservation. And an authority-graph edge that treats a family-custody meta-policy entry
itself as a capability whose audience is its named versions, so the existing checker automatically
rejects any migration or custody transfer that would widen that audience without an explicit grant.

## Trust model

No new mandatory trusted party. Existing trust: the BFT validator threshold, L1 proof-of-work and
quorum assumptions, the specific immutable program code and its declared upgrade policy, the
correctness of the runtime and native services and cryptography and the user's own proof
verification, and any application-chosen oracle, adapter, or upgrade authority the program names.
Governance is now accounted through the D14 matrix rather than one row, because verifier registration
and runtime pause are integrity and availability powers whose value rose with the execution layer.
Version-scoped principals (D10) mean an upgrade authority's blast radius is bounded to the migration
rights the old version granted, which is what v1's trust table already claimed but its mechanism
contradicted. The registered-proof-system row carries the largest exposure and is the reason the
shielded-note service is deferred to a separately reviewed record (D11).

## Feasibility items to VERIFY against the real codebase

PHASE 0 STATUS, first session 2026-08-08. Four of the carried items below are now answered or partially
answered, and `docs/PHASE0_FINDINGS.md` is authoritative for which. In short: proof shapes are present
and richer than assumed, with completeness an enforced guarantee; the cost model and price list are
exact and the cost unit is already a five-field vector, which confirms rather than merely permits D3;
the Tenderdash slice is bounded at 3 seconds for round zero with its interior split still unmeasured;
and shielded compatibility is answered, with D11's premise corrected and its decision unchanged. Two
items will NOT yield to source reading. Masternode hardware distribution needs a network survey. Real
dependency-graph width for D17 cannot be measured at all, because it asks for properties of programs
that do not exist yet, which makes that item mis-specified rather than outstanding.

Carried from v1: GroveDB proof shapes at VM throughput; real cost of authenticated-tree and index
updates per write; the Tenderdash execution slice after consensus and networking; whether
data-contract validation is callable mid-execution and whether a contract can name a program hook
(now with the D9 hook-principal and phase constraints); whether credits accounting can express
multidimensional reservations, partial refunds, and a reservation-pricing function (now the D3
version, pricing the whole vector); DIP-2 and DIP-7 acceptance of typed, escrowed, program-originated
requests under quotas (now with D16 atomic reservation); real masternode hardware distribution; and
whether the store can maintain program-declared secondary indices from a schema without trusting
program-supplied entries.

Round 1 added three more, and the 2026-08-08 reading pass reached two of them:

- **Iteration tie-break determinism. ANSWERED for the paths read.** No implementation-defined tie-break
  was found. Keys are unique within a subtree, so no two entries tie on key. Equal INDEXED values cannot
  tie either, because a non-unique index nests its members under key `[0]` keyed by the document's
  primary key, making the order total on indexed value then identifier. Direction is explicit through a
  `left_to_right` flag, and contract index definitions iterate over an ordered map rather than by
  insertion or hashing. Stated at its real width, this is the absence of a tie-break in the paths read
  rather than a determinism proof for the whole store: multi-path queries and cross-subtree result
  assembly were not examined, and a determinism claim should rest on execution evidence.
- **The D6 cleanup lane, accumulator, and height-filtered views. ANSWERED.** The accumulator half is
  native: the store's element types include `SumTree`, `CountTree`, `CountSumTree`, and `BigSumTree`,
  and the proof module carries matching aggregate count and sum proofs. That lands directly on D3b,
  which requires O(1) authenticated per-subject per-class counters so a constant-cost retirement marker
  can read a backlog size without enumerating it. A `CountTree` IS that primitive, it is authenticated,
  and its count is provable, so the rule rests on existing machinery. The height-filtered view half is
  an APPLICATION PATTERN, not a native primitive: a by-height secondary index with big-endian keys
  queried by range, used in production for shielded anchors and the withdrawal queue. So a D6 logical
  expiry view is buildable and precedented, but a read path must consult the index rather than expect
  the store to hide expired entries, which pairs with the D6 never-reach-zero cleanup obligation.
- **The credit meter across the transaction lifecycle (D3). ANSWERED for charging, with one boundary.**
  Cold-cache load is metered as the loaded-bytes dimension (contract fetch returns a metered cost, and
  program bytecode load would use the same path); rollback is metered because a failed transition with
  a proved identity becomes a `PaidConsensusError` charging work done up to the failure; commit-prep
  hashing is the node-hash dimension. The boundary: this is a fee and accounting meter, not a hard
  execution-abort meter, since Tenderdash sets block gas to unlimited. Halting execution at a resource
  bound, which D3 also needs as a correctness gate, is a VM-layer build obligation the fee meter does
  not provide. Recorded so a later reader does not mistake the accounting coverage for an execution cap.

Added by round 4, each one a quantity a v5 rule depends on and none of them measured:

- **Closure computation cost (D13).** Whether the bounded dependency closure over declared manifest
  allowlists can be computed inside a transaction-admission budget, and whether the wallet and consensus
  provably reach the same result. This decides whether the closure limits are practical or merely
  stated.
- **Real dependency-graph width (D17).** The distribution of strongly connected component sizes and
  depths in realistic application graphs. Narrow components would justify simplifying D17 toward
  environment-lift plus an accepted outage. Wide ones would justify keeping full cohort migration.
- **Terminal drain rate and the per-class terminal-work vector (D3b, v9).** The drain rate R, the
  per-batch latency W, and the worst-case terminal-work vector per object class (single-owner and
  autonomous custody, irrevocable-request settlement, deferred tasks, hook bindings, logical-only
  reclamation), in terminal-work units per dimension. GroveDB's worst-case estimators cover the storage
  dimensions; the VM work vector is an implementation obligation. These set the drain lane and, with the
  deadline-free flow condition, the meter's admission.
- **The rate-matching invariant and the terminal-work meter (D3b, v9), the single coupled number.**
  Whether the per-block deadline-free terminal-work flow ceiling can be sized at or below the drain rate
  R
  (same units) without starving legitimate creation, and whether the meter's coverage from every D15
  disposition back to the creating or mutating transition is total. This is metered in terminal-work
  units by the meter (2a), NOT by the D6 byte budget, which stays a separate storage control.
- **The known_due share for deadline-bearing settlement, and authority-upgrade liveness (D3b, v11).**
  Whether reserving worst-case irrevocable-request settlement into the dated known_due bucket at its
  terminal-timeout height fits the known_due share under a realistic density of clustered deadlines. The
  gate is certified safe (a burst that exceeds the share fails atomically to cancelable), so the open
  question is THROUGHPUT, whether irrevocability admission under the share is acceptable at realistic
  request rates, or the disclosed create-time-reservation variant is warranted. The same measurement
  should check whether positive-marginal reclassifications are delayed unacceptably under congestion,
  which would trigger the disclosed authority-maintenance subshare option.
- **Cleanup partition point (D3b, v9).** Where to split C_total among its three shares (the due-work
  budget known_due, the drain-rate share equal to R, and the overdue reserve) so realistic admission
  density and worst-case overdue feedback both fit under C_total. The partition and the invariant
  floor(h) <= C_total are folded and structural; only the split point is a measurement, coupled to the
  drain-rate item since both draw on the same ledger.
- **Creation budget for perpetual obligations (D6, v9).** The byte-weighted, system-wide creation budget
  is folded; its numbers are the VM's realistic peak per-block creation velocity that sets the byte
  budget, the progressive surcharge curve keyed to the stock of post-schedule unfunded state, and the
  comparison of a minimal obligation record's attack cost and long-run byte burden against an ordinary
  permanent document, which decides whether the post-era-50 subsidy is tolerable or a recovery path is
  needed. In v9 this is a STORAGE control (permanent bytes), separate from the terminal-work meter,
  which
  is what enforces the rate-matching invariant.
- **Obligation ledger horizon (D6). PARTIALLY ANSWERED by Phase 0, with a usable precedent.** The
  question was what absolute maximum lifetime for an abandoned custody obligation is long enough never
  to strand a live user and short enough to bound state. The pinned-record storage cost input is now
  known at 27,000 credits per durable byte. More useful is that the platform has already faced this
  exact choice in its shielded pool and answered it, which gives a precedent rather than a guess:
    - For state where expiry is UNSAFE at any horizon (nullifiers, note commitments), it does not set a
      horizon at all. It accepts permanent unbounded growth and charges durable storage up front. So
      "pick a finite horizon" may be the wrong shape for the abandoned-obligation case, and the
      alternative to weigh is a permanent obligation priced at deposit.
    - For state where expiry IS safe (anchors), it uses a bounded retention window of 1000 blocks.
    - The trap it hit is directly on point for D6 and is regression-tested upstream. Pruning the anchor
      index naively empties it, which empties the primary anchors tree, which then rejects EVERY spend
      with an invalid-anchor error until new activity refreshes state. The fix is a floor that always
      keeps the highest entry. This is the same shape as the round-4 rule that leased state may not be
      sole custody evidence, and it argues that any cleanup rule in D6 needs an explicit
      never-reach-zero floor rather than only a rate limit.
  DECIDED 2026-08-08: the permanent priced obligation, following the platform's perpetual-storage model
  with its 50-era amortized charge and refundable remainder, and the finite horizon is removed. See D6.
  What remains is the AMOUNT rather than the shape, meaning the byte size of a minimal obligation record
  at the known 27,000 credits per durable byte.

## Author-design and synthesis corrections recorded

The author's own clean-room design ranked weakest of the five and was corrected in six places (v1
recorded five plus the re-entrancy gap). Round 1 then corrected the SYNTHESIS itself, which is the
process working on the synthesizer: two blocker contradictions introduced while folding the fifth
design (family principal, alias resolution), an incomplete security claim (D4 attestations), a
self-contradiction (D2 repricing versus outcome stability), an overstated determinism claim (D1 "by
construction"), an over-marked decision (D11 chosen versus deferred), and an entire mechanism
(D6 leases) that was attacked from six angles and rebuilt.

Round 2 then did to the round-1 folds what round 1 did to the original synthesis: it found the seams.
Almost every round-2 finding was a contradiction BETWEEN two round-1 decisions rather than a flaw in
any one, which is the expected and healthy failure mode of folding fixes into a synthesized document,
and it is the reason the discipline requires iterating full passes rather than stopping at the first
clean-looking draft. v3 folded them.

Round 3 then reviewed v3 and, for the first time, showed convergence rather than a fresh crop of
contradictions: two reviewers explicitly certified sound decisions, no reviewer opened a new
architectural front, and the open work concentrated on one un-designed component (the adapter, now
CUT by owner decision) plus retirement completeness. v4 folds it: the exact-semantic environment
binding replacing the hollow-justified floor (D13), the adapter cut (D13), the one-fifth cap measured
against the total with a guaranteed cleanup service floor (D3b), leased state barred as sole custody
evidence (D6), the full retirement specification with a named subject, an exhaustive disposition
table, and pre-materialized finalization records (D14/D15), the two-phase migration TTL abort (D7),
and environment-bound receipts for non-custody async artefacts (D16). Per the discipline, v4 requires
a FOURTH fresh full pass before it is called review-complete. The trajectory finally points toward a
clean pass, but that is a prediction round 4 will confirm or refute, not a conclusion. Three rounds
found real blockers, and only a fresh full pass returning APPROVE with nothing real to fold ends it.

Round 4 ran that fourth pass and refuted the clean-pass prediction, which is worth recording plainly
because the prediction was the synthesizer's own. Two sources accepted with fixes and one blocked, and
the findings were real rather than procedural. The pattern of the round differs from rounds 1 through 3
in a way that matters. Earlier rounds found contradictions BETWEEN decisions, the characteristic defect
of folding fixes into a synthesis. Round 4 instead found INCOMPLETENESS inside three individual v4
additions, each one a case the addition's author had not enumerated: a subject that was named but not
proven unique, a protection scheduled after the threat it guards, and a closure required to be computed
without anything bounding it. That shift, from cross-decision contradiction to within-decision
incompleteness, is consistent with a design whose structure has settled and whose specification has not.

v5 folds six of those items and explicitly declines one, which turns on an unmeasured quantity. Three
further observations belong in the record. The blocking source supplied both samples for two of the six
folded items, so those folds rest on one voice rather than two, and one of them (the migration source
materialization rule) rests on a single sample. Where a second source had positively CLEARED ground that
the blocking source then attacked, the disagreement is recorded at the decision itself rather than
resolved by counting reviewers, and in both such cases the finding won on the strength of a concrete
reproduction against a general assessment, which is a judgment the next round is free to revisit. And
the sixth fold was initially scoped OUT of v5 as measurement rather than architecture, then folded after
a closer read showed its structural half (reserve terminal work at creation, refuse what cannot be
terminalized) needs no measurement at all and that the mechanism already existed in D6 for leased keys.
That misclassification is worth recording, because the same shape could hide other foldable rules
inside items dismissed as merely quantitative.

Whether a fifth review round is worth running before Phase 0 is a deliberate open decision, not an
oversight. The argument against is that the two unfolded items and five of the newly added VERIFY
entries are all denominated in numbers Phase 0 produces, so a fifth prose round would re-derive
speculation about the same unmeasured quantities. The argument for is the standing rule that only a
fresh full pass returning APPROVE with nothing real to fold ends a review loop, and v5 has not had one.
The design record is therefore CONVERGED but NOT review-complete, and those are different claims.
