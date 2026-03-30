# Vibe Everywhere Product Evolution Plan

Last updated: 2026-03-30

## Purpose

This file is the concise execution record for the repository's current product baseline.

Detailed, versioned planning lives under [`docs/plans/README.md`](./docs/plans/README.md).

## Product Direction

The current target product is:

- a self-hosted remote AI development workspace
- organized around `server -> host -> project -> conversation / task execution`
- focused on the project workspace surfaces `conversation / changes / files / logs`
- delivered across Web, Tauri Desktop, and Android

The product does not currently include the previously prototyped secondary management and runtime
surfaces in the default user-facing workflow.

## Current MVP Baseline

Delivered today:

- device registration, heartbeat, presence, and provider discovery
- relay-backed conversation and task orchestration
- workspace browse / preview and Git inspection backends
- Rust relay, Rust agent, Vue control app, Tauri desktop shell, and Android packaging
- provider integration for Codex, Claude Code, and OpenCode
- self-hosted relay as the default operating model

Active user-facing workflow:

- top-level navigation: `首页 / 项目 / 我的`
- project workspace: `会话 / 变更 / 文件 / 日志`
- per-task execution modes: `只读 / 可改文件 / 可改并测试`

Not in the current product baseline:

- extra management dashboards beyond the core workflow
- secondary runtime tooling outside the conversation-centered path
- desktop-specific branching or parallel-workspace lifecycle surfaces

## Active Plan Set

- planning index: [`docs/plans/README.md`](./docs/plans/README.md)
- process governance: [`docs/plans/process.md`](./docs/plans/process.md)
- active iteration summary: [`docs/plans/iterations/v6-summary.md`](./docs/plans/iterations/v6-summary.md)
- active iteration details: [`docs/plans/iterations/v6-details.md`](./docs/plans/iterations/v6-details.md)
- active remediation summary: [`docs/plans/remediation/v12-summary.md`](./docs/plans/remediation/v12-summary.md)
- active remediation details: [`docs/plans/remediation/v12-details.md`](./docs/plans/remediation/v12-details.md)

## Current Iteration Notes

- Iteration roadmap `v6` remains the active product-reset epoch.
- The current reduction pass keeps trimming obsolete secondary surfaces around the core workflow.
- The frontend now converges on `首页 / 项目 / 我的` plus the project workspace
  `会话 / 变更 / 文件 / 日志`.
- Relay public routes have been narrowed to the conversation, workspace, and Git inspection paths
  used by the current product workflow.

## Verification Log

- 2026-03-30: `cd apps/vibe-app && npm run build` succeeded after aligning the frontend with the
  reduced product baseline.
- 2026-03-30: `cargo check -p vibe-relay -p vibe-agent -p vibe-app` succeeded after narrowing the
  relay public router to the current product-facing workflow.
