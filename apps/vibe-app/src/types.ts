export type ExecutionProtocol = "cli" | "acp";
export type TaskExecutionMode =
  | "read_only"
  | "workspace_write"
  | "workspace_write_and_test";
export type ProviderKind = "codex" | "claude_code" | "open_code";
export type TaskStatus =
  | "pending"
  | "assigned"
  | "running"
  | "waiting_input"
  | "cancel_requested"
  | "succeeded"
  | "failed"
  | "canceled";
export type AuthMode = "disabled" | "access_token";
export type StorageKind = "file" | "memory" | "external";

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

export type DeviceRecord = {
  id: string;
  name: string;
  platform: string;
  capabilities: string[];
  metadata: Record<string, string>;
  providers: ProviderStatus[];
  online: boolean;
  lastSeenEpochMs: number;
  currentTaskId: string | null;
};

export type TaskRecord = {
  id: string;
  deviceId: string;
  conversationId: string | null;
  title: string;
  provider: ProviderKind;
  executionProtocol: ExecutionProtocol;
  executionMode: TaskExecutionMode;
  prompt: string;
  cwd: string | null;
  model: string | null;
  providerSessionId: string | null;
  pendingInputRequestId: string | null;
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
  pendingInputRequest: ConversationInputRequest | null;
};

export type ConversationRecord = {
  id: string;
  deviceId: string;
  title: string;
  provider: ProviderKind;
  executionProtocol: ExecutionProtocol;
  executionMode: TaskExecutionMode;
  cwd: string | null;
  model: string | null;
  providerSessionId: string | null;
  latestTaskId: string | null;
  pendingInputRequestId: string | null;
  archived: boolean;
  createdAtEpochMs: number;
  updatedAtEpochMs: number;
};

export type ConversationInputOption = {
  id: string;
  label: string;
  description: string | null;
  requiresTextInput: boolean;
};

export type ConversationInputRequestStatus = "pending" | "answered" | "canceled";

export type ConversationInputRequest = {
  id: string;
  conversationId: string;
  taskId: string;
  prompt: string;
  options: ConversationInputOption[];
  allowCustomInput: boolean;
  customInputPlaceholder: string | null;
  status: ConversationInputRequestStatus;
  selectedOptionId: string | null;
  responseText: string | null;
  createdAtEpochMs: number;
  answeredAtEpochMs: number | null;
};

export type ConversationDetailResponse = {
  conversation: ConversationRecord;
  tasks: TaskDetailResponse[];
  pendingInputRequest: ConversationInputRequest | null;
};

export type ConversationPanelDetail = {
  conversation: ConversationRecord | null;
  tasks: TaskDetailResponse[];
  pendingInputRequest: ConversationInputRequest | null;
};

export type WorkspaceEntryKind = "directory" | "file";

export type WorkspaceEntry = {
  path: string;
  name: string;
  kind: WorkspaceEntryKind;
  sizeBytes: number | null;
  modifiedAtEpochMs: number | null;
};

export type WorkspaceBrowseRequest = {
  deviceId: string;
  sessionCwd?: string;
  path?: string;
};

export type WorkspaceBrowseResponse = {
  deviceId: string;
  rootPath: string;
  path: string;
  parentPath: string | null;
  entries: WorkspaceEntry[];
};

export type WorkspacePreviewKind = "text" | "binary" | "directory";

export type WorkspaceFilePreviewRequest = {
  deviceId: string;
  sessionCwd?: string;
  path: string;
  line?: number;
  limit?: number;
};

export type WorkspaceFilePreviewResponse = {
  deviceId: string;
  rootPath: string;
  path: string;
  kind: WorkspacePreviewKind;
  content: string | null;
  truncated: boolean;
  line: number | null;
  totalLines: number | null;
  sizeBytes: number;
};

export type GitInspectState = "ready" | "not_repository" | "git_unavailable";

export type GitFileStatus =
  | "modified"
  | "added"
  | "deleted"
  | "renamed"
  | "copied"
  | "updated_but_unmerged"
  | "untracked"
  | "type_changed"
  | "unknown";

export type GitInspectRequest = {
  deviceId: string;
  sessionCwd?: string;
};

export type GitDiffFileRequest = {
  deviceId: string;
  sessionCwd?: string;
  repoPath: string;
};

export type GitChangedFile = {
  path: string;
  repoPath: string;
  status: GitFileStatus;
  staged: boolean;
  unstaged: boolean;
};

export type GitCommitSummary = {
  id: string;
  shortId: string;
  summary: string;
  authorName: string;
  committedAtEpochMs: number;
};

export type GitDiffStats = {
  changedFiles: number;
  stagedFiles: number;
  unstagedFiles: number;
  untrackedFiles: number;
  conflictedFiles: number;
  stagedAdditions: number;
  stagedDeletions: number;
  unstagedAdditions: number;
  unstagedDeletions: number;
};

export type GitInspectResponse = {
  deviceId: string;
  workspaceRoot: string;
  repoRoot: string | null;
  repoCommonDir: string | null;
  scopePath: string | null;
  state: GitInspectState;
  branchName: string | null;
  upstreamBranch: string | null;
  aheadCount: number;
  behindCount: number;
  hasCommits: boolean;
  changedFiles: GitChangedFile[];
  recentCommits: GitCommitSummary[];
  diffStats: GitDiffStats;
};

export type GitDiffFileResponse = {
  deviceId: string;
  workspaceRoot: string;
  repoRoot: string | null;
  repoCommonDir: string | null;
  repoPath: string;
  path: string;
  state: GitInspectState;
  status: GitFileStatus | null;
  staged: boolean;
  unstaged: boolean;
  isBinary: boolean;
  truncated: boolean;
  stagedPatch: string | null;
  unstagedPatch: string | null;
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
  authMode: AuthMode;
  storageKind: StorageKind;
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
  executionMode?: TaskExecutionMode;
  prompt: string;
  cwd?: string;
  model?: string;
  title?: string;
};

export type CreateConversationPayload = {
  deviceId: string;
  provider: ProviderKind;
  executionMode?: TaskExecutionMode;
  prompt: string;
  cwd?: string;
  model?: string;
  title?: string;
};

export type CreateConversationResponse = {
  conversation: ConversationRecord;
  task: TaskRecord;
};

export type SendConversationMessagePayload = {
  prompt: string;
  executionMode?: TaskExecutionMode;
  model?: string;
  title?: string;
};

export type SendConversationMessageResponse = {
  conversation: ConversationRecord;
  task: TaskRecord;
};

export type RespondConversationInputPayload = {
  optionId?: string;
  text?: string;
};
