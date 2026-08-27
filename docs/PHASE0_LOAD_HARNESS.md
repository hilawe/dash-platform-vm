# Phase 0 load harness, specification

Written 2026-08-09, and RUN the same day. This began as a specification rather than a script that
ran, because the devnet was wedged and, per the pre-commit playbook's rule 7, the reactive repair
loop was stopped and the harness written down instead of repaired a third time. On owner
authorisation the network was reset and rebuilt, an identity was funded and registered, and the
harness ran end-to-end. The results are in `PHASE0_FINDINGS.md` under "Load harness results,
2026-08-09". This document is retained as the harness specification and the record of how the run
was reached. The working script and its scratch dependencies live in session scratch, not the repo.

## What it establishes

Two execution-produced numbers that source reading cannot give, both denominated against a real running
Platform v4.0.0:

1. The interior split of the consensus budget under load: of the propose window (3 seconds at round
   zero, confirmed from source and the live node), how much is block assembly, execution, state-root
   computation, and gossip, once blocks actually carry state transitions.
2. A live `OperationCost` vector for real state access (the five fields: seeks, added/replaced/removed
   bytes, loaded bytes, node hashes, Sinsemilla hashes), for a set of representative operations, as the
   closest available proxy for VM-shaped state access.

## What it does NOT establish, its trust boundary

- It does not measure a VM, because none exists. It measures existing document, contract, and identity
  operations as proxies, and the mapping from those to VM state access is an argument, not a
  measurement.
- It does not establish an execution-abort bound. The fee meter is an accounting layer, and Tenderdash
  block gas is unlimited, so a hard resource ceiling remains a VM-layer design obligation regardless of
  what this harness measures (see `PHASE0_FINDINGS.md`, credit-meter section).
- Numbers are specific to this machine's masternode hardware and the dashmate local profile, so they are
  order-of-magnitude and shape evidence, not mainnet figures. Real masternode hardware distribution is a
  separate, still-open Phase 0 item.

## Inputs, and which side owes each

Status after the 2026-08-09 run: all four inputs were SATISFIED and the run completed. Recorded here as
the checklist a future run reproduces.

- A PRODUCING network. SATISFIED. The `local` group was soft-reset and rebuilt on 2026-08-09; Platform
  now produces blocks (heights 1, 2, ...) with all three validators stable at zero restarts, on a fresh
  Core chain. Idle empty-block cadence measured at ~195s (block 1 at 18:23:07, block 2 at 18:26:22),
  which confirms the ~180s heartbeat is a real property rather than a wedge artifact.
- drive-abci metrics ENABLED and SERVED. Confirmed live: local_1's metrics endpoint on host port 49090
  returns `abci_last_finalized_height` and the block metrics. Enabled on all three nodes.
- An SDK to issue state transitions. INSTALLED (pshenmic's `dash-platform-sdk` 1.4.0, in the scratch
  harness dir). It rejects `network: 'local'`, but its `SDKOptions` accept a custom `dapiUrl`, so it
  can target the local DAPI by passing a testnet network tag plus the local DAPI gRPC endpoint. It is
  Platform-only (the Core-side asset lock goes through Core RPC, not the SDK) and its input validation
  is happy-path, so the harness wraps calls defensively and asserts on returned fees.
- A FUNDED Platform identity. SATISFIED in the run. Funded a mnemonic wallet's account address from
  the seed Core wallet (~16,034 DASH available), then the official `dash` 7.0.0 SDK built the Core
  asset lock and registered the identity. The asset-lock step is the one piece pshenmic's SDK does not
  cover, so the official SDK was used.

## Behaviour when an input is absent or malformed

These are the harness's refusal rules, which held during the run (it asserted a confirmed wallet balance
and advancing height before recording anything).

- Network absent or non-producing: the harness must REFUSE to emit numbers, not emit zeros. A frozen or
  stalled chain produces empty blocks whose timing is the idle heartbeat (measured at ~180s), which is
  not the execution slice and must never be reported as one. The harness checks that height advances and
  that blocks carry the transactions it submitted before recording anything.
- Metrics disabled: the execution-slice split is UNAVAILABLE, not estimated. Report it as blocked.
- Identity funding fails: no operations issue, so no cost vectors. Report blocked, do not fall back to
  source-derived numbers relabelled as measured.
- The evidence grade of every output is stated: execution-produced only for numbers from a controlled
  run against a producing network, repository-resolved otherwise.

## Output

A table, one row per operation type exercised (document create, replace, delete; contract create;
identity top-up), each with its `OperationCost` vector and the resulting storage and processing fees,
cross-checked against the committed test-assertion figures already in `PHASE0_FINDINGS.md` (a document
delete at 71,994,700 processing credits is the anchor). Plus a per-phase timing breakdown of the propose
window under sustained submission. Each figure tagged execution-produced with the candidate commit and
the run transcript digest.

## Blocker history, all resolved

The devnet had been wedged: Platform consensus stuck at height 4958 since Aug 5 because two of three
Tenderdash validators crash-looped ~13k times each, below the 2/3 quorum. A clean restart did not
fix it. On owner authorisation the `local` group was soft-reset and rebuilt, which cleared the crash
loop (validators at zero restarts) and restored block production with metrics served. Identity
funding, the last crux, was then done with the official `dash` 7.0.0 SDK, and the run completed.
Results are in `PHASE0_FINDINGS.md`.

## To run, from the current state

Steps 1 to 3 are DONE as of 2026-08-09 (network producing, metrics served, SDK installed in the scratch
harness dir). The run resumes at step 4.

1. DONE. Local group reset and rebuilt; height advances; validators stable at zero restarts.
2. DONE. Metrics served (local_1 host port 49090 returns `abci_last_finalized_height`).
3. DONE. `dash-platform-sdk` 1.4.0 installed in the scratch harness dir; target local via a custom
   `dapiUrl` since it rejects `network: 'local'`.
4. DONE. Funded a mnemonic wallet's account address from Core; the official `dash` 7.0.0 SDK (matches
   Platform 4.0) built the asset lock and registered the identity. pshenmic's SDK rejects the local
   network and does not build asset locks, so the official SDK was used for the run.
5. DONE. Registered a data contract and created five documents; per-operation fee measured as the
   identity credit-balance delta (SDK-agnostic). The raw `OperationCost` vector is not exposed, so the
   aggregate credit fee is what was captured.
6. DONE. Scraped the drive-abci ABCI request-duration metrics across the load window and correlated with
   Tenderdash block timestamps for the phase split (~158 ms execution slice per block under 1-tx load).
7. DONE. Results written into `PHASE0_FINDINGS.md`, tagged execution-produced, cross-checked against the
   committed test figures.
