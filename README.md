# Vibe Everywhere

[![CI](https://github.com/fage-ac-org/vibe-everywhere/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/fage-ac-org/vibe-everywhere/actions/workflows/ci.yml)
[![Release](https://github.com/fage-ac-org/vibe-everywhere/actions/workflows/release.yml/badge.svg)](https://github.com/fage-ac-org/vibe-everywhere/actions/workflows/release.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)

[中文](./README.md) | [English](./README.en.md)

Vibe Everywhere 是一个面向自建场景的远程 AI 控制面。你把 AI 编码任务放到自己的服务器、
工作站或开发机上执行，再通过统一控制端发起 AI Session、查看工作区、检查 Git、打开预览，
并在确有必要时进入终端或高级连接工具。

它不是传统远程桌面，也不是把用户锁在某个托管平台里的 SaaS。产品目标很明确：
把“远程 AI 开发”变成一个可部署、可观察、可协作的工作流。

## 一眼看懂这个产品

- `vibe-relay` 是控制面的入口，负责认证、设备注册、Session 路由和状态汇总。
- `vibe-agent` 运行在目标机器上，负责真正执行 AI Provider CLI、工作区浏览、Git 检查和预览桥接。
- 控制端可以是桌面端、Android，或者你自建的 Web 客户端。
- 主流程围绕 AI Session 展开，设备管理、终端和高级连接工具属于辅助能力，而不是主入口。

## 适合谁

- 想把 AI 编码任务放到远程机器执行，但仍希望集中查看过程与结果的人
- 希望完全自建，而不是依赖托管服务的人或团队
- 需要同时管理多台设备、多个工作区和多种 AI Provider CLI 的个人或小团队
- 需要先落地一个能工作的 MVP，再逐步演进权限、审计和组织能力的团队

## 当前能做什么

- 创建、执行、取消 AI Session，并实时查看事件流
- 在同一个主界面完成 relay 连接、设备选择、Session 发起和结果审阅
- 查看设备在线状态、运行能力和 Provider 可用性
- 浏览工作区、预览文本文件、检查 Git 状态和最近提交
- 建立预览转发，并在需要时进入 Shell 或更底层的高级连接能力
- 通过桌面端、Android 和自建 Web 客户端访问同一套控制面
- 使用中文或英文界面，并支持浅色、深色和跟随系统主题

## 简单部署手册

这一节假设你的目标是尽快跑起来一套可用环境，而不是做复杂定制。

### 部署前先准备 3 个值

- `https://relay.example.com`
  这是用户和 Agent 实际访问的 relay 地址，必须是可达地址，不要用 `127.0.0.1`。
- `VIBE_RELAY_ACCESS_TOKEN`
  这是给人类控制端用的控制面 token，桌面端、Android 和自建 Web 客户端都使用它。
- `VIBE_RELAY_ENROLLMENT_TOKEN`
  这是给 Agent 首次注册用的 enrollment token，建议不要和控制面 token 共用。

### 第 1 步：部署 Relay

更完整的运维说明见 [docs/self-hosted.md](./docs/self-hosted.md)。如果你只是先部署一套可用环境，
下面的脚本就够了。

- Linux `systemd`

```bash
curl -fsSL https://raw.githubusercontent.com/fage-ac-org/vibe-everywhere/main/scripts/install-relay.sh -o install-relay.sh
sudo RELAY_PUBLIC_BASE_URL=https://relay.example.com \
  RELAY_ACCESS_TOKEN=change-control-token \
  RELAY_ENROLLMENT_TOKEN=change-agent-enrollment-token \
  bash install-relay.sh
```

- Windows 自动启动任务

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\install-relay.ps1 `
  -PublicRelayBaseUrl https://relay.example.com `
  -RelayAccessToken change-control-token `
  -RelayEnrollmentToken change-agent-enrollment-token
```

部署完成后，先确认 relay 进程健康：

```bash
curl https://relay.example.com/api/health
```

### 不要混淆监听地址和对外地址

这是最容易误解的一组配置：

- `RELAY_BIND_HOST` / `RELAY_PORT`
  安装脚本会把它们写成 `VIBE_RELAY_HOST` / `VIBE_RELAY_PORT`，它们决定 relay 实际监听在哪个地址和端口。
- `RELAY_PUBLIC_BASE_URL`
  安装脚本会把它写成 `VIBE_PUBLIC_RELAY_BASE_URL`，它决定客户端、预览链接和产品展示里“应该访问哪个地址”，但不会改变 relay 实际监听端口。

当前默认值是：

- relay 默认监听 `0.0.0.0:8787`
- 如果未显式设置 `VIBE_PUBLIC_RELAY_BASE_URL`，生产模式下不会自动为 `0.0.0.0` 推导一个可用公网地址

请按下面的规则理解：

- `0.0.0.0` 只适合作为监听地址，不适合作为给用户访问的 URL 主机名。
- `127.0.0.1` 或 `localhost` 只适合本机开发；远程电脑、手机和 Android 客户端都不能把它当作服务器地址使用。
- `RELAY_PUBLIC_BASE_URL=http://45.144.137.240` 表示对外地址是 80 端口，不会自动等于 relay 的实际监听端口。
- 如果 relay 实际监听在 `8787`，对外地址也想让客户端直连 `8787`，应写成 `http://45.144.137.240:8787`。
- `http` 可以用于本机开发、内网测试或你明确接受明文传输风险的环境。
- 面向公网、手机、桌面端或共享部署时，强烈建议使用 `https`。

常见部署形态示例：

- 公网 IP 直连

```bash
sudo RELAY_BIND_HOST=0.0.0.0 \
  RELAY_PORT=8787 \
  RELAY_PUBLIC_BASE_URL=http://45.144.137.240:8787 \
  RELAY_ACCESS_TOKEN=change-control-token \
  RELAY_ENROLLMENT_TOKEN=change-agent-enrollment-token \
  bash install-relay.sh
```

- 反向代理到本机 relay

```bash
sudo RELAY_BIND_HOST=127.0.0.1 \
  RELAY_PORT=8787 \
  RELAY_PUBLIC_BASE_URL=https://relay.example.com \
  RELAY_ACCESS_TOKEN=change-control-token \
  RELAY_ENROLLMENT_TOKEN=change-agent-enrollment-token \
  bash install-relay.sh
```

### 第 2 步：在目标机器启动 Agent

从 [GitHub Releases](https://github.com/fage-ac-org/vibe-everywhere/releases) 下载当前版本的 CLI 包，
解压后启动 `vibe-agent`：

```bash
VIBE_RELAY_URL=https://relay.example.com \
VIBE_RELAY_ENROLLMENT_TOKEN=change-agent-enrollment-token \
VIBE_DEVICE_NAME=build-node-01 \
./vibe-agent
```

你需要知道这几个关键点：

- Windows 下请保留解压后的 side-by-side 运行时文件与 `vibe-agent.exe` 同目录，不要只单独拷出可执行文件。
- 目标机器若要执行 AI Session，至少需要安装一个 Provider CLI，例如 `codex`、`claude` 或 `opencode`。
- Agent 首次注册成功后，会在工作目录下写入 `.vibe-agent/identity.json`，后续重启优先复用该设备凭证。

### Agent、Overlay 和 EasyTier 端口说明

默认情况下，agent 不会像 relay 一样占用一个固定的控制面监听端口：

- 如果没有启用 EasyTier，也就是没有设置 `VIBE_EASYTIER_NETWORK_NAME`，agent 主要是主动连接 relay，不会为业务流量额外打开固定监听端口。
- 如果启用了 EasyTier overlay，agent 会额外启动 3 个 bridge 监听端口：
  - `19090`：shell bridge
  - `19091`：port-forward bridge
  - `19092`：task bridge
- 这 3 个端口可以分别通过 `VIBE_AGENT_SHELL_BRIDGE_PORT`、`VIBE_AGENT_PORT_FORWARD_BRIDGE_PORT` 和 `VIBE_AGENT_TASK_BRIDGE_PORT` 覆盖。
- 这些 bridge 端口用于 relay 和 agent 的 overlay 内部链路，不应该误认为是给浏览器或手机客户端直接访问的公网入口。

EasyTier 自身的 listener 规则也要单独理解：

- agent 侧默认 `VIBE_EASYTIER_NO_LISTENER=true`，所以即便启用了 overlay，嵌入式 EasyTier 节点默认也不会主动对外监听 EasyTier peer 入口。
- 如果你显式设置 `VIBE_EASYTIER_NO_LISTENER=false` 且没有设置 `VIBE_EASYTIER_LISTENERS`，当前仓库会按 EasyTier 常见 listener 简写规则，默认使用 TCP/UDP `11010`。
- relay 侧如果启用了嵌入式 EasyTier 且没有设置 `VIBE_EASYTIER_LISTENERS`，当前仓库同样默认使用 TCP/UDP `11010`。

### 第 3 步：打开控制端并连接

当前公开发布物提供：

- CLI
- 桌面端
- Android

Web 控制端是现有产品形态之一，但公开 Release 目前不单独分发 Web 静态包；如果你只是想尽快开始使用，
建议优先下载桌面端。

首次连接时，按这个顺序操作：

1. 打开桌面端或 Android 客户端。
2. 输入 relay 地址，例如 `https://relay.example.com`。
3. 输入控制面 token，也就是 `VIBE_RELAY_ACCESS_TOKEN`。
4. 进入 Session 主流程，确认设备已经在线，且目标设备上至少一个 Provider 已显示可用。

## 认证方式怎么理解

推荐把人和设备的认证拆开，这样既更安全，也更容易运维：

- `VIBE_RELAY_ACCESS_TOKEN`
  给人类控制端使用，用来登录桌面端、Android 和自建 Web 客户端。
- `VIBE_RELAY_ENROLLMENT_TOKEN`
  只给 Agent 首次注册使用，用来把新设备纳入控制面。
- `.vibe-agent/identity.json`
  Agent 首次注册成功后获得并持久化的设备身份，后续心跳、任务领取、工作区和预览桥接都复用它。

如果你删除了目标机器上的 `.vibe-agent/identity.json`，下次启动 Agent 时会重新走注册流程。

## 详细使用说明

### 1. 先确认控制面连通

第一次进入客户端时，优先看三件事：

- relay 地址是否正确
- 控制面 token 是否正确
- 至少一台目标设备是否已经在线

如果这一步没有通过，不要急着创建 Session，先回到部署链路排查 relay 与 agent 的连接状态。

### 2. 选择设备并确认能力

进入主流程后，先选择你要执行任务的设备。理想状态下，你能看到：

- 设备在线
- Provider 可用，例如 `codex` 或 `claude`
- 该设备具备工作区浏览、Git 检查、Shell 或预览等能力

如果设备在线但 Provider 不可用，通常意味着目标机器还没有安装对应 CLI，或者 Agent 当前无法探测到它。

### 3. 发起 AI Session

Session 是产品的主入口。典型用法是：

1. 选择设备。
2. 输入任务目标或 prompt。
3. 选择要使用的 Provider。
4. 启动 Session 并观察事件流。

在这个阶段，你会实时看到任务执行进度、结构化事件和结果状态。相比传统 SSH 工作流，Vibe Everywhere
更强调“任务过程可见”和“结果可回看”。

### 4. 审阅工作区、Git 和预览结果

Session 执行后，常见动作是继续在同一个控制面完成结果检查：

- 浏览工作区目录和文件
- 预览文本文件内容
- 查看 Git 分支、变更文件和最近提交
- 打开预览地址检查本地 Web 服务或端口转发结果

这一步的目标不是替代 IDE，而是让你在远程执行之后，快速判断“结果是否可接受、是否需要继续追问、是否需要转入手工处理”。

### 5. 只有在需要时再使用高级工具

终端、端口转发和高级连接能力仍然重要，但它们应该是第二层工具，而不是默认入口。

推荐原则：

- 能在 Session 主流程里完成的事，先不要跳到 Shell
- 能通过工作区和 Git 审阅确认的问题，先不要建立更重的连接
- 只有在需要手工调试、临时排障或手动执行命令时，再进入高级工具

### 6. 多设备场景怎么用

如果你有多台机器，建议按角色命名并分工，例如：

- `build-node-01` 用于重构和构建
- `gpu-node-01` 用于模型或 GPU 任务
- `demo-node-01` 用于演示和预览

这样在控制端里更容易快速选择正确设备，也便于团队协作时表达“任务应该落在哪台机器上”。

## 常见问题

### Agent 启动了，但设备没有出现在控制端

优先检查：

- `VIBE_RELAY_URL` 是否指向 Agent 真正可访问的 relay 地址
- `VIBE_RELAY_ENROLLMENT_TOKEN` 是否正确
- relay 的 `/api/health` 是否正常
- 目标机器到 relay 的网络是否可达

### 设备在线了，但看不到 Provider 可用

通常是目标机器上没有安装对应 Provider CLI，或者该 CLI 不在 Agent 进程的 `PATH` 中。

### 想让设备重新注册

删除工作目录下的 `.vibe-agent/identity.json`，然后重新启动 `vibe-agent`。下一次启动会重新走 enrollment 流程。

### 预览链接打不开

重点检查 relay 的公网地址和转发主机配置。预览能力依赖 relay 生成对用户可达的链接，
因此 `RELAY_PUBLIC_BASE_URL`、`VIBE_PUBLIC_RELAY_BASE_URL` 和相关 forward host 配置必须正确。

### 我可以把 `RELAY_PUBLIC_BASE_URL` 设成 `http://127.0.0.1` 或 `http://0.0.0.0` 吗

- `http://127.0.0.1` 只适合同一台机器本机访问，不适合远程电脑、手机或 Android 客户端。
- `http://0.0.0.0` 不应该作为客户端访问地址使用。`0.0.0.0` 是监听地址，不是可路由的公网主机名。
- 如果你要让其他设备访问 relay，请使用真实 IP 或域名，例如 `http://45.144.137.240:8787` 或 `https://relay.example.com`。

## 下载

- [GitHub Releases](https://github.com/fage-ac-org/vibe-everywhere/releases)

当前公开发布物包含 CLI、桌面端和 Android 包。选择与你的平台匹配的文件即可。

## 产品结构

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

## 相关文档

- 自建部署与安装：[docs/self-hosted.md](./docs/self-hosted.md)
- 发布下载：[GitHub Releases](https://github.com/fage-ac-org/vibe-everywhere/releases)
