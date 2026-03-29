# Vibe Everywhere Iteration Specs v4

Last updated: 2026-03-29

Version note:

- This file is the versioned detailed iteration plan for roadmap epoch `v4`.
- The concise lookup view lives in [`v4-summary.md`](./v4-summary.md).
- The planning workflow rules live in [`../process.md`](../process.md).

## Purpose

This epoch moves the control app from “a capable control console” back toward “a coherent
session-first product.” The goal is not to add more top-level surfaces, but to make the main user
journey easier to understand and complete without bouncing between routes.

Use this file together with:

- [`../../../PLAN.md`](../../../PLAN.md): concise execution record, completion log, verification
  log, and decision log
- [`../../../AGENTS.md`](../../../AGENTS.md): repository-level engineering, workflow, release,
  testing, and documentation guardrails

## Roadmap Overview

| Iteration | Title | Status | Depends On |
| --- | --- | --- | --- |
| 14 | Session-First Primary Workflow Productization | completed | Iteration roadmap `v3` |

## Shared Guardrails

- Do not isolate relay connection and device-selection prerequisites on a separate top-level route
  when AI session launch is the primary user journey.
- Keep deployment metadata, governance context, and other management-only information out of the
  everyday user path unless it directly helps the session workflow.
- Keep terminal and preview available as fallback tools, but do not make them necessary to
  understand session progress or session results.

## Iteration 14: Session-First Primary Workflow Productization

### Goal

Make the control app's primary workflow obvious and contiguous: connect the relay, choose a
device, launch an AI session, and review the resulting workspace/Git context from the same route.

### User-Visible Outcome

- `Sessions` becomes the clear primary workflow surface.
- Relay connection, device choice, session launch, event review, workspace browse, and Git review
  no longer require a separate top-level `Connections` page.
- `Devices` remains available for runtime inspection and now carries deployment/current-client
  metadata plus optional governance context.
- `Advanced` remains focused on terminal and preview as explicit exception paths.

### In Scope

- remove `Connections` from top-level navigation and redirect the legacy route into `Sessions`
- integrate relay URL/token entry, locale/theme controls, and workflow context into the `Sessions`
  surface
- keep device selection, session launch, current-session review, Git review, event stream, and
  workspace browsing on the same route
- move deployment metadata and optional governance/audit visibility into the secondary `Devices`
  view
- update localized copy, README files, `TESTING.md`, release notes, plan indexes, and `PLAN.md`
  to match the new product model

### Out Of Scope

- no new relay/agent transport model or API family
- no full diff approval, merge supervision, or code-edit review workflow
- no major backend modularization pass
- no new enterprise auth/session model

### Acceptance Criteria

- top-level navigation exposes `Sessions`, `Devices`, and `Advanced`, with the old
  `#/connections` route redirecting into `Sessions`
- the `Sessions` route contains the main user path: relay connection, device choice, session
  creation, session review, Git review, and workspace browse
- the result-review area shows changed files, recent commits, Git counters, and the current session
  narrative before the user needs terminal fallback
- `Devices` still supports runtime inspection and clearly contains deployment/current-client
  metadata rather than leaving it inaccessible
- README, testing, release-note, plan, and guardrail docs no longer describe a separate top-level
  `Connections` section as part of the current product model

### Validation

- `cargo check -p vibe-relay -p vibe-agent -p vibe-app`
- `cargo test --workspace --all-targets -- --nocapture`
- `cd apps/vibe-app && npm run build`
- `./scripts/dual-process-smoke.sh relay_polling`
- `./scripts/dual-process-smoke.sh overlay`
- update the frontend manual checklist in `TESTING.md`
- note manual Web/Desktop/Android UI regression as still required when not executed locally

### Iteration Record

- Chosen implementation mode:
  `user-specified session-first primary flow with medium-strength UI restructuring`
- Implementation notes:
  removed the `Connections` nav item, redirected the legacy route into `Sessions`, rebuilt the
  `Sessions` page around relay connection plus session launch/review, shifted deployment/current-
  client and governance context into `Devices`, and updated docs/notes/guardrails to match the new
  workflow.
- Validation results:
  `2026-03-29`: `cargo check -p vibe-relay -p vibe-agent -p vibe-app` succeeded after the
  session-first workflow productization changes.
  `2026-03-29`: `cargo test --workspace --all-targets -- --nocapture` succeeded after the
  session-first workflow productization changes.
  `2026-03-29`: `cd apps/vibe-app && npm run build` succeeded after the session-first workflow
  productization changes.
  `2026-03-29`: `./scripts/dual-process-smoke.sh relay_polling` succeeded after the session-first
  workflow productization changes.
  `2026-03-29`: `./scripts/dual-process-smoke.sh overlay` succeeded after the session-first
  workflow productization changes.
- Remaining follow-up:
  interactive Web, Tauri Desktop, and Android manual regression was not run in this local turn, so
  visual and runtime QA across those clients remains required before release.
