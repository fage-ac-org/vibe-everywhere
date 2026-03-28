# Vibe Everywhere

[![CI](https://github.com/fage-ac-org/vibe-everywhere/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/fage-ac-org/vibe-everywhere/actions/workflows/ci.yml)
[![Release](https://github.com/fage-ac-org/vibe-everywhere/actions/workflows/release.yml/badge.svg)](https://github.com/fage-ac-org/vibe-everywhere/actions/workflows/release.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)

[中文](./README.md) | [English](./README.en.md)

Vibe Everywhere 是一个面向自建场景的远程 AI 控制面。它不是传统远程桌面，而是把远程开发流程组织成
`Relay + Agent + Control App` 三层：你在自己的服务器或工作站上运行 `vibe-relay` 和
`vibe-agent`，再通过 Web、桌面端或 Android 控制端发起 AI Session、查看工作区、查看 Git
状态、打开预览，必要时再进入终端和高级网络工具。

## 适合谁

- 想把 AI 编码任务放到远程机器执行，但仍希望从统一控制面查看状态和结果的人
- 想自建而不是依赖托管服务的人
- 需要同时管理多台设备、多个工作区、不同 AI Provider CLI 的个人或小团队
- 希望后续逐步演进到更完整企业能力，但当前先要一个实用 MVP 的团队

## 当前能力

- AI Session 创建、执行、取消、事件流展示
- 设备注册、在线状态、Provider 可用性展示
- 工作区浏览、文本文件预览、Git 检视
- 预览 / 转发能力，以及需要时的终端与高级连接能力
- Web、Tauri 桌面端、Android 控制端
- 中文 / 英文界面，浅色 / 深色 / 跟随系统主题
- 自建优先的 relay 配置模型，不依赖固定域名或 `127.0.0.1` 产品默认值

## 三分钟上手

### 1. 部署 Relay

推荐先看自建部署指南：

- [自建部署指南](./docs/self-hosted.md)

当前仓库提供两套一键安装脚本：

- Linux `systemd`

```bash
curl -fsSL https://raw.githubusercontent.com/fage-ac-org/vibe-everywhere/main/scripts/install-relay.sh -o install-relay.sh
sudo RELAY_PUBLIC_BASE_URL=https://relay.example.com \
  RELAY_ACCESS_TOKEN=change-me \
  bash install-relay.sh
```

- Windows 自动启动任务

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\install-relay.ps1 `
  -PublicRelayBaseUrl https://relay.example.com `
  -RelayAccessToken change-me
```

说明：

- 脚本会优先安装最新 GitHub Release；也可以通过 `VIBE_RELEASE_TAG` 或 `-ReleaseTag` 安装指定版本
- 脚本不会写死公网地址、token 或 preview host，这些都由参数或环境变量注入
- 当前没有仓库内置的 macOS 一键 relay 安装脚本，因为仓库还没有发布对应的 macOS 产物

### 2. 在目标机器启动 Agent

Agent 与 relay 分离运行。你可以从 GitHub Release 下载 CLI 包，解压后启动 `vibe-agent`：

```bash
VIBE_RELAY_URL=https://relay.example.com \
VIBE_RELAY_ACCESS_TOKEN=change-me \
VIBE_DEVICE_NAME=build-node-01 \
./vibe-agent
```

如果你希望执行 AI Session，目标机器还需要安装至少一个 Provider CLI：

- `codex`
- `claude`
- `opencode`

### 3. 打开控制端

控制端可以通过三种形态连接同一套 relay：

- Web
- Tauri 桌面端
- Android 客户端

当前推荐流程：

1. 先部署 relay
2. 在目标机器启动 agent
3. 打开 Web 或桌面端验证连接
4. 需要移动访问时再安装 Android 客户端

## 下载与发布

发布资产在 GitHub Release 页面提供，命名包含版本号，便于下载后直接识别。例如：

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

发布说明也已经固化到仓库中：

- [Release Notes Workflow](./docs/releases/README.md)
- 下一版草稿：[docs/releases/unreleased.md](./docs/releases/unreleased.md)

说明：

- Release 资产不再重复打包仓库 README 等文档，仓库文档直接在仓库中维护
- 如果仓库未配置 Android 签名 Secret，release APK 会保持 `unsigned`

## 自建配置要点

这些值不应该写死在客户端或部署脚本里：

- `VIBE_PUBLIC_RELAY_BASE_URL`
- `VIBE_RELAY_ACCESS_TOKEN`
- `VIBE_RELAY_FORWARD_HOST`
- `VIBE_RELAY_URL`

请区分三类地址：

- relay 监听地址，例如 `0.0.0.0:8787`
- agent 访问 relay 的地址，例如 `https://relay.example.com`
- 用户打开 preview 时使用的公开地址或 host

如果你要让手机或其他机器访问控制面，不要把公网 / 局域网访问地址设置成 `127.0.0.1`。

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

## 开发者入口

如果你是来做二次开发、调试或本地构建，可以从这里开始。

### 仓库结构

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
│   ├── plans            # 版本化迭代 / 修复计划
│   └── releases         # 版本化发布说明与下一版草稿
├── scripts               # 安装脚本、烟测脚本、Android doctor
└── TESTING.md            # 测试策略与回归清单
```

### 本地依赖

- Rust stable toolchain
- Node.js 24.14.x
- `protobuf-compiler` 或可用的 `protoc`
- Linux 构建 Tauri 时需要 WebKitGTK / GTK 相关开发包
- Android 构建需要 JDK 17、Android SDK cmdline-tools、`platforms;android-36`、`build-tools;35.0.0`、`ndk;25.2.9519653`
- Windows 启用 EasyTier / Overlay 相关能力时建议安装 Npcap，并启用 WinPcap API-compatible Mode

### 本地开发命令

启动 relay：

```bash
cargo run -p vibe-relay
```

启动 agent：

```bash
cargo run -p vibe-agent -- --relay-url http://127.0.0.1:8787
```

启动 Web 控制台：

```bash
cd apps/vibe-app
npm ci
npm run dev
```

启动桌面壳：

```bash
cd apps/vibe-app
npm ci
npm run tauri dev
```

构建前端：

```bash
cd apps/vibe-app
npm ci
npm run build
```

### Android 构建

调试 APK：

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

release APK / AAB：

```bash
cd apps/vibe-app
npm run android:build:apk
npm run android:build:aab
```

如果需要签名 release APK / AAB，可通过以下环境变量或
`apps/vibe-app/src-tauri/gen/android/app/keystore.properties` 提供签名信息：

- `VIBE_ANDROID_KEYSTORE_PATH`
- `VIBE_ANDROID_KEYSTORE_PASSWORD`
- `VIBE_ANDROID_KEY_ALIAS`
- `VIBE_ANDROID_KEY_PASSWORD`

### 常用验证命令

```bash
cargo fmt --all --check
cargo check --locked -p vibe-relay -p vibe-agent -p vibe-app
cargo test --locked --workspace --all-targets -- --nocapture
cd apps/vibe-app && npm run build
./scripts/dual-process-smoke.sh relay_polling
./scripts/dual-process-smoke.sh overlay
```

更完整的策略见：

- [TESTING.md](./TESTING.md)

## GitHub Actions

仓库内置两套主要工作流：

- `CI`
  - 触发：`main` 分支 push、`pull_request`、手动触发
  - 内容：格式检查、workspace 编译、测试、前端构建、`relay_polling` 烟测、Windows 兼容性校验、Android debug APK 构建
- `Release`
  - 触发：推送 `v*` tag
  - 内容：完整验证、阻塞式 `overlay` 烟测、CLI / 桌面端 / Android 打包、校验和生成、GitHub Release 发布

发布前需要：

1. 更新版本号
2. 更新 [docs/releases/unreleased.md](./docs/releases/unreleased.md)
3. 生成对应版本的 `docs/releases/vX.Y.Z.md`
4. 推送 tag

示例：

```bash
git tag v0.1.4
git push origin v0.1.4
```

## 路线图

- 补齐 iOS 客户端与移动端发布链路
- 继续提高部署体验和运维入口
- 完善企业级认证、审计和角色模型
- 增强前端自动化测试和协议 round-trip 测试
- 继续收敛高级能力与主流程之间的信息架构边界

## 贡献

欢迎提交 Issue 和 Pull Request。

建议在 PR 中包含：

- 变更背景和目标
- 受影响的 crate / app
- 已执行的验证命令
- 如果涉及 UI，附带截图
- 如果涉及环境变量或系统依赖，明确说明

提交信息建议使用 Conventional Commits，例如：

```text
feat(agent): add claude stream-json mapping
fix(relay): keep overlay transport fallback stable
docs(readme): rewrite project overview and quick start
```

## 许可证

本项目使用 [MIT License](./LICENSE)。
