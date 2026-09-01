# Direction Decision 0, method, version 5 (DRAFT, awaiting owner approval)

Dated 2026-08-30. Version 5 revises version 4 after review. Method only. No conclusion, no corpus.

STATUS: DRAFT. Not approved. No discovery begins until the owner approves this document, its weights,
the discovery manifest (`docs/DIRECTION_DECISION_0_MANIFEST.md`) and the sensitivity scenarios. All four
freeze together as FREEZE ONE.

## What this decides, and what it cannot

This decides what the RESEARCH RECOMMENDS. Adoption is decided by Dash's governance and maintainers.

## Scope: a STATIC TARGET, not a migration

CORRECTED from version 4, which noticed that governance-gated-then-permissionless is a TRANSITION
between two packages but left it unscored, so the packages could not represent it.

Direction Decision 0 selects a STATIC TARGET package. It does not select a migration path. Scoring a
transition requires its trigger, and a defensible trigger depends on the demand evidence this decision
is meant to produce, so scoring transitions here would use the answer to choose the question.

Migration is deferred to a separate decision, DD1, which runs only if DD0 returns PURSUE or DEFER. P8 is
therefore re-decided in two parts: DD0 sets the target, DD1 decides whether it is reached directly or
through a gated phase.

If the corpus shows a need that ONLY a transition serves, that is recorded as a finding and DD1 is
brought forward. It is not scored here.

## The six authority actions

CORRECTED from version 4, whose three admission questions bundled distinguishable actions. "Add
behavior" could mean changing what the runtime can do, publishing a program, or admitting one. "Invoke"
merged creating an instance with calling it.

| | Action | Authority options |
| --- | --- | --- |
| **D1** | Runtime or catalog evolution, changing what the engine or catalog can do at all | Protocol release |
| **D2** | Program or policy admission, publishing new logic to the chain | Protocol; governance; permissionless |
| **D3** | Instance creation, creating an instance of admitted logic | Protocol; governance; permissionless |
| **D4** | Invocation, calling an existing instance | Protocol; governance; permissionless |
| **D5** | Routine upgrade of admitted logic | Protocol; governance; deployer; nobody, immutable |
| **D6** | Emergency disable | Protocol; governance; deployer; nobody |

## Package derivation, evidence-independent

CORRECTED from version 4, which asserted nine packages without saying why those nine. An unexplained
reduction is a preselection route. The rules below are structural or coherence-based, never based on
which option looks attractive.

**S-1. D1 is always PROTOCOL.** Changing what the runtime or catalog can do is a consensus change under
every capability option, including A4, where adding a host function is a protocol release. D1 therefore
DISCRIMINATES NOTHING and is fixed rather than enumerated. Recorded so its absence is not mistaken for
an oversight.

**S-2. A1 has no programs.** D2 through D6 are NOT APPLICABLE. A1 yields exactly one package.

**S-3. A2 has no programs, only native operations.** D2 is not applicable, since there is nothing to
admit. D5 is not applicable, since upgrading a native operation IS D1. D3, D4 and D6 remain live.

**S-4. A3 and A4 have programs.** D2 through D6 are all live.

**C-a. openness(D2) must not exceed openness(D4).** Publishing rights looser than calling rights is
incoherent, since the right to publish what nobody may call is worth nothing. Excludes every class where
admission is permissionless while invocation is restricted.

**C-b. openness(D3) must not exceed openness(D4).** Same reasoning for instance creation.

**C-c. D6 must be no narrower than D5.** A system whose logic can be upgraded but not halted is
strictly worse than one where both are possible, at no benefit. Excludes every class pairing a
deployer-upgradeable design with no emergency authority.

**Dom-1. D3 and D4 collapse where the corpus records no need to separate them.** They are enumerated
jointly and split only if an entry records different actors for creating versus calling. The per-entry
field "who must create, invoke, modify or stop an instance" exists to detect exactly that.

