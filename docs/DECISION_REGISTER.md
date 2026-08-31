# Decision register

Dated 2026-08-30. Tracks the state of every consequential decision, so that a superseded decision
cannot keep operating quietly and an untested one cannot be cited as settled.

States. **PROPOSED** means put forward, not tested. **TESTING** means evidence is being gathered
against declared criteria. **ACCEPTED** means the criteria were met. **REJECTED** means they were not.
**SUPERSEDED** means a later decision replaced it, with the replacement named.

A decision may be cited as binding only when ACCEPTED. Anything else must be cited with its state
attached.

| ID | Decision | State | Evidence | Reversal condition |
| --- | --- | --- | --- | --- |
| D-01 | CosmWasm is the leading candidate execution engine | TESTING | Storage fit execution-produced; two gates fail, three inconclusive | Any gate reaching a settled fail, or another candidate passing more gates |
| D-02 | The EVM is not viable as a BASE layer | **REOPENED then RE-ACCEPTED on different grounds, 2026-08-30** | Its recorded reversal condition, a change to the provability requirement, FIRED when P1 narrowed what program state must prove. Contract address plus slot is point-provable, so the storage objection is gone. The conclusion survives on F4 instead: an EVM base reaches native tokens, identities and groups only through precompiles and its account model competes with Dash identities. That is a weaker and more contestable objection than the one it replaces | A host-integration design that reaches native features without wrapping them |
| D-03 | Ethereum compatibility is reachable as a GUEST | ACCEPTED | Execution-produced, interpreter as a contract with writes landing in GroveDB and proving | None foreseen |
| D-04 | MoveVM does not clear the storage requirement | **REJECTED 2026-08-30** | The screen was re-run and Move CLEARS it. Address plus type-tag resolution is a point lookup, which is what P1 now requires. Move is a live candidate with no evidence produced against it | Evidence, once any is produced |
| D-21 | The discriminating criteria are now the host extension surface and coexistence, not the engine's own storage shape | ACCEPTED | Follows from P1. Point-provability is satisfiable by almost any authenticated key-value store, so F1 eliminates nobody | P1 reversing |
| D-22 | The candidate set clearing the screen is CosmWasm and MoveVM, plus the charter control arms | ACCEPTED (screen, 2026-08-30) | ASSERTED for Move, EXECUTION-PRODUCED for CosmWasm. A ranking of who deserves evidence next, not a measurement | Evidence produced against Move |
| D-23 | Gate 3 does not discriminate between engines | ACCEPTED (screen, 2026-08-30) | Every candidate reaches proof verification through a host primitive; cost belongs to the host and the proof system, not the engine. Its evidence is engine-independent and can be produced once | A decision to bring the shielded-note service into the execution layer |
| D-24 | Proven execution is an architectural control arm, not an engine candidate | ACCEPTED (screen, 2026-08-30) | Listing zero-knowledge VMs in the engine screen was a category error made in two documents. They answer whether validators re-execute at all | A decision to pursue the proving architecture, which would replace the gate set rather than change a gate |
| D-14 | P1 split. Program-private state is point-provable; completeness-bearing data goes to native indexed collections through host functions | ACCEPTED (owner, 2026-08-30) | Decided on the property rather than the mechanism. Membership at a known key needs no ordering; completeness and absence do | Application shapes showing the native layer cannot express an index a program needs (H1, H4) |
| D-15 | A host operation for writing native indexed collections | PROPOSED, and now FEASIBLE | Checked at repository-resolved grade against Platform v4.0.0. The Drive write path has index maintenance, transaction scoping, batch composition, fee results, and estimate-without-apply. Nothing needs inventing at the storage layer | If D-14 reverses |
| D-25 | B7's cost is the authority model, not storage feasibility | ACCEPTED (repository-resolved, 2026-08-30) | The Drive layer takes owner identity as DATA and does not verify the caller may act as that owner. Enforcement lives in drive-abci action validation, driven by the signed state transition. A host operation calling Drive directly would give programs more authority than users have | Evidence that the enforcement layer is reusable as-is from a program context |
| D-26 | A B7 host operation must run the data triggers or explicitly restrict programs to contracts without them | PROPOSED | Native logic is bound to document writes for the naming service, contact and profile contracts, and withdrawals. A program-driven write reaching Drive directly would skip those rules | A design that reuses the trigger layer |
| D-05 | Adopt rather than build a bespoke VM | PROPOSED | The comparison assumed the burden lay on the build arm, which is premature while the gates are unresolved | Gate results, or the baseline arm proving sufficient |
| D-06 | Durable obligations are permanent and priced, no finite horizon | ACCEPTED | Owner decision 2026-08-08, grounded in the platform's own production model | A change to how Platform funds durable state |
| D-07 | Program effects apply as one atomic batch | ACCEPTED | Verified against the platform's execution path | Upstream change to the batch apply path |
| D-08 | The design freeze at v12 is historical, not a settlement | ACCEPTED | Charter, 2026-08-30 | None. This one exists to prevent a citation error |
| D-09 | Findings are weighted by independent source, not reviewer count | ACCEPTED | Method | None |
| D-10 | Target a supported CosmWasm line, currently 3.0.x as a bridge | PROPOSED | Support schedule verified 2026-08-30. Roughly four months of runway | Requirements register P4, once a runway policy exists |
| D-11 | Float-bearing program code should be rejected at deployment | PROPOSED | Recommended by every determinism source that addressed it | Requirements register P2, owner decision |
| D-12 | Terminal work must be resumable, with indivisible steps bounded by a guaranteed minimum allocation | PROPOSED | Remedy for the schedulability defect. Not yet designed or implemented | Requirements register P3, and the applications that decide what may be split |
| D-13 | Bounded lazy paging for range scans, designed against the target line and implemented once | PROPOSED | Remedy for the unbounded-scan defect. The obvious fix does not work, since the storage interface never receives remaining gas | The target line's interface, if it exposes a remaining-gas hook |

