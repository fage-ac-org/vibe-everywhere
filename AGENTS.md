# Repository Guidelines

## Project Structure & Module Organization
This repository is a Rust workspace with one shared crate and three apps. `crates/vibe-core` holds shared protocol types and models. `apps/vibe-relay` is the Axum relay API. `apps/vibe-agent` is the device-side daemon, task runner, and provider adapter layer. `apps/vibe-app` is the Vue 3.5 control UI, and `apps/vibe-app/src-tauri` contains the Tauri shell. In the frontend, keep reusable API/runtime code in `src/lib`, state in `src/stores`, and screens in `src/views`. Do not commit build output from `target/`, `dist/`, or `node_modules/`.

## Build, Test, and Development Commands
- `cargo check -p vibe-relay -p vibe-agent -p vibe-app`: verify all Rust targets compile.
- `cargo test -p vibe-agent -- --nocapture`: run the current Rust unit tests.
- `cargo run -p vibe-relay`: start the relay on port `8787`.
- `cargo run -p vibe-agent -- --relay-url http://127.0.0.1:8787`: start an agent against the local relay.
- `cd apps/vibe-app && npm install && npm run dev`: run the Vue control app locally.
- `cd apps/vibe-app && npm run build`: run `vue-tsc` and produce the production bundle.
- `cd apps/vibe-app && npm run tauri dev`: launch the desktop shell.

## Coding Style & Naming Conventions
Use `cargo fmt --all` for Rust formatting. Follow Rust defaults: `snake_case` for functions/modules, `PascalCase` for structs/enums, and keep shared protocol changes in `vibe-core`. For Vue and TypeScript, follow the existing Composition API style with `<script setup lang="ts">`, 2-space indentation, `PascalCase` component filenames such as `DashboardView.vue`, and `camelCase` store/actions such as `useControlStore` and `reloadAll`.

## Testing Guidelines
Prefer focused Rust unit tests near parsing, protocol, and adapter logic; current examples live in `apps/vibe-agent/src/main.rs`. Name tests by behavior, for example `claude_tool_use_maps_to_tool_call`. When changing relay or adapter behavior, add or update tests and rerun `cargo test -p vibe-agent -- --nocapture`. The frontend has no dedicated test harness yet, so at minimum run `npm run build` before submitting.

## Commit & Pull Request Guidelines
Git history is not available in this workspace snapshot, so use Conventional Commit style, for example `feat(agent): add claude stream-json mapping`. Pull requests should state the affected crate/app, summarize behavior changes, list validation commands run, and include screenshots for UI updates. Call out new `VIBE_*` environment variables or system dependencies explicitly, and never include secrets, auth tokens, or local machine-specific config in commits.