**Rep-1. Representative sampling on D2 and D5.** Where a dimension offers three or more values and no
structural rule separates adjacent ones, the enumeration takes the ENDPOINTS plus the governance middle.
Intermediate variants are represented by their nearest enumerated neighbour, and any corpus entry that
distinguishes them triggers a versioned amendment adding the package.

### The derived package set

| Package | Capability | D2 admit | D3 and D4 create and invoke | D5 upgrade | D6 disable |
| --- | --- | --- | --- | --- | --- |
| **PKG-1** | A1 | Not applicable | Not applicable | Not applicable | Not applicable |
| **PKG-2** | A2 | Not applicable | Governance | Not applicable | Governance |
| **PKG-3** | A2 | Not applicable | Permissionless | Not applicable | Governance |
| **PKG-4** | A3 | Governance | Permissionless | Nobody, immutable | Governance |
| **PKG-5** | A3 | Governance | Permissionless | Deployer | Governance |
| **PKG-6** | A3 | Permissionless | Permissionless | Deployer | Governance |
| **PKG-7** | A4 | Governance | Permissionless | Nobody, immutable | Governance |
| **PKG-8** | A4 | Governance | Permissionless | Deployer | Governance |
| **PKG-9** | A4 | Permissionless | Permissionless | Deployer | Governance |

### Why each omitted class is omitted

| Omitted class | Rule |
| --- | --- |
| Any package varying D1 | S-1, D1 is protocol under every capability |
| A1 with any admission choice | S-2, there is nothing to admit or invoke |
| A2 with a program-admission or routine-upgrade authority | S-3, A2 has no programs and upgrading its operations is D1 |
| Permissionless admission with restricted invocation | C-a |
| Permissionless instance creation with restricted invocation | C-b |
| Deployer-upgradeable with no emergency authority | C-c |
| Separate D3 and D4 authorities | Dom-1, collapsed until a corpus entry distinguishes them |
| Protocol-only or deployer-only D2, and protocol-only or governance D5 | Rep-1, represented by the nearest enumerated neighbour |
| Any transition between packages | Scope, deferred to DD1 |

The package set FREEZES with the method. Adding one later requires a versioned amendment stating why it
was missed.

## Veto applicability

Conditional on what a package actually does. Applying a requirement to a package that never encounters
it eliminates the package for lacking a property it never claimed.

| Requirement | Applies when | PKG-1 | PKG-2, 3 | PKG-4 to 9 |
| --- | --- | --- | --- | --- |
| Consensus determinism | Always | Applies | Applies | Applies |
| Native state keeps its proofs | Always | Applies | Applies | Applies |
| Existing Platform state survives | Always | Applies | Applies | Applies |
| No new mandatory trusted party | Always | Applies | Applies | Applies |
| Atomic effect batching (B4) | The package can change state | Not applicable | Applies | Applies |
| Work bounded before performed (B5) | The package performs variable-cost work | Not applicable | Applies | Applies |
| Queued obligations schedulable (B6) | The design can create queued or deferred obligations | Not applicable | Only if the catalog creates them | Only if the design creates them |
| Programs write native indexed collections (B7) | The package permits program-private state AND requires native indexed writes | Not applicable | Not applicable | Only where both hold |

For PKG-1 the universal requirements mean the EXISTING invariant must not be weakened. NOT APPLICABLE is
recorded distinctly from PASS. An UNKNOWN on an applicable veto is BLOCKING.

## Corpus rules

Sourced applications included regardless of which package they favour. Characterized, not classified.
Hypotheticals go to a stress-test appendix. No minimum count. Discovery follows the frozen manifest.
INSUFFICIENT COVERAGE CANNOT PRODUCE A NEGATIVE FINDING.

### Evidence dimensions

| Dimension | Values |
| --- | --- |
| **Dash demand provenance** | NAMED PARTY BUILDING; NAMED PARTY REQUESTING; COMMUNITY DISCUSSION; INFERRED; NONE |
| **Implementation maturity** | DEPLOYED ELSEWHERE; BUILDING ELSEWHERE; SPECIFIED; CONCEPT ONLY |
| **Source independence** | INDEPENDENT; ADJACENT, disclosed not disqualifying; SOLICITED |
| **Confidence** | HIGH, primary and unambiguous; MEDIUM, primary but partly ambiguous or reliable secondary; LOW, secondary, inferred or possibly stale. The specific uncertainty is always stated |

