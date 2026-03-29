# Self-Hosted Deployment Guide

Last updated: 2026-03-29

This guide is for operators who want to deploy the relay, keep it running after reboot, and let
Web, desktop, or Android clients connect to the same control plane.

## Before You Start

Decide these runtime values first:

- the relay address your users and agents will actually access
- the control-plane token used by Web, desktop, and Android clients
- the enrollment token used by agents during first registration
- which host should be used for preview links
- where relay state should be stored on disk

If the control plane will be accessed from phones or other machines, use a reachable domain or IP
address for the relay origin.

## Bind Address vs Public Address

These settings solve different problems and should not be treated as aliases:

- `VIBE_RELAY_HOST`
  - the address the relay process binds to locally
- `VIBE_RELAY_PORT`
  - the TCP port the relay process listens on locally
- `VIBE_PUBLIC_RELAY_BASE_URL`
  - the client-facing relay origin shown in app config and used when the relay builds preview or
    forwarding links

The install scripts expose the same concepts as:

- `RELAY_BIND_HOST` -> `VIBE_RELAY_HOST`
- `RELAY_PORT` -> `VIBE_RELAY_PORT`
- `RELAY_PUBLIC_BASE_URL` -> `VIBE_PUBLIC_RELAY_BASE_URL`

Important behavior:

- the relay listens on `0.0.0.0:8787` by default
- setting `RELAY_PUBLIC_BASE_URL=http://45.144.137.240` does not change the relay listener to port
  80; it only tells clients to use `http://45.144.137.240`
- if the relay really listens on `8787` and clients should connect directly to that port, set
  `RELAY_PUBLIC_BASE_URL=http://45.144.137.240:8787`
- `0.0.0.0` is valid as a bind host but not as a public URL host
- `127.0.0.1` or `localhost` are only correct for same-machine local development

Recommended shapes:

- direct public-IP access
  - bind `0.0.0.0`
  - keep the real port in `RELAY_PUBLIC_BASE_URL`
- reverse proxy or load balancer in front
  - bind `127.0.0.1` or a private interface
  - set `RELAY_PUBLIC_BASE_URL` to the external HTTPS origin

Examples:

```bash
# Direct public IP access on port 8787
RELAY_BIND_HOST=0.0.0.0
RELAY_PORT=8787
RELAY_PUBLIC_BASE_URL=http://45.144.137.240:8787
```

```bash
# Reverse proxy terminates TLS and forwards to local relay
RELAY_BIND_HOST=127.0.0.1
RELAY_PORT=8787
RELAY_PUBLIC_BASE_URL=https://relay.example.com
```

## HTTP vs HTTPS

The relay does not require HTTPS at the process layer. It can run behind plain HTTP or behind a
TLS terminator. Choose the scheme based on your deployment boundary:

- local development on one machine
  - `http` is acceptable
- private LAN test deployments
  - `http` can be acceptable if you control the network and understand the plaintext tradeoff
- internet-facing, mobile, desktop, or shared-team deployments
  - use `https`

Practical rule:

- if a browser, desktop client, Android client, or another user's device reaches the relay across
  an untrusted network, treat HTTPS as the expected production shape

Avoid these client-facing values outside local development:

- `http://127.0.0.1:8787`
- `http://localhost:8787`
- `http://0.0.0.0:8787`

## Supported Install Automation

The repository currently ships automation for:

- Linux: [`scripts/install-relay.sh`](../scripts/install-relay.sh)
  - downloads the published Linux CLI archive unless you point it to a local archive
  - installs `vibe-relay`
  - writes `/etc/vibe-relay/relay.env`
  - creates and optionally starts a `systemd` service
- Windows: [`scripts/install-relay.ps1`](../scripts/install-relay.ps1)
  - downloads the published Windows CLI archive unless you point it to a local archive
  - installs `vibe-relay.exe`
  - writes `relay-env.ps1` plus a launcher script
  - creates and optionally starts a Windows Scheduled Task for auto-start

Current boundary:

- there is no repository-provided one-click relay installer for macOS at this time

