# Phase 0 ground-truthing findings

First measurement session, 2026-08-08. Phase 0 exists because every resource budget in `DESIGN.md` is
denominated in numbers nobody had measured, and because five independent designs made assumptions about
what Dash Platform does. This document records what the real code does, with a file and line for every
claim.

Read the headline first. The platform is MORE capable than the designs assumed, not less. One factual
premise in the design record turned out to be wrong (a shielded subsystem exists), and one thing this
document first reported as a contradiction turned out on closer reading to be a confirmation (the
atomicity assumption holds). Both are now decided and both corrections are recorded in place rather than
edited away, since each was committed before it was checked.

## The frozen candidate

Every claim below is against these exact revisions, chosen to match the stack actually installed on the
development machine rather than whatever is current upstream.

| Component | Version | Revision | How it was pinned |
|---|---|---|---|
| Platform (Drive, DPP, Drive-ABCI) | v4.0.0 | `9f9092cc910809fd5c415b74fe939864d7bfa7ed` | The running `dashpay/drive:4` image's own `org.opencontainers.image.revision` label |
| GroveDB | v5.0.0 | `9b98a35644cdea73cc1b21d7c122cb58ae9fafd8` | The tag `packages/rs-drive/Cargo.toml` pins |
| Tenderdash | v1.6.0 | `4cba43f529eee3e8a6f29b4ff61c369bb0894134` | The running `dashpay/tenderdash:1.6.0` image |

The Platform source revision was verified equal to the running image's label rather than assumed from
the version string, so the source read here is the source that produced the binary in use. The rest of
the local stack is Dash Core 23, rs-dapi 4, and dashmate 4.0.0.

Evidence grades in this document. Most findings are read from source or from values the developers
committed as test assertions, which is repository-resolved evidence, not execution-produced. No Rust or
Go toolchain is installed on the host, so nothing was compiled. The one exception is the container-run
section below, which reads a live dashmate network of the frozen-candidate images and so carries
execution-produced evidence for what it covers, namely that the stack runs and its idle cadence. The
distinction matters for anything later recorded as closed, and it is called out per finding.

## What the designs got right

**GroveDB proof shapes are present, and there are more of them than the designs assumed.** The design
record assumed membership, non-membership, index membership, and bounded range proofs with completeness.
All are present in `grovedb/src/operations/proof/`, and the public API also carries subset proofs,
chained path queries, replication chunk proofs, and aggregate proofs over count and sum
(`aggregate_count`, `aggregate_sum`, `aggregate_count_and_sum`). The query system supports ten range
item types. Proofs are bidirectional, and limit and offset are proof-aware, with skipped nodes returned
as key-value digests so a verifier can confirm both the number skipped and that the skipped keys matched
the query.

Completeness is not incidental, it is a stated design guarantee. `adr/merk-proofs.md` names the three
properties a proof carries (complete, meaning the exact set with nothing added or removed, correct, and
fresh), and the verification algorithm enforces completeness through an in-range flag plus lower-bound
proving. A prover cannot substitute a hash node for data that is in range and in state, which is exactly
the property the design needs. The tree is a Merkle AVL structure and the hash is BLAKE3.

**Resource accounting is already multidimensional at the STORAGE layer, which supports D3's direction.**
NARROWED after round 5: this confirms multidimensional STORAGE accounting and supplies estimators, it
does NOT validate D3's full meter. The five fields below are storage-shaped and do not cover interpreter
instructions, memory-duration, or non-storage cryptography, so full-lifecycle metering, the hard
execution abort, and the reservation ledger remain open. The design chose a resource vector over a
single gas number, and reviewers treated that as a deliberate divergence needing defence. The storage
layer already works that way. `costs/src/lib.rs:104` defines `OperationCost` with five fields, one of
which is itself a three-part structure:

- `seek_count`
- `storage_cost`, holding added, replaced, and removed bytes separately
- `storage_loaded_bytes`
- `hash_node_calls`
- `sinsemilla_hash_calls`, for elliptic-curve hashing on commitment-tree anchors

**A worst-case estimator already exists, which matters for D3b.** GroveDB ships both
`estimated_costs/worst_case_costs.rs` and `estimated_costs/average_case_costs.rs`. The
reserve-at-creation rule folded into D3b needs a way to price worst-case terminal work before performing
it, and that mechanism is present at the storage layer rather than something the VM layer would have to
invent.

**Tenderdash delegates all metering to the application, which confirms the forced core.** The
clean-room round agreed that resource accounting belongs in the state transition function.
Tenderdash's default `BlockParams.MaxGas` is `-1` (`types/params.go:128`), meaning consensus imposes
no resource limit at all. Nothing in consensus will help meter execution, so the design's placement
is not a preference, it is the only option.

## The price list, and the per-write cost model

The cost question was posed as "measure the real cost of an authenticated-tree update plus index
maintenance per write." The model and the price list are now known exactly. What remains unmeasured is
narrower than the original question.

Prices, from `packages/rs-platform-version/src/version/fee/`:

