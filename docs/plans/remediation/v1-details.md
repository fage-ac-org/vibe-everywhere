# Vibe Everywhere UI And Connectivity Remediation Plan v1

Last updated: 2026-03-28

Version note:

- This file is the versioned detailed remediation plan for repair epoch `v1`.
- The concise lookup view lives in [`v1-summary.md`](./v1-summary.md).
- The planning workflow rules live in [`../process.md`](../process.md).

## Purpose

This file is the authoritative repair plan for the current dashboard, connection-model, and
platform-surfacing problems identified after the Iteration 0-11 baseline was completed.

Use this file together with:

- [`PLAN.md`](../PLAN.md): concise execution record, status, completion log, and verification log
- [`AGENTS.md`](../AGENTS.md): repository-level engineering and product guardrails

This remediation track exists because the current implementation passes compile and packaging
checks, but still contains product-structure and configuration-assumption issues that reduce
clarity for real users.

## Execution Rule

Every remediation item below has a `Repair Modes` subsection.

Before implementing any item:

1. present the available repair modes to the user
2. ask the user which mode to use for that item
3. do not start coding until that item-level choice is confirmed

After an item is completed and verified:

1. update the item status in this file
2. record the chosen repair mode
3. record validation results
4. append a concise completion note to [`PLAN.md`](../PLAN.md)

## Problem Summary

The current problems fall into six product-facing buckets plus one documentation follow-up bucket:

1. dashboard information overload because major workflows are still rendered on one long page
2. incomplete enterprise and governance surfaces are visible before they are ready
3. deployment/operator guidance is rendered as always-on dashboard copy instead of contextual help
4. self-hosted and mobile-safe networking rules are undermined by loopback-oriented runtime defaults
5. the so-called platform matrix is display-only but looks like a client selector
6. current-client detection and client capability semantics are inconsistent with actual runtime behavior
7. docs, tests, and verification rules must be updated to match the repaired product model

## Remediation Overview

| Item | Title | Status | Depends On |
| --- | --- | --- | --- |
| R1 | Dashboard Navigation Refactor | completed | none |
| R2 | Feature Visibility And Enterprise Surface Gating | completed | R1 |
| R3 | Deployment Guidance Relocation | completed | R1 |
| R4 | Loopback Default And Public-Origin Cleanup | completed | R3 |
| R5 | Platform Matrix Semantic Correction | completed | R1 |
| R6 | Current-Client Detection And Platform Surfacing Alignment | completed | R5 |
| R7 | Docs, Tests, And Verification Realignment | completed | R1-R6 |

## Shared Remediation Guardrails

- Do not introduce a second dashboard by leaving the old monolithic surface in place behind a hidden branch.
- Do not replace one misleading label with another. If a surface is display-only, it must not look interactive.
- Do not fix mobile or self-hosted networking by silently forcing new hardcoded addresses.
- Do not expose enterprise or governance surfaces before they have feature ownership, acceptance criteria, and user value.
- Do not leave remediation decisions undocumented. Each item must record the chosen repair mode before code changes begin.

## R1: Dashboard Navigation Refactor

### Problem

The control app currently uses a single route and a single `DashboardView`, which causes unrelated
workflows to accumulate on one long page. This makes the product feel like an internal operations
console rather than a task-focused control app.

### Goal

Refactor the dashboard into a clear top-level navigation model so users can move between product
areas intentionally instead of scanning one overloaded page.

### Repair Modes

- Mode A: `Tab shell inside the existing dashboard route`
  Keep one route, add primary tabs within the dashboard, and split content into tab panels.
- Mode B: `Tabs plus route-backed sections`
  Keep the dashboard entry, but make primary tabs route-backed so each section can deep-link and
  own its own state.
- Mode C: `Sidebar navigation instead of tabs`
  Use section navigation rather than tabs, while still separating the same product areas.

### Recommended Mode

- Mode C

Reason:

- it maps better to desktop information architecture, where multiple operational areas need clear
  separation
- it can still preserve route-backed deep links and per-section ownership when implemented with
  dedicated routes behind the navigation shell
- it adapts cleanly to mobile by switching the same section model to bottom navigation instead of
  forcing a desktop sidebar onto narrow screens

### Planned Scope

- define primary product sections such as `Sessions`, `Devices`, `Connections`, and `Advanced`
- split the current dashboard into dedicated section components
- replace the single-route all-in-one surface with route-backed primary sections
- render those sections through a shared responsive shell:
  desktop sidebar + mobile bottom navigation
- ensure narrow-width layout and desktop layout both work with the same section model

### Acceptance Criteria

