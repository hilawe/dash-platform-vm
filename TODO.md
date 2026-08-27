# Execution layer for Dash Platform, initial TODO

Seeded 2026-07-19 from the clean-room round's divergences, novel ideas, and VERIFY items. Ordered
riskiest assumption first, matching the unanimous verdict of all four designs that determinism is
the assumption whose failure kills the design.

This is exploration, not a commitment to build. Phase 0 exists so the rest is grounded in what Dash
Platform actually does rather than what four designs assumed it does.

## Owner decisions raised by Phase 0

Three come from `docs/PHASE0_FINDINGS.md`. Each touched a design decision rather than merely leaving
it unverified. An assistant does not get to re-decide a design decision, so these were staged for a
decision in prose. All three are now decided.

- [x] DECIDED 2026-08-08 (owner), DERIVED from the shielded decision: the abandoned custody obligation
      is PERMANENT AND PRICED, and v5's finite absolute horizon is REMOVED (D6). Phase 0 supplied a
      verified production model rather than a hunch. Platform treats durable state as perpetual and
      amortizes the ONE-TIME charge over 50 eras (one era is one year on mainnet) on a front-loaded
      declining curve, 5.0% in the first era to 0.125% in the fiftieth, summing to exactly 1.0, with the
      governing constant named for perpetual storage. The undistributed remainder is REFUNDED when a
      record is discharged early, which makes the economics two-way and rewards a prompt exit. So the
      round-4 objection (a finite prepayment cannot fund unbounded storage) is answered at the payment
      layer: the wrong prepayment was assumed, not the wrong lifetime. Decisive additional reason: every
      horizon that actually fires must burn the asset, deplete it to pay rent, or hand it to a
      governance recovery pool, so removing the horizon removes all three, the last of which widens the
      D14 surface three rounds were spent narrowing. DISCLOSED residual: after the fiftieth era the
      record is unfunded, which the platform already accepts for all durable state. A size-split
      alternative was considered and rejected because the shielded pool keeps even its bulky encrypted
      note payload permanent, so it offers no precedent for expiring a payload while keeping a minimal
      record. Carried separately: the shielded pruner's regression-tested trap, that a cleanup rule
      reaching zero froze every spend until new activity refreshed state, so any D6 cleanup rule needs
      an explicit never-reach-zero floor and not only a bounded rate of change.

- [x] DECIDED 2026-08-08 (owner): CORRECT THE PREMISE, KEEP THE DEFERRAL. A native shielded subsystem
      already exists in Platform v4.0.0, so D11's factual premise was wrong and is corrected in place.
      The DECISION is unchanged, because the two substantive grounds survive measurement while the false
      premise did not. Retention is now CONFIRMED incompatible with D6's lease model rather than merely
      argued: the real implementation keeps nullifiers permanent and never pruned (pruning permits a
      double spend), keeps notes permanent in an append-only commitment tree (pruning burns an asset),
      and prunes only anchors on a 1000-block window stable across protocol versions v2 through v8. The
      platform accepted unbounded permanent growth for the unsafe classes rather than solving the
      problem. The D14 governance lever is untouched. What changed is framing: the chosen host call is
      interoperation with a live subsystem, not preparation for a hypothetical one, and it now has a
      concrete first counterparty with a known size model (about 2,681 bytes per action plus 2,930
      fixed, so six actions fit the 20 KiB cap and the nominal 16-action limit does not bind).
- [x] DECIDED 2026-08-08 (owner): ATOMICITY ASSUMPTION CONFIRMED SOUND, one obligation added as D16a.
      This item was first raised here as a contradiction and it is not one. Closer reading of the
      execution path showed the platform DOES provide state-transition atomicity, by validate-then-apply
      (a transition is validated to completion and produces an execution event before any write, so a
      failure never starts rather than being unwound), by applying accumulated writes through the
      store's atomic batch path, and by running the whole block proposal in one rollback-able
      transaction.
      The design's assumptions all live inside a single state transition, which is exactly where those
      guarantees hold. The real defect is confined to the documents-batch transformer carrying multiple
      inner operations, capped at 1 because it applies earlier successes when a later one errors with
      undefined nonce semantics (upstream 2867). D16a now states the mechanism and adds two rules that
      were previously free properties an implementation could discard without noticing: program effects
      apply as ONE atomic batch, and the store's unbatched apply path (documented test-only, loses
      atomicity and changes root hashes) is a PROHIBITED escape hatch. The design does not use the
      upstream path and does not wait on its repair.

## Round 4 review corrections (folded into DESIGN.md v5, tracked here so none is lost)

Support is noted by REVIEW SOURCE, since two samples from one source agreeing is one voice.

- [ ] Custody POSITION becomes its own retirement subject with its own state machine, distinct from the
      program version; every pending object carries an immutable dependency set; exactly one canonical
      native lifecycle owner per object; retirement markers enqueue but hold no disposition authority;
      every disposition is an atomic compare-and-set; precedence defined for overlapping environment
      and program retirement (D15). One source, both samples, blocker. A second source had cleared this
      ground, and the disagreement is recorded in DESIGN.md.
- [ ] Custody evidence enters a NON-EXPIRING native obligation ledger at deposit acceptance, not at
      retirement; expiry and cleanup paths consult obligation state before excluding or deleting;
      compact commitments must prove owner, amount, asset, scope, and spent status without leased
      state; same-height expiry/cleanup/retirement resolves in a fixed protocol order (D6/D15). One
      source, both samples, blocker.
- [x] SUPERSEDED 2026-08-08 by the perpetual-obligation decision above. v5 folded this second source's
      finding as a FINITE absolute maximum lifetime with a NAMED terminal disposition for abandoned
      obligations (D6), and left the choice of disposition open between depleting the attached balance,
      transferring to a governance recovery pool, and burning. All three are now moot, because the
      horizon that would trigger them is removed. The finding itself was real, and it is answered at the
      payment layer instead: a perpetual record with its one-time charge amortized over 50 eras and the
      remainder refunded on early discharge. Recorded rather than deleted, because a later reviewer
      seeing only the current text should be able to tell that this reviewer's objection was addressed
      rather than dropped.
