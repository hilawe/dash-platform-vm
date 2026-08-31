# EVM as a guest over GroveDB, a running demonstration

Dated 2026-08-11. The community brief and the plain explainer describe EVM compatibility on Dash as
reachable through the "EVM as a guest" shape, an EVM interpreter running as an ordinary deployed program
over GroveDB-backed storage, rather than the EVM as the base. This note records a running spike that
turns that shape from analysis into execution-produced evidence. It does not change any recommendation,
and it is a small demonstration, not a production EVM.

## What was built and run

A minimal EVM interpreter was written as a real CosmWasm contract
(`metering-prototype/evm-guest-contract/`,
compiled to `wasm32-unknown-unknown`). It is a 256-bit stack machine supporting the opcodes a storage
demonstration needs (PUSH1 through PUSH32, ADD, POP, SLOAD, SSTORE, STOP), all integer-only. Its SSTORE
and SLOAD go to the contract's own CosmWasm storage. The spike
`metering-prototype/cosmwasm-host/src/bin/evm.rs` loads that contract into the real cosmwasm-vm (wasmer)
over a GroveDB-backed storage backend and runs it. Raw output is
`metering-prototype/results/cosmwasm_evm_guest_output.txt`.

What ran, EXECUTION-PRODUCED:

- The EVM-interpreter contract compiled and loaded into the VM.
- It executed the EVM bytecode `602a60015500` (PUSH1 0x2a, PUSH1 0x01, SSTORE, STOP), which stores the
  value 42 at storage slot 1.
- The SSTORE landed in GroveDB through the CosmWasm storage backend. A query of slot 1 through the VM
  read the 32-byte word back as `0x..2a` (42).
- The EVM guest's storage is provable in GroveDB: a `prove_query` over the contract subtree verified
  against the live root, showing the 32-byte slot key `0x..01` mapped to the 32-byte word `0x..2a`.

So real EVM bytecode executed inside a Wasm guest under the VM, its storage write landed in GroveDB, and
the result is provable in Platform's native proof format. That is the exact property that makes hosting
an EVM on Dash compatible with Platform's provability, and it is now demonstrated rather than argued.

## Claim width

This is a minimal EVM (a handful of opcodes), not a full one, and it runs a tiny program. It
demonstrates
the load-bearing claim, that an EVM guest's state can be GroveDB-backed and provable, not a complete or
performant EVM. A production EVM guest would need the full opcode set, a real gas mapping, Keccak and
the
secp256k1 precompile (secp256k1 is native to Dash), and the JSON-RPC and proof-translation layer for
Ethereum tooling that the community brief already flags. Those are the known engineering scope of EVM
compatibility, unchanged by this spike. CORRECTED 2026-08-30, the direction is also unchanged but it is
not an adoption decision. CosmWasm is the leading candidate under evaluation
first and treat EVM compatibility as its own later clean-room design question, now with a running proof
that the
guest shape works over GroveDB.

## Reproducing the build

The contract compiles with Rust 1.86 for `wasm32-unknown-unknown` (edition 2024 is needed by a
transitive dependency, and Rust 1.86 predates the wasm32 default that would emit bulk-memory operations,
which the cosmwasm-vm 1.5 validator rejects). The contract's `.cargo/config.toml` allows undefined
symbols (the host imports) and disables the newer Wasm features. The host `evm` binary embeds the
compiled `testdata/evm_guest.wasm` and runs under the usual `rust:bookworm` toolchain.
