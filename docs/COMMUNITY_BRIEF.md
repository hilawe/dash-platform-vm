# Adding a general execution layer to Dash Platform

A community brief, dated 2026-08-11. This note summarizes an exploratory research effort on what it
would take to give Dash Platform general programmability, and it addresses one question that comes up
immediately whenever on-chain programs are discussed, which is how this relates to Ethereum Virtual
Machine (EVM) compatibility. It is written for the Dash community and for Core and Platform
contributors.
It is a concept note, not a proposal to build, and it does not itself carry the weight of a
specification.
The design record it summarizes lives in `DESIGN.md`, the plain-language companion in
`docs/PLAIN_EXPLAINER.md`, and the fuller research summary in `SUMMARY.md`.

## What the research asked and where it stands

Dash Platform can store data, index it, prove a stored fact to a light client, authorize changes by
signature, and settle value. What it cannot do today is run a user-supplied program that takes inputs,
computes, and writes results back in a way every validator is guaranteed to reproduce. That missing
ability is what forces most applications to push their real logic onto trusted off-chain servers. The
research asked what execution layer would add that ability given Platform's specific stack, namely
Tenderdash consensus, GroveDB authenticated state with light-client proofs, data contracts, native
tokens, identities and groups, masternode threshold signatures, and the credit fee unit.

The effort ran a clean-room design process. The requirements were written down with no
preferred solution named, then several independent designs were produced from that packet, then the
synthesized design was attacked over twelve adversarial review rounds by reviewers drawn from
different independent sources, with each round's verified findings folded into the next version. Between the
early and late rounds, a measurement phase checked the design's assumptions against the real platform at
a frozen revision. The review loop closed when a fresh full pass found nothing left to fix.

Two boundaries on that status matter for a community reader. The design is review-complete on paper,
which means independent reviewers found nothing left to correct in the text, and it is measured where a
measurement was possible, because a metering prototype has since measured the resource dials the design
left open against the real storage engine. It is not implemented. No execution layer exists in the node,
so nothing here has been tested by running it in production, and the strongest claim available is a
hardened design with its cost model measured, not a working system.

## The architecture in one paragraph

Five independent designs converged, with no coordination, on the same core. The execution layer is a
deterministic WebAssembly (Wasm) runtime embedded in the Layer 2 state transition function. Programs
reach the platform only through host functions, so tokens, identities, groups, and documents stay native
and callable rather than reimplemented. Program state lives in the existing authenticated store, so the
light-client provability that distinguishes Platform survives. Execution is integer-only and metered in
the existing credit unit against per-block bounds. Deployment is governance-gated first and
permissionless later, and no new mandatory trusted party is introduced. Every one of those choices was
reached independently by designs from different sources, which is the main evidence that the
architecture is forced by the requirements rather than being one designer's preference.

## The EVM question, in the right order

The most common first question is whether this makes Dash EVM-compatible. The accurate answer has an
order to it, and the order is the point.

Platform is not EVM-compatible at its foundation, and that was a deliberate, unanimous finding. Every
one
of the five independent designs rejected the EVM as the base execution model for the same reason. The
EVM's account-and-storage model, in which each contract owns a separate storage trie keyed by 256-bit
slots and addressed through Keccak-256 hashing, does not fit GroveDB. Adopting it as the foundation
would
either break Platform's uniform light-client provability, which is built on a different tree and a
different hash, or force a wrapper around every native feature the platform already provides. The
rejection is about making the EVM the foundation, and it is firm.

That rejection does not close the door on EVM compatibility arriving later as a hosted layer. The design
that was chosen, a metered deterministic Wasm sandbox whose only interface is host functions, is a good
substrate for running an EVM interpreter as an ordinary deployed program. This inverts the relationship.
Instead of the platform conforming to the EVM, an EVM would run inside the platform's own sandbox, its
emulated contract storage held in GroveDB, its execution metered by the same credit unit. That guest
shape has since been demonstrated with a running spike (`docs/EVM_GUEST_SPIKE.md`), where a minimal EVM
interpreter compiled as a Wasm contract executed real EVM bytecode whose storage write landed in GroveDB
and was proven. Several properties make the route reachable rather than hand-waving:

- Determinism is already satisfied, because the EVM is integer-only and deterministic, which is the
  constraint the base design enforces regardless.
- The signature primitive is native, because Dash is a secp256k1 chain, so the one precompile that
  everything depends on, ecrecover, maps onto a host function rather than a reimplementation.
- No consensus change is required, because a guest interpreter is Wasm plus host calls, so it touches
  none of the trust model the review rounds hardened.

Three things would not come for free, and a community discussion should name them plainly:

