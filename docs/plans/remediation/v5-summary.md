# Remediation Plan v5 Summary

Last updated: 2026-03-28

## Scope

Version 5 covers the residual remediation tranche after `v4`:

- making overlay status truthful so the product and smoke harness do not treat bootstrap progress as
  confirmed connectivity
- tightening the top-level README boundary so user/operator surfaces do not act as developer/test
  navigation hubs
- reducing Android CI and release setup time further with bounded SDK-component caches

Full implementation detail lives in [`v5-details.md`](./v5-details.md).

## Status

| Item | Title | Status | Depends On | Recommended Mode |
| --- | --- | --- | --- | --- |
| R1 | Overlay Connectivity Truthfulness And Smoke Alignment | completed | none | Mode B (consistent with prior user-specified runtime-first direction) |
| R2 | README Operator Surface Tightening | completed | none | Mode B (user-specified) |
| R3 | Android Workflow Throughput Optimization | completed | R2 | Mode B (user-specified) |

## Current Target

- Active item:
  `completed`
- Required next step:
  remediation plan `v5` is complete; later packaging and hosted-runner follow-up moved into newer
  versioned remediation plans instead of extending this file into a different phase
- Previous completed plan:
  remediation plan `v4`, covering overlay runtime fallback, README/developer-guide split, and
  bounded Gradle caching

## Lookup Notes

- Need the full problem statements, repair modes, acceptance criteria, and validation rules:
  read [`v5-details.md`](./v5-details.md).
- Need the mandatory execution workflow before starting an item:
  read [`../process.md`](../process.md).
