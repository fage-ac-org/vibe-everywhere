# Iteration Plan v2 Summary

Last updated: 2026-03-28

## Scope

Version 2 starts the post-baseline roadmap epoch after Iteration 0 through Iteration 11 were
completed in `v1`.

This version currently focuses on delivery verification hardening rather than new end-user product
surfaces.

Full implementation detail lives in [`v2-details.md`](./v2-details.md).

## Status

| Iteration | Title | Status |
| --- | --- | --- |
| 12 | Delivery Verification Hardening | implemented locally |

## Current State

- Iteration 12 adds a Windows-native relay smoke path to CI so Windows is no longer validated only
  by compile and packaging checks.
- GitHub-hosted Linux `overlay` smoke remains diagnostically valuable but currently unstable, so it
  now runs in separately named non-blocking diagnostic jobs instead of the required verify path.
- The hosted-runner overlay root-cause investigation is explicitly deferred until a separate
  instruction reopens it.
- Local validation is complete for Linux and repository-owned workflow/docs changes; GitHub Actions
  verification is still required after the next push.

## Lookup Notes

- Need detailed acceptance or implementation notes:
  read [`v2-details.md`](./v2-details.md).
- Need the baseline roadmap history:
  read [`v1-summary.md`](./v1-summary.md).
- Need the active remediation track:
  read [`../remediation/v5-summary.md`](../remediation/v5-summary.md).
