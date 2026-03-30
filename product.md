# 当前产品功能说明

## 文档目的

本文档用于说明当前仓库在完成精简之后，产品实际仍保留了哪些能力。判断依据以当前代码、前端路由、relay 暴露接口、agent 运行时、README 与测试分布为准，不以历史规划或旧版本功能为准。

本文档面向交接、盘点和后续整改，重点回答三个问题：

1. 当前用户在产品里还能直接使用哪些功能
2. 当前后端和运行时还保留了哪些支撑能力
3. 哪些能力虽然源码还在，但现在不应视为当前产品功能

## 当前产品定位

当前产品仍然是一个面向自建环境的远程 AI 开发工作台，由以下三层组成：

- `vibe-relay`：控制面入口，负责认证、设备注册、会话与任务路由、状态汇总和对外 API
- `vibe-agent`：运行在目标主机上的执行端，负责 Provider CLI 调用、工作区访问、Git 检查和任务运行
- 控制端：当前仓库中的 Vue Web UI 和 Tauri 壳，负责浏览主机与项目、发起和管理长期 AI 会话、查看变更和文件

当前产品主模型仍然是：

`服务器 -> 主机 -> 项目 -> 会话/任务执行`

当前主工作面仍然是：

`会话 / 变更 / 文件`

主要代码入口：

- [README.md](/root/vibe-remote/README.md)
- [apps/vibe-app/src/router.ts](/root/vibe-remote/apps/vibe-app/src/router.ts)
- [apps/vibe-relay/src/main.rs](/root/vibe-remote/apps/vibe-relay/src/main.rs)
- [apps/vibe-agent/src/main.rs](/root/vibe-remote/apps/vibe-agent/src/main.rs)

## 当前用户可直接使用的功能

### 1. 连接和基础设置

控制端仍保留服务器连接配置能力，用户可以在设置页填写 relay 地址和 access token，并切换语言和主题。

当前设置页包含：

- relay URL 配置
- access token 配置
- 界面语言切换
- 主题模式切换

主要代码入口：

- [apps/vibe-app/src/views/SettingsView.vue](/root/vibe-remote/apps/vibe-app/src/views/SettingsView.vue)
- [apps/vibe-app/src/lib/runtime.ts](/root/vibe-remote/apps/vibe-app/src/lib/runtime.ts)
- [apps/vibe-app/src/lib/i18n.ts](/root/vibe-remote/apps/vibe-app/src/lib/i18n.ts)
- [apps/vibe-app/src/lib/theme.ts](/root/vibe-remote/apps/vibe-app/src/lib/theme.ts)

### 2. 顶层导航和主入口

当前前端仍是移动优先结构，但顶层导航已经收敛为：

- `首页`
- `项目`
- `设置`

这里的现状与部分治理文档中提到的 `首页 / 项目 / 通知 / 我的` 不一致；按代码实现，当前实际可用的是三段式导航，没有单独的通知页。

首页当前承担总览和快速回到工作区的作用，包含：

- 在线主机数
- 运行中任务数
- 需要关注的项目数
- 最近项目
- 继续最近工作
- 运行中的任务列表
- 需要复查的项目列表

项目页当前承担按主机浏览项目的作用，包含：

- 按主机分组展示项目
- 搜索项目
- 按全部、运行中、最近访问进行筛选
- 从项目卡片进入项目工作区

主要代码入口：

- [apps/vibe-app/src/router.ts](/root/vibe-remote/apps/vibe-app/src/router.ts)
- [apps/vibe-app/src/views/AppShellView.vue](/root/vibe-remote/apps/vibe-app/src/views/AppShellView.vue)
- [apps/vibe-app/src/views/HomeView.vue](/root/vibe-remote/apps/vibe-app/src/views/HomeView.vue)
- [apps/vibe-app/src/views/ProjectsView.vue](/root/vibe-remote/apps/vibe-app/src/views/ProjectsView.vue)

### 3. 主机与项目发现

当前产品仍支持从在线主机发现项目，发现逻辑以 agent 工作根和一级 Git 仓库扫描为主。

当前保留的项目发现行为包括：

