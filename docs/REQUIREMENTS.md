# Phase 0 requirements record, execution layer for Dash Platform

Elicited from the owner 2026-07-19. This is the Phase 0 output of the clean-room design review
method. The architecture-free packet built from it is retained in the project's review record.

## The question the round tests

Dash Platform can store, index, prove, and authorize state, and it can settle value, but it cannot
compute. What execution layer should give it general programmability, given its specific stack
(Tenderdash BFT with fast deterministic finality, GroveDB provable state with secondary indexes,
declarative data contracts, native tokens, identities and groups, masternode LLMQ threshold
signatures, a credits fee unit bridged from L1, and a Rust codebase)?

## Owner decisions (authority: binding on the packet)

1. **Coexistence with existing state: required, migration acceptable.** Existing data contracts,
   documents, identities, groups, and tokens must be preserved and remain usable. Migrating or
   wrapping them into a new representation is permitted. Losing or stranding them is not.
2. **Interoperability: an OPEN QUESTION, deliberately not pre-decided.** The packet instructs each
   design to decide the degree of external-ecosystem compatibility it targets and defend it against
   alternatives. This was chosen over stating a priority precisely because the author's own design had
   already recommended one answer in chat, and pre-deciding would have made the round ratify that
   answer instead of testing it.
3. **Deployment permission model: phased.** Governance-gated program deployment at launch, opening
   to permissionless deployment later. Designs must cover both phases and state what must be true
   before the gate opens.
4. **Purpose: pure exploration.** No delivery target, no DIP obligation. Designs are framed
   greenfield-ambitious with unlimited time, budget, and engineering resources, per the playbook's
   standard framing.

## Owner-adjacent defaults applied by the synthesizer (challengeable)

- Privacy is stated abstractly. The packet requires that programs be able to interoperate with a
  privacy mechanism including verifying cryptographic proofs, without naming a mechanism. Dash's
  current on-chain privacy is CoinJoin (coordinated mixing), not a cryptographic shielded pool, and
  the packet says so factually.
- Provability is stated as a first-class requirement rather than a nice-to-have, since it is the
  platform's distinguishing property. Designs may sacrifice it for some class of state but must say
  so and justify it.
- Determinism across the validator set is called out as its own numbered requirement with an
  explicit demand for the determinism argument, because it is the failure mode that presents as
  consensus forks rather than as bugs.

## Leak check record

The packet was grepped for candidate-solution vocabulary before dispatch: wasm, webassembly, evm,
ethereum, solidity, cosmwasm, ibc, cosmos, move, risc-v, ebpf, zkvm, rollup, coprocessor, wasmer,
wasmtime, host function, gas, revm, polkavm, substrate, solana, bpf. Result: clean, no matches.
Metering vocabulary is used in place of "gas".

Two domain facts are disclosed deliberately, as a recorded judgment call rather than an oversight:

- **The consensus engine's lineage** (a Tendermint/CometBFT derivative). This anchors somewhat
  toward that ecosystem's contract and inter-chain standards, which overlaps the author's own
  recommendation. It is disclosed anyway because the application interface and block model are
  load-bearing facts that any execution-layer design must account for, and withholding them would
  produce worse designs. Per the playbook, disclosing a genuine constraint is not a leak; naming the
  author's preferred structure is. If the external designs converge on that ecosystem's answer,
  the convergence should be discounted accordingly in synthesis, and this note is the reason why.
- **The Rust codebase.** A plain implementation fact affecting feasibility.

## Process notes

- The author's design was committed to the repository before
  any external output was read. Its substance was stated in chat before the round was proposed,
  which is disclosed at the top of that file.
- The author is also the synthesizer, so per the method its design gets extra skepticism,
  and the evaluation dimensions should be fixed before any external design is read.
- Each review source is run without memory of prior sessions, so every pass reads cold.
- Wild divergence among the returned designs is a verdict on this packet, not on the models. If it
  happens, tighten the requirements here and re-run rather than synthesizing across noise.
