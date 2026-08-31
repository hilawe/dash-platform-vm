# Direction Decision 0, method, version 2 (DRAFT, awaiting owner approval)

Dated 2026-08-30. Version 2 revises version 1 after review. It specifies HOW the direction question will
be answered, reaches no conclusion, and contains no application corpus.

STATUS: DRAFT. Not approved. No corpus work begins until the owner approves this document, and the
approved version is frozen at a named version before anything else proceeds.

The two-freeze sequence is the point. Criteria written after evidence describe the evidence, and an
evidence set assembled with the options in view selects its own answer. So the method freezes first, the
corpus freezes second, and only then does mapping to options begin. A later reader should be able to
verify from the commit history that each freeze preceded the work it constrains.

## What this decides, and what it cannot

CORRECTED from version 1. Version 1 said this asks "what Dash should do" and that "the owner decides
it". That overstates the authority of an unofficial research project.

This decides what the RESEARCH RECOMMENDS. Whether Dash adopts anything is decided elsewhere, by Dash's
own governance and its maintainers, and nothing in this repository binds them. The owner's authority
here is over the recommendation, its weighting, and what the project spends effort on next.

## Why this is not called a gate

The five gates test whether a candidate engine meets a technical requirement. This question is different
in kind. It weighs demand, cost, and risk, which is judgment rather than measurement, and calling it
Gate 0 would dress a judgment as a test.

## The question, on two axes

**Axis A, capability. What may be computed under consensus?**

| A1 | No new consensus computation |
| A2 | A finite catalog of native operations |
| A3 | A constrained domain-specific language or policy engine, expressive but bounded by construction |
| A4 | A general-purpose virtual machine |

**Axis B, admission. This is three questions, not one.**

CORRECTED from version 1, which asked only "who may introduce new behavior". Under that single question
a permissionless native catalog is incoherent, since A2 extends only by protocol release. Introducing
new behavior, using existing behavior, and changing behavior are separate permissions.

| | B1. Who may ADD new behavior | B2. Who may INSTANTIATE or INVOKE existing behavior | B3. Who may UPGRADE or DISABLE it |
| --- | --- | --- | --- |
| Options | Protocol release only; governance approval; permissionless | Protocol-restricted; governance-approved callers; permissionless | Protocol release; governance; the deploying party |

### Compatibility matrix

Not every combination exists. Filled in as constraints, not as preferences.

| Capability | B1 add | B2 invoke | B3 change |
| --- | --- | --- | --- |
| A1 | Protocol only, by definition | Not applicable | Protocol only |
| A2 | Protocol only, by definition. A2 cannot offer permissionless ADD | Any of the three. Permissionless INVOKE of a native catalog is coherent and is a real option | Protocol only |
| A3 | Any of the three | Any of the three | Any of the three |
| A4 | Any of the three | Any of the three | Any of the three |

The A2 row is the one version 1 could not express. A finite native catalog with permissionless
invocation is a genuine option and was previously unrepresentable.

## This reopens P8

P8 recorded deployment as governance-gated at launch and permissionless later. That assumed capability
was settled in favour of A4 and that admission was a rollout schedule. With capability open and
admission split three ways, P8 cannot stand as decided. It is reopened explicitly, and re-decided or
re-affirmed when this concludes.

## Corpus discovery, predeclared

CORRECTED from version 1, which specified inclusion rules but not where to look. Without a stated search,
a small corpus cannot be distinguished from a shallow search.

**Where.** Dash community forums and governance proposals; Dash Improvement Proposals and their
discussion; the Dash Core and Platform issue trackers; public developer channels; direct approaches to
teams known to be building on Dash; and comparable applications on other networks, recorded separately
as feasibility evidence rather than Dash demand.

**Window.** Requests and activity from the preceding 24 months, plus anything older still actively
pursued. The cutoff date is recorded when the corpus freezes.

**Deduplication.** One entry per distinct application need. Several requests from one organization are
ONE entry with the request count recorded, since repetition by a single party is intensity rather than
breadth.

**Coverage is reported, not assumed.** The corpus records which channels were searched, which were not,
and what was attempted but returned nothing.

**INSUFFICIENT COVERAGE CANNOT PRODUCE A REJECT OUTCOME.** If the search was thin, the honest result is
INCONCLUSIVE or DEFER. Absence of evidence is evidence of absence only when the search was adequate, and
whether it was adequate is a judgment recorded openly rather than assumed by default.

## Corpus composition

CORRECTED from version 1, which required including applications that "argue against" a general VM. That
classifies entries by the conclusion they support before the corpus is frozen, which is the same defect
the two-freeze sequence exists to prevent.

The corpus includes sourced applications without regard to which option they favour: existing Platform
workloads, active projects, requests, and proposals. Entries are characterized, not classified. Mapping
to A1 through A4 and B1 through B3 happens only AFTER the corpus is frozen.

