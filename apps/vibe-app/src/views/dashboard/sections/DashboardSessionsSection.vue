<script setup lang="ts">
import { computed } from "vue";
import { storeToRefs } from "pinia";
import { useI18n } from "vue-i18n";
import type { ConversationInputOption, TaskDetailResponse } from "@/types";
import SessionContextSidebar from "@/views/dashboard/session/SessionContextSidebar.vue";
import SessionConversationPanel from "@/views/dashboard/session/SessionConversationPanel.vue";
import type {
  ConversationListItem,
  TranscriptTurn,
} from "@/views/dashboard/session/types";
import { useDashboardController } from "@/views/dashboard/controller";

const CUSTOM_INPUT_SENTINEL = "__custom_input__";

const { t } = useI18n();
const dashboard = useDashboardController();
const store = dashboard.store;

const {
  formatConnectionState,
  formatExecutionProtocol,
  formatProviderKind,
  formatTaskStatus,
  formatTimestamp,
  navigateWorkspaceRoot,
  navigateWorkspaceUp,
  nativeSelectClass,
  openWorkspaceEntry,
  projectFolder,
  refreshWorkspace,
  relayPlaceholder,
  selectedDeviceWorkingRoot,
  selectedWorkspacePath,
  sessionLaunchState,
  taskStatusClass,
  workspaceError,
  workspaceListing,
  workspaceLoading,
  workspacePreview,
  workspacePreviewLoading,
} = dashboard;

const {
  conversations,
  devices,
  draft,
  eventState,
  pendingConversationInputDraft,
  relayAccessTokenInput,
  relayBaseUrl,
  relayInput,
  selectedConversationId,
  selectedDevice,
} = storeToRefs(store);

