# Vibe Everywhere Release Hygiene And User Onboarding Remediation Plan v3

Last updated: 2026-03-28

Version note:

- This file is the versioned detailed remediation plan for repair epoch `v3`.
- The concise lookup view lives in [`v3-summary.md`](./v3-summary.md).
- The planning workflow rules live in [`../process.md`](../process.md).

## Purpose

This file is the authoritative repair plan for the release, documentation, and operator-onboarding
problems identified after remediation plan `v2` completed.

Use this file together with:

- [`../../../PLAN.md`](../../../PLAN.md): concise execution record, status, completion log, and
  verification log
- [`../../../AGENTS.md`](../../../AGENTS.md): repository-level engineering, product, release, and
  testing guardrails

This remediation track exists because the repository can already build and publish artifacts, but
the current release outputs and top-level documentation still optimize for implementation detail
instead of operator consumption.

## Execution Rule

Every remediation item below has a `Repair Modes` subsection.

Before implementing any item:

1. present the available repair modes to the user
2. ask the user which mode to use
3. do not start coding until that item-level choice is confirmed

Exception:

- if the user has already explicitly specified the repair shape in a way that clearly maps to one
  of the modes, record that mode as `user-specified` and proceed without re-asking the same choice

After an item is completed and verified:

1. update the item status in this file
2. record the chosen repair mode
3. record validation results
4. append a concise completion note to [`../../../PLAN.md`](../../../PLAN.md)
5. update the next-release release-notes source file under [`../../releases`](../../releases)

## Problem Summary

The current problems fall into three related buckets:

1. release assets still contain packaging noise such as bundled repository documents and generic
   archive names that are hard to identify after download
2. GitHub Release bodies are still auto-generated and do not reliably capture which iterations or
   remediations shipped in a given cut
3. the top-level README is still written mainly for contributors, while self-hosted operators need
   a much clearer entry path and install automation

## Remediation Overview

| Item | Title | Status | Depends On |
| --- | --- | --- | --- |
| R1 | Release Asset Minimal Packaging And Versioned Naming | completed | none |
| R2 | Release Notes Governance And Automation | completed | R1 |
| R3 | User-Facing README And Self-Hosted Bootstrap Scripts | completed | R1, R2 |

## Shared Remediation Guardrails

- Do not repackage repository `README` files into release assets just to make archives look
  self-contained.
- Do not publish ambiguous asset names that omit the release tag or version identifier.
- Do not let release notes depend only on GitHub auto-generation when the project already tracks
  structured iteration/remediation history.
- Do not rewrite user documentation around contributor internals when the page is the primary entry
  point for operators.
- Do not hardcode relay addresses, tokens, or public origins into install scripts.
- Do not claim a one-click deployment path for platforms that the repository does not actually
  publish artifacts for.

## R1: Release Asset Minimal Packaging And Versioned Naming

### Problem

The current release workflow publishes generic archives that still contain repository documentation
and directory structure noise. The resulting release page makes it harder to identify which file is
the actual installer or binary set for a given platform.

### Goal

Make release assets minimal, operator-facing, and versioned.

### Repair Modes

- Mode A: `Keep current archives and only remove README files`
  Reduce some noise, but keep the current generic archive naming and desktop bundle wrapping.
- Mode B: `Publish only meaningful binaries/installers with versioned names`
  Remove bundled docs, keep CLI archives only where multiple binaries are required, and publish
  desktop/mobile outputs under explicit versioned names.
- Mode C: `Publish raw build directories for every platform`
  Maximize completeness at the cost of a noisy and confusing release page.

### Recommended Mode

- Mode B

Reason:

- it aligns with the user's request to keep only the real deliverables such as `exe`, `msi`, `apk`,
  `aab`, and compact CLI archives
- it makes the release page much easier to scan and download from
- it preserves multi-binary CLI delivery without flooding the release with internal bundle content

### Planned Scope

- remove bundled `README` and similar repository documents from release packaging
- keep Linux/Windows CLI outputs as compact archives containing only the required binaries
- publish Linux desktop outputs as actual installable bundle files rather than a copied bundle tree
- publish Windows desktop outputs as actual `.exe` and `.msi` installers only
- rename release assets to include the pushed tag/version
- update README examples to match the new asset names

### Acceptance Criteria

- no published release asset archive contains repository README files
- desktop release assets are meaningful installable outputs rather than copied bundle directories
- every published asset name includes the release tag/version
- checksum generation still works across the new asset set

### Validation

- inspect [`.github/workflows/release.yml`](../../../.github/workflows/release.yml)
- if needed, dry-run workflow logic by checking asset copy patterns and names locally

### Item Record

- Chosen repair mode:
  `Mode B (user-specified)`
- Implementation notes:
  rewired [`.github/workflows/release.yml`](../../../.github/workflows/release.yml) so CLI assets
  keep only the required binaries, desktop assets publish the actual installer outputs instead of
  copied bundle directories, Android assets are renamed with the pushed tag, and all published asset
  names now include the release tag.
- Validation results:
  `2026-03-28`: `cargo check --locked -p vibe-relay -p vibe-agent -p vibe-app` succeeded after the
  release-workflow rewrite.
  `2026-03-28`: `./scripts/dual-process-smoke.sh relay_polling` succeeded after the release asset
  packaging changes.
  `2026-03-28`: `./scripts/dual-process-smoke.sh overlay` succeeded after the release asset
  packaging changes.
  `2026-03-28`: YAML parse of
  [`.github/workflows/release.yml`](../../../.github/workflows/release.yml) succeeded.