| Quantity | Credits | Source |
|---|---|---|
| Durable storage, per byte | 27,000 | `storage/v1.rs` |
| Processing, per byte written | 400 | `storage/v1.rs` |
| Storage load, per byte | 20 | `storage/v1.rs` |
| Non-storage load, per byte | 10 | `storage/v1.rs` |
| Seek | 2,000 | `storage/v1.rs` |
| BLAKE3 node hash | 400 (100 base plus 300 per block) | `hashing/v1.rs` |
| Sinsemilla hash | 40,000, about 100 times BLAKE3 | `hashing/v1.rs` |
| Network threshold signing | 100,000,000, one thousandth of a Dash | `processing/v1.rs` |
| Key structure validation | 50 | `processing/v1.rs` |

A comment in `storage/v1.rs` records that these were originally calibrated against a price of 30 dollars
per Dash, and the threshold-signing comment prices itself at about 2.5 cents. Both are calibration
anchors that will drift, and anything quoted in dollars should be re-derived rather than repeated.

The conversion is in `packages/rs-drive/src/fees/op.rs`. Processing cost (`ephemeral_cost`, line 1104)
sums seeks at 2,000 each, all written bytes (added, replaced, and removed alike) at 400 each, loaded
bytes at 20 each, node hashes at 400 each, and Sinsemilla hashes at 40,000 each. Durable storage cost is
separate and simpler, at added bytes times 27,000 (line 233).

The formula was cross-checked against values the developers committed independently, rather than
taken on reading alone. A funds-transfer test asserts a storage fee of 6,075,000 credits, which is
exactly 225 bytes at 27,000 per byte, and a variant asserts 12,150,000, exactly 450 bytes. Committed
processing-fee assertions for real operations range from 458,920 for a funds transfer to 71,994,700
for a document delete, the latter being roughly seven tenths of one thousandth of a Dash for a
single operation.

The one thing source reading did not give, the live per-operation fee, was MEASURED by the 2026-08-09
load run (see the load-harness results section): identity create ~116M, data contract create ~12B, and
document create ~9.5M credits. The raw five-field `OperationCost` vector remains internal and is not
exposed through the SDK or metrics; the aggregate credit fee is what the run captures.

### How durable storage is FUNDED, which decided the D6 horizon question

The per-byte price is only half the model, and the other half settled an open design question. From
`packages/rs-dpp/src/fee/epoch/mod.rs` and `packages/rs-dpp/src/fee/epoch/distribution.rs`:

- State is treated as PERPETUAL. The governing constant is `PERPETUAL_STORAGE_ERAS`, set to 50, and its
  comment states that an era is one year on mainnet. `DEFAULT_EPOCHS_PER_ERA` is 40.
- The one-time storage fee is distributed to validators across those 50 eras through
  `FEE_DISTRIBUTION_TABLE`, on a front-loaded declining curve. Verified by summing the table rather than
  by reading it: exactly 50 entries, first 0.05000, last 0.00125, total exactly 1.00000. So the entire
  fee is paid out across 50 years and nothing is held back.
- The undistributed remainder is REFUNDED when a record is removed before the schedule completes, via
  `calculate_storage_fee_refund_amount_and_leftovers` and
  `subtract_refunds_from_epoch_credits_collection`.

This makes the perpetual obligation CONSISTENT with how the platform already prices durable state. A
record is permanent, its charge is levied once and amortized over 50 years, and early discharge returns
the unconsumed remainder. NARROWED after round 5, because "answered at the payment layer" overstated it:
the schedule makes perpetual storage consistent with platform policy, it does NOT make it perpetually
FUNDED. Past the fiftieth era the record persists with its fee exhausted, so validators carry it
unfunded
from then on. An attacker abandoning minimum-value positions therefore grows unfunded permanent state,
which the early-refund incentive does nothing to deter. This is the same unfunded tail the platform
already accepts for every durable record it holds, so it is a known inherited policy rather than a new
defect, but the fix owed in v6 is to say so explicitly and to apply the same creation-rate or congestion
controls used for other permanent state, not to claim the 50-era schedule fully internalises the cost.
The owner adopted this model for custody obligations on 2026-08-08 and removed D6's finite horizon,
recorded in `DESIGN.md` under D6 and in `TODO.md`.

## The Tenderdash execution envelope, partially answered

Tenderdash v1.6.0 defaults, from `types/params.go`:

- Propose timeout 3,000 ms at round zero, growing 500 ms per round
- Vote timeout 1,000 ms at round zero, growing 500 ms per round
- Synchrony precision 505 ms, message delay 12 s
- Absolute block size ceiling 100 MB

The outer envelope for a proposer to build and propose is therefore 3 seconds at round zero. How that
splits between block assembly, gossip, state-root computation, and execution is not determinable by
reading, because it depends on machine and network behaviour under load. This item is answered as to its
bound and open as to its interior, and closing it needs a running network.

## System limits, which are real and tighter than the designs assumed

From `packages/rs-platform-version/src/version/system_limits/v1.rs`. Several of these constrain design
decisions directly.

