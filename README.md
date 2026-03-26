# Vibe Remote (Rust + Tauri)

这个仓库是一个 Rust 版本的 `vibe-remote` 风格项目，目标是把参考项目里的核心思路迁移到更适合跨平台和移动端的技术栈上：

- 参考仓库: `https://github.com/cyhhao/vibe-remote`
- 可借鉴部分: 消息入口、设备注册、Agent 编排、统一控制台
- 本仓库的改造方向: `Rust relay + Rust agent + Tauri 2 app`

## 为什么不是直接照搬

`cyhhao/vibe-remote` 当前主要是 Python 技术栈，围绕聊天平台消息和本地 Agent 运行时做桥接。它更像一个“远程 AI 协作控制面板”，不是传统远程桌面产品。

如果你的目标是：

- 服务端/Agent 支持 Windows、macOS、Linux
- 客户端控制端同时支持桌面、iOS、Android
- 核心逻辑统一用 Rust

那么更稳的拆法是：

1. `vibe-core`
   定义协议、设备信息、能力声明、会话模型。
2. `vibe-relay`
   负责设备注册、在线状态、控制面 API，后续可扩展 WebSocket 信令。
3. `vibe-agent`
   跑在被控机器上，注册设备并上报能力，后续接入命令执行、文件同步、屏幕流。
4. `vibe-app`
   基于 Tauri 2，一套前端 + 一套 Rust 壳同时覆盖桌面和移动端。

## 当前骨架包含什么

- 一个 `axum` 版 relay 服务，支持设备、任务、事件流
- 一个 `clap + reqwest` 版 agent 常驻程序，支持心跳、任务轮询、provider 适配
- 一个 `Tauri 2 + Vue 3.5 + Vite` 控制端壳
- 一份共享协议 crate，给 relay、agent、app 共用

## 当前 ACP 状态

- `OpenCode` 已接入 ACP MVP，agent 会优先走 `opencode acp`，并实现基础的 `session/update`、`permission`、`fs/*`、`terminal/*` 客户端能力。
- `Codex` 现在也已切到项目内置的 ACP 风格 adapter：不依赖外部 `codex-acp` 二进制，而是由 `vibe-agent` 内部解析 `codex exec --json` 事件流并映射成统一任务事件。
- `Claude Code` 当前仍不是官方 ACP transport，但已经接入结构化 `stream-json` adapter：agent 会解析 assistant/tool/result 事件并映射成统一任务事件，而不是只把 stdout 当原始文本。
- 当前环境里 `opencode acp` 本身存在监听启动异常，所以 `OpenCode ACP` 运行链路主要做到了代码级和编译级落地，最终联调建议在正常宿主机上完成。`Codex` 内置 adapter 则已完成本地 JSON 事件样例验证。

## 目录

```text
.
├── apps
│   ├── vibe-agent
│   ├── vibe-app
│   │   └── src-tauri
│   └── vibe-relay
└── crates
    └── vibe-core
```

## 本地开发

### 1. 启动 relay

```bash
cargo run -p vibe-relay
```

默认监听 `0.0.0.0:8787`，控制端默认仍通过 `http://127.0.0.1:8787` 访问。

个人版现在还支持：

- 通过 `VIBE_RELAY_ACCESS_TOKEN` 打开单用户访问控制
- 通过 `VIBE_PUBLIC_RELAY_BASE_URL` 告诉桌面壳和前端应该访问哪个公网/内网地址
- 通过 `VIBE_RELAY_STATE_FILE` 持久化设备、任务和事件状态
- 默认把 relay 状态写到 `$HOME/.vibe-remote/relay-state.json`
- 如果设置了 `VIBE_EASYTIER_RELAY_ENABLED=true` 或配置了 `VIBE_EASYTIER_NETWORK_NAME`，relay 会自动托管一个最小 EasyTier 节点，作为 bootstrap / relay 节点使用

### 2. 启动 agent 并注册设备

```bash
cargo run -p vibe-agent -- --relay-url http://127.0.0.1:8787
```

