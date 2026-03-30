# Vibe Everywhere

[![CI](https://github.com/fage-ac-org/vibe-everywhere/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/fage-ac-org/vibe-everywhere/actions/workflows/ci.yml)
[![Release](https://github.com/fage-ac-org/vibe-everywhere/actions/workflows/release.yml/badge.svg)](https://github.com/fage-ac-org/vibe-everywhere/actions/workflows/release.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)

[中文](./README.md) | [English](./README.en.md)

Vibe Everywhere is a self-hosted remote AI development workspace. The system consists of
`vibe-relay`, `vibe-agent`, and control clients, and is centered on one stable product model:
`Server -> Host -> Project -> Conversation / Task execution`. The main project workspace is limited
to `Conversation / Changes / Files / Logs`.

This document is written for end users and operators. It provides a system overview, binary
installation entry points, relay startup references, key configuration semantics, and the standard
usage flow.

## Overview

The standard workflow is:

1. Deploy `vibe-relay` on a host reachable by clients and agents.
2. Start `vibe-agent` on target execution nodes.
3. Connect a desktop, Android, or self-hosted Web client to the relay.
4. Select an online host and open one of its projects.
5. Continue an existing conversation or start a new AI task inside that project.
6. Review conversation output, changes, files, and logs from the same project workspace.

## Components

| Component | Purpose | Typical Location |
| --- | --- | --- |
| `vibe-relay` | Control-plane entry point for auth, device registration, conversation / task routing, aggregation, and public APIs | Server, workstation, cloud host |
| `vibe-agent` | Runtime on the target host for provider execution, workspace access, Git inspection, and task execution | Machine that runs AI work |
| Control client | Connects to the relay, browses hosts and projects, manages long-lived AI conversations, and reviews results | Desktop, Android, self-hosted Web client |

## Supported Capabilities

The current release supports:

- the mobile-first navigation: `Home / Projects / My`
- a project workspace with `Conversation / Changes / Files / Logs`
- host project discovery from the agent working root and first-level Git repositories
- creation, continuation, and basic event viewing for long-lived AI conversations
- device registration, presence reporting, and provider availability display
- branch and changed-file summaries in project cards and project headers
- retained project inventory state when a host goes offline or refresh fails
- workspace browsing and text-file preview
- Git status, changed-file listing, recent-commit inspection, and file-level diff review
- per-task conversation summaries with recent execution events and expandable raw output
- direct stop action for pending or running tasks inside the conversation surface
- quick follow-up actions on task cards for retry, result explanation, and direct jumps into
  changes or logs
- per-task execution mode selection: read-only / workspace write / write + test
- inline provider choice prompts with preset options plus custom text input
- English and Simplified Chinese UI
- light, dark, and system theme modes

Still being aligned:

- deeper and more complete host project inventory beyond the current working-root scan

## Quick Start

### Prerequisites

Prepare the following values before deployment:

- the client-facing relay address, for example `https://relay.example.com` or `http://203.0.113.10:8787`
- the control-plane token used by human users: `VIBE_RELAY_ACCESS_TOKEN`
- the enrollment token used by agents during first registration: `VIBE_RELAY_ENROLLMENT_TOKEN`
- at least one provider CLI on each target machine, such as `codex`, `claude`, or `opencode`

### 1. Download or Update the CLI Binaries

#### Linux

```bash
curl -fsSL https://raw.githubusercontent.com/fage-ac-org/vibe-everywhere/main/scripts/install-relay.sh -o install-relay.sh
bash install-relay.sh install --no-gh-proxy
```

Common commands:

```bash
bash install-relay.sh install
bash install-relay.sh install --component relay
bash install-relay.sh install --component agent
bash install-relay.sh update --release-tag v0.1.11
bash install-relay.sh uninstall
```

#### Windows

```powershell
Invoke-WebRequest `
  -Uri "https://raw.githubusercontent.com/fage-ac-org/vibe-everywhere/main/scripts/install-relay.ps1" `
  -OutFile ".\install-relay.ps1"
powershell -ExecutionPolicy Bypass -File .\install-relay.ps1 -Command install -NoGhProxy
```

Common commands:

```powershell
powershell -ExecutionPolicy Bypass -File .\install-relay.ps1 -Command install
powershell -ExecutionPolicy Bypass -File .\install-relay.ps1 -Command install -Component relay
powershell -ExecutionPolicy Bypass -File .\install-relay.ps1 -Command install -Component agent
powershell -ExecutionPolicy Bypass -File .\install-relay.ps1 -Command update -ReleaseTag v0.1.11
powershell -ExecutionPolicy Bypass -File .\install-relay.ps1 -Command uninstall
```

### 2. Configure and Start the Relay

```bash
export VIBE_RELAY_HOST=0.0.0.0
export VIBE_RELAY_PORT=8787
export VIBE_PUBLIC_RELAY_BASE_URL=https://relay.example.com
export VIBE_RELAY_ACCESS_TOKEN=change-control-token
export VIBE_RELAY_ENROLLMENT_TOKEN=change-agent-enrollment-token
vibe-relay
```

### 3. Start an Agent

```bash
VIBE_RELAY_URL=https://relay.example.com \
VIBE_RELAY_ENROLLMENT_TOKEN=change-agent-enrollment-token \
VIBE_DEVICE_NAME=build-node-01 \
vibe-agent
```

### 4. Connect a Control Client

1. Open the desktop app, Android client, or self-hosted Web client.
2. Enter the relay URL and `VIBE_RELAY_ACCESS_TOKEN` in server settings.
3. Confirm that at least one host is online and a provider is available.
4. Open a project from the project list.
5. Start or continue a conversation from the `Conversation` tab.

## Configuration Semantics

### Relay Bind Address vs Public Address

| Setting | Purpose | Default | Notes |
| --- | --- | --- | --- |
| `VIBE_RELAY_HOST` | Local relay bind address | `0.0.0.0` | Selects the interface address used by the relay process |
| `VIBE_RELAY_PORT` | Local relay listener port | `8787` | Selects the TCP port used by the relay process |
| `VIBE_PUBLIC_RELAY_BASE_URL` | Client-facing relay origin | No production default | Used for client connection information and generated public links |

Key rules:

- `VIBE_PUBLIC_RELAY_BASE_URL` does not change the actual relay listener port.
- If the relay listens on `8787` and clients connect directly to that port, `VIBE_PUBLIC_RELAY_BASE_URL` must include `:8787`.
- `0.0.0.0` is valid as a bind host but not as a client-facing URL host.
- `127.0.0.1` and `localhost` are valid only for same-machine local development.

## Authentication Model

| Setting or File | Purpose | Used By |
| --- | --- | --- |
| `VIBE_RELAY_ACCESS_TOKEN` | Control-plane authentication | Desktop, Android, self-hosted Web clients |
| `VIBE_RELAY_ENROLLMENT_TOKEN` | Initial device registration | `vibe-agent` |
| `.vibe-agent/identity.json` | Persisted issued device credential | `vibe-agent` restarts |

## Standard Usage Flow

1. Configure the relay address and control-plane token.
2. Confirm that the target device is online.
3. Check provider availability on the target device.
4. Create or continue a long-lived AI conversation.
5. Review the transcript, provider prompts, and execution results.
6. Use workspace browsing, Git inspection, and logs to validate output.

## Troubleshooting

| Condition | First Checks |
| --- | --- |
| Agent is running but no device appears | Verify `VIBE_RELAY_URL`, `VIBE_RELAY_ENROLLMENT_TOKEN`, relay `/api/health`, and network reachability |
| Device is online but no provider is available | Verify that the provider CLI is installed and visible in the agent process `PATH` |
| Device must be enrolled again | Delete `.vibe-agent/identity.json` and restart the agent |

## System Layout

```text
┌──────────────────────────────────────────────────────────┐
│                     Control App                          │
│      Vue 3.5 Web UI / Tauri Desktop + Android Shell     │
└───────────────────────────┬──────────────────────────────┘
                            │ HTTP / SSE
┌───────────────────────────▼──────────────────────────────┐
│                      vibe-relay                          │
│  device registry · AI conversations · workspace · git    │
│                auth · config · public API                │
└───────────────────────────┬──────────────────────────────┘
                            │ polling / stream
┌───────────────────────────▼──────────────────────────────┐
│                      vibe-agent                          │
│    provider adapters · workspace runtime · git runtime   │
│                  conversation task runtime               │
└───────────────────────────┬──────────────────────────────┘
                            │ local process / local fs
                    ┌───────▼────────┐
                    │ target machine │
                    └────────────────┘
```

## Related Docs

- Chinese relay startup guide: [docs/relay-startup.zh-CN.md](./docs/relay-startup.zh-CN.md)
- English relay startup guide: [docs/relay-startup.md](./docs/relay-startup.md)
- Releases: [GitHub Releases](https://github.com/fage-ac-org/vibe-everywhere/releases)
