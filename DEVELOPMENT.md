# Development Guide

## Purpose

This file is the developer entry point for local setup, source builds, and common repository
commands. It is intentionally kept minimal and will be rewritten later.

## Repository Layout

```text
.
├── apps
├── crates
├── docs
├── scripts
├── AGENTS.md
└── TESTING.md
```

## Local Prerequisites

- Rust stable toolchain
- Node.js
- `protoc`
- platform-specific dependencies required by Tauri or Android builds when those surfaces are used

## Common Commands

Start the relay:

```bash
cargo run -p vibe-relay
```

Start an agent:

```bash
cargo run -p vibe-agent -- --relay-url http://127.0.0.1:8787
```

Start the Web app:

```bash
cd apps/vibe-app
npm ci
npm run dev
```

Build the frontend:

```bash
cd apps/vibe-app
npm run build
```

## Validation Entry

See `TESTING.md` for the current validation checklist and regression entry points.