| Limit | Value | Bears on |
|---|---|---|
| Max state transition size | 20,480 bytes | Every per-transaction budget |
| Max field value size | 5,120 bytes | Program and payload sizing |
| Estimated max serialized contract size | 16,384 bytes | D9 contract hooks |
| Withdrawal transactions per block | 4 | D4 quotas |
| Retry signing expired withdrawals per block | 1 | D4, D16 |
| Max withdrawal amount | 500 Dash | D4 |
| Min withdrawal amount | 190,000 credits, 190 duffs | D4 |
| Max contract group size | 256 | D10 family and group custody |
| Max token redemption cycles | 128 | Token interaction |
| Max shielded transition actions | 16 nominal, 6 effective | See below |

The withdrawal quota deserves emphasis. D4 assumed typed, escrowed, program-originated requests could be
accepted "under quotas", and the real quota is four withdrawal transactions per block with one retry.
Any design in which programs originate withdrawals is competing for four slots per block.

## Second reading pass, 2026-08-08, the six no-tooling items

Four of the six are answered or partly answered below. Two were not reached, and they are named as not
reached rather than left to look covered. Same frozen candidate as above.

### Secondary indices are schema-derived, never program-supplied. ANSWERED, and the pattern D9 needs
### already exists

The question was whether the native store can maintain program-declared secondary indices from a schema
without trusting index entries supplied by the program. It can, and that is how it already works for
documents.

- Index DEFINITIONS come from the contract's document type, held as a `BTreeMap<String, Index>` behind
  an `indexes()` accessor, with an `index_structure: IndexLevel` on the document type. A `BTreeMap`
  iterates in deterministic key order, which matters for the determinism item below.
- Index VALUES are read out of the document itself through `get_raw_for_document_type`, so the caller
  supplies a document and Drive derives the index entries. No caller-supplied index entry is trusted.
- The tree layout is documented in the code at lines 142 to 144 of
  `add_reference_for_index_level_for_contract_operations/v0/mod.rs`, under
  `packages/rs-drive/src/drive/document/insert/`. A UNIQUE index stores the document reference
  directly under key `[0]`. A NON-UNIQUE index puts a subtree at key `[0]` whose members are keyed by
  the document's primary key.

That last rule is the answer to two questions at once, because it is also how the store avoids an
ordering ambiguity when two documents share an indexed value.

### No implementation-defined tie-break was found in iteration order. ANSWERED

The clean-room round called determinism the assumption whose failure kills the design, and round 1 asked
specifically whether range and index iteration carries a tie-break not captured by canonical ordering.
Nothing of that kind was found, on four independent grounds:

- Keys are UNIQUE within a subtree, since the structure is a key-value Merkle AVL tree, so no two
  entries can tie on key.
- Equal INDEXED VALUES cannot tie either, because a non-unique index nests its members under `[0]` keyed
  by primary key, which is a 32-byte identifier. Ordering is therefore total: indexed value first, then
  primary key.
- Direction is explicit rather than implied, through a `left_to_right` flag on the query.
- Contract index iteration is over a `BTreeMap`, which is ordered by construction rather than by
  insertion or hashing.

Stated at its real width: this is the absence of a tie-break in the paths read, not a proof of
determinism for the whole store. Multi-path queries and cross-subtree result assembly were not examined,
and a determinism claim for the design should rest on execution evidence rather than on this reading.

### A contract cannot name a hook today, but the mechanism D9 wants exists natively. ANSWERED

This is the most useful result of the pass. The answer to the literal question is no, and the answer to
the question behind it is better than the design assumed.

- No contract-declarable hook exists. A sweep of `packages/rs-dpp/src/data_contract/` found no hook,
  trigger, or callback field, so a data contract cannot name a program today.
- A NATIVE data-trigger mechanism does exist, and it is keyed exactly the way a hook would need to be.
  In `data_trigger_bindings_list_v0`, each binding carries a `data_contract_id`, a `document_type`
  name, and a `transition_action_type` of Create, Replace, or Delete. The registered bindings are for
  system contracts, DPNS domains and withdrawals among them.
- The surrounding machinery is already built: a bindings list, a binding type, a `context.rs`, an
  `executor.rs`, and a consensus error range reserved for data triggers at 40500 to 40699.
- The list is HARDCODED in Rust, so triggers are native-only and not extensible by a contract author.

So D9's legacy adoption path is not a greenfield invention. It is generalising an existing native
mechanism from a hardcoded list to a contract-declared one, and the binding key it would use, meaning
contract plus document type plus action, is already the key the platform uses. The error-code space is
already allocated too. That is a materially stronger starting position than "whether a data contract can
name a program hook" implies.

### Contract validation is callable as a module, at fixed pipeline phases. ANSWERED (upgraded from partial, third pass 2026-08-08)

`validate_document` and `validate_document_properties` are trait methods on the data contract with
versioned implementations under `packages/rs-dpp/src/data_contract/methods/validate_document/`, so
validation is a callable unit rather than only a transaction-entry step, which is what D9 needs. The
third pass pinned the call sites, and the shape is exactly D9's hook-phase model:

