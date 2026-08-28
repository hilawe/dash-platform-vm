# Execution engine, adopt versus build

Dated 2026-08-12. This note is the decision synthesis the CosmWasm and EVM spikes were built to inform.
DESIGN.md v12 settled the architecture and is review-complete and frozen. The open question it left is
the execution engine itself, whether to adopt CosmWasm, backed by GroveDB, as the runtime that gives
Dash Platform general programmability, or to build a bespoke virtual machine (VM). The spikes under
`metering-prototype/` now supply running evidence for that choice. This note reads that evidence, states the recommendation,
and names the conditions that must close before the choice is committed.

Evidence grades follow the project convention. EXECUTION-PRODUCED means demonstrated by running code
against GroveDB at the frozen revision. REPOSITORY-RESOLVED means read from the checked-out GroveDB
source. ASSERTED means stated from knowledge of an external system not checked out here.

## Recommendation

Adopt CosmWasm as the execution engine, backed by GroveDB, and treat Ethereum Virtual Machine (EVM)
compatibility as a later guest-shape design question rather than a base-layer commitment. Three things
carry this. The gating provability question is answered with running evidence at every layer. The fit is
structural rather than lucky, because CosmWasm was designed against exactly the kind of authenticated
ordered key-value store GroveDB is. And the work that remains is bounded node integration and
Dash-native bindings, not open-ended VM design, which is the part where consensus forks live.

Build is justified only if a specific determinism, gas-bound, sandbox, or native-capability requirement
is shown to be unmeetable within CosmWasm. No such blocker has surfaced in the spikes. Until one does,
building a new VM would reinvent what CosmWasm already provides (deterministic metering, a contract
binary interface, a storage abstraction) and take on the largest security surface in the system without
a demonstrated reason.

## What the spikes established

The question that ruled out the EVM as a base was provability. The EVM's fixed 256-bit-slot storage
model does not keep program state in an authenticated store that light clients can prove. The gating
test for CosmWasm was whether its storage interface can sit on GroveDB with Platform's proofs intact.
That test now passes at every layer, EXECUTION-PRODUCED (`docs/COSMWASM_STORAGE_ASSESSMENT.md`,
`docs/COSMWASM_MODULE_BINDINGS.md`, `docs/EVM_GUEST_SPIKE.md`):

- The contract-facing `cosmwasm_std::Storage` trait runs over a GroveDB subtree through the real
  `cw-storage-plus` `Map`, with get, set, remove, ordered and bounded range iteration, read-your-writes
  in a transaction, deterministic root hashes across runs, and the committed state proven with
  `prove_query` and verified against the live root.
- The host-side `cosmwasm_vm::Storage` trait runs with a cost adapter that derives CosmWasm gas from
  GroveDB's measured `OperationCost` (seeks, storage bytes, loaded bytes, hash-node calls) rather than a
  flat schedule, plus concurrent iterator-id management. Pricing storage by its real measured cost is an
  improvement over the flat schedule a Cosmos chain uses on IAVL, the versioned Merkle tree that backs
  CosmWasm there.
- A real compiled contract (hackatom) instantiates and is queried through the real cosmwasm-vm (wasmer,
  singlepass compiler) over the GroveDB-backed store, and the state it writes is provable in GroveDB.
- The module bindings' read path (a bank balance query and a custom Dash token-supply query over
  Dash-native state in GroveDB) and write path (a contract emitting a `BankMsg::Send`, applied to
  GroveDB balances by a stand-in router, then proven) both run through the real VM traits.
- A minimal EVM interpreter compiled as a CosmWasm contract executes EVM bytecode whose `SSTORE` lands
  in GroveDB and is provable, which shows EVM compatibility is reachable through the guest shape without
  giving up provability.

The fit is structural. CosmWasm's storage contract assumes an authenticated, lexicographically-ordered
byte-key store with range iteration and cryptographic proofs, which is the shape GroveDB already has.
Replacing IAVL with GroveDB is a substitution between two authenticated ordered key-value stores, not a
graft of a foreign storage model onto GroveDB. That is the difference from the EVM, whose storage model
has no such affinity, and it is why the storage question came back favorable at REPOSITORY-RESOLVED
grade before any spike and EXECUTION-PRODUCED after.

## What adoption still costs

None of the remaining work is a storage question or a demonstrated soundness hole. It is node
integration and engineering scope, and it is the real content of the commitment.

- The production message router. A contract's outbound messages (a `CosmosMsg`, for example a bank
  send) are applied by the node after the contract returns. The spike used a stand-in router, and a node
  uses its production router, which applies the messages within the block transaction. This is node
  wiring against existing Dash handlers, not VM-internal work.
- The backend application binary interface. A real `cosmwasm_vm::BackendApi` binding Dash identity ids
  as addresses and Dash's secp256k1 for signature verification, replacing the mock the spikes use. Small
  and well-scoped, and secp256k1 is already native to Dash.