const availableProviders = computed(() =>
  store.availableProviders.map((provider) => ({
    kind: provider.kind,
    label: formatProviderKind(provider.kind),
  })),
);
const selectedConversation = computed(() => store.selectedConversation);
const pendingInputRequest = computed(() => store.conversationPendingInput);
const currentConversationTask = computed(() => store.conversationCurrentTask);
const selectedDraftProvider = computed(
  () =>
    store.availableProviders.find(
      (provider) => provider.kind === draft.value.provider,
    ) ?? null,
);
const canSendPrompt = computed(() => {
  if (!draft.value.prompt.trim() || !relayBaseUrl.value) {
    return false;
  }

  if (selectedConversation.value) {
    const device = devices.value.find(
      (item) => item.id === selectedConversation.value?.deviceId,
    );
    return Boolean(device?.online);
  }

  return sessionLaunchState.value === "ready" && Boolean(draft.value.provider);
});
const transcriptTurns = computed<TranscriptTurn[]>(() =>
  (selectedConversation.value
    ? (store.selectedConversationDetail?.tasks ?? [])
    : []
  ).map((detail) => {
    const toolEvents = detail.events.filter(
      (event) => event.kind === "tool_call" || event.kind === "tool_output",
    );
    const systemEvents = detail.events.filter(
      (event) =>
        event.kind === "system" ||
        event.kind === "status" ||
        event.kind === "provider_stderr",
    );
    const traceEvents = [...toolEvents, ...systemEvents].sort((left, right) => {
      if (left.timestampEpochMs === right.timestampEpochMs) {
        return left.seq - right.seq;
      }

      return left.timestampEpochMs - right.timestampEpochMs;
    });

    return {
      detail,
      assistantText: buildAssistantText(detail),
      toolEvents,
      systemEvents,
      traceEvents,
    };
  }),
);
const conversationItems = computed<ConversationListItem[]>(() =>
  [...conversations.value]
    .sort((left, right) => right.updatedAtEpochMs - left.updatedAtEpochMs)
    .map((conversation) => {
      const latestTask = latestConversationTask(conversation.id);

      return {
        id: conversation.id,
        title: conversation.title,
        meta: `${deviceName(conversation.deviceId)} · ${formatProviderKind(conversation.provider)}`,
        status: formatTaskStatus(latestTask?.status ?? "pending"),
        statusClass: taskStatusClass(latestTask?.status ?? "pending"),
        updatedAt: formatTimestamp(conversation.updatedAtEpochMs),
      };
    }),
);
const showPendingTextInput = computed(() => {
  if (!pendingInputRequest.value) {
    return false;
  }

  if (
    !pendingInputRequest.value.options.length &&
    pendingInputRequest.value.allowCustomInput
  ) {
    return true;
  }

  if (pendingConversationInputDraft.value.optionId === CUSTOM_INPUT_SENTINEL) {
    return true;
  }

  const selectedOption = pendingInputRequest.value.options.find(
    (option) => option.id === pendingConversationInputDraft.value.optionId,
  );
  return Boolean(selectedOption?.requiresTextInput);
});
const canSubmitPendingText = computed(
  () =>
    showPendingTextInput.value &&
    Boolean(pendingConversationInputDraft.value.text.trim()),
);
const activeThreadTitle = computed(
  () => selectedConversation.value?.title ?? t("dashboard.chat.composeTitle"),
);
const activeThreadSummary = computed(() => {
  if (selectedConversation.value) {
    return t("dashboard.chat.composeSummary", {
      device: deviceName(selectedConversation.value.deviceId),
      provider: formatProviderKind(selectedConversation.value.provider),
    });
  }

  return t("dashboard.chat.composeEmptySummary");
});
const currentDeviceLabel = computed(() =>
  selectedConversation.value?.deviceId
    ? deviceName(selectedConversation.value.deviceId)
    : (selectedDevice.value?.name ?? t("dashboard.shell.noDeviceSelected")),
);
const currentProviderLabel = computed(() => {
  if (selectedConversation.value) {
    return formatProviderKind(selectedConversation.value.provider);
  }

  if (draft.value.provider) {
    return formatProviderKind(draft.value.provider);
  }

  return t("common.provider");
});
const currentProtocolLabel = computed(() => {
  if (selectedConversation.value) {
    return formatExecutionProtocol(
      selectedConversation.value.executionProtocol,
    );
  }

  if (selectedDraftProvider.value) {
    return formatExecutionProtocol(
      selectedDraftProvider.value.executionProtocol,
    );
  }

  return t("common.pending");
});
const currentWorkingDirectory = computed(() => {
  if (selectedConversation.value?.cwd) {
    return selectedConversation.value.cwd;
  }

  if (projectFolder.value.trim()) {
    return projectFolder.value.trim();
  }

  return selectedDeviceWorkingRoot.value;
});
const currentModelLabel = computed(() => {
  if (selectedConversation.value?.model) {
    return selectedConversation.value.model;
  }

  if (draft.value.model.trim()) {
    return draft.value.model.trim();
  }

  return t("common.defaultModel");
});
const setupStateLabel = computed(() => {
  if (sessionLaunchState.value === "ready") {
    return t("dashboard.sessions.launchState.ready", {
      device:
        selectedDevice.value?.name ?? t("dashboard.shell.noDeviceSelected"),
    });
  }

  return t(`dashboard.sessions.launchState.${sessionLaunchState.value}`);
});
const selectedDeviceName = computed(
  () => selectedDevice.value?.name ?? t("dashboard.shell.noDeviceSelected"),
);
const workspacePath = computed(() => {
  if (workspaceListing.value?.path) {
    return workspaceListing.value.path;
  }

  return projectFolder.value.trim() || selectedDeviceWorkingRoot.value;
});
const workspaceRootPath = computed(() => {
  if (workspaceListing.value?.rootPath) {
    return workspaceListing.value.rootPath;
  }

  return projectFolder.value.trim() || selectedDeviceWorkingRoot.value;
});

function buildAssistantText(detail: TaskDetailResponse) {
  const assistant = detail.events
    .filter((event) => event.kind === "assistant_delta")
    .map((event) => event.message)
    .join("")
    .trim();

  if (assistant) {
    return assistant;
  }

  return detail.events
    .filter((event) => event.kind === "provider_stdout")
    .map((event) => event.message)
    .join("\n")
    .trim();
}

function deviceName(deviceId: string) {
  return (
    devices.value.find((device) => device.id === deviceId)?.name ?? deviceId
  );
}

function latestConversationTask(conversationId: string) {
  return (
    store.tasks.find((task) => task.conversationId === conversationId) ?? null
  );
}

function startNewConversation() {
  store.startNewConversationDraft();
}

function selectConversation(conversationId: string) {
  void store.selectConversation(conversationId);
}

function chooseProvider(provider: string) {
  draft.value.provider = provider as typeof draft.value.provider;
}

function handleDeviceSelect(deviceId: string) {
  if (!deviceId) {
    return;
  }

  void store.selectDevice(deviceId);
}

function submitPrompt() {
  void store.sendConversationPrompt();
}

