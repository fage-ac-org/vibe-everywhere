import { computed, onScopeDispose, ref, toValue, watch, type MaybeRefOrGetter } from "vue";
import {
  browseWorkspace,
  fetchGitDiffFile,
  inspectGitWorkspace,
  previewWorkspaceFile
} from "@/lib/api";
import {
  buildOptimisticTaskDetail,
  createOptimisticTaskRecord,
  createPanelDetailFromConversation,
  markOptimisticTaskFailed,
  mergeConversationPanelDetail,
  pruneResolvedOptimisticTasks,
  replaceOptimisticTask
} from "@/lib/conversationPanelState";
import { shouldRefreshConversationDetail } from "@/lib/conversationRealtime";
import { preferredProjectProvider } from "@/lib/policy";
import { parseProjectRouteParam } from "@/lib/projects";
import { buildEventStreamUrl } from "@/lib/runtime";
import { useAppStore } from "@/stores/app";
import type {
  ConversationDetailResponse,
  ConversationPanelDetail,
  ConversationRecord,
  GitDiffFileResponse,
  GitInspectResponse,
  ProviderKind,
  RelayEventEnvelope,
  TaskDetailResponse,
  TaskExecutionMode,
  WorkspaceBrowseResponse,
  WorkspaceFilePreviewResponse
} from "@/types";

export type ProjectTab = "conversation" | "changes" | "files";