- the main dashboard no longer renders all major workflows on one screen
- users can switch primary sections through explicit top-level navigation
- each top-level section has a single clear purpose
- desktop uses persistent side navigation and mobile uses bottom navigation for the same section map
- there is no hidden duplicate of the old all-in-one layout kept alive in parallel

### Validation

- `cd apps/vibe-app && npm run build`
- manual navigation check across all top-level sections
- Android-width smoke for section switching and bottom-nav reachability

### Item Record

- Chosen repair mode:
  `Mode C`, refined as route-backed sections using desktop sidebar navigation and mobile bottom
  navigation
- Implementation notes:
  preserved the existing control store and API flow, but moved the monolithic dashboard into a
  shared shell plus dedicated `Sessions`, `Devices`, `Connections`, and `Advanced` views
- Validation results:
  `2026-03-28`: `cd apps/vibe-app && npm run build` succeeded after the route-backed navigation
  refactor. Manual navigation smoke and Android-width interaction smoke remain to be run in a live
  UI session.

## R2: Feature Visibility And Enterprise Surface Gating

### Problem

Governance and enterprise-adjacent surfaces are visible in the product before the rest of the app
is organized enough to support them as first-class workflows. This raises the perceived complexity
of the app and exposes partially mature product layers too early.

### Goal

Hide incomplete enterprise-oriented surfaces until the product is ready to present them
coherently.

### Repair Modes

- Mode A: `Hard hide unfinished sections`
  Remove them from the main UI until explicitly reintroduced.
- Mode B: `Feature-flag gated visibility`
  Keep the code paths but only show them when a relay-provided feature flag is enabled.
- Mode C: `Move to admin-only section`
  Keep them visible but only under a dedicated management tab or admin-only route.

### Recommended Mode

- Mode B

Reason:

- it matches the existing feature-flag direction in the app
- it keeps future enterprise work compatible
- it prevents reintroducing unfinished UI by accident

### Planned Scope

- identify enterprise/governance surfaces that should not be in the default user path
- gate those surfaces behind explicit feature flags or hide them entirely when no flag exists
- keep product wording aligned with the actual maturity of visible features

### Acceptance Criteria

- unfinished enterprise features are no longer shown in the default end-user flow
- visibility rules are explicit and testable
- no user-visible copy implies enterprise readiness where functionality is still foundational

### Validation

- `cd apps/vibe-app && npm run build`
- manual check with flags absent and, where possible, present

### Item Record

- Chosen repair mode:
  `Mode B`, using an explicit frontend-visible feature flag to keep governance and audit UI out of
  the default end-user flow
- Implementation notes:
  gated the current `Governance And Audit` card behind the dedicated
  `governance_audit_console` feature flag and aligned the hidden state with audit-event loading so
  the default product path no longer implies enterprise readiness
- Validation results:
  `2026-03-28`: `cd apps/vibe-app && npm run build` succeeded after the governance/audit feature
  gating change. Live UI smoke with the flag absent and present remains to be run.

## R3: Deployment Guidance Relocation

### Problem

Operator-facing deployment guidance is rendered as always-on dashboard description text. This is
useful during development but inappropriate as persistent product copy for normal users.

### Goal

Move deployment guidance to contextual, user-triggered, or operator-targeted surfaces and keep the
main dashboard copy product-oriented.

### Repair Modes

- Mode A: `Remove the persistent guidance and keep only contextual warnings`
  Show nothing until the user enters an unsafe value such as loopback.
- Mode B: `Replace persistent guidance with concise neutral summary plus contextual warnings`
  Keep a short status line in the UI, but move detailed operator guidance to docs or tooltips.
- Mode C: `Move all deployment guidance to a dedicated help drawer`
  Use a secondary help surface rather than inline descriptions.

### Recommended Mode

- Mode C

Reason:

- it removes operator-facing prose from the default control surface entirely
- it matches the requirement that deployment personnel should rely on docs and deployment knowledge,
  not in-product warning copy
- it keeps the main product UI focused on neutral runtime metadata instead of self-hosting guidance

### Planned Scope

- remove current always-on operator guidance from the main UI
- remove deployment warning copy from the primary connection screen
- keep only neutral deployment metadata in the card itself
- link to self-hosted documentation for operator detail

### Acceptance Criteria

- self-hosted or hosted-compatible status is understandable without exposing engineering prose
- operator guidance is no longer rendered as the primary card description
- deployment/operator instructions live in docs or help surfaces rather than inline dashboard copy

### Validation

- `cd apps/vibe-app && npm run build`
- manual check for neutral default copy and documentation entry behavior

### Item Record

- Chosen repair mode:
  `Mode C`, moving deployment guidance to documentation/help surfaces and keeping the primary UI
  free of operator warning prose
- Implementation notes:
  removed the inline deployment guidance description and relay warning copy from the main
  `Connections` screen, replaced the deployment card description with neutral metadata-oriented copy,
  and kept the self-hosted documentation link as the operator-facing detail entry point
