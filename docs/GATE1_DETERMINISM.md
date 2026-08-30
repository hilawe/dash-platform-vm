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
| NaN bit patterns | Canonicalized. On by default in singlepass (`config.rs:21`) AND explicitly enabled by cosmwasm-vm (`engine.rs`, `compiler.canonicalize_nans(true)`) | REPOSITORY-RESOLVED |
| Metering boundary timing | Gas points injected as a compiler middleware (`engine.rs:65`) | REPOSITORY-RESOLVED |
| Compiler choice | NOT fixed. Singlepass by default, Cranelift when the `cranelift` cargo feature is set (`engine.rs`, `#[cfg(feature = "cranelift")]`). See below. | REPOSITORY-RESOLVED |
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
aarch64. The engine builder pushes the Gatekeeper and the metering middleware onto the compiler and calls
`canonicalize_nans(true)` explicitly rather than relying on the default, so the canonicalization is in
force on the path a contract actually takes.

The compiler is selected at BUILD TIME, not fixed. An earlier draft of this note said singlepass was
fixed rather than configurable. Re-reading `make_compiling_engine` disproved that. The function carries
a `#[cfg(feature = "cranelift")]` branch selecting Cranelift and a `#[cfg(not(feature = "cranelift"))]`
branch selecting singlepass, so which compiler a validator runs is a property of how its binary was
compiled. Two validators running the same cosmwasm-vm version with different cargo features would run
different compilers over the same contract. Both canonicalize NaNs here, so the float story survives,
but the general hazard does not depend on floats. This makes the version-pinning duty in gate 5 wider
than pinning version numbers, since it has to pin build configuration as well, and build configuration
is considerably easier to get wrong quietly.

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

## The design round, first returned instrument

The clean-room round is dispatched. Its packet asks for the divergence enumeration and the evidence
standard FIRST, committed before the reader sees any of this system's configuration, and then presents
the configuration as raw excerpts with no interpretation attached. The packet does not contain the
findings above, so a reader reaching them reaches them independently.

One instrument has returned so far, reasoning from the packet alone with no repository access. It is
one source, so it is one voice rather than corroboration in the plural sense, and the remaining
instruments have not run.

It independently reached all three findings above. It identified that the Gatekeeper default permits
floats while the adjacent documentation claims otherwise, that `deterministic_only` "may compile to no
check", and that the compiler is selected "according to a compile-time feature" and is therefore a
build-configuration divergence surface rather than a fixed property.

One caveat on weight. Part A4 of the packet asks the reader to enumerate failure shapes where a control
is present but inert, before showing any code. That primes the category, so locating the
`deterministic_only` instance is evidence of detection rather than of independently generating the
category. The float and compiler findings were not primed in the same way.

It also raised two divergence sources this note had not recorded, both from the same excerpts.

- **The metering limit is constructed as zero.** `make_compiling_engine` builds the metering middleware
  with `gas_limit = 0`. That is very likely a placeholder replaced per call at instantiation, but the
  excerpt does not show it being replaced, and the reviewer's point stands as an evidence question. If
  it were not replaced with a deterministic per-call limit, or if host work stayed materially
  unmetered, the resource bound would not hold. This needs checking against the instantiation path.
- **The memory limit is optional.** The function takes `memory_limit: Option<Size>`, and the limiting
  tunables are applied only when a limit is supplied. A host passing nothing gets no bound, which makes
  memory growth a property of the host's configuration rather than of the engine. Two validators
  configured differently would diverge on any program whose behaviour depends on allocation success.

The second item corroborates a gap this note had already flagged as unread, namely
`wasm_backend/limiting_tunables.rs`, and raises its priority.

## What remains open

The reading above settles which controls exist. It does not settle whether they are sufficient, and
that is the question the gate actually asks. Three items remain, and the first is the one the build
plan calls for.

1. The clean-room design round on determinism, run against the engine-general framing at the top of
   this note rather than as a CosmWasm confirmation. Independent sources, no sight of this note,
   committed before comparison.
2. Whether NaN canonicalization plus a pinned compiler build is a sufficient float story for a
   consensus system, or whether program code should be rejected for containing floats at deployment
   regardless of what the VM permits. This is a Dash-side policy question that CosmWasm leaves open.
3. Non-determinism sources outside the Wasm module itself, meaning host-function return values, gas
   exhaustion timing, and memory growth limits. The limiting tunables in
   `wasm_backend/limiting_tunables.rs` were not read for this note and should be.