- 从设备元数据中的工作根推导候选目录
- 跳过明显不应作为项目的目录，例如 `node_modules`、`target`、`dist`
- 对候选目录做 Git inspect，只有满足条件的仓库才提升为项目
- 项目 inventory 带有保留逻辑，主机离线或刷新失败时，已发现项目不会直接消失
- 项目可带有可用性状态，例如可用、离线、不可达、仅历史保留

这意味着当前“项目”并不是独立持久建模的数据库对象，而是由设备、会话、任务和 Git 发现结果综合推导出的工作对象。

主要代码入口：

- [apps/vibe-app/src/stores/app.ts](/root/vibe-remote/apps/vibe-app/src/stores/app.ts)
- [apps/vibe-app/src/lib/projects.ts](/root/vibe-remote/apps/vibe-app/src/lib/projects.ts)
- [apps/vibe-app/src/lib/projectInventory.ts](/root/vibe-remote/apps/vibe-app/src/lib/projectInventory.ts)

### 4. 项目工作区

项目详情页仍是当前产品的核心工作区。当前工作区保留三个二级面板：

- `会话`
- `变更`
- `文件`

项目头部当前仍展示项目上下文信息，包括：

- 所属主机
- 项目路径
- 会话数量
- 运行中任务数量
- 等待输入数量
- 当前分支
- 变更文件数
- 最近更新时间
- 当前状态概览

主要代码入口：

- [apps/vibe-app/src/views/ProjectWorkspaceView.vue](/root/vibe-remote/apps/vibe-app/src/views/ProjectWorkspaceView.vue)
- [apps/vibe-app/src/features/project/useProjectWorkspace.ts](/root/vibe-remote/apps/vibe-app/src/features/project/useProjectWorkspace.ts)

### 5. 会话与任务执行

会话仍然是当前产品的主线。用户可以在项目工作区内创建或继续长期 AI 会话，并围绕每一轮任务查看执行结果。

当前保留的会话与任务能力包括：

- 创建新会话
- 继续已有会话
- 在同一项目内发送 follow-up prompt
- 为每轮任务选择执行模式
- 查看任务状态和事件流
- 在任务运行中请求停止
- 对失败或取消的任务发起重试
- 对已完成或失败任务发起“解释结果”
- 从任务跳转到变更面板

当前执行模式仍保留三档：

- `read_only`
- `workspace_write`
- `workspace_write_and_test`

当前会话面板显示的内容不是传统纯聊天气泡，而是“按任务组织的会话卡片”，每张卡片包含：

- 用户请求
- 任务状态
- Provider 和模型信息
- AI 结果摘要
- 最近事件摘要
- 最近执行事件
- 可展开的原始事件输出

主要代码入口：

- [apps/vibe-app/src/features/project/ProjectConversationPanel.vue](/root/vibe-remote/apps/vibe-app/src/features/project/ProjectConversationPanel.vue)
- [apps/vibe-app/src/lib/api.ts](/root/vibe-remote/apps/vibe-app/src/lib/api.ts)
- [apps/vibe-relay/src/conversations.rs](/root/vibe-remote/apps/vibe-relay/src/conversations.rs)
- [apps/vibe-relay/src/tasks.rs](/root/vibe-remote/apps/vibe-relay/src/tasks.rs)
- [apps/vibe-agent/src/task_runtime.rs](/root/vibe-remote/apps/vibe-agent/src/task_runtime.rs)

### 6. Provider 交互和人工确认

当前产品仍支持 Provider 在执行过程中向用户发出待确认输入请求，并在当前会话中内联完成交互。

当前保留的交互形式包括：

- 固定选项选择
- 自定义文本输入
- 待确认状态的显示
- 任务与输入请求的关联

这意味着当前产品并没有把 Provider 的选择、批准或澄清流程切到单独页面，而是保持在活动会话内处理。

主要代码入口：

- [apps/vibe-app/src/features/project/ProjectConversationPanel.vue](/root/vibe-remote/apps/vibe-app/src/features/project/ProjectConversationPanel.vue)
- [apps/vibe-relay/src/conversations.rs](/root/vibe-remote/apps/vibe-relay/src/conversations.rs)
- [apps/vibe-app/src/types.ts](/root/vibe-remote/apps/vibe-app/src/types.ts)

