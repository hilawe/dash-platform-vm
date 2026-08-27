# CosmWasm module bindings over Dash-native features

Dated 2026-08-11. This note maps how a CosmWasm contract reaches services beyond its own storage, and
how each of those surfaces would bind to Dash-native features (tokens, identities, groups, the credit
unit) rather than to the Cosmos SDK modules a contract normally talks to. It is the design half of the
last remaining item on the CosmWasm adopt-versus-build track. The storage path is already demonstrated
end to end (`docs/COSMWASM_STORAGE_ASSESSMENT.md`); the bindings are what sit beside storage.

Evidence grades follow the project convention. The Querier read path and the message write path are both
EXECUTION-PRODUCED (running spikes, below; the write path uses a real contract emitting a real message,
applied by a stand-in router). The production node message router, the full custom Dash operation
catalogs, and the address and crypto API remain node integration and engineering scope, ASSERTED against
Dash's known primitives, not demonstrated here.

## The four interaction surfaces

A CosmWasm contract touches the outside world through four surfaces, three of which are host backend
traits the VM calls, and one of which is the messages the contract emits for the node to apply.

- STORAGE. The contract's own key-value state. Already demonstrated over GroveDB at every layer
  (contract-facing trait, host backend with gas and iterators, and a compiled contract end to end).
- QUERIER. Synchronous reads a contract issues during execution, for state it does not own, such as a
  token balance. This is a host backend trait (`cosmwasm_vm::Querier`). Demonstrated below.
- API. Address canonicalization and validation, and signature verification, exposed to the contract as
  pure functions. A host backend trait (`cosmwasm_vm::BackendApi`). Sketched below.
- MESSAGES. Outbound actions a contract returns from its entry points (a `CosmosMsg`), such as a bank
  send. These are not a VM backend trait. The contract returns them, and the node's message router
  applies them to chain state after the contract returns. Demonstrated below with a stand-in router.

The split matters for the binding work. Reads and the API are answered inside the VM by host traits, so
they are demonstrable the way the storage backend was. Writes leave the VM as messages and are applied
by the node, so binding them is node-router wiring against Dash-native handlers, not VM-internal work.

## The mapping to Dash-native features

| CosmWasm surface | Standard Cosmos target | Dash-native binding |
| --- | --- | --- |
| `BankQuery::Balance`, `BankQuery::Supply` | bank module | Dash token balance and supply, read from GroveDB (DEMONSTRATED below) |
| `BankMsg::Send`, `BankMsg::Burn` | bank module | Dash token transfer and burn, applied by the node router |
| `WasmQuery`, `WasmMsg::Execute`/`Instantiate` | wasm module | inter-program calls between deployed programs, native to the execution layer |
| `StakingQuery`, `StakingMsg` | staking module | not bound; masternode staking is a Core-level concern, not contract-driven, so these are left unsupported rather than mapped |
| custom query (`DashQuery`) | (none) | Dash-specific reads: token supply (DEMONSTRATED), identity existence, group membership |
| custom message (`DashMsg`) | (none) | Dash-specific writes: token mint and burn under a contract's authority, identity and group operations |
| `BackendApi` address and crypto | auth and crypto | Dash identity ids as addresses, and Dash's secp256k1 for signature verification |
| `IbcMsg`, `IbcQuery` | ibc module | out of scope; inter-blockchain communication needs ICS-23 proofs, a separate problem noted in the storage assessment |

The pattern is that the standard bank and wasm surfaces map onto Dash tokens and inter-program calls,
staking is deliberately unmapped, inter-blockchain communication is a separate later question, and
everything genuinely Dash-specific (identities, groups, token mint and burn under program authority)
rides on the CosmWasm custom message and custom query extension points rather than needing a new VM.

## What is demonstrated now: the Querier read path

The spike `metering-prototype/cosmwasm-host/src/bin/querier.rs` implements the real
`cosmwasm_vm::Querier` trait (compiler-verified against cosmwasm-vm 1.5) over Dash-native token state
held in GroveDB, and answers a contract's reads through the exact serialized request the VM sends. Raw
output is `metering-prototype/results/cosmwasm_querier_output.txt`. It answers:

