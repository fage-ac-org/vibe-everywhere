# Vibe Everywhere

[![CI](https://github.com/fage-ac-org/vibe-everywhere/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/fage-ac-org/vibe-everywhere/actions/workflows/ci.yml)
[![Release](https://github.com/fage-ac-org/vibe-everywhere/actions/workflows/release.yml/badge.svg)](https://github.com/fage-ac-org/vibe-everywhere/actions/workflows/release.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)

[中文](./README.md) | [English](./README.en.md)

Vibe Everywhere is a self-hosted remote AI control plane. You run AI coding work on your own
servers, workstations, or development machines, then use one control surface to launch sessions,
inspect workspaces, review Git state, open previews, and only drop into terminal or advanced
connection tools when you actually need them.

It is not a traditional remote desktop product, and it is not a managed SaaS that owns your
runtime. The product goal is straightforward: make remote AI development feel like a deployable,
observable workflow instead of a pile of ad hoc SSH commands.

## What This Product Is In One Minute

- `vibe-relay` is the control-plane entry point for auth, device registration, session routing, and shared state.
- `vibe-agent` runs on each target machine and executes provider CLIs, workspace actions, Git inspection, and preview bridging.
- control clients connect to the same relay through desktop, Android, or a self-hosted Web client.
- the primary product flow is session-first; device management, terminal access, and advanced tools are secondary surfaces.

## Who It Is For

- people who want AI coding tasks to run on remote machines while keeping one clear control surface
- teams that prefer self-hosting over managed infrastructure
- operators managing multiple devices, workspaces, and provider CLIs
- teams that want a practical MVP now and a path toward stronger team features later

## What It Can Do Today

- create, stream, and cancel AI sessions
- keep the main workflow on one surface: relay connection, device choice, session launch, and result review
- register devices, track presence, and show provider availability
- browse workspaces, preview text files, and inspect Git state
- expose preview flows plus terminal and advanced connection tools when needed
- connect through desktop, Android, and a self-hosted Web client
- support both English and Simplified Chinese plus light, dark, and system themes

## Simple Deployment Manual

This section is for getting a usable deployment running quickly, without diving into every advanced option.

### Prepare These 3 Values First

- `https://relay.example.com`
  This must be the real relay address your users and agents can reach. Do not use `127.0.0.1` for shared deployments.
- `VIBE_RELAY_ACCESS_TOKEN`
  This is the human control-plane token used by desktop, Android, and self-hosted Web clients.
- `VIBE_RELAY_ENROLLMENT_TOKEN`
  This is the bootstrap token used by agents during first registration. It should be separate from the human control token.

### Step 1: Deploy the Relay

For the full operator guide, see [docs/self-hosted.md](./docs/self-hosted.md). If your goal is to get a working relay online first, the install scripts are the shortest path.

- Linux with `systemd`

```bash
curl -fsSL https://raw.githubusercontent.com/fage-ac-org/vibe-everywhere/main/scripts/install-relay.sh -o install-relay.sh
sudo RELAY_PUBLIC_BASE_URL=https://relay.example.com \
  RELAY_ACCESS_TOKEN=change-control-token \
  RELAY_ENROLLMENT_TOKEN=change-agent-enrollment-token \
  bash install-relay.sh
```

- Windows with a startup task

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\install-relay.ps1 `
  -PublicRelayBaseUrl https://relay.example.com `
  -RelayAccessToken change-control-token `
  -RelayEnrollmentToken change-agent-enrollment-token
```

After installation, confirm that the relay is healthy:

```bash
curl https://relay.example.com/api/health
```

### Do Not Confuse Bind Address With Public Address

This is the most common source of deployment confusion:

- `RELAY_BIND_HOST` / `RELAY_PORT`
  The install scripts write these into `VIBE_RELAY_HOST` / `VIBE_RELAY_PORT`, and they control where the relay actually listens.
- `RELAY_PUBLIC_BASE_URL`
  The install scripts write this into `VIBE_PUBLIC_RELAY_BASE_URL`, and it controls which address the product shows to clients and uses for preview links. It does not change the actual relay listener port.

Current defaults:

- the relay listens on `0.0.0.0:8787` by default
- if `VIBE_PUBLIC_RELAY_BASE_URL` is not set, production mode does not try to invent a public URL from `0.0.0.0`

Use these rules:

- `0.0.0.0` is valid as a bind host, but not as a client-facing public URL host.
- `127.0.0.1` or `localhost` are only suitable for same-machine local development.
- `RELAY_PUBLIC_BASE_URL=http://45.144.137.240` means port 80 from the client point of view; it does not magically inherit the relay listener port.
- if the relay actually listens on `8787` and clients should connect directly to that port, use `http://45.144.137.240:8787`.
- `http` is acceptable for local development, private LAN testing, or environments where you explicitly accept the plaintext transport tradeoff.
- for internet-facing, mobile, desktop, or shared deployments, `https` is strongly recommended.

Common deployment shapes:

- direct public IP access

```bash
sudo RELAY_BIND_HOST=0.0.0.0 \
  RELAY_PORT=8787 \
  RELAY_PUBLIC_BASE_URL=http://45.144.137.240:8787 \
  RELAY_ACCESS_TOKEN=change-control-token \
  RELAY_ENROLLMENT_TOKEN=change-agent-enrollment-token \
  bash install-relay.sh
```

- reverse proxy in front of a local relay

```bash
sudo RELAY_BIND_HOST=127.0.0.1 \
  RELAY_PORT=8787 \
  RELAY_PUBLIC_BASE_URL=https://relay.example.com \
  RELAY_ACCESS_TOKEN=change-control-token \
  RELAY_ENROLLMENT_TOKEN=change-agent-enrollment-token \
  bash install-relay.sh
```

### Step 2: Start an Agent on the Target Machine

Download the current CLI package from [GitHub Releases](https://github.com/fage-ac-org/vibe-everywhere/releases), extract it, and start `vibe-agent`:

```bash
VIBE_RELAY_URL=https://relay.example.com \
VIBE_RELAY_ENROLLMENT_TOKEN=change-agent-enrollment-token \
VIBE_DEVICE_NAME=build-node-01 \
./vibe-agent
```

Important notes:

- On Windows, keep the extracted side-by-side runtime files next to `vibe-agent.exe`; do not copy the executable out by itself.
- To execute AI sessions, the target machine still needs at least one provider CLI such as `codex`, `claude`, or `opencode`.
- After the first successful registration, the agent writes `.vibe-agent/identity.json` under its working root and reuses that issued device identity on restart.

### Agent, Overlay, And EasyTier Port Notes

By default, the agent does not expose one fixed control-plane port the way the relay does:

- if EasyTier is not enabled, meaning `VIBE_EASYTIER_NETWORK_NAME` is not set, the agent mainly opens outbound connections to the relay and does not add fixed listener ports for its normal workflow
- if EasyTier overlay is enabled, the agent starts 3 bridge listeners:
  - `19090`: shell bridge
  - `19091`: port-forward bridge
  - `19092`: task bridge
- those ports can be overridden with `VIBE_AGENT_SHELL_BRIDGE_PORT`, `VIBE_AGENT_PORT_FORWARD_BRIDGE_PORT`, and `VIBE_AGENT_TASK_BRIDGE_PORT`
- these bridge ports are part of the relay-to-agent overlay path and should not be treated as public browser or mobile entry points

EasyTier listener behavior should be understood separately:

- on the agent side, `VIBE_EASYTIER_NO_LISTENER=true` by default, so the embedded EasyTier node does not accept inbound EasyTier peers unless you explicitly change that
- if you set `VIBE_EASYTIER_NO_LISTENER=false` and do not set `VIBE_EASYTIER_LISTENERS`, this repository follows the common EasyTier listener shorthand and defaults to TCP/UDP `11010`
- on the relay side, if embedded EasyTier is enabled and `VIBE_EASYTIER_LISTENERS` is unset, this repository also defaults to TCP/UDP `11010`

### Step 3: Open a Control Client and Connect

The public release artifacts currently include:

- CLI
- desktop
- Android

The Web client is part of the product model, but public Releases do not currently ship a standalone Web bundle. If you want the fastest path to first use, start with the desktop client.

For the first connection:

1. open the desktop or Android client
2. enter the relay URL, for example `https://relay.example.com`
3. enter the control-plane token, which is `VIBE_RELAY_ACCESS_TOKEN`
4. confirm that at least one device is online and that a provider is available on that device

## How Authentication Works

The recommended production shape is to separate human access from machine enrollment:

- `VIBE_RELAY_ACCESS_TOKEN`
  Used by people in desktop, Android, and self-hosted Web clients.
- `VIBE_RELAY_ENROLLMENT_TOKEN`
  Used by agents only for initial enrollment.
- `.vibe-agent/identity.json`
  The device credential issued after successful enrollment and reused for later heartbeats, task polling, workspace requests, and preview bridge traffic.

If you delete `.vibe-agent/identity.json` on the target machine, the next agent start will go through enrollment again.

## Detailed Usage Guide

### 1. Confirm the Control Plane Is Reachable

On the first login, check these three things before creating a session:

- the relay URL is correct
- the control-plane token is correct
- at least one target device is online

If that baseline is not healthy, fix deployment or networking first instead of debugging the session surface.

### 2. Pick a Device and Verify Readiness

After connecting, choose the machine that should run the work. A healthy device usually shows:

- online presence
- at least one available provider such as `codex` or `claude`
- runtime capabilities such as workspace browse, Git inspection, shell, or preview support

If the device is online but no provider is available, the most common cause is that the provider CLI is not installed or not visible in the agent process `PATH`.

### 3. Launch an AI Session

The main product flow starts here:

1. choose a device
2. enter the task goal or prompt
3. choose a provider
4. launch the session and watch the event stream

This is where Vibe Everywhere differs from a raw SSH workflow: the execution process is visible, reviewable, and tied back to the same control surface.

### 4. Review Workspace, Git, and Preview Results

After a session runs, you usually stay in the same control flow to inspect the outcome:

- browse directories and files in the workspace
- preview text files
- inspect the current Git branch, changed files, and recent commits
- open preview links to validate local Web services or forwarded ports

The goal is not to replace your IDE. The goal is to let you answer, quickly and remotely, whether the result is acceptable, needs another AI iteration, or requires manual intervention.

### 5. Use Advanced Tools Only When Needed

Shell, port forwarding, and advanced connection tools still matter, but they should not be your default entry point.

Recommended rule of thumb:

- stay in the session flow when that is enough
- use workspace and Git review before reaching for shell access
- move into advanced tools only for debugging, manual repair, or low-level runtime work

### 6. Organize Multi-Device Setups Deliberately

If you operate more than one machine, name them by role so the control plane stays readable:

- `build-node-01` for builds and refactors
- `gpu-node-01` for model or GPU-heavy work
- `demo-node-01` for previews and demos

This makes device selection faster and makes team handoffs clearer.

## Common Questions

### The agent starts, but no device appears in the control client

Check these first:

- whether `VIBE_RELAY_URL` points to the relay origin the agent can actually reach
- whether `VIBE_RELAY_ENROLLMENT_TOKEN` is correct
- whether `/api/health` on the relay is healthy
- whether the target machine can reach the relay over the network

### The device is online, but no provider is available

Usually the provider CLI is missing on the target machine or not visible in the agent process environment.

### I want the device to enroll again

Delete `.vibe-agent/identity.json` in the working directory and restart `vibe-agent`. The next start will perform enrollment again.

### Preview links do not open

Check the public relay origin and preview-forward host settings. Preview links depend on the relay generating user-reachable URLs, so `RELAY_PUBLIC_BASE_URL`, `VIBE_PUBLIC_RELAY_BASE_URL`, and related forward-host configuration need to be correct.

### Can I Set `RELAY_PUBLIC_BASE_URL` To `http://127.0.0.1` Or `http://0.0.0.0`

- `http://127.0.0.1` is only suitable for same-machine local access and should not be used for remote desktop, mobile, or Android clients.
- `http://0.0.0.0` should not be used as a client-facing URL. `0.0.0.0` is a bind address, not a routable public hostname.
- if other devices need to reach the relay, use a real IP or domain such as `http://45.144.137.240:8787` or `https://relay.example.com`.

## Downloads

- [GitHub Releases](https://github.com/fage-ac-org/vibe-everywhere/releases)

The public release page currently ships CLI, desktop, and Android artifacts. Download the package that matches your platform.

## Product Layout

```text
┌──────────────────────────────────────────────────────────┐
│                     Control App                          │
│      Vue 3.5 Web UI / Tauri Desktop + Android Shell    │
└───────────────────────────┬──────────────────────────────┘
                            │ HTTP / SSE / WebSocket
┌───────────────────────────▼──────────────────────────────┐
│                      vibe-relay                          │
│  device registry · AI sessions · workspace · preview    │
│        auth · config · transport selection               │
└───────────────────────────┬──────────────────────────────┘
                            │ polling / stream / tunnel
┌───────────────────────────▼──────────────────────────────┐
│                      vibe-agent                          │
│ provider adapters · workspace/git runtime · shell       │
│      preview / forward runtime · overlay support         │
└───────────────────────────┬──────────────────────────────┘
                            │ local process / local TCP
                    ┌───────▼────────┐
                    │ target machine │
                    └────────────────┘
```

## Related Docs

- self-hosted deployment and install: [docs/self-hosted.md](./docs/self-hosted.md)
- published downloads: [GitHub Releases](https://github.com/fage-ac-org/vibe-everywhere/releases)
