export type ExecutionProtocol = "cli" | "acp";
export type OverlayState =
  | "connected"
  | "degraded"
  | "disabled"
  | "unavailable";
export type ProviderKind = "codex" | "claude_code" | "open_code";
export type TaskStatus =
  | "pending"
  | "assigned"
  | "running"
  | "cancel_requested"
  | "succeeded"
  | "failed"
  | "canceled";
export type ShellSessionStatus =
  | "pending"
  | "active"
  | "close_requested"
  | "succeeded"
  | "failed"
  | "closed";
export type PortForwardStatus =
  | "pending"
  | "active"
  | "close_requested"
  | "closed"
  | "failed";
export type ShellStreamKind = "stdout" | "stderr" | "system";
export type PortForwardProtocol = "tcp";
export type PortForwardTransportKind = "relay_tunnel" | "overlay_proxy";

export type TaskEventKind =
  | "system"
  | "status"
  | "provider_stdout"
  | "provider_stderr"
  | "assistant_delta"
  | "tool_call"
  | "tool_output";

export type ProviderStatus = {
  kind: ProviderKind;
  command: string;
  available: boolean;
  version: string | null;
  executionProtocol: ExecutionProtocol;
  supportsAcp: boolean;
  error: string | null;
};

export type OverlayNetworkStatus = {
  mode: "easytier_embedded" | "easytier_sidecar" | "disabled";
  state: OverlayState;
  networkName: string | null;
  nodeIp: string | null;
  relayUrl: string | null;
  binaryPath: string | null;
  lastError: string | null;
};

export type DeviceRecord = {
  tenantId: string;
  userId: string;
  id: string;
  name: string;
  platform: string;
  capabilities: string[];
  metadata: Record<string, string>;
  providers: ProviderStatus[];
  overlay: OverlayNetworkStatus;
  online: boolean;
  lastSeenEpochMs: number;
  currentTaskId: string | null;
};

export type TaskRecord = {
  tenantId: string;
  userId: string;
  id: string;
  deviceId: string;
  title: string;
  provider: ProviderKind;
  executionProtocol: ExecutionProtocol;
  prompt: string;
  cwd: string | null;
  model: string | null;
  status: TaskStatus;
  cancelRequested: boolean;
  createdAtEpochMs: number;
  startedAtEpochMs: number | null;
  finishedAtEpochMs: number | null;
  exitCode: number | null;
  error: string | null;
  lastEventSeq: number;
};

export type TaskEvent = {
  seq: number;
  taskId: string;
  deviceId: string;
  kind: TaskEventKind;
  message: string;
  timestampEpochMs: number;
};

export type TaskDetailResponse = {
  task: TaskRecord;
  events: TaskEvent[];
};

export type ShellTransportKind = "relay_polling" | "overlay_proxy";

export type ShellSessionRecord = {
  tenantId: string;
  userId: string;
  id: string;
  deviceId: string;
  cwd: string | null;
  transport: ShellTransportKind;
  status: ShellSessionStatus;
  closeRequested: boolean;
  createdAtEpochMs: number;
  startedAtEpochMs: number | null;
  finishedAtEpochMs: number | null;
  exitCode: number | null;
  error: string | null;
  lastInputSeq: number;
  lastOutputSeq: number;
};

export type ShellInputRecord = {
  seq: number;
  sessionId: string;
  data: string;
  timestampEpochMs: number;
};

export type ShellOutputChunk = {
  seq: number;
  sessionId: string;
  stream: ShellStreamKind;
  data: string;
  timestampEpochMs: number;
};

export type ShellSessionDetailResponse = {
  session: ShellSessionRecord;
  inputs: ShellInputRecord[];
  outputs: ShellOutputChunk[];
};

export type PortForwardRecord = {
  tenantId: string;
  userId: string;
  id: string;
  deviceId: string;
  protocol: PortForwardProtocol;
  relayHost: string;
  relayPort: number;
  targetHost: string;
  targetPort: number;
  transport: PortForwardTransportKind;
  status: PortForwardStatus;
  createdAtEpochMs: number;
  startedAtEpochMs: number | null;
  finishedAtEpochMs: number | null;
  error: string | null;
};

export type PortForwardDetailResponse = {
  forward: PortForwardRecord;
};

export type ServiceHealth = {
  service: string;
  status: string;
  deviceCount: number;
  onlineDeviceCount: number;
  taskCount: number;
};

export type AppConfig = {
  appName: string;
  defaultRelayBaseUrl: string;
  requiresAuth: boolean;
  supportedTargets: string[];
  controlClients: string[];
  featureFlags: string[];
};

export type RelayEventEnvelope = {
  eventType: "device_updated" | "task_updated" | "task_event";
  device: DeviceRecord | null;
  task: TaskRecord | null;
  taskEvent: TaskEvent | null;
};

export type CreateTaskPayload = {
  deviceId: string;
  provider: ProviderKind;
  prompt: string;
  cwd?: string;
  model?: string;
  title?: string;
};

export type CreateShellSessionPayload = {
  deviceId: string;
  cwd?: string;
};

export type CreatePortForwardPayload = {
  deviceId: string;
  protocol?: PortForwardProtocol;
  targetHost: string;
  targetPort: number;
};
