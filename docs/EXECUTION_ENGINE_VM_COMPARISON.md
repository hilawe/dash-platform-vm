# Alternative execution engines, screened against the requirements

Dated 2026-08-27. Prompted by a question from the Dash community, relayed through @hilawe, asking how the
recommended direction in `docs/EXECUTION_ENGINE_ADOPT_VS_BUILD.md` compares with MoveVM. The question is
worth more than a one-engine answer, so this note screens the field of candidate engines against the
recorded requirements first, then runs the head-to-head that survives the screen.

Evidence grades follow the project convention. Every claim about CosmWasm and GroveDB below is
EXECUTION-PRODUCED or REPOSITORY-RESOLVED and traceable to the spikes. Every claim about MoveVM, PolkaVM,
the Solana bytecode format, and the rest is ASSERTED, meaning stated from knowledge of an external system
that is not checked out here and has not been run against GroveDB. The asymmetry is the point rather than
a flaw in the comparison. CosmWasm has been through the gating test and none of the others has, so the
verdict below is a ranking of which engine deserves the next spike, not a claim that the ranking has
been measured.

## The screen

The requirements record (`docs/REQUIREMENTS.md`) and the weighted dimensions
(`docs/EVALUATION_DIMENSIONS.md`) supply three filters that an engine has to pass before a detailed
comparison is worth anyone's time. They are the same filters that ruled out the EVM as a base layer.

1. **Provable program state.** Program-written state must keep membership, non-membership, and
   secondary-index proofs to light clients (dimension 2). In practice this means the engine's storage
   interface has to be satisfiable by an authenticated, lexicographically ordered byte-key store with
   range iteration, because that is what GroveDB is.
2. **A determinism story the engine already owns.** The engine must arrive with named answers for
   floating point, iteration order, memory growth, metering-boundary timing, and version pinning
   (dimension 1). An engine that leaves these to the integrator is a build, not an adopt, and should be
   compared against the build arm instead.
3. **Coexistence with existing Platform state.** Data contracts, documents, identities, groups, and
   tokens must survive, with migration or wrapping permitted (owner decision 1, binding).

Screened against those, the field falls out as follows.

| Engine | Provable state on GroveDB | Owns its determinism story | Verdict |
| --- | --- | --- | --- |
| CosmWasm | Demonstrated at every layer | Yes, with version pinning as the standing duty | Incumbent recommendation |
| MoveVM | Interface is key-value but unordered, no range | Yes, and stronger than CosmWasm's | Survives, run the head-to-head |
| Raw Wasm runtime plus Dash-native host (wasmtime, wasmi) | Whatever the host defines, so yes by construction | No, the integrator writes it | This is the build arm, already costed |
| PolkaVM (RISC-V) | Host-defined, so yes by construction | Partly, metering is a design goal, maturity is thin | Watch, do not adopt yet |
| Solana SBF (eBPF) | No, the account model has no state proofs | Yes | Fails filter 1 |
| EVM, and EVM-adjacent Wasm layers (Stylus) | No, fixed 256-bit slots over a Merkle Patricia trie | Yes | Already rejected as a base, reachable as a guest |
| FuelVM | UTXO-centric state, no ordered contract keyspace fit | Yes | Fails filter 1 and filter 3 |
| zkVMs (RISC Zero, Cairo, Miden) | Different class, proving execution rather than storing state | Yes | Out of scope for this question |

So the answer to "is it MoveVM or another one" is that MoveVM is the right comparator, and it is the only
mature engine outside the incumbent that clears the screen. The other genuinely informative arm is not
another product at all. It is the raw-runtime build arm the adopt-versus-build note already costed, which
is why that note remains the decision document and this one is a supplement to it.

## Why MoveVM clears the screen when the EVM did not

Move's global storage is a key-value map from an account address plus a type tag to a serialized
resource, and modules resolve out of the same store. That is a byte-key store over byte values, which
GroveDB can back. This is a real difference from the EVM, whose 256-bit-slot-per-contract model presumes
a Merkle Patricia trie and does not survive the substitution.

Two properties make MoveVM stronger than CosmWasm on the dimension weighted first, which is exactly the
dimension the adopt-versus-build note named as the assumption whose failure would sink adoption.

- **There is no floating point in the language.** Move's value types are the sized unsigned integers,
  booleans, addresses, and structs built from them. CosmWasm reaches the same place by rejecting float
  opcodes during Wasm validation, which is a check that has to be correct and has to stay correct across
  versions. Move gets there by having no such opcode to reject. Absence beats a validation pass.
- **Metering lives in the interpreter rather than in compiled-in gas points.** MoveVM charges per
  instruction as its interpreter executes, so there is no compiler in the trusted path. CosmWasm's
  determinism argument depends on pinning both cosmwasm-vm and the wasmer singlepass compiler across the
  validator set, and the spike's own build constraints (Rust 1.86 to predate the wasm32 bulk-memory
  default that the cosmwasm-vm 1.5 validator rejects) are a concrete demonstration that toolchain skew is
  a live operational hazard in a deterministic Wasm VM. Move removes the compiler from that surface.

A third property is worth naming even though it sits lower in the weighting. Move's linear resource type
system, enforced by a bytecode verifier before execution, makes it structurally impossible for a program
to duplicate or silently discard an asset. For a chain whose programs will hold value, that is a genuine
safety argument, and the Move Prover gives it formal-verification tooling that the CosmWasm ecosystem
does not match. If the requirement set were weighted toward asset safety rather than provability, this
comparison would come out differently, and saying so is more useful than pretending the incumbent wins
on every axis.