- Ethereum-style state proofs would not carry over. A hosted EVM would produce GroveDB proofs of its
  emulated state, which serve Platform's own light clients, but Ethereum tooling that expects
  Merkle-Patricia Trie (MPT) proofs from an `eth_getProof` call would not verify them without a
  proof-translation layer.
- Keccak-256 and the EVM gas schedule are not native costs. Keccak is pervasive in EVM addressing and
  becomes a metered host call, deterministic but likely priced above native operations, and the EVM's
  own gas accounting sits as a second metering layer over the credit metering.
- Wallet and JSON-RPC (JavaScript Object Notation Remote Procedure Call) tooling, such as the endpoints
  a browser wallet expects, would need a compatibility gateway that translates Ethereum-style
  transactions and calls into Platform state transitions. That gateway lives outside the virtual
  machine, at the access layer, which is the correct place for it.

There is one interaction with the hardest part of the base design worth flagging. The base design's
terminal-work meter governs how state is funded and eventually reclaimed. EVM contracts grow storage
without a lease and historically could self-destruct, a behavior mostly removed by Ethereum's EIP-6780.
A hosted EVM would need a disposition class in that meter for EVM-contract storage. That is an added
piece of design work, not a barrier.

A lighter option is worth naming so the community can weigh it against full bytecode compatibility.
Solidity, or a useful subset, can be compiled to Wasm and run directly on the base layer. That captures
much of the developer familiarity of the Ethereum toolchain, but it is language familiarity, not
bytecode
compatibility, so existing deployed contracts and the tooling that speaks to them would not run
unchanged.

The recommendation on EVM compatibility, stated as a recommendation and not a plan, is to treat it as
its
own clean-room design question if and when it is pursued, and to validate it with a thin end-to-end
slice
first, meaning one trivial contract deployed, called, and proven, before committing to the
storage-mapping and proof-translation choices, which are the expensive and hard-to-reverse parts. It is
not a bolt-on to the base layer.

## Adopting an existing Wasm execution layer, and the CosmWasm option

A separate and more promising option than EVM compatibility is to adopt an existing, hardened
WebAssembly
execution layer rather than build a bespoke one, and the specific candidate worth evaluating is
CosmWasm, the smart-contract framework used across the Cosmos ecosystem. Two facts make it more relevant
here than the EVM. First, the consensus lineage aligns, because Dash Platform runs on Tenderdash, which
is a fork of Tendermint (now CometBFT), the same Byzantine-fault-tolerant engine the Cosmos ecosystem is
built on. Second, and more important, CosmWasm is the same architecture class the clean-room designs
converged on independently. It is a deterministic, integer-only, gas-metered, sandboxed Wasm runtime
whose contracts are written in Rust and reach the chain only through a defined host-function interface.
In other words, CosmWasm is a mature, audited implementation of the architecture this research specified
from first principles.

That reframes the question. It is not CosmWasm versus the design, because they are largely the same
design. The question is whether to ADOPT a hardened engine and its contract ecosystem instead of writing
a runtime, which could shorten the path to a working system and inherit years of security review and
tooling. The deciding factor is the same provability constraint that ruled out the EVM, but the answer
is far more likely to be favorable. CosmWasm's storage interface is a generic key-value abstraction
(contracts read and write bytes by key and range over them), not the EVM's rigid fixed-slot model, and a
generic key-value interface is much more plausibly backed by GroveDB while keeping Platform's
light-client proofs intact. Whether that backing preserves provability is the first question to answer,
and unlike the EVM case it is answerable.

That first question has now been checked and comes back favorable, and confirmed by a running spike
(`docs/COSMWASM_STORAGE_ASSESSMENT.md`), not only by an interface read. Every storage primitive CosmWasm
requires (get, set, remove,
ordered range iteration, streaming iterators, lexicographic byte-key ordering, per-contract isolation,
and transactions) has a direct GroveDB counterpart verified in the GroveDB source, and GroveDB's native
proofs cover the resulting contract state without a translation layer. The structural reason is that
CosmWasm was designed against an authenticated ordered key-value store to begin with (Cosmos chains back
it with IAVL, itself a Merkle tree), so GroveDB is a substitution in kind rather than a foreign graft.
The check is interface-level, so it justifies a spike rather than concluding adoption, and it leaves a
few items (transactional iteration behavior, the gas-to-cost adapter, and the exact CosmWasm backend
trait in the targeted version) for a running end-to-end slice to settle.

Several other items would need checking before treating adoption as the plan, and none is a wall:

- Native bindings. CosmWasm's ecosystem leans on Cosmos SDK modules (bank, staking, inter-blockchain
  communication). Dash's native features (tokens, identities, groups, the credit unit) would replace
  those as host functions, which is the host-function work the base design already scopes.
