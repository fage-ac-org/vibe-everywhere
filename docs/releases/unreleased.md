## Highlights

- Embedded EasyTier agents no longer report overlay `connected` while runtime RPC is still not
  ready, which keeps relay selection, UI state, and smoke readiness aligned.
- Top-level README files are now stricter user/operator entry points and no longer act as
  developer/test navigation hubs.
- Android CI and release jobs now restore fixed-version SDK components from bounded caches in
  addition to Gradle dependency caching.

## Included Iterations And Remediations

- Remediation v5: overlay connectivity truthfulness, README boundary tightening, and Android
  workflow throughput optimization.

## Operator Notes

- No operator-facing configuration changes.

## Validation

- `cargo test --workspace --all-targets -- --nocapture`
- `bash -n scripts/dual-process-smoke.sh`
- `./scripts/dual-process-smoke.sh relay_polling`
- `./scripts/dual-process-smoke.sh overlay`
- `./scripts/render-release-notes.sh v0.0.0 >/dev/null`