## Where MoveVM fails against these requirements

The failures cluster on the second-weighted dimension and on a binding owner decision, which is what
makes them decisive rather than merely costly.

**The storage interface has no ordered keyspace.** Move resolves a resource by an exact address and type
tag. There is no range scan and no prefix iteration in the core resolver interface, because the language
deliberately models storage as a set of typed resources at addresses rather than as an ordered map. The
consequence is not that the state cannot be proven. A membership proof for a resource at a known key is
straightforward on GroveDB. The consequence is that the property GroveDB exists to provide, which is
ordered secondary-index queries with range and non-membership proofs to light clients, has no expression
in the engine's storage interface at all. CosmWasm's `Storage` trait asks for exactly that ordered
iteration, which is why the substitution of GroveDB for IAVL was a swap between two authenticated ordered
stores rather than a graft, and why the spike could demonstrate bounded ordered range iteration with
read-your-writes and then prove the result against the live root.

**Restoring the missing capability means adopting a specific chain's framework, not a neutral VM.** Both
major Move lineages solved the unordered-storage problem above the VM, and they solved it differently.
The Aptos line adds table and aggregator abstractions in its framework, and the Sui line replaces the
account-and-resource model outright with an object-centric store keyed by object identifier. So "adopt
MoveVM" resolves in practice to adopting the Aptos framework or the Sui runtime, each of which carries
its own state model, its own consensus assumptions, and its own upgrade cadence. The adopt-versus-build
note's warning about governing a large external dependency applies with more force here, not less.

**Coexistence with existing Platform state is worse, and that requirement is binding.** Existing data
contracts and documents are ordered, indexed, queryable trees in GroveDB today. Wrapping them as Move
resources keyed by address and type preserves the bytes and loses the query shape, so either the
documents stay outside the program-visible state, which defeats the purpose of adding programmability,
or the index machinery gets rebuilt inside a framework layer. CosmWasm's byte-key-plus-range interface
maps onto the existing trees without that rebuild.

**The asynchronous native-capability pattern has no home.** Dimension 5 asks for masternode threshold
signatures and bridge operations to be modeled as a request now, resolved in a later block. CosmWasm
already has the shape, with submessages, the `reply` entry point, and the inter-blockchain-communication
entry points that are precisely a request-and-later-resolution protocol. Move transactions are
synchronous within a transaction by construction, and the cross-block flows in Move ecosystems are built
at framework level rather than offered by the VM. That is a real gap against gate 4 of the adoption
conditions.

**Zero-knowledge verification is further away.** Gate 3 asks whether a contract can verify a proof
on-chain inside the gas and block bound. CosmWasm's recent versions expose pairing and curve operations
as host-provided natives, which is the shape that makes an in-contract verifier plausible. Move's
cryptographic natives are supplied by the host chain's framework, so the same answer is reachable but it
is chain-specific work rather than an ecosystem-standard surface.

**The EVM-as-guest result does not transfer.** The spike showed a minimal EVM interpreter compiled to
Wasm, running as a CosmWasm contract, with its `SSTORE` landing in GroveDB and proving. Under MoveVM the
equivalent is an EVM interpreter written in Move, so an interpreted bytecode VM runs inside an
interpreted bytecode VM. That is not impossible and the Move ecosystem has attempted it, but the
demonstrated path to Ethereum compatibility that this project already has would have to be redone.

**Worst-case block bound is unresolved in both directions.** A per-instruction bytecode interpreter is
slower per operation than singlepass-compiled Wasm, while CosmWasm carries a per-contract compile cost
that is itself an adversarial surface and is mitigated by module caching. Neither engine has been
measured against the roughly half-second under-load block cadence from Phase 0 with GroveDB writes and
proof generation included. This dimension does not separate the two on present evidence and should not
be used to argue either way until it is measured.

## Verdict

The recommendation in `docs/EXECUTION_ENGINE_ADOPT_VS_BUILD.md` stands. MoveVM is a serious engine and
it beats CosmWasm on the highest-weighted dimension, which is the honest headline of this comparison and
should not be buried. It loses on the second-weighted dimension, on a binding coexistence requirement,
and on two of the five adoption gates, and it loses there for a structural reason rather than a fixable
one. Move models storage as typed resources at addresses, Dash Platform's distinguishing property is
ordered indexed provable state, and those two shapes do not meet without rebuilding one of them.

The finding that would overturn this is narrow and worth stating so it can be looked for. If ordered
secondary-index queries over program state turn out not to be a requirement for the programs Dash
actually wants, then the second-weighted dimension stops separating the engines, Move's determinism and
asset-safety advantages become decisive, and this note should be re-run.

## What a MoveVM spike would have to show, if one is commissioned

Stated so that the ASSERTED grade above can be upgraded rather than argued about.

1. A Move resource resolver backed by a GroveDB subtree, with a resource written by a real compiled Move
   module and then proven with `prove_query` against the live root. This is the direct analogue of the
   CosmWasm storage spike and it is the minimum bar.
2. An ordered-query demonstration, or a recorded finding that there is none. Either a framework-level
   table abstraction backed by GroveDB gives range and non-membership proofs, or the spike records that
   the property is unavailable, which is the finding that settles the comparison.
3. A gas-meter binding that derives Move gas from GroveDB's measured `OperationCost`, matching what the
   CosmWasm host adapter already does, so the two engines are priced on the same evidence.
4. A cross-block request-and-resolve sketch for a masternode threshold signature, since that is the gate
   with no obvious Move-side answer.

Until at least the first two run, the comparison above is a reasoned screen and not a measurement, and
it should be cited that way.
