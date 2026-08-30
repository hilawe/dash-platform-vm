# Research summary and recommendations for a Dash Platform execution layer

Dated 2026-08-10 and updated 2026-08-11, superseding the 2026-07-20 edition. This document records what
the design research produced, what it established, and what it recommends going forward. It is written
to
be shareable, so the independent reviewers are described generically rather than by product name. Since
the July edition, the design record completed a ground-truth measurement phase against the real platform
and eight further adversarial review rounds, and the review loop CLOSED on 2026-08-10, with the final
round returning a clean approval and no findings from every independent review source. After the close, a
metering prototype measured the design's open cost numbers against the real storage engine (2026-08-11),
and a running spike confirmed that CosmWasm's storage layer can be backed by GroveDB with provability
preserved. Both are summarized below and detailed in `docs/METERING_RESULTS.md` and
`docs/COSMWASM_STORAGE_ASSESSMENT.md`.

## The question

Dash Platform can store, index, prove, and authorize state, and it can settle value, but it cannot
compute. Every application built on it must push its real logic off-chain and reduce the on-chain
footprint to signed assertions, which forces each non-trivial application to introduce its own trusted
or semi-trusted off-chain actor. The research asked what execution layer would give Platform general
programmability, given its specific stack: Tenderdash BFT consensus with fast deterministic finality,
GroveDB authenticated state with light-client proofs and secondary indexes, declarative data contracts,
native tokens, identities and groups, masternode threshold signatures, and a credit fee unit bridged
from Layer 1. A stated goal was shielded compatibility, meaning programs must be able to interoperate
with a privacy mechanism, including verifying zero-knowledge proofs on-chain, so that privacy-preserving
applications can gate actions on a proof without revealing the underlying witness.

The target is a general execution capability that complements the whole Dash and Platform technology
suite. The dash-dollar stablecoin project is one motivating case, whose current design must delegate its
price-dependent mint and redemption math to an off-chain authorized-signer set, but the capability is
justified across use cases rather than by any single application.

## Method

The work followed a clean-room design process. Requirements were elicited and packaged into
an architecture-free packet that stated Platform's facts and the required properties without naming any
candidate solution, then leak-checked to keep candidate-solution vocabulary out. Five independent
designs were produced from that packet, four from sources independent of the author,
plus the author's own design, which was committed to version control before any external
design was read so the independence was auditable.

The synthesized architecture then went through twelve adversarial review rounds across three independent
review sources, at least one reviewer per round reading the actual repository rather than a prepared
packet. Each round put the current design to reviewers charged with attacking its seams, each round's
verified findings were folded into the next version, and, by standing rule, the loop could close only
when a fresh full pass returned approval with nothing real left to fold. Round 12 was that pass. The
full evidence, every packet, every verbatim review, and a per-round adjudication weighing agreement by
independent source rather than by reviewer count, is retained in the project's review record.

Between rounds 4 and 5 the work ran Phase 0, a ground-truth measurement phase against the real platform
at a frozen candidate revision (Platform v4.0.0, GroveDB v5.0.0, Tenderdash v1.6.0). Every source
finding was read from code or from values the developers committed as test assertions, and a live local
network run produced execution-measured numbers, including per-operation fees (identity creation about
116M
credits, data contract creation about 12B, document creation about 9.5M), the per-block execution slice
under light load (about 158 ms, a floor measured across separately-timed ABCI phases), and the
under-load block cadence (about 0.5 s). Phase 0 also corrected three of the design's premises, in one
case reversing a finding the measurement record had itself overstated, and the corrections are recorded
with the overstatements left visible.

## What was produced

- `DESIGN.md` (version 12, review-complete): the architecture record, distinguishing choices forced by
  the requirements from deliberate decisions, with the reasoning for each, the full revision lineage,
  and the measurement items still open.
- `docs/PHASE0_FINDINGS.md`: the ground-truth measurement record against the frozen platform revision.
- `docs/PLAIN_EXPLAINER.md`: a plain-language companion for readers who are not blockchain engineers.
- `TODO.md`: the phased plan, now centred on the remaining measurement items.
- `docs/REQUIREMENTS.md`, `docs/EVALUATION_DIMENSIONS.md`: the requirements record and the
  pre-registered scoring rubric.
