# B7 feasibility, programs writing native indexed collections

Dated 2026-08-30. B7 is the requirement the P1 split created. Programs must be able to write native
indexed collections through host functions, with the same proofs native data carries, because that is
where completeness-bearing data goes once program-private state is only point-provable.

It was recorded as NOT satisfied and untested, which made it the load-bearing unverified assumption
under the current screen. If B7 were impractical, P1 would have no landing zone, P1's reversal would
restore ordered iteration as an engine requirement, and the engine screen that admitted MoveVM would
have to be re-run again. So it was checked before any further engine work.

Evidence grade REPOSITORY-RESOLVED. Read from Dash Platform at commit
`9f9092cc910809fd5c415b74fe939864d7bfa7ed`, which is the v4.0.0 revision Phase 0 pinned and verified
against the running image's own label. Nothing here was executed.

## Verdict

STORAGE SUBSTRATE REPOSITORY-RESOLVED. HOST OPERATION AND AUTHORITY MODEL UNPROVEN.

CORRECTED 2026-08-30. An earlier version of this line said simply FEASIBLE, which reads as a wider
claim than the evidence carries, since nothing here was executed and the authority model is undesigned.
The narrower statement is the accurate one and it is still a useful result.

The storage mechanism B7 needs already exists at the Drive layer and has every property the
requirement asks for. The real work is not storage mechanics, it is the AUTHORITY MODEL, and that work
is nameable rather than open-ended.

P1 does not reverse. The engine screen stands, and the MoveVM arm remains eligible.

## The mechanism that exists

`Drive::add_document_for_contract` in
`packages/rs-drive/src/drive/document/insert/add_document_for_contract/mod.rs` takes a document, its
contract and its document type, and returns a `FeeResult`. Its operations-level sibling
`add_document_for_contract_operations` returns `Vec<LowLevelDriveOperation>`.

Four properties matter, and the interface has all four.

**Index maintenance comes with it.** The insert module carries
`add_indices_for_top_index_level_for_contract_operations` and its per-level and reference counterparts,
so writing a document maintains the contract's declared indexes rather than only the primary record.
That is what makes the result a native INDEXED collection rather than a blob at a key.

**It is transaction-scoped.** Both entry points take a `TransactionArg`, and the operations variant
accepts `previous_batch_operations: &mut Option<&mut Vec<LowLevelDriveOperation>>`, so a document write
composes into a caller's existing batch rather than forming its own. That is what B4 requires, since
program effects must apply as one atomic batch.

**It is costed.** The higher-level call returns a `FeeResult` and the lower-level one returns priced
operations, so a program-driven write can be charged against the program's budget in the existing
credit unit rather than needing a parallel accounting path.

**It can be costed WITHOUT being performed.** The higher-level call takes `apply: bool`, and the
operations variant takes `estimated_costs_only_with_layer_info`, which produces an estimate rather than
a mutation. This matters beyond B7 and is noted again below.

Critically, none of this requires a signed state transition. The Drive layer is already separable from
the transaction path, which is precisely the separation a host function would need.

## What the Drive layer does NOT do, and this is the finding

`OwnedDocumentInfo` carries `owner_id: Option<[u8; 32]>` as DATA. The Drive layer records who owns the
document. It does not verify that whoever is calling has the right to act as that owner.

Authorization lives one layer up, in `rs-drive-abci`, driven by the signed state transition. Two
concrete examples read from that layer. A delete checks `fetched_document.owner_id() != owner_id` and
raises a document-owner-mismatch error. A create checks `owner_id != data_contract.owner_id()` for
document types restricted to the contract owner.

So a host operation that exposed document writes to programs by calling Drive directly would bypass the
checks that make document writes safe for signed transactions. Programs would hold more authority than
users do. That is not a defect in the Drive interface, which is doing its job, but it is the design
question B7 actually poses, and it was invisible from outside the source.

The authority model therefore has to answer, deliberately: under whose owner identity does a program
write, its own program identity or a user's delegated one, and what constrains a program to document
types it should be able to touch?

## The second bypass risk, data triggers

The batch validation path contains `data_triggers`, native Rust logic bound to document operations on
specific system contracts, present for the naming service, the contact and profile contracts, and
withdrawals.

This cuts two ways and both are worth recording.

It is a PRECEDENT. Platform already runs native code attached to document writes, so the shape B7
proposes, programmatic logic producing native indexed writes, is not foreign to the architecture. It is
an inversion of something the platform already does.

It is also a RISK with a name. A program-driven write that reaches Drive without passing through the
trigger layer would create naming-service or withdrawal documents without the native rules those
triggers enforce. Any B7 host operation must either run the triggers or explicitly restrict programs to
contracts that have none, and the second option should be a stated restriction rather than an
unexamined default.

## A finding useful beyond B7

The estimate-without-applying capability, `apply: bool` and `estimated_costs_only_with_layer_info`, is
the exact shape the range-scan defect needs. That defect (D-13, and the reason gate 2 fails) is that
work is performed and then charged, which is accounting rather than admission control. Platform's own
document path already demonstrates the opposite pattern, deriving a cost estimate before mutating.

This does not fix the CosmWasm storage adapter, whose problem is that its storage trait never receives
the remaining budget. It does show that the platform this integrates with already treats
cost-before-work as normal, which strengthens the case for designing the bounded paging path that way
rather than treating pre-bounding as an unusual demand.

## What this does not establish

It was not executed. That the operations compose cleanly into a program's effect batch, that the fees
land correctly against a program budget, and that index maintenance behaves under a program-driven
write are all EXECUTION-PRODUCED claims and none has been produced.

It does not design the authority model. It establishes that the authority model, rather than storage
feasibility, is what B7 costs.

It reads one revision, v4.0.0. Protocol v14 in the 4.2 development line reportedly adds ranked
aggregate indexes with provable top-K queries, which would extend what native collections can express.
That was not read here and it is ASSERTED.

## Consequences

B7's STORAGE SUBSTRATE moves from unverified to repository-resolved, so the P1 split keeps its landing
zone and nothing above it needs re-running. The host operation and authority model remain unproven, so
B7 as a whole is not established. CORRECTED 2026-08-30, this line previously said B7 moves to feasible,
which contradicted the narrowed verdict at the top of this document.

D-15 becomes a design task with a known shape rather than an open question, and its difficulty is the
authority model plus the trigger boundary.

The MoveVM spike is unblocked, since the assumption its eligibility rested on now holds.
