# Direction Decision 0, method

Dated 2026-08-30. This document specifies HOW the direction question will be answered. It deliberately
reaches no conclusion and contains no application corpus.

The separation is the point. Criteria written after the evidence arrives describe the evidence. So the
method is committed first, approved by the owner, and only then is the corpus assembled and scored. A
later reader should be able to check that the criteria predate the answer by reading the commit history.

## Why this is not called a gate

The five gates test whether a candidate engine meets a technical requirement. This question is different
in kind. It asks what Dash should do, which is partly a product-direction and community-value judgment
rather than a measurement. Calling it Gate 0 would dress a judgment as a test, and the project has
already corrected several claims that did exactly that.

It is a DIRECTION DECISION. The owner decides it. The method's job is to make the decision informed,
reversible, and legible, not to make it automatic.

## The question, on two axes

An earlier framing offered four alternatives in one list, which conflated two independent questions.
Whether a general virtual machine is warranted is about CAPABILITY. Whether programs are deployed freely
is about ADMISSION. A restricted general VM and a permissionless native catalog are both coherent, and a
single list cannot express either.

**Axis A, capability. What may be computed under consensus?**

| A1 | No new consensus computation. Platform keeps storing, indexing, proving and settling |
| A2 | A finite catalog of native operations, extended only by protocol release |
| A3 | A constrained domain-specific language or policy engine, expressive but bounded by construction |
| A4 | A general-purpose virtual machine |

**Axis B, admission. Who may introduce new behavior, and how?**

| B-i | Protocol-defined only. New behavior requires a protocol release |
| B-ii | Governance-approved programs. Deployment exists but is gated |
| B-iii | Permissionless deployment |

The cells are not all meaningful. A1 admits only B-i. But A3 with B-ii is a real option nobody has
costed, and so is A4 with B-ii held permanently rather than as a launch phase.

## This reopens P8, explicitly

P8 currently records the deployment permission model as DECIDED, governance-gated at launch and
permissionless later. That decision assumed the capability question was already settled in favour of a
general VM, with admission a rollout detail.

If axis B is genuinely open, P8 cannot stand as decided. Direction Decision 0 therefore REOPENS P8
rather than quietly contradicting it, and P8's state changes to reflect that. The specific thing P8
assumed and that is now in question is whether permissionless deployment is the eventual destination or
one option among three.

Recording this rather than letting two documents disagree is the whole reason the decision register
exists.

## Application selection, predeclared

Applications are the evidence base, so how they are chosen decides the answer. The rules are fixed here.

- **Selection is by SOURCE, not by appeal.** An application enters the corpus because someone asked for
  it, is building it, or is affected by its absence. It does not enter because it illustrates a point.
- **Every entry names its source and its evidence grade** from the scale below. Entries that cannot be
  sourced are excluded rather than downgraded.
- **The corpus must include applications that ARGUE AGAINST a general VM**, meaning things a finite
  native catalog serves adequately. A corpus containing only cases that need generality has selected its
  answer.
- **No minimum count.** A small corpus is an honest finding about demand, and padding it would destroy
  the only signal it carries.

## Evidence grades for demand

Separate from the project's existing grades, which describe technical claims. These describe demand.

| Grade | Meaning |
| --- | --- |
| **DEPLOYED** | Someone runs this today, on Dash or elsewhere, and the need is observable |
| **BUILDING** | Active development exists, with code or a funded effort |
| **REQUESTED** | A named party has asked for it, traceable to a request rather than to inference |
| **PROPOSED** | It appears in a specification, proposal, or roadmap without a named requester |
| **HYPOTHETICAL** | Constructed to illustrate. Admissible, but may never carry a decision on its own |

The last row is the one that matters. A direction supported only by hypothetical applications is a
direction supported by imagination, and the method should make that visible rather than let volume
disguise it.

## What is recorded per application

Each entry records these, because they are the inputs H1, H2 and P3 need, and gathering them once is the
difference between this exercise informing the rest of the project and being a detour.

- Workload shape and expected frequency.
- Fan-out, meaning how many parties or records a single action touches.
- Long-lived obligations created, if any.
- Asynchronous operations required, and whether they must span blocks.
- Native state accessed, and whether read, write, or both.
- Authority required, meaning whose identity the logic must act under.
- **Whether new instances of this application would require a protocol release.** This is the axis-B
  discriminator and it should be answered for every entry.

## Decision criteria, and who weighs them

The criteria are fixed here. The WEIGHTING is the owner's, and that division is deliberate, because a
method that also fixed the weights would be making the decision rather than informing it.

- What each option makes possible that the others do not.
- What trust each option introduces or removes, including any new privileged role.
- Developer autonomy and composability, meaning whether builders can proceed without the protocol's
  permission.
- Security surface, governance burden, upgrade cost, and ongoing operating cost.
- Time to first useful capability.
- Reversibility, meaning how hard each option is to walk back.

## Outcomes

Three, declared before scoring.

**PURSUE.** The corpus shows demand that a finite native catalog cannot serve, at a grade above
hypothetical, and the costs are judged acceptable. The engine work resumes with its existing gates.

**DEFER.** Demand is real but thin, or concentrated in applications a native catalog could serve. The
execution-layer research pauses, the native-catalog arm is developed instead, and a reversal condition
is recorded with a date to revisit.

**REJECT.** The corpus does not support general programmability. The research concludes with that
finding, which is a genuine result and should be published as one rather than treated as a failure.

## Reversal conditions

Recorded now, so that a later change of direction is a decision rather than a drift.

- A DEPLOYED or BUILDING application appears that a native catalog cannot serve.
- Dash's competitive position changes such that absence of programmability becomes a stated cause of
  loss, traceable to specific parties rather than inferred.
- The native catalog arm is attempted and its own costs prove worse than the general option.
- A PURSUE outcome reverses if the corpus that justified it turns out to have been mostly hypothetical.

## How the results feed the rest of the project

Not a by-product. This is a reason to do the exercise even if the direction is never in doubt.

- **H1**, the application mix, is populated directly by the corpus and stops being a guess.
- **H2**, fan-out distributions, comes from the per-application fan-out figures, and the schedulability
  defect was found at a fan-out this corpus would tell us is common or pathological.
- **P3**, what terminal work may be split across blocks, depends on the rights model of real
  applications and cannot be decided sensibly without them.
- **P10**, the authority model, is informed by the authority column, since whose identity programs must
  act under is an application property before it is a design choice.

## Sequence

1. This method is committed and approved by the owner. No corpus work begins first.
2. The corpus is assembled against the selection rules, each entry graded and recorded.
3. The options on both axes are scored against the criteria, with the owner weighting.
4. An outcome is recorded with its reversal conditions, and P8 is re-decided or re-affirmed explicitly.
5. H1, H2, P3 and P10 are updated from the corpus.

Steps 2 onward do not start until step 1 is approved.