### Recorded per entry, architecture-free

Workload shape and frequency. Fan-out. Long-lived obligations. Asynchronous operations and whether they
span blocks. Native state accessed. Authority required. How often genuinely NEW behavior is needed.
Whether instances differ only by configuration. Who must create, invoke, modify or stop an instance.
How quickly behavior must change.

## Criteria

Nine criteria, each 0 to 4 or UNKNOWN. **Every criterion is scored against the WHOLE PACKAGE**, meaning
capability and admission together.

**C1. Coverage of sourced Dash needs uniquely served. Weight 25.**
CORRECTED from version 4 to operate on packages. A need is served only if BOTH the capability and the
admission choices satisfy it. A need requiring permissionless deployment is NOT served by a
governance-gated package whose capability would otherwise suffice.
0, serves no sourced need. 1, serves only needs a lower-capability package also serves. 2, uniquely
serves a need of INFERRED or COMMUNITY provenance. 3, uniquely serves a need with a NAMED REQUESTING
party. 4, uniquely serves several, one with a NAMED BUILDING party.

**C2. Trust removed or introduced. Weight 20.**
0, introduces a new mandatory trusted party, also a veto. 1, introduces a discretionary trusted role.
2, neutral. 3, removes a discretionary dependency. 4, removes one users must currently accept.

**C3. Security surface and auditability. Weight 20.**
0, unbounded new surface, no audit path. 1, large, specialist audit only. 2, moderate, established
practice. 3, small, mostly existing reviewed code. 4, no new consensus-critical surface.

**C4a. Governance burden. Weight 5.** 0, governance acts on every change. 1, frequently. 2,
periodically. 3, rarely. 4, never beyond today.

**C4b. Upgrade and maintenance burden. Weight 5.** 0, standing duty, no tooling, fork risk on skew. 1,
regular coordinated upgrades. 2, periodic and tooled. 3, rare. 4, none beyond today.

**C4c. Operating burden. Weight 5.** 0, new always-on responsibility for node operators. 1, significant.
2, modest. 3, negligible. 4, none.

**C5. Developer autonomy and composability. Weight 10.** 0, every capability needs a protocol release.
1, governance approval per capability. 2, approval once per class. 3, independent within bounds. 4,
independent and composing freely.

**C6. Reversibility. Weight 5.** 0, effectively irreversible. 1, high cost with user impact. 2, needs a
migration. 3, reversible by disabling, state retained. 4, trivially reversible.

**C7. Time to first useful capability. Weight 5.**
CORRECTED from version 4 to define its comparison set. C7 is scored PER SOURCED NEED, among the packages
that serve that need, meaning those scoring 2 or above on C1 for it. A package's C7 is the mean of its
per-need scores across the needs it serves, rounded to the nearest integer.
0, longest by a wide margin. 1, longer than median. 2, mid-range. 3, shorter than median. 4, shortest.
**A package serving no sourced need scores C7 as NOT APPLICABLE, never 4.** Doing nothing is always
fastest and that is not a result.

## Weights, AWAITING OWNER APPROVAL

| Criterion | Weight |
| --- | ---: |
| C1 Coverage of sourced needs | 25 |
| C2 Trust | 20 |
| C3 Security surface and auditability | 20 |
| C4a Governance burden | 5 |
| C4b Upgrade and maintenance burden | 5 |
| C4c Operating burden | 5 |
| C5 Developer autonomy and composability | 10 |
| C6 Reversibility | 5 |
| C7 Time to first useful capability | 5 |

Total 100. Vetoes sit outside and cannot be traded against these.

## The score formula

CORRECTED from version 4, which left the arithmetic unstated.

**Normalized package score** = ( sum over criteria of weight times score ) divided by 4, giving a 0 to
100 scale, since the maximum is 4 times 100 weight.