这个进程现在会常驻，并持续：

- 注册设备
- 上报心跳
- 轮询任务
- 调用 `codex` / `claude` / `opencode` provider
- 对 `Codex` 和 `OpenCode` 优先使用 ACP transport，并把协议状态回写到 relay / app
- agent 现在会按 task 上记录的 `execution_protocol` 执行，不再只按 provider 默认值重算
- 把执行日志流式回传给 relay
- 如果配置了 `VIBE_EASYTIER_NETWORK_NAME`，agent 会直接在进程内启动 EasyTier 节点，并连接到配置的 bootstrap peer
- 如果 relay 重启后丢失设备注册状态，agent 会在心跳或 claim 任务时自动重新注册

### 2.1 服务端中转 Shell Session MVP

这一版已经先把"服务端中转"跑通；即使不开 EasyTier，不具备公网 IP 的被控端也能先通过 relay-first 模式工作。

- 控制端先向 relay 创建 shell session
- agent 通过 `/api/devices/:device_id/shell/claim-next` 轮询认领会话
- agent 在本地启动系统 shell，Unix 默认走 `/bin/sh -i`，Windows 默认走 `cmd.exe /Q`
- 控制端选中终端后，会优先通过 `/api/shell/sessions/:session_id/ws` 接收 relay 主动推送的会话快照
- agent 到 relay 这一段当前仍是 claim/input/output 轮询模型，所以这一步属于“控制端实时订阅 + agent 轮询执行”的混合模型
- 当前只覆盖文本终端，不包含屏幕流

### 2.2 Relay-first TCP 端口转发 MVP

这条链路现在已经从“只有控制面”补到了“relay 实际监听端口并转发字节流”。

- 控制端通过 `/api/port-forwards` 创建转发，relay 分配对外暴露的 `relay_host:relay_port`
- agent 通过 `/api/devices/:device_id/port-forwards/claim-next` 认领 pending forward，并自动恢复自己名下的 `active` / `close_requested` forward
- agent 会建立 `/api/port-forwards/:forward_id/tunnel/ws?deviceId=...` 隧道 websocket
- relay 在 `VIBE_RELAY_FORWARD_BIND_HOST` 上监听分配出来的端口，对外接收 TCP 连接
- relay 与 agent 之间通过 websocket 二进制帧转发原始 TCP 字节流；agent 再连接 `target_host:target_port`
- 关闭 forward 时，agent 会把状态回报到 `/api/port-forwards/:forward_id/report`，relay 负责收口 `close_requested -> closed`
- 当前是单 forward 单活跃外部连接的个人版 MVP，后续再扩多连接复用、限流和审计

### 2.3 内建 EasyTier 最小集成

现在已经把 EasyTier 以“项目内直接集成 crate”的方式接进来了，目标只覆盖基础组网和中转：

- `vibe-relay` 会在进程内启动一个最小 EasyTier 节点，作为 bootstrap / relay 节点
- `vibe-agent` 在配置了网络名后会直接启动内嵌 EasyTier 实例，默认启用 `private-mode` 并优先走最小暴露模式
- agent 默认不打开 listener，减少端口暴露；需要时可以再通过 `VIBE_EASYTIER_LISTENERS` 显式打开监听
- 如果你给 agent 配了 `VIBE_EASYTIER_NODE_IP`，会固定虚拟 IP；否则默认走 DHCP
- 当前实现不再依赖外部 `easytier-core` 可执行文件，但首次构建需要系统里可用的 `protoc` / `protobuf-compiler`

最小自托管示例：

```bash
# relay 节点
export VIBE_EASYTIER_RELAY_ENABLED=true
export VIBE_EASYTIER_NETWORK_NAME=personal-net
export VIBE_EASYTIER_NETWORK_SECRET=change-me
cargo run -p vibe-relay

# agent 节点
export VIBE_EASYTIER_NETWORK_NAME=personal-net
export VIBE_EASYTIER_NETWORK_SECRET=change-me
export VIBE_EASYTIER_BOOTSTRAP_URL=tcp://your-relay-host:11010
cargo run -p vibe-agent -- --relay-url http://your-relay-host:8787
```

