# Vibe Everywhere

[![CI](https://github.com/fage-ac-org/vibe-everywhere/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/fage-ac-org/vibe-everywhere/actions/workflows/ci.yml)
[![Release](https://github.com/fage-ac-org/vibe-everywhere/actions/workflows/release.yml/badge.svg)](https://github.com/fage-ac-org/vibe-everywhere/actions/workflows/release.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)

[中文](./README.md) | [English](./README.en.md)

Vibe Everywhere is a self-hosted remote AI control plane. It is not a traditional remote desktop
product. Instead, it organizes remote development around `Relay + Agent + Control App`: run
`vibe-relay` and `vibe-agent` on your own infrastructure, then use the Web UI, desktop shell, or
Android client to drive AI sessions, inspect workspaces, review Git state, open previews, and only
drop into terminal or advanced networking tools when needed.

## Who It Is For

- people who want AI coding tasks to run on remote machines while keeping one control surface
- teams that prefer self-hosting over a managed service
- operators managing multiple devices, workspaces, and provider CLIs
- teams that want a practical MVP now and a path toward stronger enterprise capabilities later

## What It Can Do Today

- create, stream, and cancel AI sessions
- register devices, track presence, and show provider availability
- browse workspaces, preview text files, and inspect Git state
- expose preview / forwarding flows plus terminal and advanced connection tools when needed
- connect through Web, Tauri desktop, and Android control clients
- ship bilingual UI support for English and Simplified Chinese, plus light / dark / system themes
- follow a self-hosted-first relay model without product defaults that assume fixed loopback
  addresses

## Three-Minute Start

### 1. Deploy the Relay

Start with the operator guide:

- [Self-Hosted Deployment Guide](./docs/self-hosted.md)

The repository currently ships two bootstrap installers:

- Linux with `systemd`

```bash
curl -fsSL https://raw.githubusercontent.com/fage-ac-org/vibe-everywhere/main/scripts/install-relay.sh -o install-relay.sh
sudo RELAY_PUBLIC_BASE_URL=https://relay.example.com \
  RELAY_ACCESS_TOKEN=change-me \
  bash install-relay.sh
```

- Windows with a startup task

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\install-relay.ps1 `
  -PublicRelayBaseUrl https://relay.example.com `
  -RelayAccessToken change-me
```

Notes:

- both installers default to the latest GitHub Release, but you can pin a version with
  `VIBE_RELEASE_TAG` or `-ReleaseTag`
- the scripts do not hardcode public relay addresses, tokens, or preview hosts
- the repository does not yet ship a one-click macOS relay installer because macOS release assets
  are not published yet

### 2. Start an Agent on the Target Machine

Download the CLI release package, extract `vibe-agent`, and start it with the values for your own
environment:

```bash
VIBE_RELAY_URL=https://relay.example.com \
VIBE_RELAY_ACCESS_TOKEN=change-me \
VIBE_DEVICE_NAME=build-node-01 \
./vibe-agent
```

To execute AI sessions, the target machine still needs at least one provider CLI:

- `codex`
- `claude`
- `opencode`

### 3. Open a Control Client

The same relay can be reached from:

- Web
- Tauri desktop
- Android

Recommended flow:

1. deploy the relay
2. start at least one agent
3. validate from Web or desktop first
4. add the Android client when mobile access is needed

## Downloads And Releases

GitHub Release assets are now versioned so operators can identify them after download. Examples:

- `vibe-everywhere-cli-v0.1.4-x86_64-unknown-linux-gnu.tar.gz`
- `vibe-everywhere-cli-v0.1.4-x86_64-pc-windows-msvc.zip`
- `vibe-everywhere-desktop-v0.1.4-linux-x86_64.AppImage`
- `vibe-everywhere-desktop-v0.1.4-linux-x86_64.deb`
- `vibe-everywhere-desktop-v0.1.4-windows-x86_64.exe`
- `vibe-everywhere-desktop-v0.1.4-windows-x86_64.msi`
- `vibe-everywhere-android-v0.1.4-arm64-debug.apk`
- `vibe-everywhere-android-v0.1.4-arm64-release-unsigned.apk`
- `vibe-everywhere-android-v0.1.4-arm64-release.aab`
- `SHA256SUMS.txt`

Release notes are also repository-owned now:

- [Release Notes Workflow](./docs/releases/README.md)
- next release draft: [docs/releases/unreleased.md](./docs/releases/unreleased.md)

Notes:

- release assets no longer repackage repository README files
- if Android signing secrets are not configured, the release APK remains `unsigned`

## Self-Hosted Configuration Basics

These values must remain deployment-time configuration, not hardcoded product defaults:

- `VIBE_PUBLIC_RELAY_BASE_URL`
- `VIBE_RELAY_ACCESS_TOKEN`
- `VIBE_RELAY_FORWARD_HOST`
- `VIBE_RELAY_URL`

Keep these address roles distinct:

- the relay bind address, for example `0.0.0.0:8787`
- the relay origin that agents use
- the public origin or host that users open for preview links

If phones or other machines need to connect, do not use `127.0.0.1` as the user-facing relay
origin.

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

## Developer Entry

If you are here to build from source or contribute changes, start here instead of the operator
sections above.

### Repository Layout

```text
.
├── apps
│   ├── vibe-relay        # Relay API / control plane
│   ├── vibe-agent        # Device agent / runtimes / providers
│   └── vibe-app          # Vue control app
│       └── src-tauri     # Tauri desktop shell + Android shell
├── crates
│   └── vibe-core         # Shared protocol / models
├── docs
│   ├── plans            # Versioned iteration / remediation plans
│   └── releases         # Versioned release notes and next-release draft
├── scripts               # Installers, smoke tests, Android doctor
└── TESTING.md            # Test strategy and regression checklist
```

### Local Prerequisites

- Rust stable toolchain
- Node.js 24.14.x
- `protobuf-compiler` or another working `protoc`
- WebKitGTK / GTK development packages when building Tauri on Linux
- JDK 17, Android SDK cmdline-tools, `platforms;android-36`, `build-tools;35.0.0`, and
  `ndk;25.2.9519653` for Android builds
- Npcap with WinPcap API-compatible mode enabled on Windows if you need EasyTier / overlay support

### Local Development Commands

Start the relay:

```bash
cargo run -p vibe-relay
```

Start an agent:

```bash
cargo run -p vibe-agent -- --relay-url http://127.0.0.1:8787
```

Start the Web UI:

```bash
cd apps/vibe-app
npm ci
npm run dev
```

Start the desktop shell:

```bash
cd apps/vibe-app
npm ci
npm run tauri dev
```

Build the frontend:

```bash
cd apps/vibe-app
npm ci
npm run build
```

### Android Builds

Debug APK:

```bash
rustup target add aarch64-linux-android

export JAVA_HOME=/path/to/jdk-17
export ANDROID_HOME=$HOME/Android/Sdk
export ANDROID_SDK_ROOT=$ANDROID_HOME
export NDK_HOME=$ANDROID_HOME/ndk/25.2.9519653
export ANDROID_NDK_HOME=$NDK_HOME

cd apps/vibe-app
npm ci
npm run android:doctor
npm run android:build:debug:apk
```

Release APK / AAB:

```bash
cd apps/vibe-app
npm run android:build:apk
npm run android:build:aab
```

To sign release APK / AAB outputs, provide the signing values through environment variables or
`apps/vibe-app/src-tauri/gen/android/app/keystore.properties`:

- `VIBE_ANDROID_KEYSTORE_PATH`
- `VIBE_ANDROID_KEYSTORE_PASSWORD`
- `VIBE_ANDROID_KEY_ALIAS`
- `VIBE_ANDROID_KEY_PASSWORD`

### Common Validation Commands

```bash
cargo fmt --all --check
cargo check --locked -p vibe-relay -p vibe-agent -p vibe-app
cargo test --locked --workspace --all-targets -- --nocapture
cd apps/vibe-app && npm run build
./scripts/dual-process-smoke.sh relay_polling
./scripts/dual-process-smoke.sh overlay
```

See also:

- [TESTING.md](./TESTING.md)

## GitHub Actions

The repository ships two primary workflows:

- `CI`
  - triggers on pushes to `main`, pull requests, and manual dispatch
  - runs formatting, workspace builds, tests, frontend build, `relay_polling` smoke, Windows
    compatibility checks, and Android debug APK packaging
- `Release`
  - triggers on `v*` tags
  - runs full verification, blocking `overlay` smoke, CLI / desktop / Android packaging, checksum
    generation, and GitHub Release publication

Before pushing a release tag:

1. update the version
2. update [docs/releases/unreleased.md](./docs/releases/unreleased.md)
3. create the matching `docs/releases/vX.Y.Z.md`
4. push the tag

Example:

```bash
git tag v0.1.4
git push origin v0.1.4
```

## Roadmap

- add iOS client support and a broader mobile release path
- continue improving deployment and operator onboarding
- deepen enterprise authentication, audit, and role management
- add stronger frontend automation and protocol round-trip coverage
- keep separating advanced capabilities from the main user workflow more cleanly

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
