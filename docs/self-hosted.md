# Self-Hosted Deployment Guide

Last updated: 2026-03-28

Vibe Everywhere is designed to run in self-hosted mode first. This guide is for operators who want
to deploy the relay, keep it running after reboot, and connect Web, desktop, or Android clients
without hardcoded loopback assumptions.

## What You Need To Decide Up Front

Before installation, decide these runtime values:

- the public relay URL clients will actually use, for example
  `https://relay.example.com` or `http://192.168.1.20:8787`
- whether the relay requires an access token
- which host should be used for preview/forward links
- where you want relay state persisted on disk

Do not hardcode `127.0.0.1` or `localhost` as the user-facing relay origin for cross-device use.

## Supported Bootstrap Automation

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

- macOS release automation is not shipped yet, so there is no repository-supported one-click relay
  installer for macOS at this time

## Linux Quick Install

Run the installer from a cloned repository or a downloaded copy of
[`scripts/install-relay.sh`](../scripts/install-relay.sh):

```bash
curl -fsSL https://raw.githubusercontent.com/fage-ac-org/vibe-everywhere/main/scripts/install-relay.sh -o install-relay.sh
sudo RELAY_PUBLIC_BASE_URL=https://relay.example.com \
  RELAY_ACCESS_TOKEN=change-me \
  bash install-relay.sh
```

Optional inputs:

- `VIBE_RELEASE_TAG`
  - install a specific GitHub release tag instead of the latest release
- `RELAY_CLI_ARCHIVE_URL`
  - override the download URL entirely
- `RELAY_CLI_ARCHIVE_PATH`
  - install from a local CLI archive path
- `RELAY_BIND_HOST`
  - defaults to `0.0.0.0`
- `RELAY_PORT`
  - defaults to `8787`
- `RELAY_FORWARD_HOST`
  - defaults to the host parsed from `RELAY_PUBLIC_BASE_URL` when provided
- `RELAY_ACCESS_TOKEN`
  - optional access token
- `CREATE_SYSTEMD_SERVICE`
  - set to `0` if you only want the binary and env file
- `ENABLE_AND_START_SERVICE`
  - set to `0` if you want to inspect the config before first start

Resulting paths:

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

## Windows Quick Install

Run the installer from an elevated PowerShell session:

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\install-relay.ps1 `
  -PublicRelayBaseUrl https://relay.example.com `
  -RelayAccessToken change-me
```

Optional inputs:

- `-ReleaseTag`
  - install a specific GitHub release tag instead of the latest release
- `-ArchiveUrl`
  - override the download URL entirely
- `-ArchivePath`
  - install from a local CLI archive path
- `-RelayBindHost`
  - defaults to `0.0.0.0`
- `-RelayPort`
  - defaults to `8787`
- `-RelayForwardHost`
  - defaults to the host parsed from `-PublicRelayBaseUrl` when provided
- `-RelayAccessToken`
  - optional access token
- `-SkipStartupTask`
  - install the binary and config without registering auto-start

Resulting paths:

- binary: `C:\Program Files\Vibe Everywhere\vibe-relay.exe`
- env script: `C:\Program Files\Vibe Everywhere\relay-env.ps1`
- launcher: `C:\Program Files\Vibe Everywhere\Start-VibeRelay.ps1`
- state file default: `%ProgramData%\Vibe Everywhere\state\relay-state.json`

Useful commands:

```powershell
Get-ScheduledTask -TaskName VibeRelay
Start-ScheduledTask -TaskName VibeRelay
Stop-Process -Name vibe-relay
```

## Agent Connection Example

Once the relay is reachable, start an agent on the target machine with values for your actual
environment:

```bash
VIBE_RELAY_URL=https://relay.example.com \
VIBE_RELAY_ACCESS_TOKEN=change-me \
VIBE_DEVICE_NAME=build-node-01 \
./vibe-agent
```

Notes:

- agents still need a provider CLI such as `codex`, `claude`, or `opencode` if you want AI session
  execution
- `VIBE_RELAY_URL` should match the real relay origin the agent can reach
- preview/mobile users depend on `VIBE_PUBLIC_RELAY_BASE_URL` and `VIBE_RELAY_FORWARD_HOST` being
  configured correctly on the relay side

## Configuration Guardrails

Keep these rules in place for production-style self-hosting:

- relay bind host and public relay URL are different concerns
- public/mobile clients must not rely on loopback defaults
- relay URL, access token, and preview host must stay runtime configuration, not compiled constants
- user choice and persisted client settings still override relay-provided config in the app

## Related Documents

- [README.md](../README.md)
- [README.en.md](../README.en.md)
- [TESTING.md](../TESTING.md)
- [docs/releases/README.md](./releases/README.md)