## Recommended Auth Split

Use two different relay secrets in self-hosted deployments:

- `VIBE_RELAY_ACCESS_TOKEN`
  - the human control-plane token for Web, desktop, Android, and operator API use
- `VIBE_RELAY_ENROLLMENT_TOKEN`
  - the bootstrap token used by `vibe-agent` during initial device registration

After a successful registration, the agent stores its issued device credential in
`<working-root>/.vibe-agent/identity.json` and uses that device identity for heartbeats, task
claiming, shell polling, workspace/Git requests, and preview bridge traffic on later restarts.

Compatibility note:

- if you omit `VIBE_RELAY_ENROLLMENT_TOKEN`, the relay still accepts the control-plane token for
  agent registration as an admin/compatibility path
- the recommended deployment shape is still to keep the control-plane token off the device and use
  a dedicated enrollment token instead
- relay identity is derived from configured tokens and issued device credentials; external
  `x-vibe-*` actor headers are not a supported auth mechanism

## Linux Quick Install

```bash
curl -fsSL https://raw.githubusercontent.com/fage-ac-org/vibe-everywhere/main/scripts/install-relay.sh -o install-relay.sh
sudo RELAY_PUBLIC_BASE_URL=https://relay.example.com \
  RELAY_ACCESS_TOKEN=change-control-token \
  RELAY_ENROLLMENT_TOKEN=change-agent-enrollment-token \
  bash install-relay.sh
```

Optional inputs:

- `VIBE_RELEASE_TAG`
- `RELAY_CLI_ARCHIVE_URL`
- `RELAY_CLI_ARCHIVE_PATH`
- `RELAY_BIND_HOST`
- `RELAY_PORT`
- `RELAY_FORWARD_HOST`
- `RELAY_ACCESS_TOKEN`
- `RELAY_ENROLLMENT_TOKEN`
- `CREATE_SYSTEMD_SERVICE`
- `ENABLE_AND_START_SERVICE`

Installed paths:

- binary: `/usr/local/bin/vibe-relay`
- env file: `/etc/vibe-relay/relay.env`
- state file default: `/var/lib/vibe-relay/relay-state.json`
- service file: `/etc/systemd/system/vibe-relay.service`

Useful commands:

```bash
sudo systemctl status vibe-relay
sudo journalctl -u vibe-relay -f
sudo systemctl restart vibe-relay
```

Direct public-IP example:

```bash
curl -fsSL https://raw.githubusercontent.com/fage-ac-org/vibe-everywhere/main/scripts/install-relay.sh -o install-relay.sh
sudo RELAY_BIND_HOST=0.0.0.0 \
  RELAY_PORT=8787 \
  RELAY_PUBLIC_BASE_URL=http://45.144.137.240:8787 \
  RELAY_ACCESS_TOKEN=change-control-token \
  RELAY_ENROLLMENT_TOKEN=change-agent-enrollment-token \
  bash install-relay.sh
```

Reverse-proxy example:

```bash
curl -fsSL https://raw.githubusercontent.com/fage-ac-org/vibe-everywhere/main/scripts/install-relay.sh -o install-relay.sh
sudo RELAY_BIND_HOST=127.0.0.1 \
  RELAY_PORT=8787 \
  RELAY_PUBLIC_BASE_URL=https://relay.example.com \
  RELAY_ACCESS_TOKEN=change-control-token \
  RELAY_ENROLLMENT_TOKEN=change-agent-enrollment-token \
  bash install-relay.sh
```

## Windows Quick Install

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\install-relay.ps1 `
  -PublicRelayBaseUrl https://relay.example.com `
  -RelayAccessToken change-control-token `
  -RelayEnrollmentToken change-agent-enrollment-token
```

Optional inputs:

- `-ReleaseTag`
- `-ArchiveUrl`
- `-ArchivePath`
- `-RelayBindHost`
- `-RelayPort`
- `-RelayForwardHost`
- `-RelayAccessToken`
- `-RelayEnrollmentToken`
- `-SkipStartupTask`

Installed paths:

- binary: `C:\Program Files\Vibe Everywhere\vibe-relay.exe`
- side-by-side runtime files: `Packet.dll`, `wintun.dll`, `WinDivert64.sys` and `WinDivert.dll`
  when present
- env script: `C:\Program Files\Vibe Everywhere\relay-env.ps1`
- launcher: `C:\Program Files\Vibe Everywhere\Start-VibeRelay.ps1`
- state file default: `%ProgramData%\Vibe Everywhere\state\relay-state.json`

Useful commands:

```powershell
Get-ScheduledTask -TaskName VibeRelay
Start-ScheduledTask -TaskName VibeRelay
Stop-Process -Name vibe-relay
```

Health check examples:

```bash
curl http://127.0.0.1:8787/api/health
curl http://45.144.137.240:8787/api/health
curl https://relay.example.com/api/health
```

## Start an Agent

Once the relay is reachable, start an agent on the target machine:

```bash
VIBE_RELAY_URL=https://relay.example.com \
VIBE_RELAY_ENROLLMENT_TOKEN=change-agent-enrollment-token \
VIBE_DEVICE_NAME=build-node-01 \
./vibe-agent
```

Notes:

- install at least one provider CLI such as `codex`, `claude`, or `opencode` if you want AI
  session execution
- on Windows, keep the extracted CLI runtime files beside `vibe-agent.exe`; do not copy only the
  executable out of the archive
- `VIBE_RELAY_URL` should point to the relay origin the agent can actually reach
- the first successful registration writes `<working-root>/.vibe-agent/identity.json`; later
  restarts reuse that issued device credential instead of needing the control-plane token on the
  agent host
- deleting the identity file forces a fresh enrollment on the next agent start
- `VIBE_PUBLIC_RELAY_BASE_URL` and `VIBE_RELAY_FORWARD_HOST` affect client-facing links generated
  by the relay

## Agent Ports And Overlay Behavior

In the default relay-polling deployment, the agent does not expose one fixed public control-plane
port the way the relay does. It mainly opens outbound requests to the relay.

When EasyTier overlay is enabled on the agent by setting `VIBE_EASYTIER_NETWORK_NAME`, the agent
also starts local bridge listeners for overlay traffic:

- `19090`
  - shell bridge
- `19091`
  - port-forward bridge
- `19092`
  - task bridge

These ports can be changed with:

- `VIBE_AGENT_SHELL_BRIDGE_PORT`
- `VIBE_AGENT_PORT_FORWARD_BRIDGE_PORT`
- `VIBE_AGENT_TASK_BRIDGE_PORT`

Operational meaning:

- these are relay-to-agent overlay bridge ports, not browser-facing or mobile-facing public ports
- if another service already uses one of them on the target host, change the corresponding env var
  before starting the agent

## EasyTier Defaults In This Repository

Relay side:

- embedded EasyTier starts when `VIBE_EASYTIER_NETWORK_NAME` is set or
  `VIBE_EASYTIER_RELAY_ENABLED=true`
- if `VIBE_EASYTIER_LISTENERS` is not set, the relay defaults to TCP and UDP listener port `11010`

Agent side:

- embedded EasyTier starts when `VIBE_EASYTIER_NETWORK_NAME` is set
- `VIBE_EASYTIER_NO_LISTENER=true` by default on the agent, so the agent does not accept inbound
  EasyTier peers unless you explicitly disable that setting
- if you set `VIBE_EASYTIER_NO_LISTENER=false` and leave `VIBE_EASYTIER_LISTENERS` empty, this
  repository defaults the agent listener to TCP and UDP `11010`

What this means in practice:

- the relay commonly acts as a reachable EasyTier entry point
- the agent commonly dials out to relay peers instead of listening for arbitrary inbound overlay
  peers
- enabling overlay does not change the relay HTTP listener port; it adds separate overlay-related
  listeners

## Related Documents

- [README.md](../README.md)
- [README.en.md](../README.en.md)
- [DEVELOPMENT.md](../DEVELOPMENT.md)
- [TESTING.md](../TESTING.md)