- SCHEMA validation runs in the ADVANCED STRUCTURE phase, per document transition, in
  `.../batch/action_validation/document/document_create_transition_action/advanced_structure_v0/mod.rs`
  and its replace-transition sibling.
- The DATA-TRIGGER executor runs in the STATE validation phase, per document transition, invoked as
  `document_transition.validate_with_data_triggers(bindings, context, ...)` from
  `.../batch/state/v0/mod.rs` (around lines 299 to 309). It is gated by an execution config flag
  `use_document_triggers`, which has a default in `config.rs`.

So there are TWO distinct validation phases with distinct callable units, which matches D9's requirement
for hooks that fire at defined phases rather than at one entry point. The one boundary to state plainly:
both units are invoked BY THE PIPELINE at fixed phases, and nothing invokes them mid-execution on behalf
of a running program. A VM host call into contract validation would therefore be a NEW caller built on
existing callable units, not a reuse of an existing program-facing entry point. That is a small addition
rather than new validation machinery.

### The accumulator primitives D3b and D6 need already exist. PARTIALLY ANSWERED

The item asked whether the D6 cleanup lane, accumulator, and height-filtered views are expressible over
the store. The accumulator half is answered and the answer is yes, natively. The element types include
`SumTree`, `CountTree`, `CountSumTree`, and `BigSumTree` alongside `Tree`, `Item`, `Reference`, and
`SumItem`. Those are aggregate trees maintained by the store, and the proof module carries matching
`aggregate_count`, `aggregate_sum`, and `aggregate_count_and_sum` proofs.

This lands directly on the sixth round-4 fold. D3b requires O(1) authenticated per-subject per-class
counters so a constant-cost retirement marker can read a backlog size without enumerating it. A
`CountTree` is that primitive, it is authenticated, and its count is provable. So that rule is
implementable on existing machinery rather than needing a new one.

The HEIGHT-FILTERED VIEW half was examined in the third pass (2026-08-08) and is ANSWERED as an
application pattern rather than a native primitive. There is no built-in "logically absent past
height H" view in the store. The pattern is an explicit secondary index keyed by big-endian block
height, queried with `RangeTo` and `RangeFrom` items, and it is used in production twice: the
shielded anchors-by-height index (`prune_anchors/v0/mod.rs`) and the untied-withdrawal queue, which
dequeues by height. Big-endian height keys give ascending numeric order, so the range query is
deterministic. What this means for D6 is that a logical expiry view is buildable and precedented,
but a read path must consult the by-height index rather than expecting the store to hide expired
entries on its own. That pairs with the earlier D6 never-reach-zero warning, since both are
obligations on the D6 read and cleanup paths rather than services the store provides for free.

### The credit meter spans the transaction lifecycle for CHARGING. ANSWERED (third pass 2026-08-08)

The item asked whether the credit meter covers the full transaction lifecycle including cold-cache code
load, rollback, and commit preparation. It does, for accounting, and the one thing it does NOT do is
worth stating as clearly as the things it does.

The accumulated unit is `FeeResult` (`packages/rs-dpp/src/fee/fee_result/mod.rs`): `storage_fee`,
`processing_fee`, `fee_refunds`, and `removed_bytes_from_system`. Against the three lifecycle stages the
item names:

- COLD-CACHE LOAD is metered. `ephemeral_cost` charges `storage_loaded_bytes` at the load rate
  (`op.rs`, around line 1128), and `fetch_contract_v0` takes an optional epoch and returns a metered
  `CostResult`, so loading a contract from disk is billed. A VM loading program bytecode would use the
  same loaded-bytes dimension.
- ROLLBACK is metered, and this is the sharp part. The execution-result enum
  (`state_transitions_processing_result/mod.rs`) has a `PaidConsensusError` variant documented as "we
  can deduct processing fees calculated until this validation error happened," carrying the actual
  fees charged. So a transition that fails validation and has its state effects rejected is STILL
  charged for the work performed up to the failure, whenever a proved identity exists to bill.
  `UnpaidConsensusError` is the no-charge case (no proved identity, or a revision mismatch) and the
  enum notes it must never reach process-proposal. That is precisely the
  pay-for-work-even-on-rejection property D3 wants.
- COMMIT PREPARATION is metered through the `hash_node_calls` dimension for in-consensus tree and index
  update hashing, and `SuccessfulExecution` carries both estimated and actual fees, so the estimate and
  the settled charge are distinct.

THE BOUNDARY. This is a FEE meter, an economic accounting layer, not a hard execution-abort meter with a
block resource ceiling. Tenderdash sets block `MaxGas` to -1, so consensus imposes no execution limit,
and the platform's own accounting is what prices work. So the lifecycle is covered for CHARGING, but
HALTING execution at a resource bound, which is a correctness gate D3 also needs, is a VM-layer
obligation that the fee system alone does not establish. The design should carry that as an explicit
build obligation rather than assume the fee meter provides it.