### 7. Git 变更审查

当前产品仍保留针对项目仓库的 Git 审查能力，且这是“会话之后的审查面”之一。

当前保留的 Git 能力包括：

- Git inspect
- 当前分支信息
- upstream、ahead/behind 信息
- 变更文件列表
- staged/unstaged/untracked/conflicted 统计
- 最近提交摘要
- 单文件 diff 加载

当前前端仍保留“变更”作为显式面板，而不是把 Git 信息混回首页或通用仪表盘。

主要代码入口：

- [apps/vibe-app/src/features/project/ProjectChangesPanel.vue](/root/vibe-remote/apps/vibe-app/src/features/project/ProjectChangesPanel.vue)
- [apps/vibe-app/src/features/project/changeReview.ts](/root/vibe-remote/apps/vibe-app/src/features/project/changeReview.ts)
- [apps/vibe-app/src/lib/api.ts](/root/vibe-remote/apps/vibe-app/src/lib/api.ts)
- [apps/vibe-relay/src/git.rs](/root/vibe-remote/apps/vibe-relay/src/git.rs)
- [apps/vibe-agent/src/git_runtime.rs](/root/vibe-remote/apps/vibe-agent/src/git_runtime.rs)

### 8. 工作区浏览和文件预览

当前产品仍保留工作区浏览能力，用于在项目上下文中查看目录树和预览文本文件。

当前保留的文件能力包括：

- 浏览目录
- 进入子目录
- 返回上级目录
- 打开文件
- 文本文件预览
- 二进制和目录类型区分
- 受工作区根限制的路径访问

当前没有证据表明前端保留了完整编辑器或写文件功能；现状更接近只读浏览与预览。

主要代码入口：

- [apps/vibe-app/src/features/project/ProjectFilesPanel.vue](/root/vibe-remote/apps/vibe-app/src/features/project/ProjectFilesPanel.vue)
- [apps/vibe-app/src/lib/api.ts](/root/vibe-remote/apps/vibe-app/src/lib/api.ts)
- [apps/vibe-relay/src/workspace.rs](/root/vibe-remote/apps/vibe-relay/src/workspace.rs)
- [apps/vibe-agent/src/workspace_runtime.rs](/root/vibe-remote/apps/vibe-agent/src/workspace_runtime.rs)

### 9. 运行日志与事件信息

当前版本已经不再保留单独的“日志”工作区面板。任务事件仍作为会话卡片内部摘要和必要输出的一部分存在，用于帮助用户理解执行过程，但不再作为独立产品表面暴露。

排障所需的运行日志改为保留在前端控制台、relay 进程日志和 agent 进程日志中。

## 当前后端与运行时仍保留的支撑能力

### 1. 设备注册、认证和在线状态

relay 和 agent 当前仍保留完整的设备接入主流程，包括：

- agent 首次注册
- 设备凭证签发和持久化
- 设备心跳
- 在线状态维护
- 设备能力上报
- Provider 可用性上报

其中 agent 当前对外宣称的主能力仍包括：

- `AiSession`
- `Shell`
- `WorkspaceBrowse`
- `GitInspect`

但能力声明不等于当前产品面一定开放，是否对用户开放还取决于 relay 路由和前端接入。

主要代码入口：

- [apps/vibe-relay/src/auth.rs](/root/vibe-remote/apps/vibe-relay/src/auth.rs)
- [apps/vibe-relay/src/main.rs](/root/vibe-remote/apps/vibe-relay/src/main.rs)
- [apps/vibe-agent/src/main.rs](/root/vibe-remote/apps/vibe-agent/src/main.rs)
- [apps/vibe-agent/src/config.rs](/root/vibe-remote/apps/vibe-agent/src/config.rs)

### 2. Relay 当前对外 API 面

按 `apps/vibe-relay/src/main.rs` 中实际挂载的路由，当前对外 API 仍包括：

