# An execution layer for Dash Platform

Dash Platform can store, index, prove, and authorize state, and it can settle value, but it cannot
compute. It cannot run a program that takes inputs, does arithmetic or logic, and writes the result
back in a way every participant is guaranteed to agree on. This repository asks one question, what
would it take to add that ability without giving up what makes the platform distinctive, and works
the answer out as a design backed by running code.

This is exploratory research by [@hilawe](https://github.com/hilawe). It is not a Dash roadmap item,
not a Dash Improvement Proposal, and not an official position of the Dash project. Nothing here is
built into a shipping product.

## The control package

Four short documents govern the active research. Read these before the older material, because they say
what is settled and what is not.

- **[docs/RESEARCH_CHARTER.md](docs/RESEARCH_CHARTER.md)** states the position, that CosmWasm is the
  leading candidate and may lose, and names the four candidate arms including a no-general-VM baseline.
- **[docs/REQUIREMENTS_REGISTER.md](docs/REQUIREMENTS_REGISTER.md)** separates binding platform
  properties from owner policy choices, workload guesses, and preferences.
- **[docs/GATE_SPECIFICATION.md](docs/GATE_SPECIFICATION.md)** gives each gate a candidate-neutral
  question with pass, fail, inconclusive, evidence, and reversal declared in advance. Two gates
  currently FAIL and three are inconclusive. None passes.
- **[docs/DECISION_REGISTER.md](docs/DECISION_REGISTER.md)** tracks each decision's state, and lists
  what has been superseded and by what.

## Start here

- **[docs/PLAIN_EXPLAINER.md](docs/PLAIN_EXPLAINER.md)** is the plain-language walkthrough. It assumes
  no blockchain engineering background and covers the whole arc, the gap, the method, the hard
  problem, and the recommendation.
- **[docs/COMMUNITY_BRIEF.md](docs/COMMUNITY_BRIEF.md)** is the short version for a reader who already
  knows the ecosystem.
- **[SUMMARY.md](SUMMARY.md)** is the research summary with the findings and recommendations.
- **[docs/ENGINE_SCREEN.md](docs/ENGINE_SCREEN.md)** is the CURRENT screen, re-run after the P1
  decision changed its first filter. CosmWasm and MoveVM both clear it.
- **[docs/B7_FEASIBILITY.md](docs/B7_FEASIBILITY.md)** checks whether programs can write native indexed
  collections at all, which is the assumption the current requirement split rests on. They can, and the
  real cost is the authority model rather than storage.
- **[docs/GATE3_ZK_SCREEN.md](docs/GATE3_ZK_SCREEN.md)** screens the zero-knowledge engines and finds
  that gate 3 ranks nobody, since every candidate verifies proofs through a host primitive. The engines
  that PROVE execution answer a different question and are now a control arm rather than a candidate.
- **[docs/EXECUTION_ENGINE_VM_COMPARISON.md](docs/EXECUTION_ENGINE_VM_COMPARISON.md)** is the earlier
  head-to-head, retained for its per-engine reasoning. Its screen section is superseded.
- **[docs/EXECUTION_ENGINE_ADOPT_VS_BUILD.md](docs/EXECUTION_ENGINE_ADOPT_VS_BUILD.md)** is the
  decision document, which weighs adopting an existing engine against building one and states its
  recommendation together with the five conditions that must close before any choice is committed.
- **[docs/EVALUATION_DIMENSIONS.md](docs/EVALUATION_DIMENSIONS.md)** is the scoring rubric, fixed
  before the designs were compared.
- **[docs/FORK_OR_INTEGRATE.md](docs/FORK_OR_INTEGRATE.md)** asks whether an engine should be forked
  the way Tendermint became Tenderdash, or integrated through the extension points it already provides.
- **[DESIGN.md](DESIGN.md)** is the full architecture, version 12, frozen as the reviewed HISTORICAL
  record. It is not a statement of current conclusions, which remain open.

## How engines are evaluated

No engine is assumed here. Candidates are screened against Platform's own requirements, and each filter
traces to a numbered requirement rather than to a capability the storage engine happens to have. The
current screen is [docs/ENGINE_SCREEN.md](docs/ENGINE_SCREEN.md), re-run on 2026-08-30.

Four filters. Program state must be provable at a known key. The engine must offer a principled way for
the host to expose native operations, since that is how programs reach Dash tokens, identities, groups,
and the native indexed collections that carry completeness proofs. The engine must own its determinism
story rather than leaving it to the integrator. And it must coexist with existing Platform state instead
of forcing every native feature through a wrapper.

The second and fourth filters are the discriminating ones. That is a change. An earlier screen demanded
that a candidate's own storage interface support ordered range iteration, which turned out to be a
storage-engine capability promoted to a selection criterion. Once the requirement was restated as what a
light client must VERIFY, point-provability turned out to be satisfiable by almost any authenticated
key-value store, so an engine can no longer be removed for the shape of its own storage.

CosmWasm and MoveVM both clear the screen. CosmWasm leads on evidence rather than on demonstrated merit,
since nothing has been run against Move at all. The Ethereum Virtual Machine's storage objection fell
with the same filter change, and it is now rejected on coexistence instead, which is a weaker and more
contestable objection than the one it replaces. Recording that substitution rather than keeping the
conclusion and quietly swapping its foundation.

No gate passes today, two fail, and any recommendation should be read with its conditions attached.

## Dependency support policy

A consensus runtime cannot be patched quickly, because a fix to it is a consensus change. The policy
that follows from that is recorded in the requirements register and enforced rather than remembered.

A candidate line must have at least two Dash upgrade cycles of remaining support, about 180 days, when
integration begins. Every gate is re-run on a new major line by default, and carrying one forward needs
a written justification, since gate evidence is tied to an exact dependency graph. A binary may hold
several runtime versions, but the active one is a function of the activated protocol version, so exactly
one executes at any height.

The CosmWasm evidence here was produced against 1.5.11, whose security support ended on 2025-04-30, so
it is feasibility evidence and not a basis for integration. `tools/check_dependency_support.sh` compares
the recorded support status against what these documents actually say and fails when they disagree. It
runs in continuous integration on every push, because an expiry date passes on its own while nobody is
looking.

## Evidence grades

Claims in this repository are graded, and the grade is part of the claim.

- **EXECUTION-PRODUCED** means demonstrated by running code against the real storage engine at a
  frozen revision. The metering results and the CosmWasm and Ethereum spikes are this grade.
- **REPOSITORY-RESOLVED** means read from checked-out source of the system in question.
- **ASSERTED** means stated from knowledge of an external system that is not checked out here and has
  not been run. Everything said about MoveVM and the other alternative engines is this grade, and the
  comparison says so rather than implying it was measured.

## The prototype

`metering-prototype/` holds the running code. It is prototype-grade and its boundary is written down
in [metering-prototype/SCOPE_AND_LIMITATIONS.md](metering-prototype/SCOPE_AND_LIMITATIONS.md), which
names four things it deliberately does not do. Read that before drawing conclusions from it.

| Crate | What it demonstrates |
| --- | --- |
| `meter-core` | The terminal-work meter, with its invariants certified under fault injection |
| `cosmwasm-host` | CosmWasm's host and contract storage traits running over GroveDB, with a cost adapter deriving gas from GroveDB's measured operation cost |
| `cosmwasm-spike` | The contract-facing storage trait through the real `cw-storage-plus` map |
| `evm-guest-contract` | A minimal Ethereum interpreter compiled to WebAssembly as a CosmWasm contract |
| `compute-bench` | The compute-cost measurements |

`meter-core` is self-contained and needs nothing but a Rust toolchain.

```bash
cargo test --manifest-path metering-prototype/meter-core/Cargo.toml
```

The GroveDB-backed crates need two things. A Linux toolchain with clang, since GroveDB pulls in
RocksDB, and a GroveDB checkout as a SIBLING of this repository, because the crates depend on it
through a relative path. The revision is pinned by commit rather than by branch, since every
measurement recorded here was taken against that exact revision.

```bash
git clone https://github.com/dashpay/grovedb.git ../grovedb
git -C ../grovedb checkout 9b98a35644cdea73cc1b21d7c122cb58ae9fafd8
cargo test --manifest-path metering-prototype/cosmwasm-host/Cargo.toml
```

The continuous integration workflow does exactly this, so it is the reference for a working setup.

The spike binaries print their measurements and are run the same way, for example
`cargo run --manifest-path metering-prototype/cosmwasm-host/Cargo.toml --bin evm`. Recorded output
from each is kept under `metering-prototype/results/`.

## Method, and what "review-complete" means

The design came out of a clean-room exercise. The requirements were written down with no hint of a
preferred solution and leak-checked to keep candidate-solution vocabulary out, then several
independent sources each designed the system from those requirements alone, with no contact between
them and no sight of the author's design, which was committed to version control first so the
independence is auditable. Agreements were treated as forced by the requirements. Disagreements became
deliberate decisions with reasons recorded.

One limit of that exercise is worth stating, because it bounds what it can be cited for. The
convergence supports a broad architecture FAMILY, a deterministic metered sandboxed WebAssembly runtime
reached through host functions. It does not establish that any particular implementation of that family
is the right one, so it cannot be cited as having selected CosmWasm.

The synthesized architecture then went through twelve adversarial review rounds, and the metering
prototype through fifteen more. Findings are attributed by independent review source rather than by
reviewer headcount, because two samples from one source are one voice, not two. A round could close
only when a fresh full pass returned approval with nothing real left to fold.

Review-complete means exactly one thing, that independent reviewers attacked the full design and found
nothing left to fix in the text. It is a statement about a design on paper and deliberately nothing
more. Nothing here is deployed, so nothing has been tested by running at scale, and by this project's
own evidence rules no part of the design is closed in the strong sense until an implementation exists
and independent review confirms the behaviour against it.

The raw review record is kept with the project rather than published, since it is reviewers' verbatim
output. The adjudications and the findings that survived them are reflected throughout `DESIGN.md`.

## Status and license

`DESIGN.md` is at version 12 and frozen as a historical record rather than as a settled conclusion.
The execution engine remains open. A direction is recommended in the decision document subject to five
conditions, none of which has closed.

Two defects are currently open in the prototype and both are recorded rather than pending quietly. A
range scan performs unbounded work before its gas charge is enforced, so the charge is accounting
rather than admission control. And a terminal-work item larger than the per-block drain rate is never
drained and blocks everything behind it, which a published measurement here had misread as a tuning
result. Separately, all CosmWasm evidence was produced against a release line whose security support
ended on 2025-04-30, so it is feasibility evidence rather than a basis for a current decision.

`TODO.md` carries the remaining work.

Licensed under Apache-2.0, see [LICENSE](LICENSE). `metering-prototype/cosmwasm-host/testdata/`
contains `hackatom.wasm`, third-party test data from the CosmWasm project, also Apache-2.0, with its
provenance recorded in that directory's README.
