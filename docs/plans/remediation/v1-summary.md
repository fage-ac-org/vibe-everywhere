# Remediation Plan v1 Summary

Last updated: 2026-03-28

## Scope

Version 1 covers the first post-baseline repair tranche for the control app's navigation model,
feature visibility, deployment guidance, loopback defaults, and platform surfacing.

Full implementation detail lives in [`v1-details.md`](./v1-details.md).

## Status

| Item | Title | Status | Depends On | Recommended Mode |
| --- | --- | --- | --- | --- |
| R1 | Dashboard Navigation Refactor | completed | none | Mode C (chosen) |
| R2 | Feature Visibility And Enterprise Surface Gating | completed | R1 | Mode B (chosen) |
| R3 | Deployment Guidance Relocation | completed | R1 | Mode C (chosen) |
| R4 | Loopback Default And Public-Origin Cleanup | completed | R3 | Mode B (chosen) |
| R5 | Platform Matrix Semantic Correction | completed | R1 | Mode B (chosen) |
| R6 | Current-Client Detection And Platform Surfacing Alignment | completed | R5 | Mode A (chosen) |
| R7 | Docs, Tests, And Verification Realignment | completed | R1-R6 | Mode C (chosen) |

## Current Target

- Active item: `completed`
- Required next step:
  remediation plan `v1` is complete; if a new repair tranche is needed, start a new versioned
  remediation plan instead of extending this file into a different phase
- Last completed item:
  `R7` completed with `Mode C`, reconciling README, testing guidance, process rules, and manual QA
  checklists with the repaired navigation, visibility, and networking model

## Lookup Notes

- Need the full problem statements, repair modes, acceptance criteria, and validation rules:
  read [`v1-details.md`](./v1-details.md).
- Need the mandatory execution workflow before starting an item:
  read [`../process.md`](../process.md).
