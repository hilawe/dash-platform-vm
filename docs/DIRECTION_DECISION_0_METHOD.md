# Direction Decision 0, method, version 3 (DRAFT, awaiting owner approval)

Dated 2026-08-30. Version 3 revises version 2 after review. It specifies HOW the direction question will
be answered, reaches no conclusion, and contains no application corpus.

STATUS: DRAFT. Not approved. No corpus work begins until the owner approves this document and its
weights, after which it is frozen at a named version.

## What this decides, and what it cannot

This decides what the RESEARCH RECOMMENDS. Whether Dash adopts anything is decided by Dash's governance
and its maintainers, who are bound by nothing here. The owner's authority is over the recommendation,
the weights, and what this project spends effort on.

## Why this is not a gate

The five gates test whether a candidate meets a technical requirement. This weighs demand, cost and
risk, which is judgment rather than measurement.

## Axis A, capability

| A1 | No new consensus computation |
| A2 | A finite catalog of native operations |
| A3 | A constrained domain-specific language or policy engine, bounded by construction |
| A4 | A general-purpose virtual machine |

## Axis B, admission, three separate questions

| | Options |
| --- | --- |
| **B1. Who may ADD new behavior** | Protocol release only; governance approval; permissionless |
| **B2. Who may INSTANTIATE or INVOKE existing behavior** | Protocol-restricted; governance-approved callers; permissionless |
| **B3. Who may UPGRADE or DISABLE it** | Protocol release; governance; the deploying party |

### Compatibility matrix

| Capability | B1 add | B2 invoke | B3 change |
| --- | --- | --- | --- |
| A1 | Protocol only, by definition | Not applicable | Protocol only |
| A2 | Protocol only, by definition | Any of the three. Permissionless INVOKE of a native catalog is coherent | Protocol only |
| A3 | Any of the three | Any of the three | Any of the three |
| A4 | Any of the three | Any of the three | Any of the three |

## This reopens P8

P8 recorded deployment as governance-gated then permissionless. That assumed capability was settled at
A4 and admission was a schedule. Both are open, so P8 is reopened and re-decided when this concludes.

## Veto applicability

CORRECTED from version 2, which applied the binding requirements B1 through B7 as universal vetoes.
That was a preselection defect. B7 requires PROGRAMS to write native indexed collections, and A1 has no
programs, so applying it universally would eliminate the baseline options by requiring a property they
deliberately do not need, handing the result to A3 and A4 before any evidence existed.

**NOT APPLICABLE is not PASS.** An option that never faces a requirement is not credited with meeting
it, and the distinction is recorded explicitly in the scoring sheet.

| Requirement | A1 | A2 | A3 | A4 | Class |
| --- | --- | --- | --- | --- | --- |
| Consensus determinism (B2) | Applies | Applies | Applies | Applies | Universal |
| Native state keeps its proofs (B1) | Applies | Applies | Applies | Applies | Universal |
| Existing Platform state survives (B3) | Applies | Applies | Applies | Applies | Universal |
| No new mandatory trusted party | Applies | Applies | Applies | Applies | Universal |
| Atomic effect batching (B4) | Not applicable | Applies | Applies | Applies | Native-operation |
| Work bounded before performed (B5) | Not applicable | Applies | Applies | Applies | Native-operation |
| Queued obligations schedulable (B6) | Not applicable | Applies if the catalog creates obligations | Applies | Applies | Native-operation |
| Programs write native indexed collections (B7) | Not applicable | Not applicable | Applies | Applies | Program-execution |

An UNKNOWN on an applicable veto is BLOCKING. It is never resolved in the option's favour.

## Corpus discovery, reproducible

The corpus report must record all of the following, because a claim about absence of demand is only as
good as the search behind it.

- The exact forums, repositories, issue trackers, proposal systems and channels searched, named
  individually rather than by category.
- The search terms used and the date range applied to each source.
- Which teams or individuals were approached directly, and why each was selected.
- The count of INDEPENDENT Dash parties behind each need, separate from the total request count.
- Exclusions, duplicates, sources that could not be accessed, and approaches that received no response.
- The criteria used to judge coverage adequate, stated before the search and assessed after it.