### 3. 启动 Tauri 前端

```bash
cd apps/vibe-app
npm install
npm run dev
```

前端还支持通过 `.env` 或运行环境注入：

- `VITE_RELAY_BASE_URL`
- `VITE_RELAY_ACCESS_TOKEN`

### 4. 启动完整 Tauri 桌面壳

```bash
cd apps/vibe-app
npm install
npm run tauri dev
```

## 当前 API

当前 relay 已提供：

- `GET /api/health`
- `GET /api/app-config`
- `GET /api/devices`
- `POST /api/devices/register`
- `POST /api/devices/:device_id/heartbeat`
- `POST /api/devices/:device_id/tasks/claim-next`
- `GET /api/tasks`
- `POST /api/tasks`
- `GET /api/tasks/:task_id`
- `POST /api/tasks/:task_id/cancel`
- `POST /api/tasks/:task_id/events`
- `GET /api/shell/sessions`
- `POST /api/shell/sessions`
- `GET /api/shell/sessions/:session_id`
- `GET /api/shell/sessions/:session_id/ws`
- `GET /api/shell/sessions/:session_id/input`
- `POST /api/shell/sessions/:session_id/input`
- `POST /api/shell/sessions/:session_id/output`
- `POST /api/shell/sessions/:session_id/close`
- `POST /api/devices/:device_id/shell/claim-next`
- `GET /api/port-forwards`
- `POST /api/port-forwards`
- `GET /api/port-forwards/:forward_id`
- `POST /api/port-forwards/:forward_id/report`
- `GET /api/port-forwards/:forward_id/tunnel/ws`
- `POST /api/port-forwards/:forward_id/close`
- `POST /api/devices/:device_id/port-forwards/claim-next`
- `GET /api/events/stream`

说明：

- `GET /api/health` 和 `GET /api/app-config` 保持公开，方便控制端启动时探测。
- 其它设备、任务、事件流接口在配置了 `VIBE_RELAY_ACCESS_TOKEN` 后都需要认证。
- 浏览器端的 SSE 由于 `EventSource` 不能自定义 Header，当前通过 `?access_token=...` 连接事件流。
- relay shell session 现在是“控制端 ws 订阅 + agent 轮询执行”的混合模型。
- relay 端口转发现在已经具备完整的 relay-first MVP：控制面创建/claim/close，加上 agent websocket 隧道和真实 TCP 字节转发。
- 当前端口转发数据面仍是单连接 MVP，后续再补并发连接、认证细粒度控制和审计。

## 关键环境变量

### relay

- `VIBE_RELAY_HOST`
- `VIBE_RELAY_PORT`
- `VIBE_PUBLIC_RELAY_BASE_URL`
- `VIBE_RELAY_ACCESS_TOKEN`
- `VIBE_RELAY_STATE_FILE`
- `VIBE_RELAY_FORWARD_HOST`
- `VIBE_RELAY_FORWARD_BIND_HOST`
- `VIBE_RELAY_FORWARD_PORT_START`
- `VIBE_RELAY_FORWARD_PORT_END`
- `VIBE_EASYTIER_RELAY_ENABLED`
- `VIBE_EASYTIER_NETWORK_NAME`
- `VIBE_EASYTIER_NETWORK_SECRET`
- `VIBE_EASYTIER_BOOTSTRAP_URL`
- `VIBE_EASYTIER_LISTENERS`
- `VIBE_EASYTIER_PRIVATE_MODE`
- `VIBE_EASYTIER_INSTANCE_NAME`
- `VIBE_EASYTIER_HOSTNAME`

### agent

