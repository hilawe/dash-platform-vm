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
| D-02 | The EVM is not viable as a BASE layer | ACCEPTED | Storage model is structurally incompatible with the provability requirement, repository-resolved | A change to the provability requirement itself |
| D-03 | Ethereum compatibility is reachable as a GUEST | ACCEPTED | Execution-produced, interpreter as a contract with writes landing in GroveDB and proving | None foreseen |
| D-04 | MoveVM does not clear the storage requirement | **REOPENED 2026-08-30** | The finding rested on ordered secondary-index queries over program state, which the P1 decision no longer requires of the engine. The screen must be re-run, not adjusted | Resolved by re-running the screen under the P1 split |
| D-14 | P1 split. Program-private state is point-provable; completeness-bearing data goes to native indexed collections through host functions | ACCEPTED (owner, 2026-08-30) | Decided on the property rather than the mechanism. Membership at a known key needs no ordering; completeness and absence do | Application shapes showing the native layer cannot express an index a program needs (H1, H4) |
| D-15 | A host operation for writing native indexed collections | PROPOSED | Derived from D-14 and recorded as B7. Nothing satisfies it today; the catalog covers identities, groups, and token mint and burn only | If D-14 reverses |
| D-05 | Adopt rather than build a bespoke VM | PROPOSED | The comparison assumed the burden lay on the build arm, which is premature while the gates are unresolved | Gate results, or the baseline arm proving sufficient |
| D-06 | Durable obligations are permanent and priced, no finite horizon | ACCEPTED | Owner decision 2026-08-08, grounded in the platform's own production model | A change to how Platform funds durable state |
| D-07 | Program effects apply as one atomic batch | ACCEPTED | Verified against the platform's execution path | Upstream change to the batch apply path |
| D-08 | The design freeze at v12 is historical, not a settlement | ACCEPTED | Charter, 2026-08-30 | None. This one exists to prevent a citation error |
| D-09 | Findings are weighted by independent source, not reviewer count | ACCEPTED | Method | None |
| D-10 | Target a supported CosmWasm line, currently 3.0.x as a bridge | PROPOSED | Support schedule verified 2026-08-30. Roughly four months of runway | Requirements register P4, once a runway policy exists |
| D-11 | Float-bearing program code should be rejected at deployment | PROPOSED | Recommended by every determinism source that addressed it | Requirements register P2, owner decision |
| D-12 | Terminal work must be resumable, with indivisible steps bounded by a guaranteed minimum allocation | PROPOSED | Remedy for the schedulability defect. Not yet designed or implemented | Requirements register P3, and the applications that decide what may be split |
| D-13 | Bounded lazy paging for range scans, designed against the target line and implemented once | PROPOSED | Remedy for the unbounded-scan defect. The obvious fix does not work, since the storage interface never receives remaining gas | The target line's interface, if it exposes a remaining-gas hook |

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

RETIRED-CLAIM-QUOTED, covering the lower rows of the table above as well as the upper ones, since the
gate looks within a fixed window rather than at the whole section.

Every superseded entry names its replacement, so a reader meeting the old claim in an older document can
find what replaced it. The export refuses to ship a superseded claim that appears without a correction
marker, which is the mechanical half of the same discipline.
