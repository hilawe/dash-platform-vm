# Can CosmWasm's key-value storage be backed by GroveDB? An interface-level assessment

Dated 2026-08-11. This note answers the gating question that decides whether adopting CosmWasm as Dash
Platform's execution engine is viable, namely whether CosmWasm's storage interface can be backed by
GroveDB while preserving Platform's light-client provability. It began as an interface-level assessment
and has since been confirmed by two running spikes (see the final sections), the contract-facing storage
layer and the host-side cosmwasm-vm backend with its gas adapter, so the answer is now
execution-produced
for the storage path, not only an interface read. It was raised as
an
option in `docs/COMMUNITY_BRIEF.md` after a Dash Platform core developer suggested CosmWasm.

Evidence grades, following this project's convention. ASSERTED means stated from knowledge of an
external
system not checked out here. REPOSITORY-RESOLVED means read from source in the checked-out GroveDB at
the
frozen revision 9b98a35644cdea73cc1b21d7c122cb58ae9fafd8 (v5.0.0). EXECUTION-PRODUCED means demonstrated
by running code, which the three spike sections at the end now provide for the whole storage path (the
contract-facing trait, the host-side cosmwasm-vm backend, and a compiled contract run end to end).
The GroveDB-side claims are repository-resolved with file and
line citations. The CosmWasm-side claims are asserted from knowledge of its storage trait and its IAVL
backing, and are marked for confirmation against the targeted CosmWasm version.

## The question, and the verdict

CosmWasm contracts read and write through a small storage interface. If that interface can sit on
GroveDB with proofs intact, then adopting CosmWasm keeps the property that ruled out the Ethereum
Virtual
Machine (EVM), which is that program state stays in the authenticated store and light clients can prove
it. If it cannot, adoption is off the table for the same reason the EVM was.

The interface-level verdict is favorable. Every primitive CosmWasm's storage contract requires has a
direct GroveDB counterpart, verified in the GroveDB source, and Platform's native proofs cover the
result
without a translation layer. This does not prove a working integration, which needs a spike, but it
removes the objection that sank the EVM.

## Why the fit is structural, not coincidental

CosmWasm was designed against an authenticated, ordered key-value store from the start. Cosmos chains
back CosmWasm with IAVL, which is a versioned Merkle tree with lexicographically-ordered byte keys,
range
iteration, and cryptographic proofs (ASSERTED, from knowledge of the Cosmos SDK). So CosmWasm's storage
contract does not assume a plain hashmap. It assumes the shape GroveDB already has. Replacing IAVL with
GroveDB is a substitution between two authenticated ordered key-value stores, not a graft of a foreign
model. That is the structural difference from the EVM, whose fixed 256-bit-slot storage model has no
such
affinity.

## The primitive-by-primitive mapping

CosmWasm's storage requirements are on the left (ASSERTED, from its `Storage` trait and its VM backend).
The GroveDB counterpart is on the right (REPOSITORY-RESOLVED, cited).

| CosmWasm storage requirement | GroveDB counterpart (frozen revision) |
| --- | --- |
| `get(key) -> Option<bytes>` | `GroveDb::get(path, key)` returning `Element::Item(bytes)` |
| `set(key, value)` | `GroveDb::insert(path, key, Element::new_item(value))` |
| `remove(key)` | `GroveDb::delete(path, key)` |
| ordered `range(start, end, Order)` | `QueryItem` variants `Range`, `RangeInclusive`, `RangeFull`, `RangeFrom`, `RangeTo`, `RangeToInclusive` (`grovedb-query/src/query_item/mod.rs:46`), with direction via `left_to_right` (`grovedb-query/src/lib.rs:71`) |
| streaming iterator (`scan` then `next`) | `RawIterator` trait (`storage/src/storage.rs:279`), methods `raw_iter` (`:236`), `next` (`:293`), `valid` (`:305`), over RocksDB's native ordered iterators |
| lexicographic byte-key ordering | Merk keys are `Vec<u8>` compared with the default byte-lexicographic order |
| per-contract storage isolation | each contract as a GroveDB subtree with its own prefixed storage context |
| transaction with read-your-writes | `start_transaction` (`grovedb/src/lib.rs:909`) and `PrefixedRocksDbTransactionContext` (`storage/src/storage.rs:100`) |
| provable state for light clients | `prove_query` (`grovedb/src/operations/proof/generate.rs:73`) and `verify_query` (`grovedb/src/operations/proof/verify.rs:1976`) |

## The provability outcome

The property the EVM could not keep, GroveDB keeps by default. A contract's state, stored as GroveDB
items under the contract's subtree, is provable with `prove_query` in Platform's native proof format, so
light clients that already verify Platform proofs verify contract state with no new proof system.
Backing
CosmWasm with GroveDB does not merely survive the provability constraint. It satisfies it in Platform's
own format.

## What an interface read cannot settle, and needs a spike