## Container run, 2026-08-09, what a live v4.0.0 network gives and what it does not

UPDATE later on 2026-08-09: the frozen devnet described below was RESET AND REBUILT on owner
authorisation, and the crash loop is cleared. All three validators now run at zero restarts and
Platform produces blocks (heights 1, 2, ...) on a fresh Core chain, with metrics served (local_1
host port 49090 returns `abci_last_finalized_height`). The fresh idle empty-block cadence measured
~195s (block 1 at 18:23:07, block 2 at 18:26:22), which confirms the ~180s heartbeat below was a
real property, not a wedge artifact. pshenmic's `dash-platform-sdk` 1.4.0 is installed and can
target the local network via a custom `dapiUrl`. The producing-network precondition for the load
harness is now MET; the remaining crux is funding a Platform identity via a Core asset lock (~16,034
DASH available in the seed wallet). Details and run steps are in `PHASE0_LOAD_HARNESS.md`. The
diagnostic of the wedged state is kept below as the record of what was found.

This section is the first EXECUTION-adjacent evidence in the record, and it is bounded carefully because
at first reading the network was frozen rather than producing load.

What is confirmed by the running containers, which is genuine execution-produced evidence:

- The full stack RUNS on this machine at the exact frozen-candidate versions. A dashmate local group is
  up with three nodes plus a seed, on `dashpay/drive:4`, `dashpay/tenderdash:1.6.0`, and
  `dashpay/dashd:23`. Tenderdash reports version 1.6.0 and application version 12, network
  `dashmate_local_20`. So "can the stack build and run here" is answered yes, which the native-daemon
  restrictions on this machine had left open.
- The Tenderdash consensus timeouts read from source are the ones the running node reports through its
  status and block headers, so the 3-second propose budget is the live value, not just a default in a
  config file.

What the network does NOT give, and why:

- IT IS FROZEN. Latest block height is 4958 with a block time of 2026-08-05, four days before this
  reading, and `catching_up` is false. Block production has stopped rather than idling. So there is no
  live cadence to sample right now.
- Over the last 20 blocks it did produce (heights 4939 to 4958), inter-block spacing was about 180
  seconds (mean 180.85, median 180.52, range 180.00 to 182.30). That is the EMPTY-BLOCK HEARTBEAT, not
  the consensus round budget (3 seconds) and not the execution slice. Configured `epochTime` is 1200
  seconds, which is the epoch length and a separate quantity again.
- EVERY BLOCK ON THE CHAIN IS EMPTY. Sampled across heights 100, 500, 1000, 2000, 3000, and every 200
  up to 4958, transaction count is zero throughout. The devnet was stood up but never carried Platform
  state-transition traffic, so there is no historical load to mine for execution timing.
- drive-abci Prometheus metrics are DISABLED in this config (`abci.metrics.enabled = false`), so the
  block-execution histograms the code emits are not being served. Getting them needs enabling metrics
  and restarting a node.

Consequence for the two execution-dependent numbers. The interior split of the consensus budget
(execution versus consensus, gossip, and state-root computation) and the live OperationCost vector for
real state access both require GENERATED LOAD against a producing network. That is a load harness:
restart block production on the frozen devnet, fund a Platform identity through a Core asset lock, issue
state transitions through an SDK, and read the returned fees while metrics are enabled. It is a distinct
and substantial piece of work, it changes the state of a devnet another session left in place, and it
was not started here without a decision. The per-operation FEE figures already extracted from committed
test assertions (a document delete at 71,994,700 processing credits, a transfer at 458,920) remain the
best numbers in hand until that harness runs, and they are repository-resolved rather than
execution-produced. (SUPERSEDED by the load-harness run below, which produced live numbers.)

## Load harness results, 2026-08-09, EXECUTION-PRODUCED

The harness ran end-to-end against the rebuilt v4.0.0 local network: it funded a wallet from Core,
registered a Platform identity, published a data contract, and created five documents, measuring
each operation's fee as the identity credit-balance delta (SDK-agnostic). This is the first
execution-produced fee and timing evidence in the record. The SDK used was the official `dash`
7.0.0, which matches Platform 4.0 by the npm dist-tag mapping (`4.0-rc -> 7.0.0`); pshenmic's SDK
was evaluated first and set aside for this task because it is Platform-only and rejects the local
network, and the Core-side asset lock is outside its scope (both noted in the operator rule).

**Per-operation fees, measured as credit-balance deltas (1000 credits per duff).**

| Operation | Fee (credits) | Note |
|---|---|---|
| Identity create | ~116,000,000 | funded 40,000,000,000 credits, ~115.9M to 116.3M consumed across two runs |
| Data contract create | 12,035,452,340 | storage-dominated; a contract is durable state at 27,000 credits/byte |
| Document create | 9,493,240 to 9,720,420 | five documents; rises slightly then stabilises as the index fills |

