# Direction Decision 0, method, version 4 (DRAFT, awaiting owner approval)

Dated 2026-08-30. Version 4 revises version 3 after review. Method only. No conclusion, no corpus.

STATUS: DRAFT. Not approved. No corpus discovery begins until the owner approves this document, its
weights, its discovery manifest and its sensitivity scenarios, after which all four freeze together as
FREEZE ONE at a named version.

## What this decides, and what it cannot

This decides what the RESEARCH RECOMMENDS. Whether Dash adopts anything is decided by Dash's governance
and maintainers, who are bound by nothing here.

## The evaluation unit is a POLICY PACKAGE

CORRECTED from version 3, which scored capability options A1 to A4 in isolation. That was wrong, because
trust, security surface, governance burden, reversibility and delivery time all depend on who may add,
invoke, upgrade and disable behavior. Scoring capability alone would let admission be chosen after
seeing which capability led, which is the preselection this method exists to prevent.

The unit is a COMPATIBLE PACKAGE: one capability choice plus explicit admission choices on every
admission question. Packages are enumerated at freeze one. Adding a package later requires a versioned
amendment stating why it was missed.

### Axis A, capability

| A1 | No new consensus computation |
| A2 | A finite catalog of native operations |
| A3 | A constrained domain-specific language or policy engine, bounded by construction |
| A4 | A general-purpose virtual machine |

### Axis B, admission, four questions

CORRECTED from version 3, whose B3 bundled routine upgrades with emergency disable authority. Those can
belong to different actors, and neither covers the case where nobody holds the power at all.

| | Options |
| --- | --- |
| **B1. Who may ADD new behavior** | Protocol release only; governance approval; permissionless |
| **B2. Who may INSTANTIATE or INVOKE it** | Protocol-restricted; governance-approved callers; permissionless |
| **B3a. Who may UPGRADE it routinely** | Protocol release; governance; the deploying party; NOBODY, behavior is immutable once live |
| **B3b. Who may DISABLE it in an emergency** | Protocol release; governance; the deploying party; NOBODY |

B3a and B3b are separate because an immutable program that governance can still halt is a coherent and
common design, and version 3 could not express it.

### The enumerated package set

| Package | A | B1 add | B2 invoke | B3a upgrade | B3b emergency |
| --- | --- | --- | --- | --- | --- |
| **PKG-1** | A1 | Protocol | Not applicable | Protocol | Protocol |
| **PKG-2** | A2 | Protocol | Governance-approved callers | Protocol | Protocol |
| **PKG-3** | A2 | Protocol | Permissionless | Protocol | Protocol |
| **PKG-4** | A3 | Governance | Permissionless | Nobody, immutable | Governance |
| **PKG-5** | A3 | Governance | Permissionless | Deploying party | Governance |
| **PKG-6** | A3 | Permissionless | Permissionless | Deploying party | Governance |
| **PKG-7** | A4 | Governance | Permissionless | Nobody, immutable | Governance |
| **PKG-8** | A4 | Governance | Permissionless | Deploying party | Governance |
| **PKG-9** | A4 | Permissionless | Permissionless | Deploying party | Governance |

Nine packages. P8's original position, governance-gated then permissionless, is a TRANSITION between
PKG-8 and PKG-9 rather than a package, and whether that transition is intended is part of what this
decides.

## This reopens P8

P8 recorded deployment as governance-gated then permissionless, which assumed capability was settled at
A4 and admission was a schedule. Both are open. P8 is re-decided when this concludes.

## Veto applicability

Vetoes cannot be traded against weighted criteria. Applicability is conditional on what a package
actually does, since applying a requirement to a package that does not need it eliminates the package
for failing to have a property it never claimed.

| Requirement | Applies when | PKG-1 | PKG-2, 3 | PKG-4 to 9 |
| --- | --- | --- | --- | --- |
| Consensus determinism (B2) | Always | Applies | Applies | Applies |
| Native state keeps its proofs (B1) | Always | Applies | Applies | Applies |
| Existing Platform state survives (B3) | Always | Applies | Applies | Applies |
| No new mandatory trusted party | Always | Applies | Applies | Applies |
| Atomic effect batching (B4) | The package can change state | Not applicable | Applies | Applies |
| Work bounded before performed (B5) | The package can perform variable-cost work | Not applicable | Applies | Applies |
| Queued obligations schedulable (B6) | **The design can create queued or deferred obligations** | Not applicable | Applies only if the catalog creates them | Applies only if the design creates them |
| Programs write native indexed collections (B7) | **The package permits program-private state AND requires native indexed writes** | Not applicable | Not applicable | Applies only to packages meeting both conditions. An A3 policy engine exposing only native operations, with no private state, does not face it |

