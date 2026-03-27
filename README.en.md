# Vibe Remote

[![CI](https://github.com/fage-ac-org/vibe-everywhere/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/fage-ac-org/vibe-everywhere/actions/workflows/ci.yml)
[![Release](https://github.com/fage-ac-org/vibe-everywhere/actions/workflows/release.yml/badge.svg)](https://github.com/fage-ac-org/vibe-everywhere/actions/workflows/release.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)

[中文](./README.md) | [English](./README.en.md)

Rust-first remote AI control plane: `Rust relay + Rust agent + Vue 3.5 + Tauri 2 app`.

This is not a traditional remote desktop product. It is a control system for multi-device AI workflows, task execution, shell sessions, and TCP port forwarding. The relay provides the control-plane API and state management, the agent runs on the target machine, and the control UI connects through Web or a Tauri desktop shell.

## Status

- Positioning: personal-edition MVP / open source experimental project
- Working flows: device registration, task execution, event streaming, shell sessions, relay-first port forwarding, overlay-assisted transport
- Technical direction: Rust for protocol, backend, and agent; Vue + Tauri for the control client
- Best-fit use cases: self-hosted personal AI operations console, multi-device control plane, cross-platform experimentation

## Features

- Rust workspace with shared protocol, backend, agent, and desktop app
- `vibe-relay` for Axum APIs, device state, task scheduling, shell sessions, and port forwarding
- `vibe-agent` for registration, polling, provider adapters, shell runtime, and forwarding runtime
- `vibe-app` for the Vue 3.5 control UI, with `src-tauri` as the desktop shell
- Provider integration for `Codex`, `Claude Code`, and `OpenCode`
- Relay-first task, shell, and TCP forwarding paths
- EasyTier-based overlay-assisted transport
- SSE / WebSocket / tunnel based real-time updates

## Architecture

```text
┌──────────────────────────────────────────────────────────┐
│                     Control App                          │
│           Vue 3.5 Web UI / Tauri Desktop Shell          │
└───────────────────────────┬──────────────────────────────┘
                            │ HTTP / SSE / WebSocket
┌───────────────────────────▼──────────────────────────────┐
│                      vibe-relay                          │
│   device registry · task control · shell · port proxy   │
│   auth · persistence · overlay-aware transport choice    │
└───────────────────────────┬──────────────────────────────┘
                            │ HTTP polling / bridge / tunnel
┌───────────────────────────▼──────────────────────────────┐
│                      vibe-agent                          │
│ provider adapters · task runtime · shell runtime         │
│ port-forward runtime · embedded overlay node             │
└───────────────────────────┬──────────────────────────────┘
                            │ local process / local TCP
                    ┌───────▼────────┐
                    │ target machine │
                    └────────────────┘
```

## Repository Layout

```text
.
├── apps
│   ├── vibe-relay        # Relay API / control plane
│   ├── vibe-agent        # Device agent / runtimes / providers
│   └── vibe-app          # Vue control app
│       └── src-tauri     # Tauri desktop shell
├── crates
│   └── vibe-core         # Shared protocol / models
├── scripts               # Smoke tests and helper scripts
└── TESTING.md            # Testing strategy and validation matrix
```

## Quick Start

### Prerequisites

- Rust stable toolchain
- Node.js 20+
- `protobuf-compiler` or another working `protoc`
- WebKitGTK / GTK development packages when building Tauri on Linux
- On Windows, install Npcap with WinPcap API-compatible mode enabled if you want EasyTier / overlay networking features
- At least one provider CLI installed locally if you want to execute AI tasks
  - `codex`
  - `claude`
  - `opencode`

### 1. Clone the repository

```bash
git clone https://github.com/fage-ac-org/vibe-everywhere.git
cd vibe-everywhere
```

### 2. Start the relay

```bash
cargo run -p vibe-relay
```

The default address is `http://127.0.0.1:8787`.

To enable single-user access control:

```bash
export VIBE_RELAY_ACCESS_TOKEN=change-me
```

### 3. Start the agent

```bash
cargo run -p vibe-agent -- --relay-url http://127.0.0.1:8787
```

If no provider CLI is installed, the device will still register successfully, but AI task execution will be unavailable.

### 4. Start the Web control UI

```bash
cd apps/vibe-app
npm ci
npm run dev
```

Default UI address:

- `http://127.0.0.1:1420`

If the relay requires an access token, you can enter it in the UI or set:

```bash
export VITE_RELAY_BASE_URL=http://127.0.0.1:8787
export VITE_RELAY_ACCESS_TOKEN=change-me
```

### 5. Start the desktop shell

```bash
cd apps/vibe-app
npm ci
npm run tauri dev
```

The Tauri shell reads:

- `VIBE_PUBLIC_RELAY_BASE_URL`
- `VIBE_RELAY_ACCESS_TOKEN`

### 6. Verify the stack

After the steps above, you should be able to:

- connect the UI to the relay
- see the agent in the device list
- create and execute tasks if a provider CLI is available
- open shell sessions
- create TCP port forwards

## Development

```bash
cargo check -p vibe-relay -p vibe-agent -p vibe-app
cargo test --workspace --all-targets -- --nocapture
cd apps/vibe-app && npm ci && npm run build
```

Common local entrypoints:

```bash
cargo run -p vibe-relay
cargo run -p vibe-agent -- --relay-url http://127.0.0.1:8787
cd apps/vibe-app && npm run dev
cd apps/vibe-app && npm run tauri dev
```

## Testing

See [TESTING.md](./TESTING.md) for the full test strategy.

Recommended local baseline:

```bash
cargo fmt --all --check
cargo check -p vibe-relay -p vibe-agent -p vibe-app
cargo test --workspace --all-targets -- --nocapture
cd apps/vibe-app && npm ci && npm run build
./scripts/dual-process-smoke.sh relay_polling
```

For overlay, EasyTier, shell, and forwarding transport changes, also run:

```bash
./scripts/dual-process-smoke.sh overlay
```

## GitHub Actions

The repository includes two workflows:

- `CI`
  - Triggers on `push` to `main`, `pull_request`, and manual dispatch
  - Runs formatting checks, workspace builds, workspace tests, frontend build, `relay_polling` smoke tests, and Windows Rust/Tauri MSI bundling validation
- `Release`
  - Triggers on `v*` tags
  - Runs full verification, best-effort `overlay` smoke tests, Linux and Windows CLI packaging, Linux and Windows Tauri desktop packaging, and GitHub Release asset publishing

Release example:

```bash
git tag v0.1.0
git push origin v0.1.0
```

Expected release assets include:

- `vibe-remote-cli-x86_64-unknown-linux-gnu.tar.gz`
- `vibe-remote-desktop-x86_64-unknown-linux-gnu.tar.gz`
- `vibe-remote-cli-x86_64-pc-windows-msvc.zip`
- `vibe-remote-desktop-x86_64-pc-windows-msvc.zip`
- `SHA256SUMS.txt`

## Common Environment Variables

### relay

- `VIBE_RELAY_HOST`
- `VIBE_RELAY_PORT`
- `VIBE_PUBLIC_RELAY_BASE_URL`
- `VIBE_RELAY_ACCESS_TOKEN`
- `VIBE_RELAY_STATE_FILE`
- `VIBE_RELAY_FORWARD_HOST`
- `VIBE_RELAY_FORWARD_BIND_HOST`

### agent

- `VIBE_RELAY_URL`
- `VIBE_RELAY_ACCESS_TOKEN`
- `VIBE_DEVICE_NAME`
- `VIBE_DEVICE_ID`
- `VIBE_WORKING_ROOT`
- `VIBE_CODEX_COMMAND`
- `VIBE_CLAUDE_COMMAND`
- `VIBE_OPENCODE_COMMAND`

### overlay

- `VIBE_EASYTIER_RELAY_ENABLED`
- `VIBE_EASYTIER_NETWORK_NAME`
- `VIBE_EASYTIER_NETWORK_SECRET`
- `VIBE_EASYTIER_BOOTSTRAP_URL`
- `VIBE_EASYTIER_LISTENERS`

### frontend / desktop

- `VITE_RELAY_BASE_URL`
- `VITE_RELAY_ACCESS_TOKEN`
- `VIBE_PUBLIC_RELAY_BASE_URL`
- `VIBE_RELAY_ACCESS_TOKEN`

## Roadmap

- stronger authentication, auditing, and production deployment support
- frontend automated tests and protocol round-trip tests
- continued extraction of large `main.rs` responsibilities into stable modules
- richer file sync, workspace browsing, and notification capabilities
- better desktop and mobile UX

## Contributing

Issues and pull requests are welcome.

Please include:

- the problem statement and change goal
- the affected crate / app
- the validation commands you ran
- screenshots for UI changes
- any new environment variables or system dependencies

Conventional Commits are recommended, for example:

```text
feat(agent): add claude stream-json mapping
fix(relay): keep overlay transport fallback stable
docs(readme): rewrite project overview and quick start
```

## License

This project is licensed under the [MIT License](./LICENSE).