**Window.** Activity in the preceding 24 months, plus older items still ACTIVELY PURSUED, which requires
at least one observable: commits in the last 12 months, current funding, a live proposal, or a named
party confirming continued work. Anything else is historical and recorded as such.

**Deduplication.** One entry per distinct need. Several requests from one organization are ONE entry,
with request count recorded as intensity and independent party count recorded as breadth.

**INSUFFICIENT COVERAGE CANNOT PRODUCE A NEGATIVE FINDING.** A thin search yields INCONCLUSIVE or
DEFER, never DO NOT PURSUE.

## Corpus composition

Sourced applications are included without regard to which option they favour. Entries are characterized,
not classified. Mapping to A and B options happens only after the corpus freezes.

Hypotheticals do not enter the corpus. They go to a scenario and stress-test appendix, used to probe
whether an option breaks, never to establish demand.

No minimum count. A small well-searched corpus is an honest finding.

## Evidence dimensions, anchored

| Dimension | Values and definitions |
| --- | --- |
| **Dash demand provenance** | NAMED PARTY BUILDING, a specific Dash party with code or funding. NAMED PARTY REQUESTING, a specific party has asked, traceable to a message or proposal. COMMUNITY DISCUSSION, raised publicly without a named party committing. INFERRED, no direct request, need deduced from an adjacent one. NONE |
| **Implementation maturity** | DEPLOYED ELSEWHERE, running in production on another network. BUILDING ELSEWHERE. SPECIFIED, a written spec exists. CONCEPT ONLY |
| **Source independence** | INDEPENDENT, the source has no relationship to this project or its author. ADJACENT, the source is a Dash contributor or a party this project has previously worked with, which is disclosed rather than disqualifying. SOLICITED, this project asked for the input |
| **Confidence** | HIGH, primary source read directly and unambiguous. MEDIUM, primary source read but partly ambiguous, or a reliable secondary source. LOW, secondary or inferred, or the source may be stale. Every entry states the specific uncertainty rather than only the level |

## Recorded per entry, architecture-free

CORRECTED from version 2, which asked whether new instances would require a protocol release. That
answer depends on which architecture is chosen, so recording it before mapping violates the two-freeze
rule. Only application FACTS are recorded before freeze two.

- Workload shape and expected frequency.
- Fan-out, meaning parties or records touched by one action.
- Long-lived obligations created.
- Asynchronous operations, and whether they must span blocks.
- Native state accessed, read or write.
- Authority required, meaning whose identity the logic must act under.
- **How often genuinely new behavior is needed**, as distinct from new instances of existing behavior.
- **Whether instances differ only by configuration**, or require new logic.
- **Who needs to create, invoke, modify, or stop an instance.**
- **How quickly behavior must be able to change.**

Whether those facts imply a protocol release is decided in post-freeze mapping, not here.

## Criteria, atomic and anchored

Version 2 bundled security, governance, upgrade and operating cost into one score. Split below. Each is
scored 0 to 4, or UNKNOWN.

**C1. Coverage of sourced Dash needs and uniquely enabled capabilities. Weight 25.**
Measures SOURCED needs served, not abstract expressiveness, because scoring expressiveness hands A4 the
result automatically.
0, serves no sourced need. 1, serves sourced needs that a lower-capability option also serves. 2, serves
at least one sourced need uniquely, provenance INFERRED or COMMUNITY only. 3, serves at least one
sourced need uniquely with a NAMED REQUESTING party. 4, serves several sourced needs uniquely, at least
one with a NAMED BUILDING party.

**C2. Trust removed or introduced. Weight 20.**
0, introduces a new mandatory trusted party (also a veto). 1, introduces a discretionary trusted role.
2, neutral. 3, removes a discretionary trust dependency. 4, removes a trust dependency users currently
must accept.