function updateProjectFolder(folder: string) {
  store.setProjectFolder(folder);
  void navigateWorkspaceRoot();
}

function archiveConversation() {
  void store.archiveSelectedConversation();
}

function refreshSessionSurface() {
  void Promise.all([store.reloadAll(), refreshWorkspace()]);
}

function choosePendingOption(option: ConversationInputOption) {
  pendingConversationInputDraft.value.optionId = option.id;

  if (!option.requiresTextInput) {
    void store.respondToConversationInput({ optionId: option.id });
  }
}

function choosePendingCustomInput() {
  pendingConversationInputDraft.value.optionId = CUSTOM_INPUT_SENTINEL;
}

function submitPendingInput() {
  const optionId = pendingConversationInputDraft.value.optionId;

  void store.respondToConversationInput({
    optionId:
      optionId && optionId !== CUSTOM_INPUT_SENTINEL ? optionId : undefined,
    text: pendingConversationInputDraft.value.text.trim() || undefined,
  });
}
</script>

<template>
  <section class="grid gap-4 xl:grid-cols-[340px_minmax(0,1fr)]">
    <SessionContextSidebar
      :relay-input="relayInput"
      :relay-access-token-input="relayAccessTokenInput"
      :relay-placeholder="relayPlaceholder"
      :connection-state-label="formatConnectionState(eventState)"
      :selected-device-id="selectedDevice?.id ?? null"
      :selected-device-name="selectedDeviceName"
      :devices="devices"
      :native-select-class="nativeSelectClass"
      :available-providers="availableProviders"
      :selected-provider="draft.provider"
      :project-folder="projectFolder"
      :project-folder-placeholder="selectedDeviceWorkingRoot"
      :setup-state-label="setupStateLabel"
      :selected-conversation-id="selectedConversationId"
      :conversation-items="conversationItems"
      :workspace-loading="workspaceLoading"
      :workspace-error="workspaceError"
      :workspace-path="workspacePath"
      :workspace-root-path="workspaceRootPath"
      :workspace-entries="workspaceListing?.entries ?? []"
      :workspace-preview="workspacePreview"
      :workspace-preview-loading="workspacePreviewLoading"
      :selected-workspace-path="selectedWorkspacePath"
      @update:relay-input="relayInput = $event"
      @update:relay-access-token-input="relayAccessTokenInput = $event"
      @connect-relay="store.applyRelayBaseUrl"
      @select-device="handleDeviceSelect"
      @choose-provider="chooseProvider"
      @update-project-folder="updateProjectFolder"
      @start-new-conversation="startNewConversation"
      @select-conversation="selectConversation"
      @refresh-workspace="refreshSessionSurface"
      @navigate-workspace-up="navigateWorkspaceUp"
      @navigate-workspace-root="navigateWorkspaceRoot"
      @open-workspace-entry="openWorkspaceEntry"
    />

    <SessionConversationPanel
      :selected-conversation="selectedConversation"
      :current-conversation-task="currentConversationTask"
      :transcript-turns="transcriptTurns"
      :active-thread-title="activeThreadTitle"
      :active-thread-summary="activeThreadSummary"
      :current-device-label="currentDeviceLabel"
      :current-provider-label="currentProviderLabel"
      :current-protocol-label="currentProtocolLabel"
      :current-working-directory="currentWorkingDirectory"
      :current-model-label="currentModelLabel"
      :prompt="draft.prompt"
      :title="draft.title"
      :model="draft.model"
      :can-send-prompt="canSendPrompt"
      :pending-input-request="pendingInputRequest"
      :pending-option-id="pendingConversationInputDraft.optionId"
      :pending-text="pendingConversationInputDraft.text"
      :show-pending-text-input="showPendingTextInput"
      :can-submit-pending-text="canSubmitPendingText"
      :custom-input-sentinel="CUSTOM_INPUT_SENTINEL"
      :format-timestamp="formatTimestamp"
      :format-task-status="formatTaskStatus"
      :format-execution-protocol="formatExecutionProtocol"
      :task-status-class="taskStatusClass"
      @update:prompt="draft.prompt = $event"
      @update:title="draft.title = $event"
      @update:model="draft.model = $event"
      @submit-prompt="submitPrompt"
      @archive-conversation="archiveConversation"
      @choose-pending-option="choosePendingOption"
      @choose-pending-custom-input="choosePendingCustomInput"
      @update-pending-text="pendingConversationInputDraft.text = $event"
      @submit-pending-input="submitPendingInput"
    />
  </section>
</template>
