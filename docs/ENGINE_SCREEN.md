# Engine screen, re-run under the P1 split

Dated 2026-08-30. This is the current screen. It supersedes the screen section of
`docs/EXECUTION_ENGINE_VM_COMPARISON.md`, which is retained for its per-engine reasoning and carries a
notice saying so.

The re-run was required rather than optional. The previous screen's first filter demanded that a
candidate engine's own storage interface support ordered range iteration, because program-written state
was assumed to need completeness and absence proofs. The P1 decision settled that the other way. A
filter that changes changes every row beneath it, so editing the old table would have produced a
verdict nobody had actually re-derived.

## The filters, derived from the binding requirements

Each filter now traces to a numbered requirement rather than to a capability the storage engine happens
to have. That was the defect in the previous screen and it is the reason this one is written from the
register outward.

**F1. Point-provable program state (B1, as narrowed by P1).** The engine's storage interface must be
satisfiable by an authenticated key-value store that can produce membership proofs for a record at a
known key. Ordered iteration is NOT required of the engine, because completeness-bearing data does not
live in program-private state under the split.

**F2. A host extension surface (B7, and the native capability requirement).** The engine must provide a
principled mechanism by which the host exposes native operations to programs, so that programs can
write native indexed collections and reach Dash-native tokens, identities and groups WITHOUT forking
the engine. Under the split this is where the platform's distinguishing property is actually preserved,
which makes it the load-bearing filter.

**F3. A determinism story the engine owns (B2).** Named answers for floating point, iteration order,
memory growth, metering-boundary timing, and build configuration. An engine that leaves these to the
integrator is a build arm, not an adopt arm.

**F4. Coexistence with existing Platform state (B3).** Data contracts, documents, identities, groups
and tokens survive and stay usable, with migration or wrapping permitted. An engine whose own account
and state model forces every native feature through a wrapper fails here even if it passes F1.

The relocation of weight from F1 to F2 and F4 is the substantive result of this re-run, and it is worth
stating before the table. F1 no longer eliminates anyone. Almost any authenticated key-value store
satisfies point-provability, so an engine can no longer be removed for the shape of its own storage.
What separates candidates now is how well they reach outward and how badly they fight the existing
platform.

## The screen

| Engine | F1 point-provable | F2 host extension | F3 owns determinism | F4 coexistence | Verdict |
| --- | --- | --- | --- | --- | --- |
| CosmWasm | Yes, demonstrated | Yes, custom message and query types plus a backend interface, demonstrated | Partly, see gate 1 | Good, host functions reach native state | Clears, leading candidate |
| MoveVM | **Yes, now.** Address plus type-tag resolution is point-provable | Yes, native functions registered by the host | Yes, and stronger than CosmWasm's on floats and metering | Uncertain, adopting a framework brings its own state model | **Clears. Reopened by this re-run** |
| EVM | **Yes, now.** Contract address plus slot is point-provable | Yes, precompiles, though a crude mechanism | Yes, no floats, mature gas model | Poor, wraps every native feature and the account model conflicts | **Reopened, then fails on F4 rather than F1** |
| Strict integer-only interpreted Wasm | Yes, host-defined | Yes, host-defined | Partly, the integrator writes it | Good | Reference arm, per the charter |
| Solana SBF | Yes, host-defined | Yes, syscalls | Yes | Poor, the declare-accounts-up-front model conflicts with B3 | Fails F4, and see the correction below |
| FuelVM | Yes, per-contract key-value | Yes | Yes | Poor, UTXO-centric state model | Fails F4 |
| PolkaVM | Yes, host-defined | Yes, host-defined | Partly, maturity thin | Neutral | Build arm, watch |
| Raw Wasm plus Dash host | Yes, by construction | Yes, by construction | No, the integrator writes it | Good | Build arm, already costed |
| zkVMs (RISC Zero, Cairo, Miden) | Not applicable | Not applicable | Dissolves the question | Unknown, likely poor | REMOVED from this screen. Architectural control arm, not an engine row |

