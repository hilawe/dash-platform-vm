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
| B1 | Program-written state keeps membership, non-membership, and secondary-index proofs to light clients | The platform's distinguishing property |
| B2 | Execution is identical on every validator for the same input | Consensus. Divergence is a fork, not a bug |
| B3 | Existing data contracts, documents, identities, groups, and tokens survive, with migration or wrapping permitted | Owner decision 1, binding |
| B4 | Program effects apply as one atomic batch within a state transition | D16a, verified against the platform's execution path |
| B5 | Work performed is bounded BEFORE it is performed, not charged after | Consensus liveness. Currently violated on the range-scan path |
| B6 | Every queued obligation is schedulable, meaning it can eventually be serviced | Currently violated. An item larger than the per-block rate blocks the queue permanently |

B5 and B6 are stated as requirements rather than as defects because they are properties any candidate
and any implementation must have. The two open defects are instances of failing them.

## Policy, owner decisions

| ID | Question | State |
| --- | --- | --- |
| P1 | Are ordered range queries required over PROGRAM-private state, or only over native host state? | **OPEN, and decisive.** If only native state needs them, the Move arm becomes live and the candidate set widens materially. Nothing else in the register changes the field this much |
| P2 | Should float-bearing program code be rejected at deployment regardless of what the engine permits? | **OPEN.** Every determinism source that addressed it recommended forbidding over canonicalizing, on the grounds that forbidding is checked once while canonicalization must be re-established on every backend and every upgrade |
| P3 | What terminal work may be split across blocks, and what must remain atomic? | **OPEN.** Decides the remedy for B6. Depends on the applications and the rights model |
| P4 | How much support runway must a candidate line have when integration begins? | **OPEN.** The current leader's evidence sits on an expired line and the stable target has roughly four months |
| P5 | Must the gates be repeated for every major engine line? | **OPEN** |
| P6 | How quickly must Dash upgrade after an upstream release, and may two runtime versions coexist during activation? | **OPEN** |
| P7 | What happens if the evaluated line expires before the evaluation closes? | **OPEN.** Not hypothetical, it is the current situation |
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

## The one that changes the most

P1 deserves separate emphasis. The engine comparison currently turns on ordered secondary-index queries
over program state, and that single criterion is what removes the Move arm. If it applies only to
native host state, the comparison must be re-run rather than adjusted. It should be decided before more
evidence is gathered against the current framing, because gathering evidence under a criterion that may
not apply is the expensive kind of wasted work.