**C3. Security surface and auditability. Weight 20.**
0, unbounded new attack surface with no audit path. 1, large new surface, auditable only by specialists.
2, moderate surface with established audit practice. 3, small surface, most of it existing reviewed
code. 4, no new consensus-critical surface.

**C4. Governance, upgrade and operating burden. Weight 15.**
0, every change needs consensus governance and there is a standing operational duty with no tooling. 1,
frequent governance involvement. 2, periodic governance, ordinary operations. 3, rare governance, low
operations. 4, no ongoing governance or operational burden beyond what exists today.

**C5. Developer autonomy and composability. Weight 10.**
0, every new capability needs a protocol release. 1, needs governance approval per capability. 2, needs
governance approval once per class. 3, builders proceed independently within bounds. 4, builders proceed
independently and compose freely.

**C6. Reversibility. Weight 5.**
0, effectively irreversible once live. 1, reversible at high cost with user impact. 2, reversible with a
migration. 3, reversible by disabling, state retained. 4, trivially reversible.

**C7. Time to first useful capability. Weight 5.**
0, longest among the options by a wide margin. 2, mid-range. 4, shortest. Scored relatively, since
absolute estimates at this stage would be invented.

## Proposed weights, AWAITING OWNER APPROVAL

| Criterion | Weight |
| --- | ---: |
| C1 Coverage of sourced needs | 25 |
| C2 Trust | 20 |
| C3 Security surface | 20 |
| C4 Governance and operating burden | 15 |
| C5 Developer autonomy | 10 |
| C6 Reversibility | 5 |
| C7 Time to capability | 5 |

Vetoes sit outside this weighting and cannot be traded against it.

## From scores to a finding

1. Eliminate any option failing an APPLICABLE veto. An unknown on an applicable veto is blocking.
2. No capability credit for hypothetical future uses. C1 counts sourced needs only.
3. A more capable option may be recommended only if it uniquely serves at least one sourced,
   non-INFERRED need. Capability that nothing sourced requires is not an advantage.
4. Recommend PURSUE only if the option ranks first across the whole approved sensitivity range.
5. If ranking changes within that range, report an UNRESOLVED SET rather than a winner.
6. An UNKNOWN on any criterion weighted 15 or above blocks a PURSUE recommendation for that option.
   This replaces version 2's arbitrary two-unknown rule with an importance-based one.

**Sensitivity analysis is mandatory**, recomputing across a plausible weight range around the approved
values.

## The result

1. **Capability finding.** One of A1 to A4, or an unresolved set.
2. **Admission finding.** A compatible position on B1, B2 and B3.
3. **Action status.** PURSUE, DEFER, DO NOT PURSUE ON CURRENT EVIDENCE, or INCONCLUSIVE.
4. **Confidence, unresolved evidence, and reversal conditions.**

## Reversal conditions

- A Dash party building or requesting something no lower-capability option serves.
- Absence of programmability becomes a stated cause of loss, traceable to named parties.
- The chosen option is attempted and costs more than an alternative.
- A PURSUE reverses if its corpus proves to rest mainly on SOLICITED or LOW-confidence entries.
- A DO NOT PURSUE reverses on new sourced demand, and is revisited on a recorded date.

## How results inform the rest of the project

A qualitative corpus INFORMS these. It does not measure them. Every derived estimate carries source,
range and confidence.

- **H1**, application mix, gains sourced candidate shapes and stays a hypothesis.
- **H2**, fan-out, gains per-application figures as ranges. The schedulability defect appeared at
  fan-out 64, and the corpus can say whether that is plausible, not how often it occurs.
- **P3**, splittable terminal work, gains the rights models real applications need.
- **P10**, the authority model, gains the authority column.

## Sequence, with two freezes

1. Owner approves this method AND the weights.
2. **FREEZE ONE**, committed at a named version.
3. Corpus assembled and characterized, architecture-free, with the discovery report.
4. Coverage, independence, duplicates and exclusions reviewed.
5. **FREEZE TWO**, corpus committed.
6. Only then are applications mapped to options and scored.