- [ ] Every deployed program version commits in its manifest to finite allowlists of exact program
      versions, host operations, and content-addressed verifier hashes; dynamic dispatch selects only
      within them and traps otherwise; closure size and computation work fit explicit limits; the signed
      transaction binds the closure commitment plus the state preconditions affecting dispatch; a stale
      exact environment is rejected, never upgraded through the compatibility relation (D13). Two
      sources, and a third cleared it conditionally on what amounts to the same fix.
- [ ] The migration SOURCE stays fully materialized and authoritative throughout migration; a journal
      is a replay feed for the destination only and never authoritative; an overlay variant must
      compact through a metered guaranteed process before journal cleanup, with the epoch not called
      clean until compaction completes (D7). One source, one sample only, the least corroborated of the
      six folded items.
- [ ] Migration granularity becomes the dependency COMPONENT with one authenticated cutover commitment
      for stateful members; composition admission refuses a graph with neither a cohort-migration path
      nor a complete native terminal path; per-dependency declaration of migratable, replaceable, or
      environment-pinning; the migration component surfaced in the signed capability preview and wallet
      tooling before any deposit; native terminal path stated as the safety floor (D17, new). All three
      sources, the best-corroborated finding of the round.

- [ ] Terminal work is RESERVED at obligation creation, not merely prepaid: requests, deferred tasks,
      hook bindings, exit records, and custody obligations each charge and reserve worst-case native
      terminal work in future-capacity buckets; a migration reserves worst-case abort cleanup at
      admission beginning at its TTL height and releases it on successful cutover; retirement
      disposition enters the ledger as immediately-matured work sized from O(1) authenticated
      per-subject per-class counters so the marker stays constant-cost; concentration limits apply
      across principal, family, and retirement cohort; admission refuses any object whose eventual
      disposition cannot be reserved; native finalization awaiting quorum work carries a terminal
      timeout; the capacity invariant continues to hold and the cost to ordinary capacity is stated
      rather than hidden (D3b). Two sources, the second-best-corroborated finding of the round. The
      structural rule is folded, the numbers are Phase 0 measurements.
- [ ] Expose the O(1) per-subject per-class counters to light clients as a membership and
      non-membership proof that every disposition of a subject under a given retirement generation has
      been processed, giving the terminal path an explicit finality signal (D3b/D15). Reviewer idea
      retained, cheap once the counters exist.

### The round-4 finding deliberately NOT folded into v5 (open, pending Phase 0)

It is real, it was reproduced by both samples from one source, and it is not closed. It is not folded
because it turns on a quantity nobody has measured.

- [ ] The cleanup service floor can EXCEED its own allocation. With C the maximum cleanup allocation in
      a dimension, admission can fill a future bucket to C, and any overdue work makes the required
      floor exceed C. Hysteresis does not resolve an addition. Candidate fixes: partition C into
      due-work and overdue reserves, or define the feedback term to claim only capacity left after
      maturing reservations. Also needs a bootstrap rule for backlog existing at activation (D3b/D6).
      One source, both samples. The choice is arithmetic over measured numbers. Folding the reservation
      rule above makes this MORE pressing, since more classes of work now draw on the same ledger, so
      the two must be resolved together.

Areas round 4 explicitly CERTIFIED sound (do not reopen without new evidence): the adapter cut as
semantically cleaner than the v3 adapter, exact environment binding under equality-at-inclusion, the
pre-materialized finalization record, the absolute one-fifth allowance, and the logical expiry view with
its expiry-aware index.

## Round 3 review corrections (folded into DESIGN.md v4, tracked here so none is lost)

- [ ] Adapter CUT (owner decision): cross-environment program-to-program calls removed; adapter only
      at the program-to-native-host boundary; async-message boundary documented as the sanctioned
      future extension, not built (D13).
- [ ] Exact semantic environment binding replacing the compatibility floor; credit prices stay
      outside the identifier per D2; compatibility is a machine-checked transitive-closure relation,
      not a signer-selected property list (D13).
- [ ] Retirement subject named precisely: separate monotonic state machines for environment vs
      program version; a user terminal exit retires only its own custody program (D14/D15).
- [ ] Retirement disposition table for every pending object class (cancelable request, irrevocable
      request, deferred task, migration, hook binding, lease exit record), no path calling retired
      code; constant-cost retirement transition (generation marker + incremental queues) that can be
      neither blocked nor over-budget (D15).
- [ ] Irrevocable requests settled from a pre-materialized immutable native finalization record via a
      separately versioned finalization kernel; loss case stated openly (D15/D16).
- [ ] Leased state barred as SOLE custody/conservation/replay/terminal-exit evidence; permanent or
      retirement-pinned exit evidence committed at deposit; retirement pins exit records before
      logical expiry (D6/D15).
- [ ] One-fifth cap measured against the TOTAL dimension; capacity invariant sum(max reserved) +
      one-fifth <= dimension; cleanup is a service GUARANTEE (floor = matured reserved work + bounded
      overdue feedback) not just a ceiling; adaptive fraction has hysteresis / bounded rate of change
      to prevent limit cycles; derive the promised outage bound (D3b/D6).
- [ ] Two-phase migration TTL abort: fixed-cost logical abort (no program, no live grant) restoring
      the source route and freezing a cleanup manifest, then prepaid batched physical cleanup to
      ABORTED_CLEAN; write restrictions lift at logical abort (D7).
- [ ] Environment-bound content-addressed receipts for non-custody async artefacts; invariant lifted
      to "no usable external artefact while the L2 obligation is cleared" (D16).
- [ ] Retirement added as its own governance-matrix row with scope, authorizer, delay, depleted-pause
      handling, value at risk, and atomic freeze-before-terminalize (D14).