- A standard `BankQuery::Balance` for a Dash token modelled as a bank denom, reading the balance from a
  GroveDB `bank` subtree (alice 1000, bob 500, an absent holder 0), with the response encoded exactly as
  a Cosmos bank query expects.
- A custom `DashQuery::TokenSupply`, reading a per-denom supply from a GroveDB `supply` subtree (1500,
  the sum of the two holders), which is the pattern every Dash-specific read (identity, group) would
  follow.

Each answer carries gas derived from the real GroveDB read cost, and repeated queries return identical
answers and gas. So the read half of the module bindings runs against Dash-native state in GroveDB
through the real VM trait, the same standard the storage path met.

## What remains, and what it is

- THE PRODUCTION MESSAGE ROUTER. The write path is demonstrated below with a real contract emitting a
  real message and a stand-in router applying it to GroveDB. What remains is replacing the stand-in with
  the node's production message router, which applies a contract's returned messages within the block
  transaction. This is node wiring against existing Dash handlers, not VM-internal work.
- THE API. A real `BackendApi` binding Dash identity ids as addresses and Dash's secp256k1 for
  signature verification, replacing the mock used in the spikes. Small and well-scoped.
- THE CUSTOM DASH OPERATIONS. The concrete `DashMsg` and `DashQuery` catalogs for identities, groups,
  and token mint and burn under program authority, each with its host-side handler. The token-supply
  query and the bank transfer demonstrate the query and message shapes; the rest follow the same
  pattern.

None of these is a storage question or a soundness question. They are the engineering scope of adopting
CosmWasm, and they are the substance of the adopt-versus-build decision now that the storage foundation,
the read path, and the write path are demonstrated. The recommended way to carry it forward is to price
this scope against the alternative of a bespoke runtime, with contributors who have Cosmos experience,
rather than to keep extending the spike.

## Claim width

The Querier read path and the message write path are both execution-produced over GroveDB-backed
Dash-native state, the write path with a real contract emitting a real message applied by a stand-in
router. The production node message router, the real API binding, and the full Dash operation catalogs
are not built here, and the API binding is asserted against Dash's known primitives. This note
establishes that the whole binding surface is well-defined and that both its read and write halves run
against GroveDB, not that the full node integration has been built.

## Update 2026-08-11: the write path is now demonstrated too

The write half is no longer only designed. The spike `metering-prototype/cosmwasm-host/src/bin/write.rs`
runs a real contract through the full loop. Hackatom's `release` entry point queries its own token
balance through the GroveDB-backed Querier (step 1's read path), decides to move it, and emits a
`BankMsg::Send` to its beneficiary. A stand-in message router then applies that emitted message to
Dash-native token balances in GroveDB, and the result is proven. Raw output is
`metering-prototype/results/cosmwasm_write_output.txt`.

What ran, EXECUTION-PRODUCED:

- The contract instantiated (writing its config through the storage backend), then its `release`
  executed through the VM, read the contract's 1000 udash balance through the Querier, and emitted
  exactly one message, a `BankMsg::Send` of 1000 udash to the beneficiary.
- The router applied that emitted message to the GroveDB bank state: the contract's balance moved from
  1000 to 0 and the beneficiary's from 0 to 1000.
- The resulting Dash token balances are provable: a `prove_query` over the bank subtree verified against
  the live root.

So the whole loop is shown against real Dash-native state in GroveDB: a running contract reads a
balance, emits a transfer, and the transfer is applied and proven. What is a stand-in, and stated
plainly, is the ROUTER. Here it is a demonstration function; in a node it is the production message
router that applies a contract's returned messages within the block transaction. The message-emission
and application SHAPE is execution-produced with a real contract and a real emitted message; the
production router that would replace the stand-in is the node-integration work.

This narrows what remains on the whole CosmWasm track to node integration and catalog-filling, none of
it a storage, soundness, or feasibility question, and they are wiring the real node message router in
place of the
stand-in, a real `BackendApi` (Dash identity ids as addresses, Dash secp256k1), and the full
`DashMsg`/`DashQuery` catalogs for identities, groups, and token mint and burn. The read path, the write
path, the storage at every layer, and a compiled contract end to end are all demonstrated over GroveDB.
