# Forking an engine, or integrating through its extension points

Dated 2026-08-27. Prompted by a question from the Dash community, relayed through @hilawe, asking
whether the Tenderdash precedent generalizes. If Dash forked Tendermint into Tenderdash, could it fork
CosmWasm into a "DashWasm", or Ethermint into an "EtherDash"?

The question is worth answering carefully, because the two cases come out differently and because the
Tenderdash precedent is easy to read as licensing a fork whenever Dash needs something specific. What
it actually licenses is narrower.

Evidence grades follow the project convention. The claim that CosmWasm can be integrated without
modification is EXECUTION-PRODUCED, since the spikes in this repository do exactly that. Claims about
Ethermint, the Cosmos SDK, and Tendermint are ASSERTED, meaning stated from knowledge of external
systems not checked out here.

## What the Tenderdash precedent actually shows

Dash forked Tendermint because it had to change consensus-level behavior that the upstream project
offered no way to change from outside. The validator set is derived from long-living masternode quorums
and block signing uses BLS threshold signatures, and neither is expressible through a configuration
option or an interface Tendermint exposes. There was no seam, so the only way through was a fork.

That is the rule the precedent supports. Fork where there is no seam. It says nothing about forking a
project that already provides the extension point you need, and reading it more broadly gets the cost
backwards.

## CosmWasm, where the seam already exists

CosmWasm is built to be embedded by a host chain. The storage trait, the querier, the backend
application binary interface, and the custom message and query types are extension points rather than
internals, and a host is expected to supply its own implementations of them.

The spikes in this repository supply exactly those implementations. GroveDB backs both the
contract-facing and host-side storage traits, a cost adapter derives gas from GroveDB's measured
operation cost, a querier answers Dash-native token and balance queries, and a router applies a
contract's outbound bank message to Dash state. All of it runs against cosmwasm-vm and cosmwasm-std as
unmodified dependencies resolved from crates.io. There is no `[patch]` section, no vendored copy, and
no forked crate anywhere in `metering-prototype/`.

So a "DashWasm" as the term is usually meant, a maintained fork of the VM, is not what the integration
requires. What exists today is a set of trait implementations against an upstream dependency, and
naming that a fork would describe the work inaccurately.

## Where a fork could still become necessary

Three places, none of which is reached yet, and each of which has a cheaper alternative worth trying
first.

The float policy is the nearest candidate and it comes out of gate 1. CosmWasm permits floating-point
operations and obtains determinism from NaN canonicalization in the compiler rather than from rejecting
the operations, which `docs/GATE1_DETERMINISM.md` records. If Dash decides float-bearing program code
should be refused outright, the component that would change is the Gatekeeper middleware inside
cosmwasm-vm. The cheaper alternative is to screen submitted bytecode at deployment on the Dash side,
which reaches the same outcome with no change to the crate, and which is the shape of control Dash
already applies to data contracts.

Gas semantics are the second. The cost adapter already prices storage by GroveDB's measured cost rather
than a flat schedule, and it does so from outside. A change to how gas is charged for Wasm instructions
themselves, rather than for host operations, would reach into the metering middleware.

The compiler is the third. Determinism depends on the singlepass compiler and its configuration, so any
requirement to patch or pin wasmer behavior beyond selecting versions lands inside that dependency
rather than in Dash's own code.

## What forking a deterministic VM actually costs

Worth stating plainly, because the maintenance cost of a fork is usually described in ordinary
engineering terms and that understates it here.

For most software, a fork means carrying patches and periodically reconciling with upstream. For a
deterministic VM inside a consensus system, every divergence from upstream is a potential consensus
surface rather than a maintenance chore. Two validators running builds that differ in any behavior the
VM exposes will disagree about a block, and that presents as a fork of the chain rather than as a bug
report. The same property that makes version pinning a standing governance duty, which is gate 5, makes
a source-level fork a permanent obligation of the same kind.

There is also a security dimension. Upstream security fixes arrive as changes to a codebase that has
moved on from the fork point, and the further the fork diverges the harder each fix is to apply
correctly, at exactly the moment when applying it quickly matters most.

Tenderdash shows this is survivable when the fork is genuinely necessary. It is not an argument for
taking on the same obligation when an extension point already exists.

## Ethermint, and why an EtherDash is the weaker idea

Two independent problems, either of which is sufficient on its own.

The first is the storage model, and it is the same objection that removed the Ethereum Virtual Machine
from consideration as a base layer. Ethermint runs the EVM, so program state lives in the EVM's fixed
256-bit-slot model. That model is what fails the provability requirement, so adopting Ethermint would
import precisely the property the research rejected, with a Dash name attached to it.

The second is structural and independent of the first. Ethermint is built on the Cosmos SDK and written
in Go. Dash Platform does not use the Cosmos SDK. It runs Tenderdash underneath its own application,
Drive, which is a Rust codebase. Ethermint is therefore not a component that can be dropped into the
existing stack. Adopting it would mean adopting the Cosmos SDK application layer as well, which is a far
larger structural commitment than the Tenderdash fork ever was, and it would sit alongside or replace
the application that holds Platform's existing data contracts, documents, identities, groups, and
tokens. Owner decision 1 requires those to survive.

None of this means Ethereum compatibility is unavailable. It is reachable through the guest shape
already demonstrated here, where a minimal Ethereum interpreter runs as a contract and its storage
writes land in GroveDB and prove. That route keeps the storage model that satisfies the requirement and
does not take on a second application framework in a second language.

## A note on naming

If a Dash-specific distribution does eventually warrant its own name, the name carries an implicit
claim. A reader who sees "DashWasm" will reasonably assume either that CosmWasm contracts run on it
unchanged or that they do not, and will be inconvenienced by guessing wrong. Whatever is chosen should
make the compatibility relationship explicit rather than leaving it to be inferred from the name.

## Summary

Fork where there is no seam, integrate where there is. Tendermint had no seam for masternode quorum
validator sets and BLS threshold signing, so Tenderdash was necessary. CosmWasm has the seam and the
integration through it is already demonstrated, so a fork is not currently justified and would convert
an upstream dependency into a permanent consensus obligation. Ethermint fails on two counts at once,
carrying the storage model the research rejected and requiring a second application framework in a
second language, while offering compatibility that the guest shape already reaches.