- Gas and credits. CosmWasm carries its own gas model, which would need reconciling with Dash's credit
  fee unit and per-block bounds.
- The cleanup meter still applies. CosmWasm has no equivalent of Dash's storage-lease and terminal-work
  accounting, so the hardest part of this design carries over whichever runtime is chosen.
- A likely favorable detail to confirm. The core execution engine is a Rust crate, with the Go binding
  most Cosmos chains use wrapping it, so a Rust node such as Platform's Drive could integrate the engine
  directly. This should be verified against the current CosmWasm sources rather than assumed.

The storage question that gated this is now answered with running evidence, and the whole path from a
compiled contract down to a proof is demonstrated (the storage backend, the host-side backend with a
cost-to-gas adapter, an end-to-end compiled contract, and the module-binding read and write paths),
so the comparison has moved from open to decided. The decision synthesis
(`docs/EXECUTION_ENGINE_ADOPT_VS_BUILD.md`) recommends adopting CosmWasm backed by GroveDB rather than
building a bespoke runtime, and treating EVM compatibility as a separate later layer on top. It names
five conditions to verify before committing, beginning with determinism across validators (the failure
that would split the record, so it goes first), then the worst-case block bound including proof
generation, on-chain zero-knowledge verification, asynchronous native capabilities such as the
masternode threshold signature, and pinning the engine version identically across validators. Adoption
is likely the shorter path, because most of the runtime already exists and is hardened and the remaining
work is node integration rather than inventing a VM, and the effort should draw on contributors with
Cosmos experience.

## On whether this becomes a DIP

A natural question is whether this work should become a Dash Improvement Proposal (DIP). A DIP is a
normative specification that contributors can implement to consensus, with exact encodings, state
transitions, validation rules, and activation. This work is one layer earlier than that. It is a design
record with a measured cost model, which is the input a DIP is built from, not the DIP itself. Filing a
normative standard for a system that has no reference implementation, and that deliberately leaves a few
choices open, would be premature.

The lighter and more useful first step is a community discussion that asks whether an execution-layer
direction is worth a DIP at all, backed by this brief, the design record, and the prototype
measurements. The single thing that step tests, which a DIP cannot test in advance, is whether the
people
who would implement it see it as a direction worth carrying. If that interest exists, the design record
and the prototype become the backing evidence for a DIP that someone with implementation intent drives,
ideally with a reference implementation or at least a thin end-to-end slice in hand.

## Where to read more, and how to engage

The depth behind this brief is in the repository. `SUMMARY.md` gives the research summary and
recommendations, `DESIGN.md` is the design record with its full revision history,
`docs/PLAIN_EXPLAINER.md`
is a non-engineer walkthrough, `docs/METERING_RESULTS.md` holds the prototype measurements, and
`docs/EXECUTION_ENGINE_ADOPT_VS_BUILD.md` is the adopt-versus-build decision synthesis.

Concrete next steps, for whoever picks this up:

- Read `SUMMARY.md` first for the research summary, then `DESIGN.md` for the design decisions and their
  reasoning.
-  Decide whether to open a community discussion thread proposing an execution-layer direction, using
  this
  brief as the opening note.
-  For Core and Platform maintainers, signal whether an execution-layer direction has appetite, since
  that
  signal, not a specification, is what gates the next step.
- If interest exists, scope a thin end-to-end implementation slice as the evidence a future DIP would
  rest on, and treat EVM compatibility as a separate later question with its own design round.
- Adopt CosmWasm as the execution engine (the direction the synthesis recommends,
  `docs/EXECUTION_ENGINE_ADOPT_VS_BUILD.md`) rather than build a bespoke runtime, and draw on
  contributors with Cosmos experience. The gating storage question is now answered by running spikes
  (`docs/COSMWASM_STORAGE_ASSESSMENT.md`, favorable), covering both the contract-facing storage over a
  GroveDB subtree with provability confirmed, and the host-side cosmwasm-vm backend with a
  GroveDB-cost-to-gas adapter and iterator management. A compiled contract has now been run
  through the VM end to end over the GroveDB backend with its state proven, so the remaining step there
  is
  the Dash-native module bindings (bank/staking/IBC replaced by host functions to Dash tokens,
  identities,
  and groups), which is no longer a storage question and is now started, with both its read path (the
  Querier
  answering a contract's queries against Dash token state in GroveDB) and its write path (a real
  contract
  emitting a transfer that a stand-in router applies to GroveDB, provably) already demonstrated
  (`docs/COSMWASM_MODULE_BINDINGS.md`); what remains there is the production node router and catalog
  work.
