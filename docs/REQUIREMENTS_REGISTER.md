# Requirements register

Dated 2026-08-30. Separates requirements by AUTHORITY, because the project had been treating a binding
platform property, a policy choice, and a guess about future workloads as though they carried the same
weight. They do not, and a candidate should only be failed against something binding.

Four classes.

- **BINDING** is a property of Dash Platform or of consensus. Not negotiable within this research. A
  candidate that cannot satisfy one is out.
- **POLICY** is the owner's choice. Changeable by decision, and each one should be decided rather than
  inherited.
- **HYPOTHESIS** is a guess about workloads or applications. Must be labeled as such, and a design that
  depends on one is only as sound as the guess.
- **PREFERENCE** is a nice-to-have. Never grounds for failing a candidate.

## Binding

| ID | Requirement | Source |
| --- | --- | --- |
| B1 | NATIVE state keeps membership, non-membership, and ordered secondary-index proofs to light clients, and adding programmability must not weaken that | The platform's distinguishing property. Note this is scoped to native state deliberately. Extending the same guarantee to PROGRAM-written state is P1, a choice, and stating it as binding here would pre-judge that decision |
| B2 | Execution is identical on every validator for the same input | Consensus. Divergence is a fork, not a bug |
| B3 | Existing data contracts, documents, identities, groups, and tokens survive, with migration or wrapping permitted | Owner decision 1, binding |
| B4 | Program effects apply as one atomic batch within a state transition | D16a, verified against the platform's execution path |
| B5 | Work performed is bounded BEFORE it is performed, not charged after | Consensus liveness. Currently violated on the range-scan path |
| B6 | Every queued obligation is schedulable, meaning it can eventually be serviced | Currently violated. An item larger than the per-block rate blocks the queue permanently |
| B7 | Programs can write NATIVE indexed collections through host functions, with the same proofs native data carries | Derived from the P1 decision 2026-08-30. Not yet BUILT, but established FEASIBLE the same day at repository-resolved grade (`docs/B7_FEASIBILITY.md`). Platform's Drive layer already exposes a document write that maintains the contract's indexes, is transaction-scoped, composes into a caller's batch, returns a fee result, and can be costed without applying. The cost of B7 is the AUTHORITY MODEL, not storage mechanics, since the Drive layer takes owner identity as data and leaves enforcement to the layer above |

B5 and B6 are stated as requirements rather than as defects because they are properties any candidate
and any implementation must have. The two open defects are instances of failing them.

## Policy, owner decisions