Reading interfaces shows the primitives exist and line up. It does not show the integration runs. Four
items needed an executable check rather than a source read. The spike settled the first three, and the
host-backend section that follows it settled the fourth (the cosmwasm-vm host trait is now
compiler-verified against the real crate). They were the content of the recommended thin slice.

- Transactional range iteration reflecting uncommitted writes in correct sorted order. RocksDB
  transactions provide this and GroveDB wraps them, so confidence is high, but it is exactly the
  behavior
  that wants a running test rather than a source read.
- The cost adapter. CosmWasm charges gas per storage backend call. GroveDB returns its measured
  `OperationCost` vector, which the metering prototype measured in phases A through E. Translating that
  vector into CosmWasm gas is straightforward adapter work, and it is arguably an improvement, because
  storage would be priced by GroveDB's real measured cost rather than a flat schedule. Authenticated
  writes cost more than a plain set, but IAVL already charges that, so it is not a new penalty.
- Iterator lifecycle, since a contract can hold several open iterators at once over transactional state,
  which is implementation surface to manage.
- The exact CosmWasm backend trait shape in the targeted version. The mapping above is asserted from
  knowledge of CosmWasm's storage trait and should be confirmed against the CosmWasm sources being
  targeted, because the VM backend trait has changed across CosmWasm versions.

One further item is a design choice rather than a blocker. GroveDB-native proofs serve Platform's
clients, but they are not ICS-23 proofs, so if inter-blockchain communication (IBC) compatibility ever
becomes a goal, that is its own proof-translation problem, the same shape as the EVM's.

## Claim width

This assessment establishes that CosmWasm's storage primitives map cleanly onto GroveDB and that
Platform's provability is preserved, at REPOSITORY-RESOLVED grade for the GroveDB side and, after the
two
spikes below, EXECUTION-PRODUCED grade for the whole storage path (the contract-facing trait and the
host-side cosmwasm-vm backend with its gas adapter and iterator management, and a compiled contract run
end to end through the VM). It does not settle the cost model's economic acceptability, and it does not
cover the Dash-native module bindings, which remain open. The gating storage question comes back
favorable and is now backed by running evidence, which is enough to carry the adopt-versus-build
evaluation forward, not to conclude adoption on its own.

## Recommended next artifact

Implement CosmWasm's storage backend trait over a GroveDB subtree and run one simple `cw-storage-plus`
Map contract end to end, exercising get, set, remove, and an ordered range scan inside a transaction,
then
prove a piece of the resulting contract state with `prove_query` and verify it. That single slice
converts the four open items above from asserted or high-confidence to execution-produced, and it is the
thin end-to-end slice the community brief recommends before any adopt-versus-build commitment.

## Spike results (2026-08-11): the interface-level answer is now execution-produced

The recommended next artifact was built and run. `metering-prototype/cosmwasm-spike/` implements the
real
`cosmwasm_std::Storage` trait (version 1.5.11) over a GroveDB subtree and drives it through the real
`cw-storage-plus` `Map` (version 1.2.0), so the trait shape is compiler-verified against the actual
crate
rather than asserted. Raw output is `metering-prototype/results/cosmwasm_spike_output.txt`. It does not
boot the wasmer VM, by design, because that path exercises cosmwasm-vm's contract-calling rather than
the
GroveDB-backing question.

What the spike demonstrated, all EXECUTION-PRODUCED against GroveDB at the frozen revision:

- get, set, and remove through `cw-storage-plus` `Map::save`, `load`, `may_load`, and `remove`, with
  read-your-writes inside a GroveDB transaction (a saved value loads back, a removed key reads absent,
  both before commit).
- Ordered range iteration ascending (alice, bob, dave, after removing carol) and descending (the
  reverse), through `Map::range`.
- A bounded range `[bob, dave)` returning exactly `bob`, confirming CosmWasm's start-inclusive,
  end-exclusive semantics map correctly onto GroveDB's range query items.
- Determinism: the committed root hash is identical across two independent runs (asserted in the
  harness).
- Provability: after commit, `prove_query` over the contract subtree produced a proof that
  `verify_query` checked against the live root hash, and alice's balance (100) was decoded out of the
  proven results, so the proof carries real contract data, not just a matching root.

One implementation detail, recorded honestly. GroveDB's native descending traversal of a full range
returned empty in this spike, so the adapter queries ascending and reverses for a descending request.
That is correct for CosmWasm here because the adapter uses no query limit, so direction does not change
the result set. Whether to rely on GroveDB's native descending path (which would matter under a query
limit) is a follow-up to understand rather than a blocker, since the adapter produces correct CosmWasm
order either way.

What the spike still does NOT establish, so the claim stays bounded. It exercises the
`cosmwasm_std::Storage` trait and cw-storage-plus, which is the contract-facing storage layer. It does
not exercise cosmwasm-vm's host-side `BackendStorage` trait (the one that adds gas accounting and
iterator ids), and it does not run a compiled Wasm contract through wasmer. The cost adapter (mapping
GroveDB's `OperationCost` to CosmWasm gas) and the module bindings (replacing Cosmos SDK modules with
Dash-native host functions) remain design work. The gating storage question, though, is now settled with
running evidence rather than an interface read: CosmWasm's storage layer works over GroveDB with
provability preserved.

