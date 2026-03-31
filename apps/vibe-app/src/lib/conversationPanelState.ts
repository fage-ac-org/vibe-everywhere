import type {
  ConversationPanelDetail,
  ConversationRecord,
  TaskDetailResponse,
  TaskRecord
} from "@/types";

export function buildOptimisticTaskDetail(task: TaskRecord): TaskDetailResponse {
  return {
    task,
    events: [],
    pendingInputRequest: null
  };
}

export function mergeConversationPanelDetail(
  detail: ConversationPanelDetail | null,
  optimisticTasks: TaskDetailResponse[]
): ConversationPanelDetail | null {
  if (!detail && !optimisticTasks.length) {
    return null;
  }

  const mergedTasks = [...(detail?.tasks ?? [])];
  const existingIds = new Set(mergedTasks.map((entry) => entry.task.id));
  for (const optimisticTask of optimisticTasks) {
    if (!existingIds.has(optimisticTask.task.id)) {
      mergedTasks.push(optimisticTask);
    }
  }

  mergedTasks.sort((left, right) => left.task.createdAtEpochMs - right.task.createdAtEpochMs);

  return {
    conversation: detail?.conversation ?? null,
    tasks: mergedTasks,
    pendingInputRequest: detail?.pendingInputRequest ?? null
  };
}

export function replaceOptimisticTask(
  tasks: TaskDetailResponse[],
  optimisticTaskId: string,
  nextTask: TaskRecord
): TaskDetailResponse[] {
  return tasks.map((entry) =>
    entry.task.id === optimisticTaskId ? buildOptimisticTaskDetail(nextTask) : entry
  );
}

export function markOptimisticTaskFailed(
  tasks: TaskDetailResponse[],
  optimisticTaskId: string,
  message: string
): TaskDetailResponse[] {
  return tasks.map((entry) =>
    entry.task.id === optimisticTaskId
      ? {
          ...entry,
          task: {
            ...entry.task,
            status: "failed",
            error: message,
            finishedAtEpochMs: Date.now()
          }
        }
      : entry
  );
}

export function pruneResolvedOptimisticTasks(
  tasks: TaskDetailResponse[],
  detail: ConversationPanelDetail | null
): TaskDetailResponse[] {
  if (!detail) {
    return tasks;
  }

  const serverTaskIds = new Set(detail.tasks.map((entry) => entry.task.id));
  return tasks.filter((entry) => !serverTaskIds.has(entry.task.id));
}

export function createOptimisticTaskRecord(
  prompt: string,
  deviceId: string,
  provider: TaskRecord["provider"],
  executionProtocol: TaskRecord["executionProtocol"],
  executionMode: TaskRecord["executionMode"],
  cwd: string | null,
  model: string | null,
  conversationId: string | null
): TaskRecord {
  const now = Date.now();
  return {
    id: `optimistic-${crypto.randomUUID()}`,
    deviceId,
    conversationId,
    title: prompt.slice(0, 60),
    provider,
    executionProtocol,
    executionMode,
    prompt,
    cwd,
    model,
    providerSessionId: null,
    pendingInputRequestId: null,
    status: "running",
    cancelRequested: false,
    createdAtEpochMs: now,
    startedAtEpochMs: now,
    finishedAtEpochMs: null,
    exitCode: null,
    error: null,
    lastEventSeq: 0
  };
}

export function createPanelDetailFromConversation(
  conversation: ConversationRecord | null,
  tasks: TaskDetailResponse[]
): ConversationPanelDetail {
  return {
    conversation,
    tasks,
    pendingInputRequest: null
  };
}
