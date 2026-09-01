# Direction Decision 0, discovery manifest (DRAFT, awaiting owner approval)

Dated 2026-08-30. Companion to `docs/DIRECTION_DECISION_0_METHOD.md` version 5. This names WHERE to
search, WITH WHAT, and WHAT COUNTS AS ENOUGH, before any searching happens.

STATUS: DRAFT. No searching has begun. This freezes with the method as FREEZE ONE.

Its purpose is narrow and worth stating. A claim that demand is absent is only as strong as the search
behind it, and a search described afterwards cannot be distinguished from a search shaped to its result.
So the search is written down first, and departures from it become visible.

## Sources, named individually

### Dash governance and proposals

| Source | What is searched | Range |
| --- | --- | --- |
| Dash governance proposals, on-chain and the proposal portal | Proposal titles and bodies | 24 months to the cutoff |
| Dash Improvement Proposals repository | All DIPs and their pull request discussion | All, since the corpus is small |
| Dash Trust Protectors and Dash Core Group public reporting | Published reports and roadmaps | 24 months |

### Code and issue trackers

| Source | What is searched | Range |
| --- | --- | --- |
| `dashpay/platform` issues and discussions | Open and closed | 24 months |
| `dashpay/dash` issues and discussions | Open and closed | 24 months |
| `dashpay/dips` issues and pull requests | All | All |
| Public repositories depending on the Dash Platform SDKs, by dependency search | Repository purpose and README | 24 months of activity |

### Community channels

| Source | What is searched | Range |
| --- | --- | --- |
| Dash forum | Threads in development and proposal categories | 24 months |
| Dash public Discord, developer channels | Text search where history is accessible | 24 months, subject to access |
| Dash subreddit | Posts flaired or titled for development | 24 months |

### Comparable applications elsewhere

Recorded as FEASIBILITY evidence and never as Dash demand. Searched only to characterize what such
applications require, never to establish that Dash needs them.

| Source | What is searched | Range |
| --- | --- | --- |
| Public application catalogs for smart-contract platforms | Application category and stated purpose | Current |

## Search strings

Applied to each text-searchable source above. Recorded per source with hit counts, including zero.

- smart contract, smart contracts
- programmability, programmable
- scripting, script
- virtual machine, VM, WASM, WebAssembly
- custom logic, business logic, on-chain logic
- escrow, vesting, subscription, recurring payment, streaming payment
- multisig policy, spending policy, spending limit
- DAO, treasury automation, automated payout
- conditional transfer, atomic swap
- oracle, price feed
- token rules, transfer restriction, allowlist, denylist
- marketplace, order book, auction

The last group is deliberately application-shaped rather than technology-shaped, because a party
describing a need rarely names a virtual machine.

## Outreach selection rule

Direct approaches are made to parties selected by this rule, applied mechanically:

1. Every party with a Dash governance proposal in the window mentioning any search string.
2. Every party with a public repository depending on a Dash Platform SDK with activity in the window.
3. Every party named in a DIP as an interested implementer.

No party is approached because they are expected to give a useful answer, and none is skipped because
they are expected not to. The list produced by the rule is recorded in full, including parties who were
approached and did not reply.

## Treatment of inaccessible sources and nonresponses

Decided now rather than when they occur.

- A source that cannot be accessed is recorded as INACCESSIBLE with the reason, and counted against
  coverage rather than ignored.
- A nonresponse after one approach and one follow-up is recorded as NO RESPONSE. It is never recorded
  as an absence of need, since silence is not an answer.
- A source with a partial history, such as a chat archive with limited scrollback, records the extent
  actually searched.

## Minimum coverage for a NEGATIVE finding

A DO NOT PURSUE ON CURRENT EVIDENCE outcome requires ALL of the following. Falling short of any of them
yields INCONCLUSIVE instead.

1. Every source in the governance and code sections above searched, or recorded INACCESSIBLE with a
   reason.
2. Every search string applied to every text-searchable source, with hit counts recorded including
   zeros.
3. The outreach rule applied in full, with every selected party approached and given one follow-up.
4. At least 60 percent of approached parties either responding or explicitly declining. Below that, the
   sample is too thin to support an absence claim.
5. The community channels searched or recorded inaccessible.

Coverage below this threshold does not weaken a POSITIVE finding, since finding demand does not require
having looked everywhere. The asymmetry is deliberate: proving presence needs one good instance, proving
absence needs a thorough search.

## Cutoff

The cutoff date is recorded when discovery begins and is the same for every source. The 24-month window
runs backwards from it. Anything discovered after the cutoff goes to an addendum and does not silently
join the corpus.

## Amendment

Adding a source, a search string, or an outreach criterion after freeze requires a versioned amendment
to this manifest, stating what was missed and why. Amending is expected and fine. Amending silently is
what this document prevents.

## What this manifest does not do

It does not say what the corpus will contain, and it does not anticipate the finding. It does not rank
sources by expected usefulness, since ranking by expectation is how a search finds what it went looking
for.
