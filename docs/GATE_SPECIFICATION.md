# Gate specification

Dated 2026-08-30. Every gate is stated as a candidate-neutral question with thresholds declared BEFORE
the evidence arrives. A threshold written afterwards describes the evidence rather than testing it.

Each gate carries an INCONCLUSIVE outcome as well as pass and fail, because "mechanism exists but has
not been demonstrated" is a real and common state, and collapsing it into either pass or fail is how a
gate stops discriminating.

No gate may be passed on ASSERTED evidence.

---

## G1. Determinism

**Question.** For the same input, does the candidate produce identical results on every validator, and
is each divergence source closed by a mechanism whose ENFORCEMENT is demonstrated rather than
documented?

**Pass.** Every source in the divergence catalog has a named mechanism, and each mechanism is shown in
force by execution-produced evidence across at least two processor architectures and every build
configuration that can ship. Execution-affecting build choices are pinnable and pinned.

**Fail.** Any divergence source with no mechanism. Or any mechanism whose enforcement cannot be
demonstrated. Or any execution-affecting behavior selectable at build time outside consensus control.

**Inconclusive.** Mechanism present and plausible, evidence not yet produced.

**Evidence required.** Source-resolved reading against the exact lockfile and feature matrix, plus
differential execution producing bit-identical results across architectures and backends.

**Reversal.** Any engine or toolchain version change. Any addition to the divergence catalog. A
previously passing gate reverts to inconclusive on either.

**Current state.** INCONCLUSIVE for the leading candidate, and tied to an expired dependency line, so
it must be rerun after any port. Several controls are resolved. Floating point is permitted rather than
rejected, one named control was found to enforce nothing, and the compiler is selected by build feature.

---

## G2. Worst-case block bound

**Question.** Under adversarial load, does execution plus state writes plus proof generation complete
within the consensus block cadence, with all work bounded before it is performed?

**Pass.** Measured worst case within the cadence under adversarial fixtures, including proof generation
and any compilation performed at admission, with every path bounded in advance.

**Fail.** Any path performs unbounded work before its bound is enforced. Charging for work already done
is accounting, not admission control.

**Inconclusive.** All paths bounded, worst case not yet measured under adversarial load.

**Evidence required.** Execution-produced measurement under adversarial fixtures, not typical load.

**Reversal.** A change to the cadence target, the workload hypothesis, or the hardware profile.

**Current state.** FAIL. The range-scan path performs unbounded work before its charge is enforced.
CORRECTED 2026-08-30, this previously said it was the only gate at fail. Gate 5 also fails.

---

## G3. Shielded and zero-knowledge compatibility

**Question.** Can a program gate an action on a cryptographic proof, verified within the gas and block
bound, without revealing the witness?

**Pass.** Execution-produced verification of a proof inside the budget, whether through a host-provided
primitive or in-program.

**Fail.** No supported path verifies within the budget.

**Inconclusive.** Reachable in principle through a host primitive that does not yet exist.

**Evidence required.** A running verification with measured cost against the block budget.

**Reversal.** A change of proof system or of the platform's shielded subsystem.

**Current state.** INCONCLUSIVE, and screened 2026-08-30 in `docs/GATE3_ZK_SCREEN.md`. The mechanism is
chosen, a bounded metered `verify_proof` host call for governance-registered systems and circuits, so
what is missing is a running verification with a measured cost rather than a design.

Two notes from that screen. This gate does NOT discriminate between candidates, because every engine
reaches proof verification through a host primitive and the cost is a property of the host and the proof
system rather than of the engine. And its evidence is largely engine-independent, so it can be produced
once and carried, with only the metering binding re-confirmed per engine. That makes it the cheapest of
the five to close and the only one whose evidence transfers.

---

## G4. Asynchronous native capabilities

**Question.** Can an operation that cannot complete inside one block be expressed as a request now and
a resolution in a later block, preserving atomicity, funding, and rights?

**Pass.** Demonstrated request and later-block resolution for a masternode threshold signature or an
equivalent, with the reserved funding and the rights model intact across the boundary.

**Fail.** The candidate's execution model requires synchronous completion for such operations.

**Inconclusive.** Expressible in the model, not demonstrated.

**Evidence required.** Execution-produced, spanning at least two blocks.

**Reversal.** A change to the native capability set.

**Current state.** INCONCLUSIVE.

---

## G5. Version and build governance

**Question.** Can the exact executing artifact be pinned, reproduced independently, and upgraded under
consensus governance, with adequate support runway?

**Pass.** Reproducible build, pinned lockfile AND feature matrix, published artifact identity that
validators can check against each other, a defined upgrade path, and a support line meeting the runway
policy.

**Fail.** Any execution-affecting choice selectable outside consensus. Or a support line already
expired at the point of integration.

**Inconclusive.** Policy defined but not enforced.

**Evidence required.** Two independent builders producing identical artifact hashes from the pinned
inputs.

**Reversal.** Support line expiry, a new upstream release, or a change to the runway policy.

**Current state.** FAIL as things stand, on two counts. The evidence line's security support ended
2025-04-30, and the compiler is selectable by build feature rather than fixed by consensus. CORRECTED
2026-08-30, the runway policy is no longer open. P4 set it at two Dash upgrade cycles, about 180 days,
which is what makes the current line fail rather than merely lack a threshold.

---

## Summary of current standing

| Gate | State | Blocking issue |
| --- | --- | --- |
| G1 Determinism | Inconclusive | Controls partly resolved, tied to an expired line, no cross-architecture evidence |
| G2 Block bound | **Fail** | Unbounded work before charge on the range-scan path |
| G3 Shielded and zero-knowledge | Inconclusive | Mechanism chosen and screened, no measured verification cost yet. Does not discriminate between candidates |
| G4 Async capabilities | Inconclusive | Not demonstrated |
| G5 Version governance | **Fail** | Expired support line, build-selectable compiler, runway policy undecided |

Two gates fail and three are inconclusive. No gate passes. That is the accurate current standing for
the leading candidate, and it is why the charter's position is leading candidate rather than anything
stronger.