| ID | Question | State |
| --- | --- | --- |
| P1 | Must a light client be able to verify COMPLETENESS and ABSENCE over program-written state, or only verify records whose keys it already holds? | **DECIDED 2026-08-30 (owner). SPLIT.** Program-private state is point-provable only. Anything requiring completeness or absence proofs is written to NATIVE indexed collections through host functions, so programs supply compute over a data layer that stays native and stays provable. Consequences below. Reworded before deciding, and the earlier wording is kept here because it was the defect: It previously asked whether ordered range queries were required, which put a storage-engine capability forward as a requirement. That is backwards. Ordered iteration is a MECHANISM; the requirement is what a light client must be able to verify. Membership at a known key needs no ordering. Non-membership and completeness do, since absence is proven by exhibiting the adjacent keys that bracket it |
| P2 | Should float-bearing program code be rejected at deployment regardless of what the engine permits? | **OPEN.** Every determinism source that addressed it recommended forbidding over canonicalizing, on the grounds that forbidding is checked once while canonicalization must be re-established on every backend and every upgrade |
| P3 | What terminal work may be split across blocks, and what must remain atomic? | **OPEN.** Decides the remedy for B6. Depends on the applications and the rights model |
| P4 | How much support runway must a candidate line have when integration begins? | **DECIDED 2026-08-30 (owner).** At least TWO Dash upgrade cycles of remaining support, measured when integration begins. At the observed release cadence that is 180 days. Derived from Dash's own ability to move rather than chosen, so that a line can be prepared, activated, and left again before support ends. CONSEQUENCE: 3.0.x, at roughly four months, is a RESEARCH target and not an integration target |
| P5 | Must the gates be repeated for every major engine line? | **DECIDED 2026-08-30 (owner).** Default is re-run every gate. A gate may be carried forward only with a written note naming what was checked to establish the old evidence still holds. Grounded in experience rather than caution, since the gate 1 findings were tied to an exact dependency graph, feature matrix and toolchain. The harness must therefore make repetition cheap |
| P6 | How quickly must Dash upgrade after an upstream release, and may two runtime versions coexist during activation? | **DECIDED 2026-08-30 (owner).** A binary may contain several runtime versions, but the active one is a deterministic function of the ACTIVATED PROTOCOL VERSION, so every node at a given height runs exactly one. Coexistence in the binary, never in execution, since two runtimes live at one height is a fork by construction. OPEN SUB-QUESTION, whether contracts are pinned to the runtime version they were admitted under or re-validated at upgrade. That needs deciding before the first runtime upgrade, not before the first integration |
| P7 | What happens if the evaluated line expires before the evaluation closes? | **DECIDED 2026-08-30 (owner).** Expiry does not invalidate architecture-level feasibility evidence, which turns on the shape of interfaces rather than a patch version. It does make every version-bound gate stale, and the line may not be described as an integration target. ENFORCED by `tools/check_dependency_support.sh`, which compares the recorded support status against what the shipped documents say and fails when they disagree. It is satisfiable by disclosure and never by waiting |
| P8 | Deployment permission model | DECIDED. Governance-gated at launch, permissionless later, with both phases covered |
| P9 | Durable obligations are permanent and priced rather than expiring | DECIDED 2026-08-08 |

## Hypotheses, labeled as guesses

| ID | Hypothesis | Why it matters |
| --- | --- | --- |
| H1 | The application mix that programs will actually serve | Indexes into the measured partition curve. Two open cost numbers depend on it |
| H2 | Fan-out distributions in real use | The schedulability defect was found at fan-out 64. Whether that is common or pathological changes the remedy |
| H3 | Masternode hardware profile | Needs real network data. The worst-case block bound is meaningless without it |
| H4 | That general programmability is wanted at all, rather than a small set of native modules | The baseline arm exists to test this. It is the largest unexamined assumption in the project |

## Preferences

| ID | Preference |
| --- | --- |
| R1 | Rust-first, matching the existing node implementation |
| R2 | An existing contract ecosystem and toolchain |
| R3 | Ethereum compatibility, reachable as a guest and not required of the base layer |

None of these may fail a candidate on its own.

## What the current design assumes about P1

Recorded so the decision is made deliberately rather than inherited. DESIGN.md item 3 promises program
state carries "the same membership and non-membership proofs as native state", and the design treats a
secondary index as an APPLICATION PATTERN built by programs out of ordered keys rather than as a native
primitive. So the current answer to P1 is the strongest one, and ordered iteration is load-bearing for
program state as a consequence of that choice.

That choice is a design decision from the frozen v12, not a binding platform property, which is why it
sits here as policy rather than in the binding table.

## What the P1 decision changes

Three consequences, and the third is a cost rather than a benefit.

The engine screen must be RE-RUN rather than adjusted. Its first filter required an engine's storage
interface to support ordered range iteration, because program state was assumed to need completeness
proofs. That is no longer required of the engine. Any candidate whose storage is an authenticated
key-value store now clears that filter, which is what the comparison had used to remove MoveVM.

A new requirement appears that nothing currently satisfies, recorded as B7. If completeness-bearing data
lives in native collections, programs must be able to write them, and the operation catalog has no such
call today. This is now on the critical path for the chosen direction rather than a later nicety.

The cost is expressiveness. Native collections carry contract-defined schemas and index definitions, so
a program wanting an index shape the native layer cannot express has nowhere to put it. That limit
should be tested against real application shapes (H1, H4) rather than assumed tolerable, and it is the
condition most likely to reopen this decision.
