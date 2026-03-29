## Highlights

- README and self-hosted deployment docs now explain the difference between relay bind address and
  public relay origin, when `http` versus `https` is appropriate, and which fixed ports appear only
  when EasyTier overlay mode is enabled.

## Included Iterations And Remediations

- Documentation follow-up after `v0.1.8` auth and deployment clarification work.

## Operator Notes

- Avoid using `0.0.0.0` or `127.0.0.1` as client-facing relay origins outside local development.
- If clients should reach a non-default relay port directly, keep that port in
  `VIBE_PUBLIC_RELAY_BASE_URL`.
- Agent bridge ports `19090` to `19092` are expected only when EasyTier overlay mode is enabled.

## Validation

- `./scripts/render-release-notes.sh v0.0.0 >/dev/null`