**For PKG-1, the universal requirements mean the EXISTING invariant must not be weakened.** They are not
a positive pass for capabilities PKG-1 does not provide. A package earns nothing for a requirement it
never encounters, and the scoring sheet records NOT APPLICABLE distinctly from PASS.

An UNKNOWN on an applicable veto is BLOCKING and is never resolved in the package's favour.

## Discovery manifest, frozen before searching

CORRECTED from version 3, which required the eventual report to disclose sources but did not predeclare
them. Disclosure after the fact cannot distinguish a planned search from a convenient one.

The manifest is written and frozen BEFORE any searching, and contains:

- The exact repositories, forums, governance records, proposal systems and support channels to be
  searched, each named individually.
- The exact search strings, and the date range applied to each source.
- The outreach selection rule, meaning how parties to approach directly are chosen, written so that
  someone else would select the same set.
- Treatment of inaccessible sources and of nonresponses, decided in advance rather than when they occur.
- **The minimum coverage required to support a NEGATIVE finding**, stated as a specific condition rather
  than as a judgment to be made later.

Additions or changes after freeze require a VERSIONED AMENDMENT with an explanation of why the original
manifest was inadequate. Amending is permitted. Amending silently is not.

**Window.** 24 months, plus older items still ACTIVELY PURSUED, which requires an observable: commits in
the last 12 months, current funding, a live proposal, or a named party confirming continued work.

**Deduplication.** One entry per distinct need. Request count records intensity, independent party count
records breadth.

**INSUFFICIENT COVERAGE CANNOT PRODUCE A NEGATIVE FINDING.**

## Corpus composition

Sourced applications included regardless of which package they favour. Characterized, not classified.
Hypotheticals go to a stress-test appendix, never the demand corpus. No minimum count.

## Evidence dimensions, anchored

| Dimension | Values |
| --- | --- |
| **Dash demand provenance** | NAMED PARTY BUILDING, code or funding exists. NAMED PARTY REQUESTING, traceable to a message or proposal. COMMUNITY DISCUSSION, raised publicly with nobody committing. INFERRED. NONE |
| **Implementation maturity** | DEPLOYED ELSEWHERE. BUILDING ELSEWHERE. SPECIFIED. CONCEPT ONLY |
| **Source independence** | INDEPENDENT, no relationship to this project or its author. ADJACENT, a Dash contributor or a party this project has worked with, disclosed rather than disqualifying. SOLICITED, this project asked |
| **Confidence** | HIGH, primary source read directly, unambiguous. MEDIUM, primary source read but partly ambiguous, or reliable secondary. LOW, secondary, inferred, or possibly stale. The specific uncertainty is always stated |

## Recorded per entry, architecture-free

Workload shape and frequency. Fan-out. Long-lived obligations. Asynchronous operations and whether they
span blocks. Native state accessed. Authority required. How often genuinely NEW behavior is needed, as
distinct from new instances. Whether instances differ only by configuration. Who must create, invoke,
modify or stop an instance. How quickly behavior must change.

Whether these imply a protocol release is decided after freeze two, not here.

## Criteria, atomic and anchored

Nine criteria. Each scored 0 to 4, or UNKNOWN.

**C1. Coverage of sourced Dash needs uniquely served. Weight 25.**
Scores SOURCED needs, never abstract expressiveness, since expressiveness hands A4 the result
automatically. 0, serves no sourced need. 1, serves needs a lower-capability package also serves. 2,
uniquely serves a need whose provenance is INFERRED or COMMUNITY only. 3, uniquely serves a need with a
NAMED REQUESTING party. 4, uniquely serves several, at least one with a NAMED BUILDING party.

**C2. Trust removed or introduced. Weight 20.**
0, introduces a new mandatory trusted party, which is also a veto. 1, introduces a discretionary trusted
role. 2, neutral. 3, removes a discretionary trust dependency. 4, removes one users must currently
accept.

**C3. Security surface and auditability. Weight 20.**
0, unbounded new attack surface, no audit path. 1, large surface, specialist audit only. 2, moderate
surface, established audit practice. 3, small surface, mostly existing reviewed code. 4, no new
consensus-critical surface.

**C4a. Governance burden. Weight 5.**
0, governance must act on every change. 1, frequently. 2, periodically. 3, rarely. 4, never beyond today.

**C4b. Upgrade and maintenance burden. Weight 5.**
0, a standing upgrade duty with no tooling and a fork risk on skew. 1, regular upgrades needing
coordination. 2, periodic upgrades, tooled. 3, rare upgrades. 4, none beyond today.

