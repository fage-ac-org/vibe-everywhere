# Vibe Everywhere Iteration Specs v2

Last updated: 2026-03-28

Version note:

- This file is the versioned detailed iteration plan for roadmap epoch `v2`.
- The concise lookup view lives in [`v2-summary.md`](./v2-summary.md).
- The planning workflow rules live in [`../process.md`](../process.md).

## Purpose

This file records the first post-baseline roadmap epoch after the product baseline in `v1` was
completed.

The current epoch is intentionally narrow: it hardens cross-platform delivery verification without
pretending the GitHub-hosted Linux overlay instability is solved.

Use this file together with:

- [`../../../PLAN.md`](../../../PLAN.md): concise execution record, completion log, verification
  log, and decision log
- [`../../../AGENTS.md`](../../../AGENTS.md): repository-level engineering, workflow, documentation,
  and testing guardrails

## Roadmap Overview

| Iteration | Title | Status | Depends On |
| --- | --- | --- | --- |
| 12 | Delivery Verification Hardening | implemented locally | Iteration roadmap `v1` baseline |

## Shared Guardrails

- Do not delete overlay coverage entirely just to make GitHub-hosted runners look green.
- Do not leave an intentionally non-blocking diagnostic buried inside the required verify job.
- Any intentionally non-blocking diagnostic must live in a clearly named diagnostic job and remain
  recorded here until the underlying issue is either fixed or explicitly retired.
- Add cross-platform verification with runner-native tooling; do not assume a Unix smoke harness
  runs unchanged on Windows.

## Iteration 12: Delivery Verification Hardening

### Goal

Strengthen CI signal across Linux and Windows while honestly tracking the unresolved GitHub-hosted
Linux overlay instability as deferred follow-up work.

### User-Visible Outcome

- CI validates a real Windows relay-plus-agent smoke path instead of only compiling Windows
  artifacts.
- GitHub-hosted Linux `overlay` smoke still runs for signal in both `CI` and `Release`, but it no
  longer blocks mainline verification or packaging.
- The unresolved hosted-runner overlay problem is documented in the roadmap so it is not silently
  forgotten.

### In Scope

- add a Windows-native relay-polling smoke harness
- run that harness in CI on a Windows runner
- keep Linux relay-polling smoke as a blocking verify step
- move GitHub-hosted Linux `overlay` smoke into explicitly named non-blocking diagnostic jobs in
  both `CI` and `Release`
- record the deferred hosted-runner overlay investigation in the iteration record and workflow docs

### Out Of Scope

- no attempt to make GitHub-hosted Linux `overlay` smoke stable in this iteration
- no Windows `overlay` smoke attempt in this iteration
- no product-runtime behavior changes made only to satisfy hosted CI

### Acceptance Criteria

- CI includes a blocking Windows smoke test that starts real `vibe-relay` and `vibe-agent`
  processes and validates relay-polling task execution
- the `CI` and `Release` required verify jobs no longer block on GitHub-hosted Linux `overlay`
  smoke
- the non-blocking Linux `overlay` jobs are separately named as diagnostics and keep producing
  failure signal plus retained artifacts
- the deferred root-cause investigation is recorded in the active iteration plan, `PLAN.md`, and
  workflow/test guardrails

### Validation

- `cargo fmt --all`
- `cargo test --workspace --all-targets -- --nocapture`
- `bash -n scripts/dual-process-smoke.sh`
- inspect `.github/workflows/ci.yml`, `.github/workflows/release.yml`, and
  `scripts/dual-process-smoke.ps1`

### Iteration Record

- Chosen implementation mode:
  `user-specified`
- Implementation notes:
  added a Windows-native PowerShell smoke harness for relay-polling task and shell validation,
  introduced blocking Windows smoke coverage in CI, and moved GitHub-hosted Linux `overlay` smoke
  into explicit non-blocking diagnostic jobs for both `CI` and `Release`.
- Validation results:
  `2026-03-28`: `cargo fmt --all --check` succeeded after the workflow, documentation, and
  planning updates.
  `2026-03-28`: `cargo test --locked --workspace --all-targets -- --nocapture` succeeded after the
  workflow, documentation, and planning updates.
  `2026-03-28`: `bash -n scripts/dual-process-smoke.sh` succeeded.
  `2026-03-28`: `./scripts/dual-process-smoke.sh relay_polling` succeeded after the CI workflow
  restructuring.
  `2026-03-28`: `./scripts/render-release-notes.sh v0.0.0 >/dev/null` succeeded after the release
  note update.
  `2026-03-28`: `pwsh` was not available in the local environment, so
  `scripts/dual-process-smoke.ps1` was reviewed statically but not executed locally.
- Deferred follow-up:
  GitHub-hosted Linux `overlay` instability remains open as a recorded roadmap issue. Do not treat
  the non-blocking diagnostic move as a root-cause fix.