- The custom Dash operation catalogs. The concrete custom-message and custom-query sets for identities,
  groups, and token mint and burn under a program's authority, each with its host-side handler. The
  token-supply query and the bank transfer already demonstrate the query and message shapes, and the
  rest follow the same extension-point pattern rather than needing a new VM.
- Deliberate non-mappings. Masternode staking is a Core-level concern and is left unsupported rather
  than bound. Inter-blockchain communication (IBC) is out of scope, because it needs ICS-23 proofs,
  which is a proof-translation problem of the same shape the EVM faced, and is a separate later
  question.
- The metering integration. The terminal-work meter is demonstrated and reviewed to
  prototype-completeness, with its boundary written down in `metering-prototype/SCOPE_AND_LIMITATIONS.md`.
  Its four non-goals (a consensus block coordinator, a host and contract sandbox privilege boundary, a
  byte-complete production gas schedule, and serializable isolation beyond GroveDB's optimistic
  transaction) are execution-layer design questions, not spike patches, and they land on the adoption
  side as real work.

## The tradeoff, stated plainly

Adopting CosmWasm brings a mature, deterministic VM (wasmer with the singlepass compiler and gas
metering), a contract ecosystem and Rust toolchain, and a demonstrated GroveDB fit, at the cost of a
large external dependency whose version and validator quirks must be governed. The build gymnastics the
EVM guest needed (Rust 1.86 to predate the wasm32 bulk-memory default the cosmwasm-vm 1.5 validator
rejects, and the edition-2024 transitive dependency) are a concrete sign that the VM and its toolchain
must be pinned across the validator set and upgraded under consensus governance, because a version skew
in a deterministic VM presents as a consensus fork, not a bug.

Building a bespoke VM buys full control over determinism, the gas schedule, the sandbox boundary, and
native integration with no Cosmos impedance, at the cost of the largest security and engineering surface
in the system. The metering prototype's fifteen-round adversarial review is a useful calibration here:
it took that many rounds to make a MODEL of the meter arithmetic sound, which is a fraction of what a
real VM's determinism and metering demand. Building also discards the demonstrated CosmWasm and GroveDB
fit and starts the ecosystem from zero. The control a bespoke VM offers is real, but nothing measured so
far shows CosmWasm cannot meet the requirements, so paying that surface cost is not yet justified.

## Conditions to close before committing

These are the gates. Each maps to a high-weight evaluation dimension in `docs/EVALUATION_DIMENSIONS.md`
and is the kind of failure that presents as a fork or a dropped platform property, so each wants a
verified answer, not an assurance, before adoption is final.

1. Determinism, the first-weighted dimension. Establish what any candidate engine must satisfy to
   execute identically on every validator, and which candidates satisfy it. For CosmWasm the controls
   are the fixed singlepass compiler, gas-metering boundary timing, the Gatekeeper middleware's feature
   rejections, NaN canonicalization, and version pinning of cosmwasm-vm and wasmer across validators.
   NOTE that an earlier version of this list named the absence of floating point as one of those
   controls, which reading the source disproved. Floats are permitted and NaN canonicalization carries
   the determinism instead, recorded in `docs/GATE1_DETERMINISM.md`. This is the failure mode that
   presents as consensus forks.
2. Worst-case block bound, the third-weighted dimension. The metering prototype measured the storage and
   compute cost dials. Confirm that VM execution plus GroveDB writes plus proof generation stays inside
   the roughly 0.5-second under-load block cadence measured in Phase 0, under adversarial load and not
   just typical load.
3. Shielded and zero-knowledge compatibility, a stated first-class goal. Confirm a CosmWasm contract can
   verify a zero-knowledge proof on-chain within the gas and block bound, whether through a precompile
   or in-Wasm, since gating an action on a proof without revealing its witness is a named requirement.
4. Asynchronous native capabilities, the fifth-weighted dimension. Masternode threshold signatures and
   bridge operations cannot complete synchronously inside one block. Confirm the request-now,
   resolve-in-a-later-block pattern maps onto CosmWasm's message and reply model rather than assuming a
   synchronous call.
5. Version-pinning governance. Decide how the VM and toolchain versions are pinned and upgraded under
   consensus governance, since the spike's build constraints show this is a standing operational duty,
   not a one-time setup.

## Suggested next step if the direction is taken

The conditions above are design and verification questions, and condition 1 (determinism) is the
assumption whose failure would kill adoption. It should go first, and because the execution engine is
architecture-bearing, the natural way to answer it is the clean-room design round the
project uses for such decisions, not another extension of the spike. The bindings work is well-defined
and can proceed in parallel once determinism clears, since its shape is already demonstrated and its
remaining pieces are node integration against known Dash primitives.