Areas round 3 explicitly CERTIFIED sound (do not reopen without new evidence): D16 escrow ordering,
D11 immutable content-addressed verifiers, D6 logical views + expiry-aware index, D3b allocation-
ledger model, D7 incremental authenticated cutover.

## Round 2 review corrections (folded into DESIGN.md v3, tracked here so none is lost)

- [ ] Bind execution environment to the signature as a COMPATIBILITY FLOOR, not exact-match; env
      activations only at epoch boundaries; persist resolved env id into async requests, deferred
      tasks, and migration epochs (D13). Blocker-class (3 reviewers).
- [ ] Replace the hard global cleanup-debt ceiling with per-principal/per-family scoping, future-
      expiry-bucket reservation at creation, and an adaptive progressive surcharge; state and bound
      the longest possible global creation outage (D6). Blocker-class (3 reviewers).
- [ ] One allocation ledger per resource dimension; invariant sum(reserved fractions)+one-fifth ≤ 1;
      guaranteed positive minimum for ordinary execution; one-fifth measured against the non-reserved
      portion; deterministic reclaim of idle lanes (D3b).
- [ ] Permanent defunct/retirement state distinct from bounded pause; executing any terminal exit
      makes the runtime permanently defunct; resume-from-pause denies if a terminal exit was invoked;
      retirement allows only native terminal exits + irrevocable-request drainage (D14/D15). Blocker.
- [ ] Escrow (never burn) through every cancelable async-request state; finalized transition makes
      the request irrevocable and consumes escrow before any quorum share; only pre-irrevocable
      requests cancel (D16). Blocker-class; recurring Platform-bridge rule.
- [ ] Intrinsic custody authority bound to permitted origins and full authenticated caller paths, not
      just the executing frame; entry via hook/migration/recovery/other program cannot reactivate
      ambient authority (D10).
- [ ] Family custody = immutable meta-policy (who may propose a successor, max transferable authority)
      + per-transition exact-successor grants; never general family-principal authority (D10).
- [ ] Migration cutover incremental and authenticated (batches advance a committed verification state;
      cutover is a fixed-cost final check), or a consensus cap on migratable state; migration-epoch
      TTL for a deterministic aborted-and-cleaned fallback (D7).
- [ ] Lease deletion-proof claim and horizon stated explicitly (current-absence-only vs durable
      deletion evidence with a defined permanent witness policy) (D6).
- [ ] Expiry-aware authenticated index so range/index reads are bounded by live entries plus fixed
      metadata, with provable forward progress across expired entries and cross-transaction
      continuation (D6).
- [ ] Fail-closed permissionless activation under an objective predicate even at the sunset height;
      missing the predicate leaves the feature inactive, not governance-discretionary (D14).
- [ ] Round-2 novel ideas: adaptive cleanup fraction (in D6), ephemeral state class, pre-flight
      compatibility simulation on the capability preview, authority-graph family-custody edge.

Round-2 ADDENDUM (fourth sample, three new majors + one sharpening):
- [ ] Cross-environment call rule: single signed environment per transaction, incompatible calls
      rejected, cross-environment interaction only via a versioned canonical adapter; one reservation
      vector upper-bounds every frame (D13).
- [ ] Migration composed with lease expiry: pin source bytes/metadata until cutover/abort, charge to
      migration, expiry and renewal as ordered journal events, one bound logical evaluation height
      (D6/D7).
- [ ] Immutable content-addressed verifier entries; call + capability preview bind exact verifier
      and parameter hashes; governance may only ADD entries, never repoint a deployed program's
      verifier without a new version or owner-opt-in policy transition (D11).
- [ ] Split the cleanup payment into a non-refundable prepaid cleanup charge (pays the lane's real
      work) and a refundable congestion bond (backpressure); define price-change effect on
      outstanding deposits (D6).

## Round 1 review corrections (folded into DESIGN.md v2, tracked here so none is lost)

- [ ] Version binding client-side: client resolves alias, signs exact version, consensus never
      mutates a signed transaction at admission (D7). Blocker fixed in prose, must hold in SPEC.
- [ ] Version-scoped principals; family custody behind an immutable native version-policy; alias
      confers no authority; capabilities carry audience + call path + delegation flag, non-delegable
      by default (D10). Blocker fixed in prose, must hold in SPEC.
- [ ] Execution-environment identifier committing to interpreter, ABI, encodings, native semantics,
      crypto, metering, and traversal; program version + tx resolves to exactly one (D13).
- [ ] Metering covers the FULL transaction lifecycle (decode, sig check, code load/hash, validation,
  arg conversion, schema validation, index derivation, journaling, rollback, invariant checks, commit
  prep); prechargeability is an admission requirement; cache hits never reduce charges; every
  dimension a proven upper bound on real work on min hardware (D3). PHASE 0: the platform's fee meter
  already spans the lifecycle for CHARGING (cold load, rollback via PaidConsensusError, commit-prep
  hashing all metered; see PHASE0_FINDINGS.md). The gap this item keeps open is HALTING at a resource
  bound, which the fee meter does not provide because Tenderdash block gas is unlimited, so it remains
  a VM-layer design obligation.
- [ ] Reservation charge prices the whole declared vector and its exclusion, not a flat fee (D3).
- [ ] One-fifth cap stated as a correctness inequality: every indivisible operation must fit below
      the per-tx fraction in every dimension, or get a reserved lane (D3a).
- [ ] Attestation domains are deterministic predicates over committed state, adapter derives the
      payload; non-predicate domains are opaque acknowledgments only (D4).
- [ ] Async request state machine + atomic credit escrow before signing, unique request id,
      idempotent completion, deterministic pause boundary (D5, D16).
