# Vibe Everywhere Iteration Specs v6

Last updated: 2026-03-30

## Purpose

Version 6 tracks the product-reset epoch for the current remote AI development workspace.

The stable user-facing model is:

1. server
2. host
3. project
4. conversation / task execution
5. changes / files / logs

## Current Decisions

- The default top-level navigation is `首页 / 项目 / 我的`.
- The default project workspace is `会话 / 变更 / 文件 / 日志`.
- Conversation remains the default entry inside a project.
- Execution modes remain supported per task: `只读 / 可改文件 / 可改并测试`.
- Older secondary management and desktop-only workflow surfaces are removed from the shipped
  product baseline.

## Delivered In v6 So Far

- product and plan narrative reset around the reduced AI development workspace
- retained host/project inventory with availability states
- project workspace with conversation-first flow plus changes/files/logs
- per-task summaries, raw event expansion, task stop, retry, and explain-result actions
- inline provider prompts with preset options and custom input
- frontend simplification of top-level navigation and settings to the reduced baseline
- relay public-route simplification so the shipped UI only depends on the current product workflow

## Remaining Focus

- deepen host project discovery without regressing responsiveness
- keep the conversation and review path coherent across mobile and desktop widths
- continue trimming obsolete backend/public-surface capabilities that no longer belong to the
  reduced product model

## Validation

- `cargo check -p vibe-relay -p vibe-agent -p vibe-app`
- `cargo test --workspace --all-targets -- --nocapture`
- `cd apps/vibe-app && npm run build`