export function useProjectWorkspace(
  deviceIdSource: MaybeRefOrGetter<string>,
  projectPathSource: MaybeRefOrGetter<string | string[] | undefined>,
  providerSource?: MaybeRefOrGetter<ProviderKind | null | undefined>,
  enabledSource?: MaybeRefOrGetter<boolean>
) {
  const store = useAppStore();
  const deviceId = computed(() => toValue(deviceIdSource));
  const cwd = computed(() => parseProjectRouteParam(toValue(projectPathSource)));
  const provider = computed(() => toValue(providerSource));
  const enabled = computed(() => toValue(enabledSource) ?? true);
  const project = computed(() => store.findProject(deviceId.value, cwd.value));
  const conversations = computed(() =>
    enabled.value ? store.listProjectConversations(deviceId.value, cwd.value, provider.value) : []
  );
  const tasks = computed(() =>
    enabled.value ? store.listProjectTasks(deviceId.value, cwd.value, provider.value) : []
  );
  const activeConversationId = ref<string | null>(null);
  const serverConversationDetail = ref<ConversationDetailResponse | null>(null);
  const gitInspect = ref<GitInspectResponse | null>(null);
  const gitDiff = ref<GitDiffFileResponse | null>(null);
  const activeDiffRepoPath = ref<string | null>(null);
  const workspace = ref<WorkspaceBrowseResponse | null>(null);
  const filePreview = ref<WorkspaceFilePreviewResponse | null>(null);
  const browserPath = ref<string>("");
  const activeTab = ref<ProjectTab>("conversation");
  const isDraftConversation = ref(false);
  const isLoading = ref(false);
  const isRestoringConversation = ref(false);
  const errorMessage = ref("");
  const draftConversation = ref<ConversationRecord | null>(null);
  const optimisticTasksByConversation = ref<Record<string, TaskDetailResponse[]>>({});
  let activeEventSource: EventSource | null = null;
  let realtimeRefreshTimer: number | null = null;
  let conversationLoadVersion = 0;

  const currentConversationKey = computed(() => activeConversationId.value ?? "__draft__");
  const conversationDetail = computed<ConversationPanelDetail | null>(() => {
    const optimisticTasks = optimisticTasksByConversation.value[currentConversationKey.value] ?? [];
    const baseDetail = activeConversationId.value
      ? serverConversationDetail.value
      : isDraftConversation.value
        ? createPanelDetailFromConversation(draftConversation.value, [])
        : null;
    return mergeConversationPanelDetail(baseDetail, optimisticTasks);
  });

  watch(
    conversations,
    (value) => {
      if (isDraftConversation.value) {
        serverConversationDetail.value = null;
        return;
      }

      if (!value.length) {
        activeConversationId.value = null;
        serverConversationDetail.value = null;
        return;
      }

      if (!activeConversationId.value || !value.some((entry) => entry.id === activeConversationId.value)) {
        activeConversationId.value = value[0].id;
      }
    },
    { immediate: true }
  );

  function updateOptimisticTasks(
    conversationKey: string,
    updater: (tasks: TaskDetailResponse[]) => TaskDetailResponse[]
  ) {
    const currentTasks = optimisticTasksByConversation.value[conversationKey] ?? [];
    optimisticTasksByConversation.value = {
      ...optimisticTasksByConversation.value,
      [conversationKey]: updater(currentTasks)
    };
  }

  function moveOptimisticTasks(fromKey: string, toKey: string) {
    const existing = optimisticTasksByConversation.value[fromKey] ?? [];
    const nextState = { ...optimisticTasksByConversation.value };
    delete nextState[fromKey];
    nextState[toKey] = [...(nextState[toKey] ?? []), ...existing];
    optimisticTasksByConversation.value = nextState;
  }

  function addOptimisticTask(
    conversationKey: string,
    prompt: string,
    model: string | undefined,
    executionMode: TaskExecutionMode | undefined
  ) {
    const preferredProvider = provider.value ?? preferredProjectProvider(project.value?.providers);
    const executionProtocol =
      store.devices
        .find((entry) => entry.id === deviceId.value)
        ?.providers.find((entry) => entry.kind === preferredProvider)
        ?.executionProtocol ?? "acp";
    const optimisticTask = buildOptimisticTaskDetail(
      createOptimisticTaskRecord(
        prompt,
        deviceId.value,
        preferredProvider,
        executionProtocol,
        executionMode ?? store.defaultExecutionMode,
        cwd.value,
        model ?? null,
        activeConversationId.value
      )
    );
    updateOptimisticTasks(conversationKey, (tasks) => [...tasks, optimisticTask]);
    return optimisticTask.task.id;
  }

  function clearConversationState() {
    serverConversationDetail.value = null;
    draftConversation.value = null;
  }

  function clearDraftOptimisticTasks() {
    if (!optimisticTasksByConversation.value.__draft__) {
      return;
    }
    const nextState = { ...optimisticTasksByConversation.value };
    delete nextState.__draft__;
    optimisticTasksByConversation.value = nextState;
  }

  async function loadPanels() {
    gitInspect.value = await inspectGitWorkspace(
      store.relayBaseUrl,
      {
        deviceId: deviceId.value,
        sessionCwd: cwd.value ?? undefined
      },
      store.relayAccessToken
    );
    if (gitInspect.value?.changedFiles.length) {
      const selectedRepoPath = gitInspect.value.changedFiles.some(
        (file) => file.repoPath === activeDiffRepoPath.value
      )
        ? activeDiffRepoPath.value
        : gitInspect.value.changedFiles[0]?.repoPath;
      if (selectedRepoPath) {
        await selectChangeFile(selectedRepoPath);
      }
    } else {
      activeDiffRepoPath.value = null;
      gitDiff.value = null;
    }

    workspace.value = await browseWorkspace(
      store.relayBaseUrl,
      {
        deviceId: deviceId.value,
        sessionCwd: cwd.value ?? undefined,
        path: browserPath.value || undefined
      },
      store.relayAccessToken
    );
  }

  async function refreshProject(options?: { includePanels?: boolean; refreshStore?: boolean }) {
    const includePanels = options?.includePanels ?? true;
    const refreshStore = options?.refreshStore ?? true;
    if (!enabled.value || !deviceId.value || !cwd.value) {
      clearConversationState();
      gitInspect.value = null;
      gitDiff.value = null;
      activeDiffRepoPath.value = null;
      workspace.value = null;
      filePreview.value = null;
      errorMessage.value = "";
      return;
    }

    isLoading.value = true;
    errorMessage.value = "";
    console.info("[vibe-app] project refresh start", {
      deviceId: deviceId.value,
      cwd: cwd.value
    });

    try {
      if (refreshStore) {
        await store.refreshAll();
      }

      if (activeConversationId.value) {
        await loadConversationContext(activeConversationId.value);
      } else {
        clearConversationState();
      }

      if (includePanels) {
        await loadPanels();
      }
      console.info("[vibe-app] project refresh success", {
        deviceId: deviceId.value,
        cwd: cwd.value,
        conversations: conversations.value.length,
        changedFiles: gitInspect.value?.changedFiles.length ?? 0,
        entries: workspace.value?.entries.length ?? 0
      });
    } catch (error) {
      errorMessage.value = error instanceof Error ? error.message : String(error);
      console.error("[vibe-app] project refresh failed", error);
    } finally {
      isLoading.value = false;
    }
  }

  async function selectConversation(conversationId: string) {
    isDraftConversation.value = false;
    serverConversationDetail.value = null;
    activeConversationId.value = conversationId;
  }

  async function loadConversationContext(conversationId: string) {
    const loadVersion = ++conversationLoadVersion;
    isRestoringConversation.value = true;

    try {
      const detail = await store.loadConversation(conversationId);
      if (loadVersion !== conversationLoadVersion || activeConversationId.value !== conversationId) {
        return;
      }

      serverConversationDetail.value = detail;
      errorMessage.value = "";
      updateOptimisticTasks(conversationId, (tasks) => pruneResolvedOptimisticTasks(tasks, detail));
    } catch (error) {
      if (loadVersion !== conversationLoadVersion || activeConversationId.value !== conversationId) {
        return;
      }
      errorMessage.value = error instanceof Error ? error.message : String(error);
    } finally {
      if (loadVersion === conversationLoadVersion && activeConversationId.value === conversationId) {
        isRestoringConversation.value = false;
      }
    }
  }

  function disposeRealtimeUpdates() {
    if (realtimeRefreshTimer !== null) {
      window.clearTimeout(realtimeRefreshTimer);
      realtimeRefreshTimer = null;
    }
    if (activeEventSource) {
      activeEventSource.close();
      activeEventSource = null;
    }
  }

  function scheduleConversationReload(conversationId: string) {
    if (realtimeRefreshTimer !== null) {
      window.clearTimeout(realtimeRefreshTimer);
    }
    realtimeRefreshTimer = window.setTimeout(() => {
      realtimeRefreshTimer = null;
      void loadConversationContext(conversationId);
    }, 250);
  }

  function handleRelayEvent(payload: string, conversationId: string) {
    let event: RelayEventEnvelope;
    try {
      event = JSON.parse(payload) as RelayEventEnvelope;
    } catch {
      return;
    }

    store.applyRelayEvent(event);
    if (!shouldRefreshConversationDetail(event, conversationId, serverConversationDetail.value)) {
      return;
    }

    scheduleConversationReload(conversationId);
  }

  function connectRealtimeUpdates(conversationId: string) {
    disposeRealtimeUpdates();
    if (!enabled.value || !store.relayBaseUrl.trim() || !store.relayAccessToken.trim()) {
      return;
    }

    const eventSource = new EventSource(
      buildEventStreamUrl(store.relayBaseUrl, store.relayAccessToken)
    );
    eventSource.addEventListener("task_updated", (event) => {
      handleRelayEvent((event as MessageEvent<string>).data, conversationId);
    });
    eventSource.addEventListener("task_event", (event) => {
      handleRelayEvent((event as MessageEvent<string>).data, conversationId);
    });
    eventSource.onerror = () => {
      console.warn("[vibe-app] conversation realtime stream interrupted", {
        conversationId
      });
    };
    activeEventSource = eventSource;
  }

  function startNewConversation() {
    isDraftConversation.value = true;
    activeConversationId.value = null;
    clearConversationState();
    clearDraftOptimisticTasks();
    activeTab.value = "conversation";
  }

  async function openEntry(path: string, kind: "directory" | "file") {
    if (kind === "directory") {
      browserPath.value = path;
      workspace.value = await browseWorkspace(
        store.relayBaseUrl,
        {
          deviceId: deviceId.value,
          sessionCwd: cwd.value ?? undefined,
          path
        },
        store.relayAccessToken
      );
      return;
    }

    filePreview.value = await previewWorkspaceFile(
      store.relayBaseUrl,
      {
        deviceId: deviceId.value,
        sessionCwd: cwd.value ?? undefined,
        path
      },
      store.relayAccessToken
    );
  }

  async function selectChangeFile(repoPath: string) {
    activeDiffRepoPath.value = repoPath;
    gitDiff.value = await fetchGitDiffFile(
      store.relayBaseUrl,
      {
        deviceId: deviceId.value,
        sessionCwd: cwd.value ?? undefined,
        repoPath
      },
      store.relayAccessToken
    );
  }

  async function createTopic(prompt: string, model?: string, executionMode?: TaskExecutionMode) {
    const optimisticTaskId = addOptimisticTask("__draft__", prompt, model, executionMode);
    const preferredProvider = preferredProjectProvider(project.value?.providers);
    draftConversation.value = null;

    try {
      const response = await store.createProjectConversation({
        deviceId: deviceId.value,
        provider: provider.value ?? preferredProvider,
        executionMode,
        prompt,
        cwd: cwd.value ?? undefined,
        model: model || undefined,
        title: prompt.slice(0, 60)
      });
      isDraftConversation.value = false;
      draftConversation.value = response.conversation;
      moveOptimisticTasks("__draft__", response.conversation.id);
      updateOptimisticTasks(response.conversation.id, (tasks) =>
        replaceOptimisticTask(tasks, optimisticTaskId, response.task)
      );
      activeConversationId.value = response.conversation.id;
      store.markProjectVisited(deviceId.value, cwd.value);
      void loadConversationContext(response.conversation.id);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      updateOptimisticTasks("__draft__", (tasks) => markOptimisticTaskFailed(tasks, optimisticTaskId, message));
      throw error;
    }
  }

  async function sendFollowUp(
    prompt: string,
    model?: string,
    executionMode?: TaskExecutionMode
  ) {
    if (!activeConversationId.value) {
      await createTopic(prompt, model, executionMode);
      return;
    }

    isDraftConversation.value = false;
    const conversationId = activeConversationId.value;
    const optimisticTaskId = addOptimisticTask(conversationId, prompt, model, executionMode);

    try {
      const response = await store.sendProjectMessage(conversationId, {
        prompt,
        executionMode,
        model: model || undefined
      });
      updateOptimisticTasks(conversationId, (tasks) =>
        replaceOptimisticTask(tasks, optimisticTaskId, response.task)
      );
      void loadConversationContext(conversationId);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      updateOptimisticTasks(conversationId, (tasks) => markOptimisticTaskFailed(tasks, optimisticTaskId, message));
      throw error;
    }
  }

  async function respondToInput(optionId?: string, text?: string) {
    const request = conversationDetail.value?.pendingInputRequest;
    if (!request) {
      return;
    }

    await store.respondToInput(request.taskId, request.id, optionId, text);
    if (activeConversationId.value) {
      await loadConversationContext(activeConversationId.value);
    }
  }

  async function cancelTask(taskId: string) {
    await store.cancelProjectTask(taskId);
    if (activeConversationId.value) {
      await loadConversationContext(activeConversationId.value);
    }
  }

  watch(activeConversationId, async (value) => {
    disposeRealtimeUpdates();
    if (!value) {
      isRestoringConversation.value = false;
      return;
    }

    serverConversationDetail.value = null;
    await loadConversationContext(value);
    connectRealtimeUpdates(value);
  }, { immediate: true });

  watch(
    () => store.lastRefreshEpochMs,
    async () => {
      if (!enabled.value || isDraftConversation.value || !activeConversationId.value) {
        return;
      }

      await loadConversationContext(activeConversationId.value);
    }
  );

  watch(enabled, async (value) => {
    if (!value) {
      clearConversationState();
      gitInspect.value = null;
      gitDiff.value = null;
      activeDiffRepoPath.value = null;
      workspace.value = null;
      filePreview.value = null;
      activeConversationId.value = null;
      isRestoringConversation.value = false;
      disposeRealtimeUpdates();
      return;
    }

    await refreshProject();
  }, { immediate: false });

  watch(
    () => [store.relayBaseUrl, store.relayAccessToken],
    () => {
      if (!activeConversationId.value || isDraftConversation.value) {
        disposeRealtimeUpdates();
        return;
      }
      connectRealtimeUpdates(activeConversationId.value);
    }
  );

  onScopeDispose(() => {
    disposeRealtimeUpdates();
  });

  return {
    cwd,
    project,
    tasks,
    conversations,
    activeConversationId,
    isDraftConversation,
    isRestoringConversation,
    conversationDetail,
    gitInspect,
    gitDiff,
    activeDiffRepoPath,
    workspace,
    filePreview,
    activeTab,
    isLoading,
    errorMessage,
    refreshProject,
    startNewConversation,
    selectChangeFile,
    selectConversation,
    openEntry,
    createTopic,
    sendFollowUp,
    respondToInput,
    cancelTask
  };
}