No minimum count. A small well-searched corpus is an honest finding.

## Evidence dimensions, recorded separately

CORRECTED from version 1, whose single grade scale conflated unrelated facts. A deployed application on
another network proves feasibility and external demand but says little about Dash demand. A named Dash
request may be stronger Dash-demand evidence with nothing built.

Four independent dimensions per entry.

| Dimension | Values |
| --- | --- |
| **Dash demand provenance** | Named Dash party requesting; Dash party building; Dash community discussion; inferred from adjacent need; none |
| **Implementation maturity** | Deployed elsewhere; building elsewhere; specified; concept only |
| **Source independence** | Independent of this project; adjacent; solicited by this project |
| **Confidence** | High, medium, low, with the uncertainty stated |

Hypothetical applications do NOT enter the corpus. They go to a separate scenario and stress-test
appendix, used to probe whether an option breaks under load, never to establish demand.

## Recorded per entry

Workload shape and expected frequency. Fan-out. Long-lived obligations. Asynchronous operations and
whether they must span blocks. Native state accessed. Authority required. And whether new instances
would require a protocol release, which is the B1 discriminator.

## Criteria, vetoes, and weights

CORRECTED from version 1, which deferred weighting until after the corpus. That is another post-evidence
degree of freedom.

**Vetoes, not criteria.** These cannot be outweighed by convenience. An option failing any of them is
excluded regardless of its score.

- Consensus safety. Any option that cannot execute identically on every validator is out.
- The binding platform properties B1 through B7 in the requirements register.
- Any option requiring a new mandatory trusted party.

**Weighted criteria.** The owner approves weights or a priority ordering BEFORE the corpus is assembled.

- What each option makes possible that others do not.
- Trust introduced or removed.
- Developer autonomy and composability.
- Security surface, governance burden, upgrade and operating cost.
- Time to first useful capability.
- Reversibility.

**Scoring.** Each criterion scored 0 to 4 against stated anchors. UNKNOWN is a permitted value and is
never silently treated as zero, since a missing measurement is not a bad one. Any option carrying more
than two UNKNOWNs on weighted criteria cannot be recommended as PURSUE without those being resolved.

**Sensitivity analysis is mandatory.** The result is recomputed across a plausible range of weights. If
the ordering changes within that range, the finding is that the evidence does not separate the options,
and it is reported that way rather than as a winner.

## The result

CORRECTED from version 1, whose three outcomes collapsed the option space, dropped A3 entirely, and
never mentioned admission.

The result states four things.

1. **Capability finding.** A single option from A1 to A4, or an unresolved SET the evidence cannot
   separate.
2. **Admission finding.** A compatible position on B1, B2 and B3, constrained by the matrix above.
3. **Action status.** PURSUE, DEFER, DO NOT PURSUE ON CURRENT EVIDENCE, or INCONCLUSIVE. The third is
   deliberately distinct from rejection, since evidence can fail to support a direction without
   establishing that the direction is wrong.
4. **Confidence, unresolved evidence, and reversal conditions.**

## Reversal conditions

- A Dash party building or requesting something no lower capability option serves.
- Absence of programmability becomes a stated cause of loss, traceable to named parties.
- The chosen option is attempted and its costs prove worse than an alternative.
- Any PURSUE reverses if its supporting corpus proves to rest mainly on solicited or low-confidence
  entries.
- Any DO NOT PURSUE reverses on new sourced demand, and it must be revisited on a recorded date.

## How results inform the rest of the project

CORRECTED from version 1, which said the corpus populates H1 and makes it stop being a guess. A
qualitative corpus INFORMS these. It does not measure them.

- **H1**, the application mix, gains sourced candidate shapes. It remains a hypothesis, expressed as
  ranges weighted by expected use, with source and confidence preserved per estimate.
- **H2**, fan-out, gains observed or stated per-application figures, again as ranges. The schedulability
  defect appeared at fan-out 64, and the corpus can say whether that is plausible, not how often it
  occurs.
- **P3**, splittable terminal work, gains the rights models real applications need.
- **P10**, the authority model, gains the authority column.

No estimate derived here is recorded without its source, its range, and its confidence.

## Sequence, with two freezes

1. The owner approves this method, including discovery protocol, evidence dimensions, vetoes, weights,
   scoring rules, and decision authority.
2. **FREEZE ONE.** The method is committed at a named version.
3. The corpus is assembled and characterized, with no evaluation of architectures.
4. Coverage, independence, duplicates and exclusions are reviewed.
5. **FREEZE TWO.** The corpus is committed.
6. Only then are applications mapped to capability and admission options and scored.

Steps 3 onward do not begin before freeze one. Step 6 does not begin before freeze two.