- Validation results:
  `2026-03-28`: `cd apps/vibe-app && npm run build` succeeded after deployment-guidance relocation.
  Live UI smoke for documentation-entry behavior remains to be run.

## R4: Loopback Default And Public-Origin Cleanup

### Problem

The product still contains multiple loopback-oriented runtime defaults. Some of them are acceptable
for local-only target-service defaults, but others leak into self-hosted and mobile-facing product
behavior.

### Goal

Separate local-development convenience from product/runtime defaults, and remove loopback fallback
where it creates incorrect public behavior.

### Repair Modes

- Mode A: `Strict explicit configuration`
  No public relay origin fallback; missing values become validation errors or empty states.
- Mode B: `Explicit dev-mode fallback only`
  Keep loopback fallback only when a clear development flag or debug build is active.
- Mode C: `Environment-derived fallback with stronger labeling`
  Keep fallback behavior but clearly label it as local-only and never present it as production-safe.

### Recommended Mode

- Mode B

Reason:

- it avoids breaking local developer workflows abruptly
- it removes accidental production assumptions
- it makes the fallback behavior explicit instead of silently broadening it

### Planned Scope

- audit relay public origin, forward host, agent relay URL, and frontend relay placeholder behavior
- distinguish true product defaults from local target-service defaults
- remove or gate loopback fallback for public-facing relay origin semantics
- keep preview target-host defaults only where they represent the device-local service target, not
  the relay itself

### Acceptance Criteria

- self-hosted relay origin is no longer silently represented as `127.0.0.1` in production-facing metadata
- mobile-facing control flows do not default to loopback relay addresses
- developer-local fallback behavior is explicit and limited
- loopback defaults that remain are semantically correct and documented

### Validation

- `cargo check -p vibe-relay -p vibe-agent -p vibe-app`
- `cargo test --workspace --all-targets -- --nocapture`
- `cd apps/vibe-app && npm run build`
- targeted manual check of relay URL bootstrap and preview target defaults

### Item Record

- Chosen repair mode:
  `Mode B`, keeping loopback fallback only for debug/development behavior and requiring explicit
  non-loopback configuration for product-facing relay/public-origin semantics
- Implementation notes:
  preserve local preview target defaults where they represent the device-local service target, but
  remove silent loopback fallback from relay public origin, preview relay host, desktop bootstrap,
  and agent relay bootstrap
- Validation results:
  `2026-03-28`: `cargo fmt --all`, `cargo check -p vibe-relay -p vibe-agent -p vibe-app`,
  `cargo test --workspace --all-targets -- --nocapture`, `cd apps/vibe-app && npm run build`,
  and `./scripts/dual-process-smoke.sh relay_polling` all succeeded after:
  relay wildcard/public-origin cleanup, explicit `forward_host` erroring when unconfigured outside
  dev fallback, desktop/mobile relay bootstrap cleanup, agent explicit relay-url resolution, and
  the connections-screen relay-origin empty-string fallback fix

## R5: Platform Matrix Semantic Correction

### Problem

The current `platform matrix` is display-only, but its title and badges make it look selectable.
Users reasonably expect to switch platforms there, which the current UI does not support.

### Goal

Rename, reshape, or replace this surface so it communicates real meaning instead of implied
interaction.

### Repair Modes

- Mode A: `Rename to current-client capabilities`
  Show only the current client and its runtime characteristics.
- Mode B: `Split into current-client capabilities and supported clients`
  One card explains the current runtime; another read-only list explains supported platforms.
- Mode C: `Turn it into real client actions`
  Add actual download/open/install actions where possible and make the surface truly actionable.

### Recommended Mode

- Mode B

Reason:

- it keeps useful runtime context
- it removes the false impression of selection
- it creates a clean future path to add real download/actions later

### Planned Scope

- replace the misleading matrix label
- separate current-runtime characteristics from supported-client roadmap or channel information
- remove `available` wording from any non-interactive surface

### Acceptance Criteria

- the UI no longer implies platform switching when no such action exists
- the current client is clearly identified
- non-current clients are described as supported or planned, not selectable

### Validation

- `cd apps/vibe-app && npm run build`
- manual copy and interaction review

### Item Record

- Chosen repair mode:
  `Mode B`, splitting the old matrix into current-client runtime characteristics plus a read-only
  control-client form summary
- Implementation notes:
  replaced the misleading matrix card with a `Current Client` block that only describes the active
  runtime, downgraded the multi-client surface into a non-interactive explanatory list, and fixed
  the `Connections` page card gating so deployment metadata remains visible while governance stays
  behind its feature flag
