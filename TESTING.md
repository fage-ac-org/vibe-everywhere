# Testing Guide

## Purpose

This file is the testing entry point for repository changes. It is intentionally reduced to the
minimum structure needed for later rewrite.

## Core Commands

```bash
cargo fmt --all --check
cargo check -p vibe-relay -p vibe-agent -p vibe-app
cargo test --workspace --all-targets -- --nocapture
cd apps/vibe-app && npm run build
./scripts/dual-process-smoke.sh relay_polling
./scripts/dual-process-smoke.sh overlay
```

## Minimum Checklist

- run the relevant Rust checks and tests
- build the frontend when touching `apps/vibe-app`
- run smoke tests when changing relay or agent control-plane behavior
- check GitHub Actions after push

## Manual Regression Placeholder

The detailed manual regression checklist will be rewritten later. Add product-specific manual cases
here when the new documentation set is ready.