- [ ] Lease rework: metered reserved cleanup lane, cleanup deposit funded at creation, future-bucket
      reservation plus adaptive cleanup plus the D3b terminal-work admission (NOT a hard global
      backlog-debt ceiling, which round 2 REMOVED because principal-splitting turns it into a global
      creation denial; corrected 2026-08-09 from a round-8 wide-scope repo-scan catch), height-filtered
      logical views independent of physical deletion, bounded deletion-evidence accumulator (not
      permanent per-key receipts), rent-sponsorship separated from semantic validity and authorization
      (D6).
      PHASE 0: the deletion-evidence accumulator is native (CountTree/SumTree, provable), and the
      height-filtered view is a precedented app pattern (by-height index plus range query), not a native
      primitive, so the read path must consult the index. See PHASE0_FINDINGS.md.
- [ ] Migration lifecycle: consensus-visible epoch + source snapshot, concurrent-write rule, calls
      name expected epoch, atomic cutover verifying destination invariant, abort/retry/timeout (D7).
- [ ] Data-contract hooks: distinct hook principal + capability envelope, read-only validation
      snapshot, no mutation of the invoking contract, native frames in the unified call graph (D9).
- [ ] Terminal-state exit policy declared before any custody-bearing program takes deposits; runtime
      terminal-state drills a release gate (D15).
- [ ] Governance power-by-phase matrix with sunset height, bounded non-resettable pause, verifier
      registration gated by delay + review + owner opt-in (D14).
- [ ] Determinism honesty: replace "eliminates by construction" with an executable consensus
      semantics + validated core; second interpreter is a diagnostic, not an oracle (D1).
- [ ] Split the native shielded-note service into its own architecture record; keep only generic
      bounded proof verification in the base VM (D11).
- [ ] Randomness liveness stated separately from signature uniqueness; reliable share/result
      dissemination; retry/timeout cannot become a fresh draw (D5).
- [ ] State preconditions scoped to specific keys, not the state root (novel-ideas correction).

## Phase 0, ground-truth the assumptions (blocking everything downstream)