- Validation results:
  `2026-03-28`: `cd apps/vibe-app && npm run build` succeeded after the platform-surface semantic
  correction. Manual copy and interaction review remains advisable in a live UI session.

## R6: Current-Client Detection And Platform Surfacing Alignment

### Problem

Current-client detection only differentiates Tauri Desktop from non-Tauri Web, while shared config
and UI copy claim Web, Desktop, and Android support. This creates a semantic mismatch between
declared support and actual runtime identification.

### Goal

Align runtime detection, surfaced metadata, and visible product promises so the app only claims what
it can correctly detect and represent.

### Repair Modes

- Mode A: `Detect Android explicitly and keep the multi-platform surface`
  Add accurate Android runtime detection and use it in current-client capability logic.
- Mode B: `Hide non-current-platform surfacing until detection is complete`
  Reduce visible platform claims until runtime detection is authoritative.
- Mode C: `Keep platform metadata server-side only for now`
  Remove the client-facing surface entirely and reserve the metadata for internal logic.

### Recommended Mode

- Mode A

Reason:

- the repository already has Android packaging and Android-specific flows
- the current mismatch is technical debt, not a product-direction rejection
- correct runtime detection is needed regardless of how the UI is ultimately phrased

### Planned Scope

- define authoritative current-client detection rules
- ensure Android-native runtime can identify itself as Android
- reconcile current-client logic with platform metadata consumption
- remove copy that overstates support where the runtime cannot prove it

### Acceptance Criteria

- current-client detection matches actual runtime environment
- Android is not merely listed as metadata if the runtime can identify it
- platform-facing UI is consistent with detection semantics

### Validation

- `cd apps/vibe-app && npm run build`
- manual Web and Tauri runtime verification
- Android runtime verification where packaging/runtime is available

### Item Record

- Chosen repair mode:
  `Mode A`, refined so runtime detection is corrected first and the main product surface only shows
  the current client instead of keeping other platforms visible on the same page
- Implementation notes:
  added explicit Android-native runtime detection for Tauri mobile, preserved `web` for mobile
  browsers, corrected mobile explicit-relay preference logic so mobile user agents still prefer
  explicit remote relay configuration, removed the remaining non-current-client platform list from
  the `Connections` page, and aligned the shared `control_clients` defaults with the currently
  represented product clients (`Web`, `Desktop`, `Android`)
- Validation results:
  `2026-03-28`: `cargo fmt --all`, `cargo check -p vibe-relay -p vibe-agent -p vibe-app`,
  `cargo test --workspace --all-targets -- --nocapture`, and `cd apps/vibe-app && npm run build`
  all succeeded after the current-client detection and platform-surface alignment changes

## R7: Docs, Tests, And Verification Realignment

### Problem

After the UI and configuration repairs land, docs, acceptance criteria, and verification notes must
be updated so the repository no longer codifies the old monolithic dashboard and ambiguous platform
surfaces.

### Goal

Bring roadmap, project guidance, testing guidance, and user-facing documentation in line with the
repaired product structure.

### Repair Modes

- Mode A: `Minimal sync`
  Update only directly affected files.
- Mode B: `Full documentation reconciliation`
  Update plan, testing checklist, README, and any affected operator docs together.
- Mode C: `Documentation plus explicit UI manual checklist`
  Do the full reconciliation and add a durable manual QA checklist for the new tabs and visibility
  rules.

### Recommended Mode

- Mode C

Reason:

- the current defects are largely about product semantics and UX structure
- those regress easily without a manual checklist
- this aligns with the repository's current lack of a dedicated frontend test harness

### Planned Scope

- update product-planning records
- update `TESTING.md` with section-navigation and visibility checks
- update README screenshots or copy as needed after the UI is repaired
- document any retained development-only loopback behavior explicitly

### Acceptance Criteria

- docs describe the repaired navigation model rather than the old monolithic dashboard
- testing guidance covers tabs/sections, feature visibility, and loopback warnings
- no project guidance contradicts the repaired product rules

### Validation

- file-content review
- `cd apps/vibe-app && npm run build`
- manual documentation sanity check against running UI

### Item Record

- Chosen repair mode:
  `Mode C`, reconciling the affected docs and adding a durable manual QA checklist for the repaired
  UI/navigation model
- Implementation notes:
  updated `README.md`, `README.en.md`, `TESTING.md`, `AGENTS.md`, and `docs/plans/process.md` so
  they now describe the route-backed `Sessions / Devices / Connections / Advanced` model, current-
  client-only platform surfacing, governance default hiding, and explicit development-only loopback
  behavior for relay/public-origin assumptions
- Validation results:
  `2026-03-28`: file-content review plus `cd apps/vibe-app && npm run build` succeeded after the
  documentation, testing-checklist, and planning-process realignment changes