| D-16 | Runway policy. A candidate line needs two Dash upgrade cycles of remaining support, about 180 days, when integration begins | ACCEPTED (owner, 2026-08-30) | Derived from Dash's own upgrade cadence rather than chosen | A change in Dash's release cadence |
| D-17 | Gates are re-run per major engine line by default; carry-forward requires written justification | ACCEPTED (owner, 2026-08-30) | Gate 1's findings were tied to an exact dependency graph and feature matrix | None foreseen |
| D-18 | Runtime version is selected by activated protocol version. Several may exist in a binary, exactly one executes at a height | ACCEPTED (owner, 2026-08-30) | Two runtimes at one height is a fork by construction | None foreseen. Sub-question open on contract pinning versus re-validation at upgrade |
| D-19 | An expired line keeps its feasibility evidence, loses its version-bound gate results, and may not be called an integration target | ACCEPTED (owner, 2026-08-30) | Enforced by tools/check_dependency_support.sh rather than by remembering | None foreseen |
| D-20 | 3.0.x is a RESEARCH target, not an integration target | ACCEPTED (owner, 2026-08-30) | Follows from D-16. Roughly four months of runway against a 180-day policy | 3.1.x stabilizing with adequate runway |

## Superseded

RETIRED-CLAIM-QUOTED. The rows below restate claims this project no longer holds, in order to record
what replaced them. They are quoted here deliberately and are not current positions. The export gate
requires this annotation, which is why it appears rather than being left implicit.

| ID | Decision | Superseded by | Why |
| --- | --- | --- | --- |
| S-01 | Adopt CosmWasm as the execution engine | D-01 | The recommendation ran ahead of the evidence. No gate passes |
| S-02 | A finite absolute horizon for abandoned obligations | D-06 | Every horizon that fires requires burning, depleting, or a governance recovery pool, and removing it removes all three |
| S-03 | CosmWasm achieves determinism partly through the absence of floating point | Gate 1 findings | False. Floats are permitted and determinism comes from NaN canonicalization in the compiler |
| S-04 | The fan-out-64 terminalization can be serviced across blocks | D-12 | The queue has no partial-progress mechanism, so an oversized item is never drained |
| S-05 | Unbounded work cannot run for a bounded charge | D-13 | False on the range-scan path |
| S-06 | Program state carries the same membership and non-membership proofs as native state | D-14 | DESIGN.md item 3's promise. Superseded as a CURRENT-FACING commitment by the P1 split. The frozen design still records it, which is what a frozen historical record is for |
| S-07 | An engine must support ordered range iteration over program state to be a candidate | D-14 | Screen filter 1. It promoted a storage-engine capability to a selection criterion, which put the mechanism before the property |
| S-08 | The EVM's storage model is structurally incompatible with the provability requirement | D-02 as re-accepted | True under the old requirement, false under P1. The EVM is now rejected on coexistence instead, and the difference matters because the new objection is weaker |
| S-09 | Solana's bytecode format fails because its account model has no state proofs | The re-run screen | Conflated the engine with the chain. Solana serves no state proofs, but that is the network's design, not a property of the bytecode format |

RETIRED-CLAIM-QUOTED, covering the lower rows of the table above as well as the upper ones, since the
gate looks within a fixed window rather than at the whole section.

Every superseded entry names its replacement, so a reader meeting the old claim in an older document can
find what replaced it. The export refuses to ship a superseded claim that appears without a correction
marker, which is the mechanical half of the same discipline.
