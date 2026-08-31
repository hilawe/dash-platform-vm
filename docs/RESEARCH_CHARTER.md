# Research charter

Dated 2026-08-30. This charter governs the active research. It supersedes any earlier framing in which
an engine had been selected.

## What this project is

Requirements-led comparative research into what execution layer would give Dash Platform general
programmability. It is exploratory. There is no delivery target, no Dash Improvement Proposal
obligation, and no official standing.

## What is settled and what is not

`DESIGN.md` v12 is frozen as the reviewed HISTORICAL architecture. It records what was concluded at the
time and it is not a statement of current conclusions. The freeze does not prevent current-facing
requirements, gates, or candidate rankings from changing, and it must not be cited as though it does.

The clean-room round established a broad architecture FAMILY, a deterministic metered sandboxed
WebAssembly runtime reached through host functions. It did not establish that any particular
implementation of that family is the right one. It therefore cannot be cited as having selected a
candidate.

## The standing position on candidates

CosmWasm is the LEADING CANDIDATE under evaluation. It may lose.

That sentence is the charter's operative commitment. A candidate leads because it has been examined
most, not because it has won, and the gates exist to give it a way to lose. Two consequences follow and
both are binding on how this research is written.

- No document may describe an engine as chosen, adopted, forced by the requirements, or settled, until
  the gates have produced comparable evidence across the candidate set.
- Every gate must be stated as a question a candidate can FAIL. A gate phrased so that its expected
  answer confirms the leader is not a gate, and rewriting such a phrasing is a correction, not a
  preference.

## The candidate set

Four arms, kept deliberately small. Each exists to test something the others cannot.

| Arm | What it is | Why it is in the set |
| --- | --- | --- |
| CosmWasm on a supported line | The current leader, ported off the expired 1.5.x line | Most evidence exists here, and the storage fit is demonstrated |
| Strict integer-only interpreted Wasm | A reference implementation that forbids floats outright and interprets rather than compiles | Isolates how much of the determinism difficulty is inherent to WebAssembly versus specific to a compiling runtime |
| Move, or a concrete Move framework | **LIVE, and it clears the re-run screen** | The storage objection that removed it is gone under the P1 split, confirmed by the re-run in `docs/ENGINE_SCREEN.md`. It carries the stronger determinism story of the two mature candidates. Its remaining obstacles are coexistence and integration rather than disqualifications: adopting the Aptos or Sui framework brings a state model with it, there is no request-and-resolve pattern for asynchronous capabilities, and the Ethereum-guest result does not transfer. It has NO evidence produced against it, which is now the main thing separating it from the leader |
| Narrow native modules, no general VM | The baseline | The honest alternative to adding programmability at all. Without it, every comparison is between ways of doing the thing rather than whether to do it |

The baseline arm is not a formality. If the applications Platform actually wants can be served by a
small number of native modules, then the entire general-execution direction is the wrong answer however
well a candidate performs on the gates.

## Evidence discipline

Unchanged from the project's existing convention and restated here because it governs the gates.

- **EXECUTION-PRODUCED** means demonstrated by running code against the real system at a pinned
  revision.
- **REPOSITORY-RESOLVED** means read from checked-out source of the system in question.
- **ASSERTED** means stated from knowledge of a system not checked out and not run.

A gate cannot be passed on ASSERTED evidence. Documentation, field names, and defaults are not evidence
that a control is in force, which the determinism round demonstrated concretely by finding a named
control that enforced nothing.

## How this package is used

Four documents, each with one job.

- This charter states the position and the candidate set.
- `docs/REQUIREMENTS_REGISTER.md` separates what is binding from what is chosen, guessed, or preferred.
- `docs/GATE_SPECIFICATION.md` gives every gate a candidate-neutral question with pass, fail,
  inconclusive, evidence, and reversal conditions declared in advance.
- `docs/DECISION_REGISTER.md` tracks each decision's state so that a superseded decision cannot quietly
  keep operating.
- `docs/ENGINE_SCREEN.md` is the current screen, re-run after the P1 decision changed its first filter.

Predeclaring the thresholds is the point. A threshold written after the evidence arrives is a
description of the evidence.
