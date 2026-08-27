# An execution layer for Dash Platform

Dash Platform can store, index, prove, and authorize state, and it can settle value, but it cannot
compute. It cannot run a program that takes inputs, does arithmetic or logic, and writes the result
back in a way every participant is guaranteed to agree on. This repository asks one question, what
would it take to add that ability without giving up what makes the platform distinctive, and works
the answer out as a design backed by running code.

This is exploratory research by [@hilawe](https://github.com/hilawe). It is not a Dash roadmap item,
not a Dash Improvement Proposal, and not an official position of the Dash project. Nothing here is
built into a shipping product.

## Start here

- **[docs/PLAIN_EXPLAINER.md](docs/PLAIN_EXPLAINER.md)** is the plain-language walkthrough. It assumes
  no blockchain engineering background and covers the whole arc, the gap, the method, the hard
  problem, and the recommendation.
- **[docs/COMMUNITY_BRIEF.md](docs/COMMUNITY_BRIEF.md)** is the short version for a reader who already
  knows the ecosystem.
- **[SUMMARY.md](SUMMARY.md)** is the research summary with the findings and recommendations.
- **[docs/EXECUTION_ENGINE_ADOPT_VS_BUILD.md](docs/EXECUTION_ENGINE_ADOPT_VS_BUILD.md)** is the
  decision synthesis, which recommends adopting CosmWasm backed by GroveDB and names five conditions
  to close before that choice is committed.
- **[docs/EXECUTION_ENGINE_VM_COMPARISON.md](docs/EXECUTION_ENGINE_VM_COMPARISON.md)** screens the
  alternative engines, MoveVM included, against the same requirements.
- **[DESIGN.md](DESIGN.md)** is the full architecture, version 12, review-complete and frozen.

## The finding, in one paragraph

The question that decides the engine is provability. Dash Platform's distinguishing property is that
state carries membership, non-membership, and secondary-index proofs to light clients, and an
execution engine that cannot preserve that is not a candidate however good it is otherwise. The
Ethereum Virtual Machine fails that test as a base layer, because its fixed 256-bit-slot storage model
presumes a Merkle Patricia trie. CosmWasm passes it, because its storage interface assumes an
authenticated, lexicographically ordered byte-key store with range iteration and proofs, which is the
shape GroveDB already has. Backing CosmWasm with GroveDB is a substitution between two authenticated
ordered stores rather than a graft, and that is demonstrated here at every layer by running code, up
to and including a minimal Ethereum interpreter running as a guest contract whose storage writes land
in GroveDB and prove.

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

`DESIGN.md` is at version 12 and frozen. The open question is the execution engine, and the decision
synthesis recommends adopting CosmWasm subject to five conditions, of which determinism across
validators is the one to answer first, because it is the assumption whose failure would end the
approach. `TODO.md` carries the remaining work.

Licensed under Apache-2.0, see [LICENSE](LICENSE). `metering-prototype/cosmwasm-host/testdata/`
contains `hackatom.wasm`, third-party test data from the CosmWasm project, also Apache-2.0, with its
provenance recorded in that directory's README.
