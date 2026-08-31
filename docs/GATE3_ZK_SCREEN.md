# Gate 3 and the zero-knowledge engines

Dated 2026-08-30. Closes an item flagged as an unscreened arm in two previous documents. The screen was
run and its main finding is that the item was miscategorized, so this note records the correction as
well as the result.

## What gate 3 asks

"Can a program gate an action on a cryptographic proof, verified within the gas and block bound, without
revealing the witness?"

That is a question about a HOST PRIMITIVE. It asks whether the platform can expose proof verification to
a program at a bounded, metered cost. It is not a question about the engine's instruction set, its
storage model, or its provenance.

The design already chose the mechanism. `DESIGN.md` specifies a bounded, metered `verify_proof` host
call for governance-registered systems, circuits and keys, with fixed parser rules, input encoding and
size limits. It deliberately did NOT adopt a native shielded-note service (commitments, nullifiers, tree
roots, note accounting) for the execution-layer release, on the grounds that it enlarges the trust and
cost model and has no retention model compatible with the durable-obligation decision, since an unspent
commitment cannot expire without burning an asset and a spent nullifier cannot expire without permitting
a double spend.

So the scope of gate 3 is verifying a proof, not operating a shielded pool. Platform already has a
shielded subsystem of its own for the latter.

## The finding: gate 3 does not discriminate between engines

Every candidate that clears the engine screen can satisfy gate 3 the same way, by calling a host-provided
verification primitive. CosmWasm reaches it through its custom query and message types. Move reaches it
through host-registered native functions. The Ethereum Virtual Machine reaches it through precompiles,
which is how every Ethereum-family chain has done pairing checks for years.

The cost and the bound are properties of the host implementation and the proof system, not of the engine
running above them. The one engine-specific part is the metering binding, meaning how the host's measured
verification cost is charged against the program's budget, and that is a small integration surface rather
than a selection criterion.

The practical consequence is that gate 3 should not be expected to rank candidates. It gates the
PLATFORM, and it gates every candidate identically.

## The correction

Two earlier documents recorded the zero-knowledge-native engines as an unscreened arm of the engine
comparison, and named gate 3 as the reason they deserved screening. That was a category error, made
twice, and it survived because "we have not screened them" reads like diligence rather than confusion.

The engine screen asks which runtime should execute Dash programs. The zero-knowledge engines mostly
answer a different question, which is whether execution should be RE-EXECUTED by every validator at all.
Putting them in the same table implied a comparison that the table could not make.

## The two things called zero-knowledge engines

Separating them is what dissolves the confusion.

**Proof verification inside a program.** Any engine, given the right host primitive. This is gate 3 and
it is settled in shape if not yet in evidence.

**Proving execution itself.** A zero-knowledge virtual machine (RISC Zero, Cairo, Miden, and the newer
provers) executes a program off-chain and produces a succinct proof that it ran correctly. The chain
verifies the proof rather than repeating the work. That is not an engine swap. It is a different
architecture for the whole execution layer.

## The architectural arm, stated properly

Since it is a genuine alternative, it belongs beside the no-general-VM baseline as a control arm rather
than as a row in the engine screen. Its properties are interesting enough to record even though nothing
here has evaluated it.

It DISSOLVES gate 1 rather than passing it. Determinism across validators exists because every validator
re-executes and must agree. If validators verify a proof instead, they never execute the program, so
floating point, compiler selection, and build configuration stop mattering. The hardest gate in this
project becomes irrelevant rather than satisfied, which is a genuinely different answer to the problem
than any engine choice provides.

It breaks gate 2's assumption. Proving is expensive and slow, seconds to minutes for non-trivial
programs. It cannot sit in a half-second block path, so proving must happen off-chain and the block path
must carry verification only. That is workable and it is how such systems are built, but it changes the
transaction model, the latency profile, and who does the work.

It introduces a prover role that does not exist today. Someone runs the proving infrastructure, which is
a new participant with cost and centralization consequences, and the fee model has to pay for proving
somewhere. This is precisely the class of new trusted role the evaluation rubric asks designs to confront
rather than assume away.

Coexistence is unknown and probably poor. Platform's existing state, contracts, identities and documents
assume re-execution.

One further observation, worth stating because it looks like a shortcut and is not. A zero-knowledge VM
can be used as a plain deterministic interpreter with the proving turned off, which would make it merely
another engine with an unusual instruction set and a small ecosystem. Doing that discards the only reason
to have chosen it.

## Gate 3's actual state

INCONCLUSIVE, unchanged, but for a better-understood reason. The mechanism is chosen and the shape is
settled. What is missing is execution-produced evidence, meaning a running verification with a measured
cost against the block budget, on a registered proof system with real parameters.

That evidence is largely ENGINE-INDEPENDENT and can be produced once. Under the per-line gate policy it
would still need its metering binding re-confirmed per engine and per major line, but the expensive part,
establishing that verification fits the budget at all, does not have to be repeated for each candidate.

That makes gate 3 cheaper to close than gate 1 or gate 2, and it is the only one of the five whose
evidence transfers between candidates.

## What would change this

A decision to bring the shielded-note service into the execution layer, which the design currently
excludes, would enlarge gate 3 considerably and reintroduce the retention problem the durable-obligation
decision was written to avoid.

A decision to pursue the proving architecture would not change gate 3. It would replace the gate set,
since gate 1 would no longer be asking anything.
