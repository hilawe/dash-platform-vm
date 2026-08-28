# Gate 1, determinism, first findings

Dated 2026-08-27. Gate 1 is the first of the five conditions in
`docs/EXECUTION_ENGINE_ADOPT_VS_BUILD.md` and the one the build plan puts first, because a determinism
failure across the validator set presents as a consensus fork rather than as a bug. This note records
the first substantive step, which resolved several determinism controls from source rather than
assuming them.

## A reframing, recorded rather than done quietly

The gate was originally written as "confirm CosmWasm's determinism controls satisfy the named
divergence sources". That is a confirmatory framing, and a question phrased to be confirmed usually is.
It is restated here as an engine-general question. What must hold for ANY candidate engine to execute
identically on every validator, and which candidates hold it. The criteria below apply to any engine
under consideration, and CosmWasm is simply the first one with enough evidence to answer them.

## What was checked, and how

Read directly from the crate sources at the versions the prototype pins, which raises these claims from
ASSERTED to REPOSITORY-RESOLVED. The versions are cosmwasm-vm 1.5.11, wasmer 4.2.2,
wasmer-compiler-singlepass 4.2.2, and wasmparser 0.95 as re-exported by wasmer.

| Divergence source | Control found | Grade |
| --- | --- | --- |
| SIMD | Rejected twice, at validation (`parsed_wasm.rs:34`) and in the Gatekeeper middleware (`gatekeeper.rs:64`) | REPOSITORY-RESOLVED |
| Threads and atomics | Rejected at both layers (`parsed_wasm.rs:36`, `gatekeeper.rs:66`) | REPOSITORY-RESOLVED |
| Bulk memory operations | Rejected at both layers (`parsed_wasm.rs:33`, `gatekeeper.rs:63`) | REPOSITORY-RESOLVED |
| Reference types | Rejected at both layers (`parsed_wasm.rs:32`, `gatekeeper.rs:64`) | REPOSITORY-RESOLVED |
| Exception handling | Rejected at both layers (`parsed_wasm.rs:38`, `gatekeeper.rs:65`) | REPOSITORY-RESOLVED |
| Floating point | NOT rejected. See below. | REPOSITORY-RESOLVED |
| NaN bit patterns | Canonicalized by the singlepass compiler, on by default (`singlepass/src/config.rs:21`) | REPOSITORY-RESOLVED |
| Metering boundary timing | Gas points injected as a compiler middleware (`engine.rs:65`) | REPOSITORY-RESOLVED |
| Compiler choice | Singlepass, fixed rather than configurable (`engine.rs:61`) | REPOSITORY-RESOLVED |
| Compiler and VM version pinning across validators | Not a code property. Remains a governance question, gate 5. | OPEN |
| Iteration order over program state | Demonstrated in the storage spikes, ordered and proven | EXECUTION-PRODUCED |

## The correction, stated plainly

Two documents in this repository stated that CosmWasm achieves determinism partly through the ABSENCE
of floating point, by rejecting float opcodes during Wasm validation. That is wrong for cosmwasm-vm
1.5.11, and the corrected account is as follows.

Floating-point operations are explicitly ALLOWED. The Gatekeeper middleware, which is the component
that filters non-deterministic operations, ships a default configuration with `allow_floats: true`
(`wasm_backend/gatekeeper.rs:60`). The crate carries a test contract named `floaty.wasm` and a test
named `contract_with_floats_passes_check` asserting that a contract containing floats compiles, plus a
test named `float_instrs_are_deterministic` in `calls.rs`. So permitting floats is a deliberate design
decision by the CosmWasm authors, not an oversight.

Determinism for floats is instead obtained downstream, in the compiler. Wasm floating-point arithmetic
is fully specified except for NaN payload bits, and the singlepass compiler canonicalizes NaNs with
`enable_nan_canonicalization` defaulting to true, with architecture support present for both x86-64 and
aarch64. The engine builder fixes singlepass as the compiler and pushes the Gatekeeper and the metering
middleware onto it (`wasm_backend/engine.rs:61` through `65`), so the canonicalization is in force on
the path a contract actually takes.

Two smaller observations came out of the same reading, both worth carrying into the design round.

The `deterministic_only: true` flag that cosmwasm-vm sets in its validator features
(`parsed_wasm.rs:30`) appears to be inert. In wasmparser 0.95 the corresponding check is
`check_non_deterministic_enabled`, whose body is guarded by `cfg!(feature = "deterministic")`, and
wasmer declares its wasmparser dependency with `default-features = false` without selecting that
feature. The flag therefore reads as a determinism control while enforcing nothing, which is a
different failure shape from a missing control and a harder one to notice.

The Gatekeeper's own documentation comment describes it as a middleware that "ensures only
deterministic operations are used (i.e. no floats)", directly above a default configuration that allows
floats. Anyone reasoning about CosmWasm determinism from its documentation rather than its code will
reach the wrong conclusion, which is presumably how the error in this repository arose.

## What this does to the engine comparison

It widens MoveVM's advantage on this dimension rather than narrowing it.
`docs/EXECUTION_ENGINE_VM_COMPARISON.md` argued that Move's freedom from floating point is structural,
since the language has no float type, while CosmWasm reached the same place through a validation pass
that has to be correct and stay correct. The corrected picture is that CosmWasm does not reach that
place at all. Floats execute, and determinism rests on a compiler flag that defaults to on and can be
turned off by a caller constructing its own compiler. That is a weaker guarantee than a validation
rejection would have been, and considerably weaker than not having the type.

This does not overturn the recommendation, because provability remains the criterion that separates the
engines and Move still loses there. It does mean the determinism argument for CosmWasm has to be made
in terms of compiler configuration and version pinning rather than in terms of absent operations, and
that pinning is exactly gate 5.

## What remains open

The reading above settles which controls exist. It does not settle whether they are sufficient, and
that is the question the gate actually asks. Three items remain, and the first is the one the build
plan calls for.

1. The clean-room design round on determinism, run against the engine-general framing at the top of
   this note rather than as a CosmWasm confirmation. Independent sources, no sight of this note,
   committed before comparison.
2. Whether NaN canonicalization plus fixed singlepass is a sufficient float story for a consensus
   system, or whether program code should be rejected for containing floats at deployment regardless of
   what the VM permits. This is a Dash-side policy question that CosmWasm leaves open.
3. Non-determinism sources outside the Wasm module itself, meaning host-function return values, gas
   exhaustion timing, and memory growth limits. The limiting tunables in
   `wasm_backend/limiting_tunables.rs` were not read for this note and should be.