**C4c. Operating burden. Weight 5.**
0, new always-on operational responsibility for node operators. 1, significant new duties. 2, modest.
3, negligible. 4, none.

Split from version 3's single C4, since governance, upgrade and operating burden can point in different
directions and one score hid that.

**C5. Developer autonomy and composability. Weight 10.**
0, every new capability needs a protocol release. 1, governance approval per capability. 2, governance
approval once per class. 3, builders proceed independently within bounds. 4, independently and compose
freely.

**C6. Reversibility. Weight 5.**
0, effectively irreversible once live. 1, reversible at high cost with user impact. 2, reversible with a
migration. 3, reversible by disabling, state retained. 4, trivially reversible.

**C7. Time to first useful capability. Weight 5.**
Completed from version 3, which defined only 0, 2 and 4. 0, longest by a wide margin. 1, longer than
median. 2, mid-range. 3, shorter than median. 4, shortest. Scored relatively.

**C7 CONSTRAINT.** Time is compared only among packages that actually serve the sourced need under
consideration. A package that serves no sourced need cannot win on speed, because doing nothing is
always fastest and that is not a result.

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

## Sensitivity scenarios, frozen now

CORRECTED from version 3, whose "plausible weight range" left discretion after scoring. The scenarios
are fixed here. A leading package must lead in ALL of them.

| Scenario | Change from baseline |
| --- | --- |
| **S1 Baseline** | The approved table |
| **S2 Security-first** | C3 to 30, C1 to 15 |
| **S3 Demand-first** | C1 to 35, C2 to 15, C3 to 15 |
| **S4 Operations-first** | C4a, C4b, C4c to 10 each, C1 to 15, C5 to 5 |
| **S5 Autonomy-first** | C5 to 20, C4a, C4b, C4c to 2, C6 to 4, C7 to 4 |

Each scenario totals 100. Adding a scenario after scoring requires a versioned amendment.

## From scores to a finding

1. Eliminate any package failing an APPLICABLE veto. An unknown on an applicable veto is blocking.
2. No capability credit for hypothetical uses. C1 counts sourced needs only.
3. A more capable package may be recommended only if it uniquely serves at least one sourced,
   non-INFERRED need.
4. An UNKNOWN on any criterion weighted 15 or above, meaning C1, C2 or C3, blocks PURSUE for that
   package.
5. PURSUE requires the package to rank first in ALL five scenarios.
6. If the leading package changes across scenarios, report an UNRESOLVED SET.

## The four outcomes, each defined

**PURSUE.** Coverage adequate per the manifest. No unresolved applicable veto. One package leads in all
five scenarios. That package uniquely serves at least one sourced, non-inferred need. No UNKNOWN on C1,
C2 or C3.

**DEFER.** Coverage adequate, and either demand is real but thin, or the leading package is blocked only
by unknowns that further work could resolve. Defer names what would resolve it and a date to revisit.

**DO NOT PURSUE ON CURRENT EVIDENCE.** REQUIRES ADEQUATE COVERAGE. No sourced, non-inferred need
requires capability beyond the lowest-capability package that satisfies its vetoes. This is a statement
about current evidence, not about the idea, and it carries a reversal condition by construction.

**INCONCLUSIVE.** Any of: coverage below the manifest minimum; an unresolved applicable veto; the
leading package changes across scenarios; or a material unknown capable of reversing the ranking. A thin
search always lands here rather than in DO NOT PURSUE.

## Reversal conditions

- A Dash party building or requesting something no lower-capability package serves.
- Absence of programmability becomes a stated cause of loss, traceable to named parties.
- The chosen package is attempted and costs more than an alternative.
- A PURSUE reverses if its corpus proves to rest mainly on SOLICITED or LOW-confidence entries.
- A DO NOT PURSUE reverses on new sourced demand, and is revisited on a recorded date.

## How results inform the rest of the project

A qualitative corpus INFORMS these and does not measure them. Every derived estimate carries source,
range and confidence. H1 gains sourced candidate shapes and stays a hypothesis. H2 gains per-application
fan-out as ranges. P3 gains rights models. P10 gains the authority column.

## Sequence, with two freezes

1. Owner approves this method, the weights, the discovery manifest and the sensitivity scenarios.
2. **FREEZE ONE**, all four committed at a named version.
3. Corpus assembled and characterized, architecture-free, following the frozen manifest.
4. Coverage, independence, duplicates and exclusions reviewed against the manifest minimum.
5. **FREEZE TWO**, corpus committed.
6. Only then are packages scored.

The discovery manifest itself is written as part of step 1 and is not yet drafted. It is the one
remaining artifact before approval.