- `VIBE_RELAY_URL`
- `VIBE_RELAY_ACCESS_TOKEN`
- `VIBE_DEVICE_NAME`
- `VIBE_DEVICE_ID`
- `VIBE_WORKING_ROOT`
- `VIBE_POLL_INTERVAL_MS`
- `VIBE_HEARTBEAT_INTERVAL_MS`
- `VIBE_CODEX_COMMAND`
- `VIBE_CLAUDE_COMMAND`
- `VIBE_OPENCODE_COMMAND`
- `VIBE_EASYTIER_NETWORK_NAME`
- `VIBE_EASYTIER_NETWORK_SECRET`
- `VIBE_EASYTIER_BOOTSTRAP_URL`
- `VIBE_EASYTIER_NODE_IP`
- `VIBE_EASYTIER_NO_LISTENER`
- `VIBE_EASYTIER_LISTENERS`
- `VIBE_EASYTIER_INSTANCE_NAME`
- `VIBE_CLAUDE_PERMISSION_MODE`
- `VIBE_CLAUDE_ALLOWED_TOOLS`

### frontend / desktop shell

- `VITE_RELAY_BASE_URL`
- `VITE_RELAY_ACCESS_TOKEN`
- `VIBE_PUBLIC_RELAY_BASE_URL`
- `VIBE_RELAY_ACCESS_TOKEN`

说明：

- `Claude Code` 默认走 `--permission-mode acceptEdits`，这样在 root 环境下不会触发 `--dangerously-skip-permissions` 的限制。
- 如果你希望 Claude 在无人值守模式下直接使用 Bash / Edit / Read 等工具，可以通过 `VIBE_CLAUDE_ALLOWED_TOOLS` 传给 `--allowedTools`，例如：`Bash,Read,Edit,Write,Glob,Grep`。
- Tauri 桌面壳会读取 `VIBE_PUBLIC_RELAY_BASE_URL` 和 `VIBE_RELAY_ACCESS_TOKEN` 作为默认连接配置。
- Web 前端会优先读取浏览器本地保存的 relay 地址和 token，其次再回退到 `VITE_*` 环境变量。

## Linux 下的 Tauri 桌面依赖

如果你在 Debian / Ubuntu 系 Linux 上开发桌面壳，Tauri 官方文档当前列出的常见依赖包括：

```bash
sudo apt update
sudo apt install libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libxdo-dev \
  libssl-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev
```

如果当前机器缺少 `glib-2.0` / `gobject-2.0` 相关系统库，`cargo check -p vibe-app` 或 `npm run tauri dev` 仍会卡在 Linux 桌面 WebView 依赖上；这属于系统环境问题，不是项目代码本身的问题。

## Tauri Mobile 前置条件

按 Tauri 2 官方文档：

- Android 需要 Android Studio、`JAVA_HOME`、`ANDROID_HOME`、`NDK_HOME`
- 还要加 Rust Android targets：
  `aarch64-linux-android`、`armv7-linux-androideabi`、`i686-linux-android`、`x86_64-linux-android`
- iOS 只能在 macOS 上构建
- iOS 需要 Xcode，并加 Rust iOS targets：
  `aarch64-apple-ios`、`x86_64-apple-ios`、`aarch64-apple-ios-sim`

## 推荐的下一步实现顺序

1. 把 agent 当前已经拿到的 EasyTier peer / 虚拟 IP 状态继续透出到控制端 UI，并补上 relay 侧状态展示。
2. 让 relay-first 端口转发可以优先尝试 EasyTier 内网地址，再回退到 websocket relay 隧道。
3. 给端口转发补多连接复用、连接上限、带宽/时长限制和操作审计。
4. 把 OpenCode ACP 从 MVP 补到可稳定联调，包括权限策略、错误恢复和更多 session/update 映射。
5. 继续推进 Claude Code / Codex 的 ACP 兼容层，而不是长期停留在 CLI / 内嵌适配混合状态。

## 关于 iOS / Android

Tauri 2 已支持移动端壳，但真正的“被控端”能力在移动端会受到系统权限限制：

- iOS 几乎不适合做完整被控端
- Android 可以做部分能力，但完整远控也会受限
- 更现实的产品路径是：
  `Windows/macOS/Linux 做 Agent`，`iOS/Android 做控制端`

这个仓库现在就是按这个方向建的。
