# Test data

`hackatom.wasm` is the canonical CosmWasm test contract, copied verbatim from the cosmwasm-vm 1.5.11
crate's own test data (`testdata/hackatom.wasm`). It is part of the CosmWasm project and is licensed
Apache-2.0. It is committed here so the end-to-end spike (`src/bin/e2e.rs`) is self-contained and
reproducible. The binary instantiates and queries this contract through the real cosmwasm-vm over a
GroveDB-backed host store. It is third-party test data, not project source.

`evm_guest.wasm` is a minimal EVM interpreter written for this project and compiled as a CosmWasm contract (source in
`metering-prototype/evm-guest-contract/`, built for wasm32-unknown-unknown). It is used by the
`--bin evm` spike to run real EVM bytecode as a guest over the GroveDB-backed CosmWasm storage. It is
project source, rebuildable from the evm-guest-contract crate.