## Host backend (2026-08-11): the cosmwasm-vm Storage trait and the gas adapter

The spike above proved the contract-facing `cosmwasm_std::Storage` trait over GroveDB. The next layer
out
is the host-side trait the VM actually calls, `cosmwasm_vm::Storage` (version 1.5.11), which adds gas
accounting on every operation and iterator-id management. That layer is now built and passing in
`metering-prototype/cosmwasm-host/`, output `metering-prototype/results/cosmwasm_host_output.txt`. It
implements the REAL trait, so the host trait shape is now compiler-verified against cosmwasm-vm rather
than asserted (this was the fourth open item from the earlier section). It drives the trait directly
through the call sequence a contract would cause, not through a compiled Wasm contract, so it still does
not run wasmer end to end.

What it demonstrated, EXECUTION-PRODUCED:

- The cost-to-gas adapter. Each operation returns CosmWasm gas derived deterministically from GroveDB's
  measured `OperationCost` (seeks, storage bytes, loaded bytes, hash-node calls), not a flat schedule.
  In
  the run, a set consumed 175,400 gas from a measured cost of 151 added bytes, 333 loaded bytes, 15
  hash-node calls, and 8 seeks; a get consumed 65,400. The absolute scale is a policy calibration for a
  real deployment; the demonstrated property is that gas tracks the store's real measured cost, which is
  the open cost-adapter question the assessment flagged, and it answers the note's own observation that
  this would price storage by real cost rather than a flat schedule.
-  Iterator-id management. `scan` returns a distinct iterator id and `next` advances it. Two iterators
  (an
  ascending and a descending scan) were opened concurrently and their `next` calls interleaved, each
  advancing independently by id, and a `next` on an unknown id returned `IteratorDoesNotExist` rather
  than a silent empty. This settles the iterator-lifecycle item.
- Provability and determinism carry over. After commit, the contract state proved and verified against
  the live root, and the gas figures and the root hash were identical across two runs.

What remains open on the CosmWasm track after this step. The storage path is now settled with running
evidence at both the contract-facing and host layers. Two items remain, and neither is a storage
question. First, running a compiled Wasm contract through wasmer end to end, which would exercise the
VM's contract-calling over this backend (well established on every Cosmos chain, but not yet shown here
on
GroveDB). Second, the module bindings, replacing the Cosmos SDK modules a contract may call (bank,
staking, inter-blockchain communication) with host functions to Dash-native tokens, identities, and
groups. Those are the substance of an adopt-versus-build decision, and the storage foundation under them
is now demonstrated rather than assumed.

## End to end (2026-08-11): a compiled contract runs on GroveDB and its state is provable

The last storage-adjacent open item, running a compiled Wasm contract through the VM end to end, is now
done. `metering-prototype/cosmwasm-host/src/bin/e2e.rs` takes a real compiled contract (hackatom, the
canonical cosmwasm-vm test contract, copied from the cosmwasm-vm 1.5.11 test data, Apache-2.0),
instantiates it through the real cosmwasm-vm (wasmer, singlepass compiler) with a GroveDB-backed host
store, queries it back through the VM, and then proves the contract's stored state with GroveDB
`prove_query`. Raw output is `metering-prototype/results/cosmwasm_e2e_output.txt`.

What ran, EXECUTION-PRODUCED:

- The contract compiled and loaded into the VM.
- Its instantiate entry point ran through the VM and wrote its config into GroveDB via the host storage
  backend.
- Its query entry point ran through the VM and read the stored verifier back, returning
  `{"verifier":"verifies"}`.
- The state the contract wrote is provable in GroveDB: a `prove_query` over the contract subtree
  produced a proof that verified against the live committed root, with one proven state entry.

One structural detail worth recording. The VM requires the storage to be `'static` (it is cloned into
the VM's execution environment), so the end-to-end store owns an `Arc<GroveDb>` and writes directly,
without a transaction, rather than borrowing a transaction the way the host demo does. In a real
integration the contract call would run inside a block-level transaction that the node owns and commits
or rolls back, which is a wiring choice at the node boundary, not a storage-capability question. The
capability, that a running contract's writes land in GroveDB and are provable, is what this shows.

With this, the CosmWasm storage path is demonstrated end to end, from a compiled contract executing in
the VM down to a light-client proof of the state it wrote. What remains on the CosmWasm track is no
longer a storage question at all, namely the module bindings that replace the Cosmos SDK modules a
contract may call (bank, staking, inter-blockchain communication) with host functions to Dash-native
tokens, identities, and groups. That is the substance of an adopt-versus-build decision, resting now on
a storage foundation shown to work at every layer from the contract down to the proof. That bindings
work has now been started and is laid out in `docs/COSMWASM_MODULE_BINDINGS.md`, whose read half (the
Querier over Dash-native token state in GroveDB) is already execution-produced.