- 健康检查：`/api/health`
- 应用配置：`/api/app-config`
- 设备：`/api/devices`、设备注册、设备心跳、任务 claim
- 会话：会话列表、详情、发消息、归档
- 任务：任务列表、详情、取消、事件追加、输入请求相关接口
- 工作区：浏览、预览、请求 claim 和完成
- Git：inspect、diff-file、请求 claim 和完成
- 事件流：`/api/events/stream`

这说明当前精简后，产品主路径仍然围绕“设备、会话、任务、工作区、Git、事件流”六组能力构成。

主要代码入口：

- [apps/vibe-relay/src/main.rs](/root/vibe-remote/apps/vibe-relay/src/main.rs)

### 3. Agent 当前运行时能力

agent 当前仍保留以下运行时模块：

- Provider 适配与命令构造
- 任务执行 runtime
- Git runtime
- 工作区 runtime
- overlay 网络集成
- shell bridge
- task bridge
- port forward bridge

其中当前明确处于产品主路径中的，是：

- Provider 适配
- 任务执行
- Git
- 工作区
- overlay 下的任务传输回退

主要代码入口：

- [apps/vibe-agent/src/providers.rs](/root/vibe-remote/apps/vibe-agent/src/providers.rs)
- [apps/vibe-agent/src/task_runtime.rs](/root/vibe-remote/apps/vibe-agent/src/task_runtime.rs)
- [apps/vibe-agent/src/git_runtime.rs](/root/vibe-remote/apps/vibe-agent/src/git_runtime.rs)
- [apps/vibe-agent/src/workspace_runtime.rs](/root/vibe-remote/apps/vibe-agent/src/workspace_runtime.rs)
- [apps/vibe-agent/src/easytier.rs](/root/vibe-remote/apps/vibe-agent/src/easytier.rs)

### 4. 传输与实时更新

当前代码仍保留两类重要传输支撑能力：

- relay polling 路径
- overlay 优先、失败后回退到 relay polling 的任务路径

同时前后端之间仍保留 SSE 事件流，用于刷新设备、任务和任务事件。

这说明当前产品仍然具备“任务执行 + 实时状态更新”的基本闭环，而不是单纯的轮询式静态面板。

主要代码入口：

- [apps/vibe-relay/src/main.rs](/root/vibe-remote/apps/vibe-relay/src/main.rs)
- [apps/vibe-relay/src/tasks.rs](/root/vibe-remote/apps/vibe-relay/src/tasks.rs)
- [apps/vibe-agent/src/task_bridge.rs](/root/vibe-remote/apps/vibe-agent/src/task_bridge.rs)
- [apps/vibe-agent/src/easytier.rs](/root/vibe-remote/apps/vibe-agent/src/easytier.rs)

## 当前源码中仍存在，但不应视为当前产品功能的能力

### 1. Shell 会话

relay 和 agent 源码中仍保留 shell 相关模块，包含 shell session、输入输出追加、WebSocket 会话和 overlay shell bridge 等实现。

但按当前 `apps/vibe-relay/src/main.rs` 实际挂载的 API 路由，shell 相关接口并未暴露；同时当前前端代码中也没有 shell 入口、shell 页面或 shell API 调用。

因此当前可以明确判断：

- shell 代码仍在
- shell 不是当前已开放的产品功能
- shell 更接近“保留代码”或“未来可恢复能力”

主要代码入口：

- [apps/vibe-relay/src/shell.rs](/root/vibe-remote/apps/vibe-relay/src/shell.rs)
- [apps/vibe-agent/src/shell_runtime.rs](/root/vibe-remote/apps/vibe-agent/src/shell_runtime.rs)
- [apps/vibe-agent/src/shell_bridge.rs](/root/vibe-remote/apps/vibe-agent/src/shell_bridge.rs)

### 2. 端口转发

relay 和 agent 源码中仍保留 port forward 相关模块，包括转发创建、状态上报、隧道 WebSocket 和 overlay/relay tunnel 处理逻辑。

但与 shell 类似，当前 relay 主路由没有挂载 port forward 对外接口，前端也没有发现任何端口转发的入口或调用。

因此当前也应将其视为：

- 源码残留能力
- 当前未开放能力
- 不计入当前产品主功能

主要代码入口：