## R2: Release Notes Governance And Automation

### Problem

The current release workflow relies on GitHub auto-generated notes, so shipped remediation and
iteration work is not recorded in a durable, repository-owned way.

### Goal

Make release notes part of the repository workflow, with a predictable source file that must be
updated as work ships.

### Repair Modes

- Mode A: `Keep GitHub auto notes and add manual post-release editing`
  Requires human cleanup after every release and is easy to forget.
- Mode B: `Store release-note source in the repo and render it during release`
  Make the release body reproducible from versioned repository content.
- Mode C: `Generate release notes only from git history`
  Avoids manual updates, but loses plan-level intent and acceptance context.

### Recommended Mode

- Mode B

Reason:

- it satisfies the user's requirement that every iteration and remediation be reflected in the
  release record
- it keeps release notes reviewable in code review
- it avoids depending on commit-message quality alone

### Planned Scope

- add a dedicated `docs/releases/` area for release-note sources and process guidance
- add a script that renders the release body from the versioned or next-release note source
- update the release workflow to use the rendered file instead of GitHub auto-generation
- document the mandatory update flow in `AGENTS.md`, `PLAN.md`, and `docs/plans/process.md`

### Acceptance Criteria

- the release workflow publishes a repository-owned body instead of auto-generated notes only
- the repository contains a discoverable location for next-release and versioned release notes
- governance docs require iteration/remediation work to update the next-release notes source

### Validation

- inspect the release workflow body-generation steps
- inspect `docs/releases/README.md` and the current note source files

### Item Record

- Chosen repair mode:
  `Mode B (user-specified)`
- Implementation notes:
  added [`docs/releases/README.md`](../../releases/README.md),
  [`docs/releases/unreleased.md`](../../releases/unreleased.md), a frozen
  [`docs/releases/v0.1.4.md`](../../releases/v0.1.4.md), and
  [`scripts/render-release-notes.sh`](../../../scripts/render-release-notes.sh); the release
  workflow now renders its body from repository-owned notes instead of GitHub auto-generation only,
  and project governance docs now require release-affecting work to update `docs/releases/`.
- Validation results:
  `2026-03-28`: `./scripts/render-release-notes.sh v0.1.4` succeeded and rendered the repository
  release body from [`docs/releases/v0.1.4.md`](../../releases/v0.1.4.md).
  `2026-03-28`: local inspection confirmed the publish job now uses `body_path` instead of
  `generate_release_notes: true`.

## R3: User-Facing README And Self-Hosted Bootstrap Scripts

### Problem

The top-level README still reads mainly like an engineering overview. Users who simply want to
self-host the relay or download a client do not get a fast operator path, and there is no install
script that configures auto-start for a supported deployment target.

### Goal

Rewrite the top-level documentation around user onboarding and add practical self-hosted bootstrap
scripts that avoid hardcoded networking assumptions.

### Repair Modes

- Mode A: `README rewrite only`
  Improves discoverability, but still leaves operators to configure services manually.
- Mode B: `README rewrite plus supported install scripts with auto-start`
  Give operators a short path from repository page to a working self-hosted relay.
- Mode C: `Split all user docs into a separate docs site first`
  Likely overkill for the current project state and slower to ship.

### Recommended Mode

- Mode B

Reason:

- it directly matches the user's request
- it improves both the first impression and the actual deployment path
- it keeps the automation honest by limiting it to supported artifact platforms

### Planned Scope

- rewrite the top of `README.md` and `README.en.md` around what the product is, who it is for, how
  to download clients, and how to self-host
- move deeper development/build details lower in the file instead of making them the primary entry
  point
- replace the current self-hosted note with a deployment guide that points to the new scripts
- add a Linux install script that can download the published CLI asset, install `vibe-relay`, and
  configure `systemd` auto-start without hardcoded public relay defaults
- add a Windows install script that can download the published CLI asset, install `vibe-relay.exe`,
  and configure auto-start through a scheduled task
- document supported automation boundaries clearly if macOS or other platforms are not yet shipped

### Acceptance Criteria

- the top of `README.md` is user-facing and links clearly to self-hosting and release downloads
- self-hosted deployment docs describe the supported install scripts and configuration inputs
- install scripts do not hardcode relay URLs, tokens, or public origins
- install scripts configure auto-start on the supported target platforms they claim to handle

### Validation

- inspect `README.md`, `README.en.md`, and `docs/self-hosted.md`
- run shell or syntax checks for the new install scripts where feasible

### Item Record

- Chosen repair mode:
  `Mode B (user-specified)`
- Implementation notes:
  rewrote [`README.md`](../../../README.md) and [`README.en.md`](../../../README.en.md) around an
  operator-first entry path, replaced the old deployment note with the user-facing
  [`docs/self-hosted.md`](../../../docs/self-hosted.md), and added supported Linux/Windows relay
  bootstrap scripts at [`scripts/install-relay.sh`](../../../scripts/install-relay.sh) and
  [`scripts/install-relay.ps1`](../../../scripts/install-relay.ps1).
- Validation results:
  `2026-03-28`: `bash -n scripts/install-relay.sh` succeeded.
  `2026-03-28`: `pwsh` was not available in the local environment, so PowerShell parsing could not
  be executed locally; Windows workflow validation remains required after push.
  `2026-03-28`: `cd apps/vibe-app && npm run build` succeeded after the README, deployment-doc, and
  version updates.