STARTED 2026-08-08. Findings, with a file and line for every claim, are in `docs/PHASE0_FINDINGS.md`
against a frozen candidate (Platform v4.0.0 at 9f9092cc, GroveDB v5.0.0 at 9b98a356, Tenderdash v1.6.0
at 4cba43f5, the Platform revision verified equal to the running Drive image's own label). That document
is authoritative for status; the boxes below carry the short version.

Two results outrank the rest. The platform is MORE capable than the five designs assumed, and in two
places the design record is now factually wrong rather than merely unverified: batch state transitions
are not atomic today, and a native shielded subsystem already exists. Both need an owner decision and
neither has been folded.

- [x] GroveDB proof shapes CONFIRMED and richer than assumed. Membership, absence, subset, chained path
      queries, replication chunk proofs, and aggregate count/sum proofs all exist; ten range item types;
      bidirectional; limit and offset are proof-aware. Completeness is an explicit design guarantee
      enforced by an in-range flag plus lower-bound proving, so a prover cannot substitute a hash node
      for in-range data. Merkle AVL, BLAKE3. One documentation gap: the authoritative proof ADR carries
      an unresolved TODO in the not-requested-node branch of proof generation.
- [~] Cost per write: the MODEL and PRICE LIST are now exact, the operation vectors are not. The cost
      unit is already a five-field vector (seeks, added/replaced/removed bytes, loaded bytes, node
      hashes, Sinsemilla hashes), which validates D3's multidimensional choice. Prices: 27,000 credits
      per durable byte, 400 per byte written, 20 per byte loaded, 2,000 per seek, 400 per node hash,
      40,000 per Sinsemilla hash, 100,000,000 for network threshold signing. Formula cross-checked
      against independently committed test values (a 6,075,000 storage fee is exactly 225 bytes).
      REMAINING: the OperationCost vector for VM-shaped state access, which needs a compiled benchmark.
- [~] Tenderdash execution slice: the ENVELOPE is answered, the interior split is not. Propose timeout
      is 3,000 ms at round zero plus 500 ms per round, vote 1,000 ms plus 500 ms. How that 3 seconds
      divides between assembly, gossip, state-root computation, and execution needs a running network.
      Related and settled: Tenderdash's default block MaxGas is -1, so consensus does no metering at
      all and the forced-core placement of accounting in the state transition function is the only
      option rather than a preference.
- [x] Data-contract validation IS callable as a module, and the call sites are now pinned (D9). Schema
      validation (`validate_document`) runs in the ADVANCED STRUCTURE phase per document transition;
      the data-trigger executor runs in the STATE phase per transition via `validate_with_data_triggers`
      from `.../batch/state/v0/mod.rs`, gated by the `use_document_triggers` execution config flag. Two
      distinct phases with distinct callable units, which is D9's hook-phase model. Boundary: both are
      invoked BY THE PIPELINE at fixed phases, and nothing invokes them mid-execution for a running
      program, so a VM host call into validation is a new caller on existing units, not new machinery.
- [~] Credits accounting for multidimensional reservations: LARGELY ANSWERED. The vector exists, and
      GroveDB ships both a worst-case and an average-case estimator, so the mechanism D3b's
      reserve-at-creation rule needs is already present at the storage layer. Partial refunds and the
      non-refundable reservation charge are still unread.
- [~] DIP-2/DIP-7 program-originated withdrawals: the QUOTA is answered and it is tight. Four
      withdrawal transactions per block, one expired-withdrawal retry signing per block, 500 Dash
      maximum and 190,000 credits minimum per withdrawal. Whether program-originated typed requests can
      be accepted at all is still open. Also settled: max contract group size is 256, which bounds
      D10's family and group custody.
- [!] Real masternode hardware distribution: NOT OBTAINABLE FROM SOURCE. Needs a network survey or
      operator data. Every budget still depends on it, so it stays blocking.
- [x] ANSWERED, and better than assumed: a contract CANNOT name a hook today (no hook, trigger, or
      callback field exists in the contract schema), BUT a native DATA TRIGGER mechanism already exists
      keyed exactly as a hook would be, by data contract id plus document type plus transition action
      (Create/Replace/Delete). It has a bindings list, a binding type, a context, an executor, and a
      reserved consensus error range at 40500-40699. The list is hardcoded in Rust, so triggers are
      native-only. D9's adoption path is therefore GENERALISING an existing mechanism from a hardcoded
      list to a contract-declared one, not inventing one (D9).
- [x] ANSWERED, yes, and that is already how documents work. Index DEFINITIONS come from the
  contract's document type as an ordered map; index VALUES are read from the document via
  `get_raw_for_document_type`, so a caller supplies a document and Drive derives every entry. No
  caller-supplied index entry is trusted. Layout: a unique index stores the reference under key [0], a
  non-unique index puts a subtree at [0] keyed by the document's primary key.

Added after round 4, each one a quantity a v5 rule is denominated in:

- [ ] Measure whether the bounded dependency closure over declared manifest allowlists can be computed
      inside a transaction-admission budget, and prove the wallet and consensus reach the same result
      (D13). This decides whether the closure limits are practical or only stated.
- [!] Real dependency-graph width (D17): THE ITEM IS MIS-SPECIFIED AND CANNOT BE MEASURED. It asks for
      component-size distributions in realistic application dependency graphs, but no programs exist
      because the VM does not exist, so the measurement cannot precede the thing it describes. Either a
      proxy stands in (the reference graph among existing data contracts, or component sizes from
      comparable ecosystems) or D17 is decided on reasoning and revisited once real programs exist.
      Deciding it on reasoning is defensible; waiting on this measurement is not.
- [ ] Measure worst-case terminal disposition work per object class, the horizon over which future
      reservation buckets may spread, and the resulting reduction in capacity available to ordinary
      execution. The D3b reserve-at-creation rule is folded and these are the numbers it is denominated
      in, so this is what makes it implementable rather than only stated.
- [ ] (v10 terminal-work meter, the single coupled number) Under the v10 D3b mechanism the bound is a
      rate-matching invariant metered in TERMINAL-WORK UNITS (not bytes): the per-block deadline-free
      terminal-work flow stays at or below the drain rate R. Measure whether that flow ceiling can be
      sized at or below R without starving legitimate creation, whether the meter's coverage from every
      D15 disposition back to the creating or mutating transition is total, and whether the known_due
      share gives acceptable irrevocability THROUGHPUT at realistic request rates (the gate itself is
      certified safe, failing atomically to cancelable when full), or the disclosed
      create-time-reservation variant is warranted. This subsumes v7's and v8's open numbers. A keeper
      incentive funded by the D6 prepaid cleanup charge is a possible optimization, not built.
- [ ] Determine, at realistic expiry-bucket densities, whether partitioning the cleanup maximum into
      due-work and overdue reserves leaves a workable due-work budget, or whether the
      leftover-capacity formulation is the only viable one (the one unfolded item). Coupled to the
      measurement above, since both allocate from the same ledger.
- [ ] Measure the byte size of a minimal custody obligation record, which at the known 27,000 credits
  per durable byte sets the AMOUNT charged at deposit (D6). The policy shape is decided and no longer
  waits on this: the obligation is perpetual with an amortized charge, not horizon-bounded.
- [ ] (v9) Measure the VM's realistic peak per-block obligation-creation velocity, which sets the
      byte-weighted creation budget, and derive the progressive surcharge curve keyed to the stock of
      post-schedule unfunded state (D6, v9). In v9 this budget is a STORAGE control (permanent bytes)
      for the post-era-50 unfunded tail, SEPARATE from the D3b terminal-work meter, which is what
      enforces the rate-matching invariant. Both must be sized so legitimate creation is not starved.
- [ ] Build the retirement disposition model checker the reviewers specified: two users, one custody
      program, one environment, one irrevocable request, enumerating both retirement orders against
      every queue interleaving, asserting exactly one terminal owner and exactly one terminal state per
      object and no cross-user execution-rights change absent an explicit policy (D15).

## Phase 1, the determinism slice (kills the design if it fails)

- [ ] Write the executable VM specification: accepted instruction subset, trap semantics with fixed
      error codes, canonical encodings that reject alternate representations, and host interface
      versioning with conformance vectors.
- [ ] Build the consensus interpreter (no just-in-time compilation, no native code generation, per
      D1).
- [ ] Build a second, independently written reference interpreter used only as a differential
      testing and release gate.
- [ ] Cross-architecture differential fuzzing: identical modules and inputs to nodes on different
      processor architectures, asserting byte-identical receipts, resource counters, and candidate
      state roots.
- [ ] Adversarial metering search: find the input that maximizes real work per unit charged, for
      every opcode and host call.
- [ ] KILL CRITERION, stated explicitly: if conservative, architecture-independent pricing leaves
      throughput too low to be useful inside the real block slice, stop. Do not proceed to tooling.

## Phase 2, state, proofs, and leases

- [ ] Program-namespaced typed tables with declared key and value schemas, size caps, canonical
      encodings, declared secondary indexes, and index collation.
- [ ] Prove that light-client proofs still verify for program-written keys and index entries.
- [ ] State leases: prepayment covering key, value, indexes, and tree amplification; renewal by
      anyone; bounded maintenance-queue expiry; authenticated tombstones preserving value hash,
      creation height, expiry height, and deleting transition (D6).
- [ ] Move proof construction out of consensus execution; add credit-funded query vouchers and
      response-size limits (D8).
- [ ] Sustained maximum-growth and expiry-under-load testing, plus snapshot, recovery, and
      disk-pressure testing.

## Phase 3, native capability integration (one at a time, each with its own benchmark)

- [ ] Read-only identity, group, token, and document access first.
- [ ] Atomic writes through the document module, with existing schema and authorization checks still
      running (D9).
- [ ] Program principals distinct from caller identity; call-scoped token allowances bound to
      amount, method, code hash, and expiry; re-check of native token invariants after program
      logic (D10).
- [ ] Typed quorum adapters with domain separation. NO general-purpose signing oracle (D4).
      Asynchronous resolution by receipt (D5).
- [ ] Randomness beacon requiring commitment before the epoch, derived from a threshold signature
      over a previously committed root, with no ability to choose among valid signatures.
- [ ] Bridge integration with conservation checks, unique withdrawal identifiers, replay protection,
      destination validation, and per-block and per-epoch quotas.

## Phase 4, cross-program composition and the fee market

- [ ] Cross-program calls: synchronous by exact identifier and version, depth-capped (proposals
      ranged 8 to 32, pick after benchmarking), forwarded sub-budgets, capabilities narrowable only.
- [ ] Re-entry denied by default, opt-in per named method in the manifest, depth and resource limits
      still binding (D12). Deferred work only as explicit task objects invoked by a signed
      transaction, with no hidden wakeups or unmetered callbacks.
- [ ] Per-transaction cap of one fifth of any block dimension, so no single caller monopolizes a
      block.
- [ ] Enforce the repricing rule: a schedule change that alters whether a previously valid call
      succeeds is a new runtime version, not a price update.
- [ ] Multidimensional fee market with deterministic price adjustment from recent utilization,
      bounded per-block change, and hard ceilings independent of price (D2).
- [ ] Declared-reservation block validity rule, so worst-case block work is bounded by construction
      (D3).
- [ ] Reserved protocol lanes so application saturation cannot crowd out identity, bridge,
      governance, and recovery operations.
- [ ] Signed, amount-bound sponsorship allowances; state preconditions in calls (expected state and
      version hashes, minimum outputs, height bounds).
- [ ] Adversarial composition testing.

## Phase 5, upgrades

- [ ] Immutable code versions; upgrade policy fixed in the manifest at creation; notice periods;
      user exit windows; migration into a versioned namespace with a metered, funded migration and a
      short final write freeze (D7).
- [ ] Expected-code-hash in every transaction, failing before execution on mismatch.
- [ ] Platform runtime versioning with activation at a future height, validator-readiness threshold,
      old versions preserved, and retirement only by published migration or announced halt.
- [ ] Test failed migrations, frozen migrations, old-version execution, and emergency halts.

## Phase 6, privacy (separate project, largest scope increase)

- [ ] Protocol-versioned transparent proof verifier avoiding trusted setup, with bounded and metered
      verification keys, public inputs, and proof sizes.
- [ ] Native shielded-note module: commitments, nullifiers, tree roots in authenticated state.
- [ ] Independent audit of the verifier; cap financial exposure during rollout.
- [ ] Programs calling the same verifier for custom statements under registered verification keys.

## Phase 7, rollout

- [ ] Governance-gated deployment where approval permits code to exist and be invoked but confers NO
      token, document, bridge, or identity capability.
- [ ] Deployment bonds and code-storage leases for spam resistance.
- [ ] Reproducible builds and deployed-code hash verification.
- [ ] Emergency mechanism able to halt one VM version without touching unrelated native state, and
      recorded in the trust table as a real governance power.
- [ ] Gate-opening conditions met (spec stable, interpreters agree on all conformance vectors,
      cross-architecture fuzzing clean, adversarial metering benchmarks done, proof tests passing,
      gated production across multiple runtime upgrades, capability and bridge audits, lease
      operation under load, node recovery tested).
- [ ] Permissionless deployment changes only who may register a program. Bridge adapters, verifier
      additions, and runtime versions stay separately governed.

## Open questions the round did not settle

- [ ] Interpreter throughput versus compiled execution: quantify the real cost of the D1 decision.
      If interpretation is too slow to be useful, the whole approach needs revisiting, not patching.
- [ ] Whether adding a VM materially increases the value of capturing network governance, which all
      four designs treated as a pre-existing trusted party without scrutiny (shared blind spot).
- [x] Transaction-ordering value extraction. RESOLVED by the fifth design: publish proposer ordering
      and rejected-call evidence, do not promise to eliminate ordering value, and give applications
      batch auctions, commit-reveal, state preconditions, and future-height randomness. Remains a
      design obligation in Phase 4, not an open question.
- [ ] Formal verification of the interpreter, proposed by none of the four despite two naming
      interpreter defects as a top residual risk (shared blind spot).

## Positioning and community-facing explanation (added 2026-08-11)

-  [x] DONE 2026-08-11 (docs/COMMUNITY_BRIEF.md). Write up EVM compatibility as a considered, sequenced
  item so the community understands the
      order of things. The message to land: Platform is NOT EVM-compatible at the base (all five
      clean-room designs rejected the EVM as the foundation, because its account-and-storage model
      would break Platform's uniform provability under GroveDB or force wrappers around every native
      feature), but the chosen WebAssembly-plus-host-functions design can act as a SUBSTRATE that
      hosts an EVM interpreter as an ordinary deployed program later. Cover the reachable route
      (EVM-as-guest interpreter, metered by the same credit unit, secp256k1 already native for
      ecrecover, determinism already satisfied), what does NOT come for free (Ethereum-style MPT
      state proofs, Keccak and gas-schedule cost, JSON-RPC and wallet tooling as a gateway shim), the
      one interaction with the terminal-work meter (a disposition class for EVM-contract storage),
      and the lighter cousin (compile Solidity to Wasm, which is language familiarity, not bytecode
      compatibility). Recommend treating it as its own clean-room design question with a thin
      end-to-end slice first, not a bolt-on. NOTE: this is an authored positioning explanation, not a
      certified finding, and it does not touch DESIGN.md.
-  [x] DONE 2026-08-11 (Part 8, plus Part 7 on the prototype and a refreshed header). Add the same EVM
  substrate explanation to docs/PLAIN_EXPLAINER.md (which should also get a
      general refresh: it currently ends at the review close and the metering-prototype Phase A and
      Phase B work should be reflected). Keep the plain-language register with the analogies the
      explainer already uses.
- [ ] Evaluate CosmWasm as an ADOPT-versus-BUILD option for the execution engine (raised by QE, a Dash
      Platform core developer, 2026-08-11). CosmWasm is more relevant than the EVM for two reasons: the
      consensus lineage aligns (Tenderdash is a Tendermint / CometBFT fork, the Cosmos BFT engine), and
      CosmWasm is the SAME architecture class the clean-room designs converged on (deterministic,
      integer-only, gas-metered, sandboxed Wasm, Rust-first, host-function interface). So the question
      is not CosmWasm versus the design but whether to ADOPT a hardened engine and its ecosystem instead
      of building a bespoke runtime. The gating question, to answer FIRST, is whether CosmWasm's generic
      key-value storage interface can be backed by GroveDB with light-client provability preserved (more
      tractable than the EVM's rigid fixed-slot model, and unlike the EVM case, answerable). Other items
      to check: replacing Cosmos SDK module bindings (bank, staking, IBC) with Dash-native host
      functions (tokens, identities, groups, credit unit); reconciling CosmWasm gas with the credit fee
      unit; the terminal-work/storage-lease meter still applies whichever runtime is chosen; and confirm
      that the Rust core engine (cosmwasm-vm, with the Go wasmvm wrapping it) can be integrated directly
      into the Rust Drive node. Recommend running it as its own adopt-vs-build comparison, gated on the
      GroveDB-backing question, with Cosmos-experienced contributors. Recorded in
      docs/COMMUNITY_BRIEF.md
      as a named option. Authored evaluation direction, not a certified finding; does not touch
      DESIGN.md.
      UPDATE 2026-08-11: the gating storage question now has an interface-level answer,
      docs/COSMWASM_STORAGE_ASSESSMENT.md (FAVORABLE). Every CosmWasm storage primitive maps to a
      GroveDB
      counterpart verified in source (get/set/remove, ordered range via QueryItem, streaming
      RawIterator,
      byte-lexicographic keys, per-contract subtrees, transactions), and GroveDB's native prove_query
      preserves provability. Interface-level only (GroveDB side repository-resolved against 9b98a356,
      CosmWasm side asserted from knowledge), so the next step is the thin end-to-end slice that note
      recommends: a CosmWasm storage backend over a GroveDB subtree running one simple cw-storage-plus
      contract, giving execution-produced evidence. Remaining open items on this track: the Cosmos-SDK
      module bindings, the gas-to-cost adapter, iterator lifecycle, and confirming the backend trait
      shape in the targeted CosmWasm version.
      DONE 2026-08-11: the spike is built and passing (metering-prototype/cosmwasm-spike). The real
      cosmwasm_std::Storage trait (1.5.11) over a GroveDB subtree, driven through real cw-storage-plus
      (1.2.0): get/set/remove, ascending/descending/bounded range, read-your-writes in a transaction,
      deterministic root across runs, and prove_query/verify_query over the committed contract state
      with
      a real decoded value. Descending is served by reversing the ascending GroveDB query in the adapter
      (native descending RangeFull returned empty; a follow-up to understand, not a blocker absent query
      limits).
      DONE 2026-08-11 (host backend): metering-prototype/cosmwasm-host implements the REAL
      cosmwasm_vm::Storage host trait (1.5.11, compiler-verified) over a GroveDB subtree, with a
      cost-to-gas adapter deriving CosmWasm gas from GroveDB's measured OperationCost (set=175,400 gas
      from 151 added + 333 loaded bytes + 15 hash + 8 seeks; get=65,400), and scan/next iterator-id
      management (two concurrent iterators interleaved correctly; unknown id returns
      IteratorDoesNotExist).
      Provability and determinism carry over. This settles the host trait, the gas adapter, and iterator
      lifecycle. Output metering-prototype/results/cosmwasm_host_output.txt; own docker target volume
      cosmwasm_host_target (builds wasmer, several minutes). Still open on the CosmWasm track: running a
      compiled Wasm contract through wasmer end to end, and the Dash-native module bindings
      (bank/staking/
      IBC replaced by host functions to Dash tokens/identities/groups). Neither is a storage question.
      DONE 2026-08-11 (end to end): metering-prototype/cosmwasm-host/src/bin/e2e.rs instantiates and
      queries a REAL compiled contract (hackatom, from cosmwasm-vm test data, Apache-2.0) through the
      real
      cosmwasm-vm (wasmer, singlepass) over a GroveDB-backed host store (owns Arc<GroveDb>, direct
      writes,
      since the VM needs S: Storage + 'static), and proves the contract's stored state with prove_query
      against the live root. Query returned the stored verifier; one proven state entry. Output
      metering-prototype/results/cosmwasm_e2e_output.txt; contract binary at
      metering-prototype/cosmwasm-host/testdata/hackatom.wasm (provenance in testdata/README.md). The
      CosmWasm STORAGE PATH is now shown end to end at every layer. ONLY REMAINING on the CosmWasm
      track,
      and not a storage question: the Dash-native module bindings (bank/staking/IBC to host functions to
      Dash tokens/identities/groups), the substance of the adopt-vs-build decision.
      STARTED 2026-08-11 (module bindings): docs/COSMWASM_MODULE_BINDINGS.md maps the four interaction
      surfaces (storage done; querier reads; api; outbound messages) and the Dash-native mapping (bank
      to
      Dash tokens, wasm to inter-program calls, staking unmapped, IBC out of scope, and custom
      DashMsg/DashQuery for identities/groups/token mint-burn). The READ half is EXECUTION-PRODUCED:
      metering-prototype/cosmwasm-host/src/bin/querier.rs implements the real cosmwasm_vm::Querier trait
      over Dash token state in GroveDB, answering a bank balance query (Dash token as denom; alice=1000,
      bob=500, absent=0) and a custom DashQuery::TokenSupply (1500), each with gas from the real GroveDB
      read cost, deterministic. Output metering-prototype/results/cosmwasm_querier_output.txt.
      WRITE PATH DEMONSTRATED 2026-08-11 (src/bin/write.rs, fourth bin): hackatom release queries its
      own
      balance via the Querier and emits a BankMsg::Send; a STAND-IN router applies the transfer to
      GroveDB
      bank balances (contract 1000->0, beneficiary 0->1000) and the result is proven. So both read and
      write halves run over GroveDB with a real contract. Output cosmwasm_write_output.txt. STILL
      NOT-BUILT (node integration and catalog scope, not storage/soundness/feasibility): the PRODUCTION
      node message router in place of the stand-in, a real BackendApi (Dash identity ids as addresses,
      Dash secp256k1), and the concrete DashMsg/DashQuery catalogs. These are the real content of the
      adopt-vs-build decision.
      SYNTHESIS 2026-08-12 (docs/EXECUTION_ENGINE_ADOPT_VS_BUILD.md): the adopt-vs-build decision is now
      written and RECOMMENDS ADOPTING CosmWasm backed by GroveDB, treating EVM compatibility as a later
      guest-shape layer. The storage and module-binding path is demonstrated at every layer, and the
      metering prototype and spikes were themselves put through a fifteen-round adversarial review that
      closed with the spikes' scope boundary written down (metering-prototype/SCOPE_AND_LIMITATIONS.md).
      Five conditions must be VERIFIED before committing, each mapped to a high-weight evaluation
      dimension and each a place where a wrong call presents as a consensus fork or a lost platform
      property. Determinism goes first because its failure kills adoption. The bindings work above can
      proceed in parallel once determinism clears, since its shape is demonstrated and its remainder is
      node integration against known Dash primitives.

  - [ ] GATE 1, DETERMINISM (evaluation dimension 1, goes first). Confirm CosmWasm's determinism controls
        (the singlepass compiler, gas-metering boundary timing, absence of floating point, and version
        pinning of cosmwasm-vm and wasmer across validators) satisfy the dimension's named divergence
        sources. This is the failure that presents as consensus forks. Because the execution engine is
        architecture-bearing, answer it via the clean-room multi-model design round, not another spike.
  - [ ] GATE 2, WORST-CASE BLOCK BOUND (dimension 3). Confirm VM execution plus GroveDB writes plus proof
        generation stays inside the ~0.5 s under-load block cadence measured in Phase 0, under adversarial
        load and not just typical load. The metering prototype measured the storage and compute dials
        that feed this.
  - [ ] GATE 3, SHIELDED / ZERO-KNOWLEDGE COMPATIBILITY (stated first-class goal). Confirm a CosmWasm
        contract can verify a zero-knowledge proof on-chain within the gas and block bound, whether via a
        precompile or in-Wasm, since gating an action on a proof without revealing its witness is a named
        requirement.
  - [ ] GATE 4, ASYNCHRONOUS NATIVE CAPABILITIES (dimension 5). Confirm the request-now,
        resolve-in-a-later-block pattern maps onto CosmWasm's message and reply model for operations that
        cannot complete synchronously in one block (masternode threshold signatures, bridge operations).
  - [ ] GATE 5, VERSION-PINNING GOVERNANCE. Decide how the VM and toolchain versions are pinned and
        upgraded under consensus governance, since a version skew in a deterministic VM is itself a fork,
        and the spike's build constraints (edition-2024, the pre-bulk-memory Rust pin, singlepass) show
        this is a standing operational duty rather than a one-time setup.

- [x] COMPARE the recommended direction (adopt CosmWasm) against alternative execution engines.
      Requested 2026-08-15 by a Dash community member (relayed via @hilawe), naming MoveVM. Screen run
      2026-08-27, written up in docs/EXECUTION_ENGINE_VM_COMPARISON.md. MoveVM is the only mature engine
      outside the incumbent that clears the screen, and it beats CosmWasm on the first-weighted dimension
      (no floating point in the language at all, and metering in the interpreter rather than in
      compiler-injected gas points, so no compiler in the trusted path). It loses on the second-weighted
      dimension and on binding owner decision 1, because Move's global storage resolves a resource by an
      exact address and type tag with no ordered range or prefix iteration, so ordered secondary-index
      queries with range and non-membership proofs have no expression in the engine's storage interface.
      Recommendation in docs/EXECUTION_ENGINE_ADOPT_VS_BUILD.md stands. All MoveVM claims are ASSERTED
      grade. Follow-on below is optional and only worth doing if the screen is contested.

- [ ] OPTIONAL MoveVM spike, only if the screen above is contested or if ordered secondary-index queries
      over program state turn out not to be a real requirement (the one finding that would overturn the
      verdict). Minimum bar is the first two items of docs/EXECUTION_ENGINE_VM_COMPARISON.md, "What a
      MoveVM spike would have to show": a Move resource resolver backed by a GroveDB subtree with a
      resource written by a real compiled Move module and proven with prove_query against the live root,
      and an ordered-query demonstration or a recorded finding that there is none.

- [ ] EVM-as-guest demonstrated 2026-08-11 (docs/EVM_GUEST_SPIKE.md, cosmwasm-host --bin evm). A minimal
      EVM interpreter written as a real CosmWasm contract (metering-prototype/evm-guest-contract,
      wasm32)
      runs real EVM bytecode inside cosmwasm-vm over the GroveDB-backed storage; the SSTORE lands in
      GroveDB and the slot is proven. This turns the "EVM as a guest" shape from the community brief
      into
      execution-produced evidence: an EVM guest's state can be GroveDB-backed and provable. A production
      EVM guest still needs the full opcode set, gas mapping, Keccak, the secp256k1 precompile (native
      to
      Dash), and the JSON-RPC / proof-translation tooling layer; EVM compatibility remains its own later
      clean-room design question. Build gotcha: contract needs Rust 1.86 (edition 2024 dep, pre
      bulk-memory default) plus --allow-undefined and -bulk-memory,-sign-ext.
