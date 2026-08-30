# Metering and storage prototype, scope and limitations

This note draws the boundary of the metering prototype and the GroveDB-backed CosmWasm/EVM storage
spikes under `metering-prototype/`. It records what the code models and charges, and what it
deliberately does not, so the prototype's claims match its evidence. It is the closing artifact of a
fifteen-round adversarial review, run by a review source independent of the author, which drove the
metering and storage code to the state described here.

## What the prototype is for

The prototype answers three exploratory questions, not a production-readiness question:

1. Is the D3b terminal-work meter arithmetic from DESIGN.md v12 self-consistent and enforceable? The
   `meter-core` crate makes the ten certified invariants executable over a block loop and watches each
   one fail under a deliberately broken variant.
2. Can a GroveDB-authenticated store back the CosmWasm host storage and querier traits, and carry an
   EVM interpreter compiled to a guest contract? The `cosmwasm-host` crate and its bins run real
   compiled contracts through the real cosmwasm-vm over GroveDB, with the results proven by GroveDB
   light-client proofs.
3. What do the design's open cost dials measure? Phases A, C, D, and E turn the storage and compute
   cost dimensions into measured numbers.

## What the prototype models and charges

The metering that IS implemented, with tests:

- GroveDB operation cost, adapted to gas deterministically from the measured `OperationCost` (seeks,
  added/replaced/removed bytes, loaded bytes, hash-node calls), never a flat schedule.
- In-memory overlay work: an O(log N) lookup charge on every point read and write (hit and miss),
  a per-entry and per-byte charge on a range scan's overlay merge (including out-of-range keys and
  tombstones), the scan's range-bound copies, and unconditional key and value materialization on
  writes.
- Durable write cost: a commit measures the real density-dependent GroveDB insert and delete cost and
  returns it, and `commit_within_budget` enforces a budget, flushing into an uncommitted transaction
  that stops the moment the budget is exhausted and rolls back before anything becomes durable.
- The bank router runs all of its reads and writes against one GroveDB transaction (so two concurrent
  sends touching the same balance keys conflict at commit under GroveDB's optimistic concurrency),
  bounds the coin count, charges per coin before materialization, enforces a budget, and validates
  each account's gross debit against its starting balance so an over-balance send is rejected even to
  self while a within-balance self-transfer mints nothing.
- Query request parsing is charged per byte on every return path, never free for a malformed or
  unsupported request.
- Arithmetic that bears a capacity or cost decision is checked: admission, drain, partition, and burst
  use `checked_add`/`checked_mul_scalar` and refuse or flag on overflow. The id and height counters
  use `checked_add` and fail closed rather than wrapping.
- Block advancement is gated behind a `BlockTick` capability whose only field is private (type-level
  separation of a host action from the contract operations), and the per-block ledgers reset only on
  that advance.

## What the prototype deliberately does NOT model

These are non-goals. They belong to the execution layer and consensus integration, not to a
meter-arithmetic and storage-feasibility spike, and building them into the prototype would make it
appear production-ready while it is not.

1. A consensus block coordinator and a non-forgeable host/contract privilege boundary. `end_block` is
   the MODELED host block transition, and in a real deployment the consensus layer decides when a
   block ends. The `BlockTick` capability separates the host action from the contract operations at
   the type level, but the prototype is a single-process model of the meter arithmetic. It does not
   implement the runtime sandbox that would make the capability unforgeable against untrusted contract
   code, because there is no contract sandbox here to enforce it. That sandbox is the execution
   layer's responsibility and a separate design question.
2. A byte-complete production gas schedule. The absolute gas weights are a directional policy
   calibration, not a production schedule. The point demonstrated is that gas TRACKS the real measured
   cost, not that every input byte is priced at a production-correct weight.
   CORRECTED 2026-08-30. This item previously also claimed that unbounded work cannot run for a bounded
   charge. That claim was false for the range-scan path. `effective_range` builds its GroveDB query with
   no limit, materializes the full result and the merged overlay, and returns the gas charge only after
   that work has been done, so the charge is accounting rather than admission control. Tracked as a P1
   defect. The prototype therefore bounds the CHARGE but not the WORK on that path, and a production
   implementation needs the limit derived from remaining gas before execution rather than after. Some minor input paths (for example an address or denomination
   string's length, or note-string construction) are folded into base and per-operation charges rather
   than priced individually.
3. Multi-threaded serializable isolation beyond GroveDB's optimistic transaction. The router's
   single-transaction structure gives write-conflict detection for concurrent sends, and the demos are
   single-threaded. Full serializable isolation under arbitrary concurrent callers (for example
   read-set conflict detection through a GetForUpdate-style primitive) is not implemented, since
   GroveDB does not expose it here and the demonstration does not require it.
4. Consensus and networking. Already disclaimed in DESIGN.md and the meter-core module docs: there is
   no consensus, no networking, and the capacities in `meter-core` are illustrative values chosen to
   exercise the arithmetic, not platform claims.

## What was demonstrated and holds

- The meter arithmetic and the ten certified invariants, executable and fault-injected: 25 tests in
  `meter-core`, plus Phase D driving the certified scenarios under load with every invariant holding
  at every block boundary and a 100k-object mass retirement draining in the predicted blocks in O(N).
- GroveDB-backed CosmWasm storage and querier, an end-to-end compiled contract, the write path with
  the bank router, and the EVM-as-guest path: 18 library tests in `cosmwasm-host` plus the running
  demos (host backend, querier, write path, end-to-end, EVM, and the storage spike), each proving its
  result with a GroveDB proof bound to exact content.
- Rollback and budget enforcement: a failed call (inner contract error or outer VM trap, including gas
  depletion) leaves no durable write, and an over-budget commit or transfer rolls back before anything
  becomes durable.

## Review record

The metering and storage code was reviewed adversarially over fifteen rounds. Each round's real
findings were folded and re-reviewed, moving from systematic overclaiming in the early rounds to
isolated cost-accounting and encapsulation holes in the later ones. The loop was stopped by decision
at the point where the remaining findings required production runtime infrastructure (a sandbox
privilege boundary and a byte-complete gas schedule) rather than exposing an in-scope soundness hole.
Those remaining findings are recorded above as deliberate non-goals. The round outputs are under
the project's review record for the spike review.
