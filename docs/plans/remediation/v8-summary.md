# Remediation Plan v8 Summary

Last updated: 2026-03-28

## Scope

Version 8 covers the GitHub-hosted Linux overlay diagnostic repair that remained after remediation
`v7` exposed the true hosted-runner root cause:

- adapting the Linux overlay diagnostic to an explicit EasyTier `no_tun` harness mode when the
  hosted runner cannot create `/dev/net/tun`
- keeping EasyTier `no_tun` strictly behind harness-only configuration instead of changing product
  defaults
- validating truthful overlay fallback behavior on hosted Linux without claiming full hosted-runner
  overlay bridge reachability that the environment does not provide
- recording the currently deferred no_tun preview byte-path gap separately from the stable
  transport/lifecycle diagnostic path

Full implementation detail lives in [`v8-details.md`](./v8-details.md).

## Status

| Item | Title | Status | Depends On | Recommended Mode |
| --- | --- | --- | --- | --- |
| R1 | Linux Hosted Overlay No-TUN Diagnostic Path | completed | v7 | Mode A (user-confirmed) |

## Current Target

- Active item:
  `completed`
- Required next step:
  remediation plan `v8` is complete after GitHub-hosted `CI` run `23688459204` validated the
  hosted Linux `no_tun` path; later gate restoration moved into remediation `v9`
- Previous completed plan:
  remediation `v7`, which stabilized harness port allocation and preserved the raw EasyTier stop
  reason that exposed the hosted-runner TUN-permission limit

## Lookup Notes

- Need the full problem statement, repair modes, acceptance criteria, and validation rules:
  read [`v8-details.md`](./v8-details.md).
- Need the mandatory execution workflow before starting an item:
  read [`../process.md`](../process.md).
