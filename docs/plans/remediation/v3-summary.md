# Remediation Plan v3 Summary

Last updated: 2026-03-28

## Scope

Version 3 covers the next release-quality remediation tranche:

- removing non-essential files from published release assets
- making release asset names versioned and query-friendly
- turning iteration/remediation outcomes into durable GitHub Release notes
- rewriting the top-level user docs around operator onboarding instead of developer internals
- adding self-hosted bootstrap scripts that configure service auto-start without hardcoded relay
  addresses

Full implementation detail lives in [`v3-details.md`](./v3-details.md).

## Status

| Item | Title | Status | Depends On | Recommended Mode |
| --- | --- | --- | --- | --- |
| R1 | Release Asset Minimal Packaging And Versioned Naming | completed | none | Mode B (user-specified) |
| R2 | Release Notes Governance And Automation | completed | R1 | Mode B (user-specified) |
| R3 | User-Facing README And Self-Hosted Bootstrap Scripts | completed | R1, R2 | Mode B (user-specified) |

## Current Target

- Active item: `completed`
- Required next step:
  remediation plan `v3` is complete; later workflow/runtime follow-up moved into newer versioned
  remediation plans instead of extending this file into a different phase
- Last completed item:
  remediation plan `v3`, covering release-asset hygiene, repository-owned release notes, and
  user-facing onboarding/deployment improvements

## Lookup Notes

- Need the full problem statements, repair modes, acceptance criteria, and validation rules:
  read [`v3-details.md`](./v3-details.md).
- Need the mandatory execution workflow before starting an item:
  read [`../process.md`](../process.md).