These are the AGGREGATE fee per operation (storage plus processing, in credits), not the raw five-field
`OperationCost` vector, which is internal to drive-abci and not exposed through the SDK or metrics. The
document-create figures sit in the same millions-to-tens-of-millions band as the committed test
assertions (document delete at 71,994,700 processing), which is the cross-check the harness spec asked
for. The contract-create fee is three orders larger because publishing a contract writes durable state
priced at 27,000 credits per byte.

**The interior of the consensus budget, from drive-abci ABCI metrics under 1-tx-per-block load.** This
answers the Tenderdash-execution-slice item with live numbers rather than a bound.

| ABCI phase | Avg per block | What it is |
|---|---|---|
| prepare_proposal | 17.1 ms | proposer assembles the block (proposer only) |
| process_proposal | 6.3 ms | validators execute and verify the proposed block |
| finalize_block | 134.8 ms | apply state and commit the root, the dominant cost |
| extend_vote | 0.03 ms | negligible |

The drive-abci work per block totals about 158 ms under this load (prepare plus process plus finalize),
with finalize_block ~85% of it. CORRECTED after round 5: do not read this as "5.3% of the 3-second
budget." The three ABCI phases do not share one enclosing 3-second deadline (the propose timeout bounds
proposal, not finalize, and finalization delays the NEXT height), so summing them and dividing by 3000
ms conflates separate deadlines. Treat 158 ms as a LIGHT-LOAD BASELINE only. Mempool admission
(check_tx)
per state transition: identity create ~15.9 ms, documents batch create ~3.7 ms, data contract create
~9.6 ms.

**Under-load block cadence.** With one transaction per block, Platform produced blocks about every 0.5
seconds (mean 0.505 s over blocks 12 to 18, range 0.415 to 0.596 s), against the ~195 s idle heartbeat.
So the block interval is governed by transaction availability, and the drive-abci execution slice (~158
ms) is roughly a third of the ~0.5 s inter-block time and a small fraction of the 3 s budget.

**Bounds on these numbers, stated rather than smoothed.** The load was light, one transaction per block;
the execution slice grows with transactions per block, so ~158 ms is a floor for a nearly-empty chain,
not a saturation figure. finalize_block time likewise scales with the state touched, and this chain is
fresh with little state, so it too is a floor. The raw `OperationCost` vector is not captured, only the
aggregate credit fee. The numbers are from one local machine's masternode hardware, so they are
order-of-magnitude and shape evidence, not mainnet figures. The harness, its results, and the throwaway
devnet mnemonic live in scratch space and are not committed; the mnemonic is a regtest test key with no
value.

## Two findings that touched design decisions, both now decided

The first is a genuine factual correction. The second was reported here as a contradiction and is not
one; the correction is kept visible rather than removed, because it had already been committed. Both
went to the owner rather than being folded by the assistant, and both are now decided.

### The documents-batch transformer is not atomic, but the execution model is

**CORRECTED 2026-08-08, same session. The first version of this section belonged under "contradicts the
design record" and it does not belong there.** It claimed the assumption that the surrounding platform
provides atomicity was gone. That claim was wrong, and a closer read of the execution path established
the opposite. The defect is real but confined to one state transition type, and the design's atomicity
assumptions rest on guarantees that do hold. The correction is recorded rather than quietly edited,
because the original claim was committed and would have driven a defensive design requirement that is
not needed.

**What actually holds.** A state transition is atomic through three mechanisms rather than through
rollback:

- **Validate-then-apply.** `process_state_transition` validates fully and produces an execution event,
  and only then does `process_validation_result_v0` apply it, at around lines 123 to 140 of
  `process_raw_state_transitions/v0/mod.rs` under
  `packages/rs-drive-abci/src/execution/platform_events/state_transition_processing/`. A validation
  failure yields a paid or unpaid consensus error and writes nothing, so atomicity comes from never
  starting rather than from unwinding.
- **Atomic operation batches.** Drive accumulates `LowLevelDriveOperation` values and applies them
  through GroveDB's batch path, whose module documentation opens with "Apply multiple GroveDB operations
  atomically" (`grovedb/src/batch/mod.rs:1`).
- **A block-level transaction** threaded through the whole proposal, which remains rollback-able.

**One named trap.** GroveDB ships `apply_operations_without_batching` (`grovedb/src/batch/mod.rs:3576`),
documented for testing and debugging only, warning that a mid-list failure leaves preceding operations
applied and that root hashes may differ from the batch path because batching propagates hashes in one
pass. Platform never calls it: zero hits across every package, with a control search confirming the
search itself was working. A future implementer reaching for it would silently lose atomicity and change
root hashes, which is why it is named as a prohibited escape hatch in the design rather than left as
folklore.

**The actual defect, stated at its real scope.** `max_transitions_in_documents_batch` is 1, and the code
comment explains why at length. The batch
pipeline is not atomic, so when one transition errors, earlier successful transitions in the same
batch are still applied to state. Nonce-bump semantics for mixed success and failure batches are
undefined, and the dispatch code does not consistently express any policy. The cap sits at 1
specifically so those defects are not exposed to mainnet traffic, and lifting it requires fixing the
atomicity and nonce semantics first (tracked upstream as issue 2867).

