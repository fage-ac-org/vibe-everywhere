# Vibe Everywhere Testing Strategy

## Goals

This repository has four independently changing surfaces:

- `crates/vibe-core`: shared protocol, enums, and records
- `apps/vibe-relay`: relay control plane and public APIs
- `apps/vibe-agent`: device runtime and provider adapters
- `apps/vibe-app` and `apps/vibe-app/src-tauri`: Vue control UI and Tauri shell

The current product baseline is:

- top-level navigation: `首页 / 项目 / 我的`
- project workspace: `会话 / 变更 / 文件`
- stable object model: `服务器 -> 主机 -> 项目 -> 会话/任务执行`

The test plan must catch:

- protocol drift between relay, agent, and app
- conversation/task regression
- provider event mapping regression
- frontend contract drift against relay APIs
- navigation or visibility regression against the shipped product model
- environment/configuration regression before release
- release packaging drift, missing release notes, or broken operator bootstrap scripts

## Core Commands

```bash
cargo fmt --all --check
cargo check -p vibe-relay -p vibe-agent -p vibe-app
cargo test --workspace --all-targets -- --nocapture
cd apps/vibe-app && npm run build
```

Use Windows packaging and Android packaging gates when the corresponding surface changes.

## Manual Product Regression Checklist

Run this checklist whenever UI semantics, navigation, relay configuration behavior, or project
workspace layout changes.

### Home / Host / Project Entry

- configure relay URL and access token from the in-app settings page
- confirm the default mobile navigation shows `首页 / 项目 / 我的`
- confirm 首页 shows current host and project attention items instead of a legacy dashboard hero
- confirm 项目页 lists hosts and projects and can open a project workspace
- confirm a host can surface projects even when they have no prior conversation history, as long as
  the project is discoverable from the configured agent working root
- confirm previously discovered projects remain visible with a degraded state when the host goes
  offline or project refresh fails
- confirm offline and empty-state messaging is clear when no host or project is available

### Project Workspace

- open a project and confirm the header keeps host, project, branch, and AI state visible
- confirm the default tab is `会话`
- confirm the project also exposes `变更 / 文件`
- open an inventory-only project with no prior history and confirm the empty conversation state
  still allows sending the first prompt
- create a new conversation and continue an existing conversation in the same project
- switch execution mode between `只读 / 可改文件 / 可改并测试` and confirm the selected mode is
  visible in the conversation transcript metadata
- verify provider pending-input prompts can be answered inline with option chips and custom text
- verify latest task status changes appear in the project workspace after refresh or live updates
- verify conversation turns show a per-task summary, recent execution events, and expandable raw
  event output instead of only flat chat bubbles
- verify pending or running task cards can request stop directly from the conversation surface
- verify completed or failed task cards expose quick follow-up actions for retry and explanation
- verify completed tasks can jump directly to `变更`

### Review And Inspection

- verify `变更` shows Git summary information before raw detail
- verify `变更` shows review summary cards before the raw file diff panel
- verify selecting a changed file loads its staged and/or unstaged diff output
- verify `文件` can browse the project tree and preview a text file
- when an ACP-backed task runs in `只读`, verify write attempts or terminal command attempts are
  rejected; when it runs in `可改文件`, verify test-style terminal commands are rejected until
  `可改并测试` is selected
- verify CLI-backed Codex tasks derive sandbox/approval flags from execution mode, and CLI-backed
  Claude read-only tasks enter native `plan` permission mode

### Settings

- confirm 我的 contains relay settings, locale, and theme instead of primary project workflow
  content
- confirm a newly opened conversation composer inherits the configured default execution mode

## Release And Packaging Checks

- confirm `./scripts/verify-release-version.sh vX.Y.Z` succeeds for the intended release tag
- confirm release asset names in `.github/workflows/release.yml` include the tag/version
- confirm `README.md` and `README.en.md` stay user/operator-facing
- confirm deployment docs describe CLI binary installation separately from relay startup
- confirm `DEVELOPMENT.md` contains the current developer entry path when build or contributor
  instructions change

## Reporting

Each user-facing verification report should state:

- which commands were run
- whether manual checks were performed
- any skipped checks
- any residual risks or unverified areas