- [apps/vibe-relay/src/port_forwards.rs](/root/vibe-remote/apps/vibe-relay/src/port_forwards.rs)
- [apps/vibe-agent/src/port_forward_runtime.rs](/root/vibe-remote/apps/vibe-agent/src/port_forward_runtime.rs)
- [apps/vibe-agent/src/port_forward_bridge.rs](/root/vibe-remote/apps/vibe-agent/src/port_forward_bridge.rs)

### 3. 通知、管理台、企业治理类表面

当前前端路由和主要视图中没有发现单独的通知页、管理后台、治理或企业管理类主界面。结合当前导航结构，说明这类表面至少不在当前默认用户流程中。

如果仓库中还存在历史文档、规划或治理约束提及这些概念，也不应直接认定为当前现有功能。

主要代码入口：

- [apps/vibe-app/src/router.ts](/root/vibe-remote/apps/vibe-app/src/router.ts)

## 当前功能与测试覆盖的对应关系

当前自动化测试主要集中在 Rust 侧，前端自动化覆盖较少。

### 1. 已有 Rust 自动化覆盖的核心能力

根据 `cargo test --workspace --all-targets -- --list` 当前输出，自动化测试主要覆盖：

- Provider 事件映射
- Provider 命令与执行模式限制
- 任务桥接
- Git inspect 与 diff
- 工作区浏览和路径限制
- overlay 回退与恢复
- 设备认证与任务 claim

这些测试说明当前主路径中最受保护的部分，仍然是：

- AI 任务执行
- Git 检查
- 工作区浏览
- 传输稳定性
- 鉴权与状态同步

主要代码入口：

- [apps/vibe-agent/src/providers.rs](/root/vibe-remote/apps/vibe-agent/src/providers.rs)
- [apps/vibe-agent/src/task_runtime.rs](/root/vibe-remote/apps/vibe-agent/src/task_runtime.rs)
- [apps/vibe-agent/src/git_runtime.rs](/root/vibe-remote/apps/vibe-agent/src/git_runtime.rs)
- [apps/vibe-agent/src/workspace_runtime.rs](/root/vibe-remote/apps/vibe-agent/src/workspace_runtime.rs)
- [apps/vibe-relay/src/main.rs](/root/vibe-remote/apps/vibe-relay/src/main.rs)

### 2. 已有前端自动化覆盖

当前前端自动化测试很轻，主要集中在两个库级能力：

- provider 选择策略
- 项目 inventory 的隐藏和过滤逻辑

未看到针对完整 UI 交互流的自动化测试。

主要代码入口：

- [apps/vibe-app/src/lib/policy.test.ts](/root/vibe-remote/apps/vibe-app/src/lib/policy.test.ts)
- [apps/vibe-app/src/lib/projectInventory.test.ts](/root/vibe-remote/apps/vibe-app/src/lib/projectInventory.test.ts)

### 3. 现有手工回归关注点

当前 [TESTING.md](/root/vibe-remote/TESTING.md) 仍然围绕当前收敛后的产品模型编写，重点验证：

- 顶层导航
- 项目工作区三个面板
- 会话发起和继续
- 执行模式显示与限制
- Provider 待确认输入
- Git 变更审查
- 文件浏览和文本预览
- 不再提供独立日志面板
- 设置页

这与当前代码结构基本一致。

## 当前产品现状结论

精简后的当前产品已经收敛为一个以“远程 AI 会话 + 项目审查”为核心的工作台，而不是一个大而全的远程运维或多功能开发门户。

按当前代码事实，现状可以概括为：

- 用户主路径仍然存在，并且是完整可理解的
- 主路径集中在主机、项目、会话、Git 和文件
- 后端与 agent 仍保留足够的支撑能力来完成这条主路径
- shell 和端口转发等能力虽然代码仍在，但当前并未真正开放到产品面
- 前端已收敛，但与部分治理文档描述仍存在不完全一致之处，尤其是顶层导航现状

如果后续需要继续盘点，可以在此文档基础上再扩展两类附录：

- “与精简前相比已移除或降级的能力”
- “当前文档、治理约束与实际实现之间的不一致项”