Why it matters here, at the corrected scope. The design leans on atomicity in several load-bearing
places, including D16's atomic reservation before asynchronous signing, the
escrow-through-every-cancelable-state rule that three projects have now converged on, and the atomic
compare-and-set dispositions the round-4 retirement fold requires. All of those live INSIDE a single
state transition, which is exactly where validate-then-apply and the atomic batch path hold. So the
design's assumption is CONFIRMED SOUND rather than removed.

What the finding does produce is one design obligation and one prohibition, both now recorded in
`DESIGN.md`. The obligation is that a VM accumulates operations and applies them through the atomic
batch path, never operation-at-a-time, and does not replicate the multi-inner-transition pattern that is
currently capped at one because it is broken. The prohibition names
`apply_operations_without_batching` so the escape hatch is governed explicitly rather than discovered
later. Both were previously inherited assumptions, and stating them costs nothing while leaving them
implicit is how a future implementation loses a guarantee it thought it had.

Upstream issue 2867 remains open, and the design does not depend on the path it affects, so it is noted
here rather than tracked as project work.

### A native shielded subsystem already exists

**DECIDED 2026-08-08 by the owner, after the retention model below was measured. The premise is
corrected in `DESIGN.md` and the D11 decision is unchanged.** The correction is real but narrower than
it first appeared. Only the factual premise was wrong. The retention reasoning that motivated the
deferral turned out to be right, and the measured retention model is now the evidence for keeping the
deferral rather than a reason to reverse it. The retention precedent was harvested into the open D6
obligation-horizon item, which is where it does the most work.

The design record states that Dash privacy today is coordinated mixing rather than a cryptographic
shielded pool, and defers a full native shielded-asset subsystem (commitments, nullifiers, and
value-conservation circuits) as its own separate project on the grounds that it is the largest exposure
and does not fit the base lease model.

For Platform v4.0.0 the factual half of that is wrong. The subsystem exists and is actively developed:

- `packages/rs-dpp/src/shielded`, and `packages/rs-drive/src/drive/shielded` with modules for note
  insertion, nullifier insertion, paths, and estimated costs
- `packages/rs-drive/src/state_transition_action/shielded/` carrying shielded transfer, shielded
  withdrawal, and identity creation from the shielded pool
- `packages/rs-drive/src/verify/shielded`, `packages/rs-drive-abci/src/query/shielded`, and
  `packages/rs-drive-abci/src/shielded_snapshot`
- An Orchard and Halo 2 proving dependency tree, feature-gated for client-side use
- GroveDB v5.0.0 shipping a `grovedb-commitment-tree` crate, with anchor hashing metered as its own cost
  dimension
- A changelog showing shielded transfers, shielded withdrawals, shielded identity creation, transaction
  history, memos, note reservations, and wallet and SDK support

The size model is concrete. A Halo 2 proof grows at about 2,273 bytes per action on top of a
408-byte serialized action, giving roughly 2,681 bytes per action plus about 2,930 bytes fixed.
Measured points in the source are two actions at 8,294 bytes and six at 19,018 bytes. The nominal
16-action cap is therefore not the binding constraint. The 20 KiB state transition size limit is,
and it puts the effective ceiling at six actions.

**The retention model, which is the part that decided the question.** D11 deferred the subsystem partly
because it has no retention model compatible with D6's leases, on the grounds that an unspent commitment
cannot expire without burning an asset and a spent nullifier cannot expire without permitting a double
spend. The real implementation agrees with that reasoning and answers it with a three-way split rather
than a solution.

| State class | Retention | Mechanism |
|---|---|---|
| Nullifiers | Permanent, never pruned | No prune path exists for them |
| Notes and commitments | Permanent | Appended to a commitment tree frontier |
| Anchors | Pruned below a cutoff | `shielded_anchor_retention_blocks`, 1000, stable across protocol versions v2 to v8 |

The only pruning module under `packages/rs-drive/src/drive/shielded/` is `prune_anchors`. There is no
note or nullifier pruning anywhere in that tree. So the platform did not reconcile permanent shielded
state with a lease model, it accepted unbounded permanent growth for the two classes where expiry is
unsafe and charged durable storage up front at 27,000 credits per byte.

Two consequences carried into D6, both from `prune_anchors/v0/mod.rs`, and the first has since been
DECIDED. A horizon is the wrong shape for state that cannot safely expire, since the platform's
answer to that case is a permanent priced obligation rather than a longer deadline, and the funding
model above shows how it pays for one. The owner removed D6's finite horizon on that basis on
2026-08-08. The second consequence is still an obligation on D6 rather than a decision: there is a
trap that is regression-tested upstream, in a test named for the desync it prevents. Pruning the
anchor index naively empties it, which empties the primary anchors tree, which then rejects every
spend with an invalid anchor error until new activity refreshes state. The fix is a floor that
always preserves the highest entry, accepting at most one stale record. That is the same shape as
the round-4 rule barring leased state from being sole custody evidence, and it argues that a D6
cleanup rule needs an explicit never-reach-zero floor rather than only a bounded rate of change.