- The review record (the requirements packet, the five clean-room designs, and all twelve review rounds
  with every reviewer's verbatim output and a written adjudication per round) is retained with the
  project rather than published, since it carries the reviewers' raw output.
- `metering-prototype/` with `docs/METERING_RESULTS.md`, `docs/METERING_PROTOTYPE_SPEC.md`, and
  `docs/PHASE0_VERIFY_ESTIMATES.md`: the prototype that measured the design's open cost numbers against
  the real storage engine, plus the CosmWasm and EVM spikes (contract-facing storage, the host-side
  backend with a cost-to-gas adapter, an end-to-end compiled contract, the module-binding read and write
  paths, and a minimal EVM guest), each proven with the platform's own proofs, and
  `metering-prototype/SCOPE_AND_LIMITATIONS.md` recording the spikes' scope boundary.
- `docs/COMMUNITY_BRIEF.md`, `docs/COSMWASM_STORAGE_ASSESSMENT.md`, `docs/COSMWASM_MODULE_BINDINGS.md`,
  `docs/EVM_GUEST_SPIKE.md`, and `docs/EXECUTION_ENGINE_ADOPT_VS_BUILD.md`: a community-facing brief (also
  a pre-DIP concept note), the CosmWasm storage assessment and module-bindings note, the EVM-as-guest
  spike record, and the adopt-versus-build decision synthesis.

## What the research established

**The architecture is settled, and it is the same one the requirements force.** Across five independent
designs, twelve core choices converged with no coordination, headlined by a deterministic WebAssembly
runtime
embedded in the Layer 2 state transition function; host functions as the program's only interface; all
program state in the existing authenticated store so light-client provability survives; the Ethereum
Virtual Machine rejected on identical reasoning by every design; integer-only execution enforced at
deployment; canonical ordering at the host boundary; metering in the existing credit unit against hard
per-block bounds; existing data contracts, tokens, identities, and groups preserved and callable; phased
governance-gated then permissionless deployment; no new mandatory trusted party; determinism as the
assumption to validate first; and Rust as the first source language. Convergence of this breadth on a
wide design space is strong evidence the architecture is requirements-forced rather than one model's
preference. Phase 0 then found the platform MORE capable than the designs assumed, with richer proof
shapes
than expected, a cost unit that is already a multidimensional vector, and worst-case cost estimators
already shipped in the store.

**The review loop converged and closed.** The trajectory is documented round by round. Early rounds
found contradictions between decisions, middle rounds found incompleteness inside individual decisions,
and the final rounds narrowed to single sentences. The last four rounds in particular drove one hard
problem to ground (below), with each round's surface smaller than the last, until round 12 returned
approval with no findings from every source. Review-complete means exactly that and no more, namely that
three
independent review sources, one reading the repository, found nothing left to fix in the text. The
execution layer itself remains unimplemented, so the design as a whole is not closed in the assurance
sense. What has since gained execution-produced evidence is narrower and specific, namely the cost model
and the CosmWasm storage-backing question, both through the prototype described in the next finding.

**The hard problem was terminal-work accounting, and the answer is a metered lifecycle.** From round 5
onward, nearly every finding landed on one question, what happens when something ends (a program
retires, a custody
position exits, storage is reclaimed), how is that ending-work kept safe, funded, and bounded without
capping how much the platform can hold. The mechanism that survived eight rounds of attack is a
terminal-work meter. Every object that can generate ending-work carries an authenticated worst-case
work vector, class-dependent and lane-attributed, established at creation and re-charged on every
mutation that grows it. Deadline-bearing work reserves dated capacity in advance and gains a hard
completion guarantee. Deadline-free cleanup drains at a fixed reserved rate, with admission holding
creation flow at or below that rate, so backlogs stay bounded without a cap on live state. Custody
splits by ownership type, so a single-owner balance exits by an owner-initiated constant-cost pull that
enqueues its prepaid cleanup, while multi-party and programmatic custody stays under protocol-driven
settlement. Transfers of authority across programs discharge the old position and create a new one
atomically, with the old position's cleanup accounting handed to its queue item rather than released.
Each element of that summary was individually attacked and certified across families.

**The design's open cost numbers are now measured, and one adoption path has running evidence.** After
the review closed, a metering prototype built against the real storage engine at the frozen revision
turned the design's open dials into measured quantities. It calibrated the storage cost unit against the
store's shipped estimators, measured each disposition class's terminal-work vector (finding that
reclamation returns exactly the bytes a record deposited, and that autonomous distribution scales
linearly with declared fan-out), drove the meter under synthetic load to produce the drain-rate,
flow-ceiling, and partition curves, and bounded the compute dimension in deterministic fuel at about 16
fuel per integer step. Separately, a set of spikes demonstrated the CosmWasm
adoption path with running evidence at every layer, namely the contract-facing storage interface over a
GroveDB subtree with ordered range scans inside a transaction, the host-side cosmwasm-vm backend with a
cost-to-gas adapter derived from GroveDB's measured cost and concurrent iterator management, a real
compiled contract run end to end through the VM, the module-binding read and write paths against
Dash-native token state, and a minimal EVM interpreter run as a guest, each with the resulting state
proven by the platform's own proofs. The metering prototype and these spikes were themselves put through
an adversarial review, which closed with the spikes' scope boundary written down
(`metering-prototype/SCOPE_AND_LIMITATIONS.md`). These are prototype-level and store-level results, not a
running platform, and they do not implement a production VM or its node integration. What they establish
is that the cost model is measurable and internally consistent, and that CosmWasm runs over GroveDB with
provability preserved at every layer from a compiled contract down to a light-client proof, which is the
property that ruled out the Ethereum Virtual Machine as a foundation and makes adoption viable.

**Shielded compatibility is in the design at two levels, and the boundary between them is explicit.**
The base execution layer includes a bounded, metered proof-verification host call for
governance-registered proof systems, circuits, and verification keys, with every verifier entry
immutable and content-addressed so a captured governance process cannot repoint a deployed program's
verifier. A full native shielded-asset subsystem remains scoped as its own separate project. Phase 0
strengthened the factual basis for that split, because the platform already ships a native shielded
subsystem
whose retention model keeps nullifiers and note commitments permanent and never pruned, confirming that
shielded retention does not fit the base lease model and belongs to a dedicated effort with its own
threat model.

**A VM would reduce, not remove, the trust surface that motivated it.** The dash-dollar authorized
signer does two separable jobs, computing the price-dependent math and attesting the price. An on-chain
VM removes the trust in the computation, since the mint and redemption math becomes deterministic and
verifiable by every validator. But a VM cannot produce the DASH price, and the Layer 1 peg-out remains a
quorum-signed release, so an oracle attestation and the collective-custodian property remain. A VM
therefore shrinks the authorized signer to a price-attestation role. It does not delete it.

## Recommendations going forward

- **Treat the design record as review-complete and stop iterating it in prose.** The standing rule now
  in force is that any future substantive change requires a fresh adversarial round before the record
  may again be called review-complete. Absent new requirements, further prose work has nothing left to
  grind on.
- **The implementation-backed measurement recommended in July is done.** The metering prototype measured
  the terminal drain rate and throughput, the per-class terminal-work vectors, the flow ceiling, the
  partition tradeoff, and the compute unit, all against the real storage engine and none requiring the
  full VM. `docs/PHASE0_VERIFY_ESTIMATES.md` records which items moved from estimate to measured, and
  `docs/METERING_RESULTS.md` holds the numbers. The two remaining open numbers are not prototype work,
  namely the application-workload mix (a modeling choice that indexes into the measured partition curve)
  and the masternode-hardware survey (which needs real network data).
- **CosmWasm is the leading candidate for the execution engine, backed by GroveDB, rather than building a bespoke runtime.** All CosmWasm evidence was produced against cosmwasm-vm 1.5.11, whose support status is now in question and must be verified before any decision.
  CosmWasm is the same architecture class the clean-room designs converged on, it shares Platform's
  consensus lineage (Tenderdash is a Tendermint fork), and the whole storage and module-binding path is
  now demonstrated end to end with provability preserved. The decision synthesis
  (`docs/EXECUTION_ENGINE_ADOPT_VS_BUILD.md`) sets out the evidence and the five conditions to verify
  before committing, beginning with determinism across validators (the assumption whose failure would
  kill adoption, so it goes first), then the worst-case block bound including proof generation, on-chain
  zero-knowledge verification, asynchronous native capabilities, and version-pinning governance. The
  remaining engineering is node integration (the production message router, the backend API binding, and
  the Dash operation catalogs), not VM design, and the effort should draw on contributors with Cosmos
  experience.
- **Ship dash-dollar on its current actuator design, and treat on-chain math as a later upgrade, not a
  launch dependency.** This recommendation is unchanged from July and the completed review strengthens
  it: the boundary to draw now is the actuator's deterministic math, which maps onto VM host calls when
  a VM exists, collapsing the actuator to the price role with no rewrite.
- **If a VM effort proceeds, ask that its host-function set expose what a stablecoin needs,** namely
  deterministic integer math over an oracle-attested price and token mint and burn gated on the result.
- **Treat the shielded-asset subsystem as its own project, sequenced after the base VM,** for the same
  three reasons as July (largest scope increase, largest financial exposure if a circuit is unsound,
  retention needs that do not fit the lease model), now grounded in the measured retention model of the
  platform's existing shielded subsystem rather than in assumption.
- **One governance decision remains open by design:** which terminal disposition an abandoned custody
  obligation reaches under the governance matrix. The structure around it is fixed; the choice is
  policy, not mechanism, and is recorded as an open owner decision rather than forced.

## Where things stand

The design record, its measurement record, the metering prototype, the CosmWasm and EVM spikes with
their written scope boundary, the adopt-versus-build synthesis, and the complete twelve-round review
evidence are committed in this repository. The review loop is closed and the record is stable, and the
prototype has since measured the open cost numbers and confirmed the CosmWasm adoption path with running
evidence at every layer from a compiled contract to a light-client proof. Nothing here is a commitment
to build. If a build is ever
undertaken, the record's own evidence rules apply, and no design claim graduates from review-complete to
closed until an implementation exists and independent review confirms behaviour against it.