Evidence grades are unchanged and matter here. Every CosmWasm row is EXECUTION-PRODUCED or
REPOSITORY-RESOLVED. Every other row is ASSERTED, meaning reasoned from knowledge of a system not
checked out and never run against GroveDB. This screen ranks which engine deserves evidence next. It
does not report measurements.

## The two reopenings, stated plainly

**MoveVM is reopened and now clears the screen.** Its removal rested entirely on the absence of ordered
range iteration in its resolver interface, which F1 no longer asks for. Resolution by exact address and
type tag is precisely a point lookup, which is what the split requires. Move also carries the stronger
determinism story of the two mature candidates, since it has no floating-point type to permit and
charges gas in its interpreter rather than through compiler-injected points. Its remaining obstacles are
real but they are F4 and integration obstacles rather than disqualifications: adopting the Aptos
framework or the Sui runtime brings a whole state model with it, there is no request-and-resolve pattern
for asynchronous native capabilities, and the Ethereum-as-guest result does not transfer.

**The EVM is reopened on F1 and then fails on F4 instead.** This is the uncomfortable one and it needs
saying rather than managing. Decision D-02 rejected the EVM as a base layer on the grounds that its
storage model is structurally incompatible with the provability requirement, and D-02's recorded
reversal condition was "a change to the provability requirement itself". That condition has now fired.
Contract address plus 256-bit slot is a perfectly good key in an authenticated store, and a light client
can be shown that slot X of contract C holds value V. The hashed keys that Solidity mappings produce
destroy ordering, which was fatal under the old filter and is irrelevant under the new one. Unset slots
reading as zero means there is no absence to prove, which the split also tolerates.

So the storage objection to the EVM is gone. What survives is the OTHER half of the original objection,
which the previous screen bundled together with it: an EVM base layer reaches Dash-native tokens,
identities and groups only through precompiles, and its account model competes with Dash identities
rather than composing with them. That is an F4 failure, and it is a weaker and more contestable
objection than the storage one it replaces. The conclusion is unchanged and its foundation is not, which
is exactly the kind of substitution that deserves to be recorded rather than smoothed over.

## A correction to the previous screen's reasoning

The earlier screen removed Solana's bytecode format on the grounds that "the account model has no state
proofs". That conflated the ENGINE with the CHAIN. Solana the network does not serve state proofs to
light clients, but that is a property of Solana's design rather than of its bytecode format, and an
embedding that stored account data in GroveDB could prove it. The honest reasons to leave it out are
different and are recorded above: it is not built to be embedded by a foreign host the way CosmWasm is,
and its requirement that a transaction declare every account it touches in advance conflicts with how
Platform's existing state is reached.

## What this screen does not settle

It does not rank the candidates that clear it. CosmWasm leads on evidence, not on merit demonstrated
against Move, and the gate specification records that no gate passes for it today.

The zero-knowledge row above is resolved, and not as expected. `docs/GATE3_ZK_SCREEN.md` records that
listing those engines here was a category error. Gate 3 asks whether a program can verify a proof through
a host primitive, which every candidate can do identically, so the gate ranks nobody. The engines that
PROVE execution answer a different question, whether validators should re-execute at all, which is an
architectural alternative rather than an engine choice. It now sits beside the no-general-VM baseline as
a control arm.

It does not address the baseline arm. Whether a small set of native modules serves the applications
Platform actually wants is hypothesis H4, and no engine screen can answer it.

## What follows

The candidate set that clears this screen is CosmWasm and MoveVM, with the strict interpreted reference
and the no-general-VM baseline as the charter's control arms. The discriminating questions between the
two are now F4 and the gates, not storage.

Three things would change this screen. The zero-knowledge engines being screened properly could add an
arm. A finding that the native indexed-collection host operation (B7) is impractical would reopen P1 and
with it this entire filter set. And evidence produced against MoveVM, which today has none, could move
it from reasoned to measured and change which engine leads.