**NOT APPLICABLE criteria** are removed from the package's score and its weights renormalized to 100
across the remaining criteria, so a package is neither rewarded nor punished for a criterion it cannot
encounter. The renormalization is shown in the scoring sheet.

**Ties.** Two packages within 2.0 normalized points are TIED in that scenario.

**LEADS means UNIQUE first place**, more than 2.0 points above every other package. A package that is
tied for first does not lead.

## Sensitivity scenarios, frozen

| Scenario | Changes from baseline | Total |
| --- | --- | ---: |
| **S1 Baseline** | none | 100 |
| **S2 Security-first** | C3 to 30, C1 to 15 | 100 |
| **S3 Demand-first** | C1 to 35, C2 to 15, C3 to 15 | 100 |
| **S4 Operations-first** | C4a, C4b, C4c to 10 each, C1 to 15, C5 to 5 | 100 |
| **S5 Autonomy-first** | C5 to 20, C4a, C4b, C4c to 2 each, C7 to 4 | 100 |

CORRECTED from version 4, where S5 totalled 99. C6 stays at 5 and C7 drops to 4, which restores 100.
Every scenario total is checked arithmetically before freeze.

## The unknown-interval test

CORRECTED from version 4, whose "material unknown capable of reversing the ranking" was a judgment.

Each UNKNOWN is assigned its full interval, 0 through 4. Because the score is linear in each criterion,
it suffices to evaluate the CORNERS, meaning every combination of 0 and 4 across the unknown criteria.
If ANY corner, in ANY scenario, changes which package leads, the result is INCONCLUSIVE.

This replaces the earlier rule of thumb about criteria weighted 15 or above. That threshold is now
redundant, since a heavily weighted unknown will fail the corner test on its own.

## From scores to a finding

1. Eliminate packages failing an APPLICABLE veto. An unknown on an applicable veto is blocking.
2. No capability credit for hypothetical uses.
3. A more capable package may be recommended only if it uniquely serves at least one sourced,
   non-INFERRED need.
4. PURSUE requires unique first place in ALL five scenarios AND survival of the corner test.
5. Otherwise report an UNRESOLVED SET naming the packages that cannot be separated.

## The four outcomes

**PURSUE.** Coverage adequate per the manifest. No unresolved applicable veto. One package leads
uniquely in all five scenarios and survives the corner test. It uniquely serves at least one sourced,
non-inferred need.

**DEFER.** Coverage adequate, and either demand is real but thin, or the leader is blocked only by
unknowns further work could resolve. Names what would resolve it and a revisit date.

**DO NOT PURSUE ON CURRENT EVIDENCE.** REQUIRES ADEQUATE COVERAGE. No sourced non-inferred need requires
capability beyond the lowest-capability package satisfying its vetoes.

**INCONCLUSIVE.** Coverage below the manifest minimum, or an unresolved applicable veto, or no unique
leader across scenarios, or a corner-test failure. A thin search always lands here.

## Reversal conditions

- A Dash party building or requesting something no lower-capability package serves.
- Absence of programmability becomes a stated cause of loss, traceable to named parties.
- The chosen package is attempted and costs more than an alternative.
- A PURSUE reverses if its corpus proves to rest mainly on SOLICITED or LOW-confidence entries.
- A DO NOT PURSUE reverses on new sourced demand, revisited on a recorded date.

## How results inform the rest of the project

A qualitative corpus INFORMS these and does not measure them. Every derived estimate carries source,
range and confidence. H1 gains sourced shapes and stays a hypothesis. H2 gains fan-out as ranges. P3
gains rights models. P10 gains the authority column.

## Sequence

1. Owner approves this method, the weights, the manifest and the scenarios.
2. **FREEZE ONE**, all four committed at a named version, with the package set.
3. Corpus assembled per the frozen manifest, architecture-free.
4. Coverage reviewed against the manifest minimum.
5. **FREEZE TWO**, corpus committed.
6. Only then are packages scored.