One distinction is worth preserving rather than flattening. The design's claim about the Dash Core
chain is still true, since Core-level privacy is coordinated mixing. The error is about Platform,
which is the layer this project targets, and which now has a real shielded pool with commitments,
nullifiers, and proof verification. So shielded compatibility is not a future project to defer, it
is an existing subsystem a VM would have to interoperate with. The base design's bounded,
content-addressed, metered proof-verification host call now has something concrete to verify against
rather than a hypothetical, and that is a better position than the record currently claims.

## Two Phase 0 items that cannot be answered this way

Recording these plainly, because a list that quietly drops them would read as though everything is
measurable.

**Real masternode hardware distribution.** Not present in source, and not obtainable by reading code. It
needs a network survey or operator data. Every budget in the design depends on the protocol minimum
machine, so this is a real blocker that no amount of source reading will clear.

**Real dependency-graph width, for D17.** This one is a defect in the Phase 0 plan rather than a missing
measurement. The item asks for the distribution of strongly connected component sizes in realistic
application dependency graphs, and it is being used to decide whether D17 can be simplified from full
cohort migration toward environment-lift plus an accepted outage. No such graphs exist, because no
programs exist, because the VM does not exist. The measurement cannot precede the thing it describes.
Either a proxy has to stand in (the reference graph among existing data contracts, or component-size
distributions from comparable ecosystems), or D17 has to be decided on reasoning rather than data and
revisited once real programs exist. Deciding it on reasoning is defensible, but the record should say so
rather than waiting on a measurement that will not arrive.

## Status of each Phase 0 item

| Item | Status |
|---|---|
| GroveDB proof shapes | ANSWERED, richer than assumed |
| Cost model and price list per write | ANSWERED. Model, prices, and live per-operation fees measured (identity ~116M, contract ~12B, document ~9.5M credits); raw OperationCost vector not exposed, aggregate fee captured |
| Tenderdash execution slice | ANSWERED under light load: ~158 ms/block (finalize ~135 ms dominant) as a one-transaction light-load BASELINE across separately-timed ABCI phases (not a fraction of one 3s deadline); under-load cadence ~0.5s vs ~195s idle. Grows with txs/block, so a floor |
| Multidimensional reservations in credits accounting | STORAGE accounting and estimators ANSWERED; execution-wide reservations and hard-abort enforcement remain OPEN |
| DIP-2 and DIP-7 program-originated withdrawals under quotas | QUOTA ANSWERED at 4 per block; acceptance of program-originated requests still open |
| Contract group and family sizing | ANSWERED at 256 |
| Shielded compatibility | ANSWERED, premise corrected, D11 decision unchanged |
| Batch atomicity assumption | ANSWERED, assumption CONFIRMED sound; defect confined to the documents-batch transformer |
| Data-contract validation callable mid-execution | ANSWERED, callable units invoked by the pipeline at two fixed phases (advanced-structure and state); no program-mid-execution caller today |
| Data contract naming a program hook | ANSWERED, not declarable today, but a native trigger mechanism exists keyed by contract, document type, and action |
| Program-declared secondary indices without trusting program entries | ANSWERED, schema-derived, caller supplies a document and Drive derives entries |
| GroveDB iteration tie-break determinism | ANSWERED for the paths read, no implementation-defined tie-break found |
| Cleanup lane, accumulator, height-filtered views over GroveDB | ANSWERED, accumulator is native (CountTree/SumTree, provable); height-filtered view is an app pattern (by-height index plus range query), not a native primitive |
| Credit meter covering full transaction lifecycle | ANSWERED for charging (cold load, rollback via PaidConsensusError, commit-prep hashing all metered); halting at a resource bound is a VM-layer obligation the fee meter does not provide |
| Closure computation inside admission budget | NOT YET READ |
| Terminal capacity reservation sizing | NEEDS A RUN |
| Cleanup floor arithmetic at real bucket densities | NEEDS A RUN |
| Obligation ledger horizon and pinned record storage cost | PARTIALLY ANSWERED, 27,000 credits per byte is the input |
| Masternode hardware distribution | NOT OBTAINABLE FROM SOURCE |
| Dependency-graph width | NOT OBTAINABLE, the item is mis-specified |

## Method note for the next session

The frozen clones live in session scratch space and are not committed, since they are third-party source
at known revisions and are reproducible from the table above. Recreate them with a shallow clone at each
revision. The Platform repository needs a blobless sparse clone to come down in reasonable time, and the
four packages that matter are `rs-drive`, `rs-drive-abci`, `rs-dpp`, and `rs-platform-version`.

Anything requiring execution needs a container, since the host has no Rust or Go toolchain and, per
standing operating notes, a self-built node daemon does not run natively on this machine. The container
runtime is already up and a local network configuration already exists, so the path for the
measurement-dependent items is a containerised build plus a local network rather than host tooling.
