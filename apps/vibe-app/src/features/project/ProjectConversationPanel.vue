<script setup lang="ts">
import { computed, nextTick, ref, useTemplateRef, watch } from "vue";
import { useI18n } from "vue-i18n";
import { Check, Copy, Plus, Send, Settings2 } from "lucide-vue-next";
import StatusBadge from "@/components/common/StatusBadge.vue";
import { buildTaskFailureSummary } from "@/lib/conversationRealtime";
import { formatDateTime } from "@/lib/format";
import { useAppStore } from "@/stores/app";
import type {
  ConversationPanelDetail,
  ProviderKind,
  TaskDetailResponse,
  TaskExecutionMode,
  TaskStatus
} from "@/types";

const props = defineProps<{
  detail: ConversationPanelDetail | null;
  projectProviders?: ProviderKind[];
  projectTitle?: string | null;
  isDraftConversation?: boolean;
  isRestoringConversation?: boolean;
  selectedModel?: string;
  modelOptions?: { label: string; value: string }[];
  canCompose?: boolean;
  emptySummary?: string;
}>();

const emit = defineEmits<{
  sendPrompt: [prompt: string, model?: string, executionMode?: TaskExecutionMode];
  respondInput: [optionId?: string, text?: string];
  cancelTask: [taskId: string];
  openTab: [tab: "changes" | "files"];
  "update:selectedModel": [value: string];
}>();

const { t } = useI18n();
const store = useAppStore();
const prompt = ref("");
const model = ref("");
const executionMode = ref<TaskExecutionMode>(store.defaultExecutionMode);
const customReply = ref("");
const composerExpanded = ref(false);
const copiedTurnId = ref<string | null>(null);
const promptInput = useTemplateRef<HTMLTextAreaElement>("promptInput");

watch(
  () => props.selectedModel,
  (value) => {
    model.value = value ?? "";
  },
  { immediate: true }
);

watch(model, (value) => {
  emit("update:selectedModel", value);
});

watch(
  () => store.defaultExecutionMode,
  (value) => {
    executionMode.value = value;
  }
);

watch(prompt, async () => {
  await nextTick();
  resizePrompt();
});

const executionModeOptions: { value: TaskExecutionMode; labelKey: string; summaryKey: string }[] = [
  {
    value: "read_only",
    labelKey: "conversation.executionMode.readOnly",
    summaryKey: "conversation.executionMode.readOnlySummary"
  },
  {
    value: "workspace_write",
    labelKey: "conversation.executionMode.workspaceWrite",
    summaryKey: "conversation.executionMode.workspaceWriteSummary"
  },
  {
    value: "workspace_write_and_test",
    labelKey: "conversation.executionMode.workspaceWriteAndTest",
    summaryKey: "conversation.executionMode.workspaceWriteAndTestSummary"
  }
];

const selectedExecutionMode = computed(
  () => executionModeOptions.find((option) => option.value === executionMode.value) ?? executionModeOptions[1]
);
const canCompose = computed(() => props.canCompose !== false);

const turnCards = computed(() =>
  (props.detail?.tasks ?? []).map((taskDetail) => {
    const assistantText = taskDetail.events
      .filter((event) => event.kind === "assistant_delta")
      .map((event) => event.message)
      .join("")
      .trim();

    return {
      id: taskDetail.task.id,
      task: taskDetail.task,
      prompt: taskDetail.task.prompt,
      assistantText,
      summary: buildTurnSummary(taskDetail, assistantText),
      stderrLines: taskDetail.events.filter((event) => event.kind === "provider_stderr"),
      rawLines: taskDetail.events
    };
  })
);

const hasTurns = computed(() => turnCards.value.length > 0);

function submitPrompt() {
  if (!canCompose.value || !prompt.value.trim()) {
    return;
  }

  emit(
    "sendPrompt",
    prompt.value.trim(),
    model.value.trim() || undefined,
    executionMode.value
  );
  prompt.value = "";
  model.value = "";
  composerExpanded.value = false;
}

function submitCustomReply() {
  if (!canCompose.value || !customReply.value.trim()) {
    return;
  }

  emit("respondInput", undefined, customReply.value.trim());
  customReply.value = "";
}

function buildTurnSummary(taskDetail: TaskDetailResponse, assistantText: string) {
  if (assistantText) {
    return assistantText;
  }
  if (taskDetail.task.status === "failed") {
    const failureSummary = buildTaskFailureSummary(taskDetail);
    if (failureSummary) {
      return failureSummary;
    }
  }
  if (taskDetail.task.error) {
    return taskDetail.task.error;
  }

  return t(`conversation.statusSummary.${taskDetail.task.status}`);
}

function showAssistantPlaceholder(turn: (typeof turnCards.value)[number]) {
  return (
    !turn.assistantText &&
    (turn.task.status === "running" || turn.task.status === "assigned" || turn.task.status === "pending")
  );
}

function taskTone(status: TaskStatus) {
  if (status === "failed" || status === "canceled") {
    return "danger" as const;
  }
  if (status === "waiting_input" || status === "cancel_requested") {
    return "warning" as const;
  }
  if (status === "running" || status === "assigned" || status === "pending") {
    return "default" as const;
  }
  return "success" as const;
}

function canCancelTask(status: TaskStatus, cancelRequested: boolean) {
  if (cancelRequested) {
    return false;
  }

  return status === "pending" || status === "assigned" || status === "running";
}

function canRetryTask(status: TaskStatus) {
  return status === "failed" || status === "canceled";
}

function canOpenChanges(status: TaskStatus) {
  return status === "succeeded";
}

function buildRetryPrompt(turn: (typeof turnCards.value)[number]) {
  return [
    "Retry the previous request in this project.",
    `Original request: ${turn.prompt}`,
    "If the previous attempt failed, explain the failure briefly, then continue with the fix."
  ].join("\n\n");
}

function buildExplainPrompt(turn: (typeof turnCards.value)[number]) {
  return [
    "Explain the result of the previous task in this project.",
    `Original request: ${turn.prompt}`,
    "Summarize what changed, what commands or tools were used, and any remaining risk."
  ].join("\n\n");
}

function resizePrompt() {
  const element = promptInput.value;
  if (!element) {
    return;
  }

  element.style.height = "0px";
  element.style.height = `${Math.min(element.scrollHeight, 144)}px`;
}

function handlePromptKeydown(event: KeyboardEvent) {
  if (event.key !== "Enter" || event.shiftKey) {
    return;
  }

  event.preventDefault();
  submitPrompt();
}

async function copyTurn(turn: (typeof turnCards.value)[number]) {
  const sections = [turn.assistantText.trim() ? turn.assistantText.trim() : turn.summary.trim()];

  if (turn.stderrLines.length) {
    sections.push(
      ["STDERR", ...turn.stderrLines.map((event) => event.message.trim()).filter(Boolean)].join("\n")
    );
  }

  if (turn.task.error && !sections.some((section) => section.includes(turn.task.error ?? ""))) {
    sections.push(["ERROR", turn.task.error].join("\n"));
  }

  await navigator.clipboard.writeText(sections.filter(Boolean).join("\n\n"));
  copiedTurnId.value = turn.id;
  window.setTimeout(() => {
    if (copiedTurnId.value === turn.id) {
      copiedTurnId.value = null;
    }
  }, 1800);
}
</script>

<template>
  <section class="flex h-full min-h-0 flex-col overflow-hidden">
    <div class="min-h-0 flex-1 overflow-y-auto overscroll-contain px-1 pb-6 pt-2">
      <div class="space-y-6">
        <article
          v-if="isRestoringConversation && !hasTurns"
          class="mx-auto max-w-3xl rounded-[2rem] border border-border/70 bg-background/80 px-6 py-8"
        >
          <div class="space-y-4">
            <div class="h-4 w-40 animate-pulse rounded-full bg-muted" />
            <div class="h-24 animate-pulse rounded-[1.5rem] bg-muted/70" />
            <div class="ml-auto h-16 w-2/3 animate-pulse rounded-[1.5rem] bg-primary/12" />
            <div class="h-28 animate-pulse rounded-[1.5rem] bg-muted/70" />
          </div>
        </article>

        <article
          v-else-if="!hasTurns"
          class="mx-auto max-w-2xl rounded-[2rem] border border-dashed border-border bg-background/70 px-6 py-10 text-center"
        >
          <p class="text-sm font-medium text-foreground">
            {{
              isDraftConversation
                ? t("conversation.newChatTitle")
                : t("conversation.firstTurnTitle")
            }}
          </p>
          <p class="mt-3 text-sm text-muted-foreground">
            {{ emptySummary || t("conversation.firstTurnSummary", { project: projectTitle || t("workspace.title") }) }}
          </p>
        </article>

        <article
          v-for="turn in turnCards"
          :key="turn.id"
          class="mx-auto max-w-3xl space-y-4"
        >
          <div class="flex justify-end">
            <div class="max-w-[88%] rounded-[1.6rem] bg-primary px-4 py-3 text-primary-foreground shadow-sm">
              <p class="whitespace-pre-wrap text-sm leading-6">{{ turn.prompt }}</p>
              <p class="mt-2 text-[11px] text-primary-foreground/75">
                {{ formatDateTime(turn.task.createdAtEpochMs) }} ·
                {{ t(`conversation.executionModeMeta.${turn.task.executionMode}`) }}
              </p>
            </div>
          </div>

          <div class="max-w-[92%] rounded-[1.6rem] border border-border/70 bg-white/90 px-4 py-4 shadow-sm dark:bg-slate-950/50">
            <div class="flex flex-wrap items-center gap-2">
              <StatusBadge :tone="taskTone(turn.task.status)">
                {{ t(`conversation.statusLabel.${turn.task.status}`) }}
              </StatusBadge>
              <button
                v-if="canCancelTask(turn.task.status, turn.task.cancelRequested)"
                class="rounded-full border border-amber-500/30 bg-amber-500/10 px-3 py-1 text-xs font-medium text-amber-700 dark:text-amber-300"
                @click="emit('cancelTask', turn.task.id)"
              >
                {{ t("conversation.stopTask") }}
              </button>
              <button
                v-if="canRetryTask(turn.task.status)"
                class="rounded-full border border-border bg-background px-3 py-1 text-xs font-medium text-foreground"
                @click="emit('sendPrompt', buildRetryPrompt(turn), turn.task.model || undefined, turn.task.executionMode)"
              >
                {{ t("conversation.retryTask") }}
              </button>
              <button
                v-if="turn.task.status !== 'running' && turn.task.status !== 'pending' && turn.task.status !== 'assigned'"
                class="rounded-full border border-border bg-background px-3 py-1 text-xs font-medium text-foreground"
                @click="emit('sendPrompt', buildExplainPrompt(turn), turn.task.model || undefined, 'read_only')"
              >
                {{ t("conversation.explainResult") }}
              </button>
              <button
                v-if="canOpenChanges(turn.task.status)"
                class="rounded-full border border-border bg-background px-3 py-1 text-xs font-medium text-foreground"
                @click="emit('openTab', 'changes')"
              >
                {{ t("conversation.viewChanges") }}
              </button>
              <button
                class="ml-auto inline-flex items-center gap-1 rounded-full border border-border bg-background px-3 py-1 text-xs font-medium text-foreground"
                @click="copyTurn(turn)"
              >
                <component :is="copiedTurnId === turn.id ? Check : Copy" class="size-3.5" />
                {{ copiedTurnId === turn.id ? t("conversation.copied") : t("conversation.copyTurn") }}
              </button>
            </div>

            <div
              v-if="showAssistantPlaceholder(turn)"
              class="mt-4 rounded-[1.3rem] border border-dashed border-border/70 bg-background/70 px-4 py-4"
            >
              <div class="flex items-center gap-2 text-sm font-medium text-foreground">
                <span class="inline-flex gap-1">
                  <span class="size-2 animate-bounce rounded-full bg-primary [animation-delay:-0.2s]" />
                  <span class="size-2 animate-bounce rounded-full bg-primary [animation-delay:-0.1s]" />
                  <span class="size-2 animate-bounce rounded-full bg-primary" />
                </span>
                {{ t("conversation.assistantPlaceholder") }}
              </div>
              <p class="mt-2 text-sm text-muted-foreground">
                {{ turn.summary }}
              </p>
              <div class="mt-4 space-y-2">
                <div class="h-2.5 w-full animate-pulse rounded-full bg-muted/70" />
                <div class="h-2.5 w-5/6 animate-pulse rounded-full bg-muted/60 [animation-delay:120ms]" />
                <div class="h-2.5 w-2/3 animate-pulse rounded-full bg-muted/50 [animation-delay:240ms]" />
              </div>
            </div>

            <p v-else class="mt-4 whitespace-pre-wrap text-sm leading-7 text-foreground">
              {{ turn.summary }}
            </p>
            <details class="mt-4 rounded-xl border border-dashed border-border bg-background/60 px-3 py-2">
              <summary class="cursor-pointer text-xs font-medium text-muted-foreground">
                {{ t("conversation.rawEventsTitle") }}
              </summary>
              <div class="mt-3 space-y-2">
                <div
                  v-for="event in turn.rawLines"
                  :key="`${turn.id}-raw-${event.seq}`"
                  class="rounded-lg border border-border bg-background px-3 py-2 text-xs"
                >
                  <p class="font-medium">{{ event.kind }}</p>
                  <p class="mt-1 whitespace-pre-wrap leading-5">{{ event.message }}</p>
                </div>
              </div>
            </details>
          </div>
        </article>
      </div>
    </div>

    <div class="mt-auto shrink-0 border-t border-border/60 bg-[linear-gradient(180deg,rgba(244,244,240,0),rgba(244,244,240,0.96)_20%,rgba(244,244,240,0.98)_100%)] px-1 pb-[max(0.5rem,env(safe-area-inset-bottom))] pt-4 dark:bg-[linear-gradient(180deg,rgba(9,9,11,0),rgba(9,9,11,0.94)_20%,rgba(9,9,11,0.98)_100%)]">
      <div
        v-if="detail?.pendingInputRequest"
        class="mx-auto mb-4 max-w-3xl rounded-[1.4rem] border border-amber-500/30 bg-amber-500/8 p-4"
      >
        <div class="flex items-center gap-2">
          <StatusBadge tone="warning">{{ t("conversation.waitingInput") }}</StatusBadge>
          <p class="text-sm font-medium">{{ detail.pendingInputRequest.prompt }}</p>
        </div>
        <div class="mt-4 flex flex-wrap gap-2">
          <button
            v-for="option in detail.pendingInputRequest.options"
            :key="option.id"
            class="rounded-full border border-amber-500/30 bg-background px-3 py-2 text-sm"
            @click="emit('respondInput', option.id)"
          >
            {{ option.label }}
          </button>
        </div>
        <div v-if="detail.pendingInputRequest.allowCustomInput" class="mt-4 space-y-2">
          <textarea
            v-model="customReply"
            rows="3"
            class="w-full rounded-2xl border border-border bg-background px-4 py-3 text-sm"
            :disabled="!canCompose"
            :placeholder="detail.pendingInputRequest.customInputPlaceholder || t('conversation.replyPlaceholder')"
          />
          <button class="rounded-full bg-primary px-4 py-2 text-sm font-medium text-primary-foreground disabled:cursor-not-allowed disabled:opacity-50" :disabled="!canCompose" @click="submitCustomReply">
            {{ t("conversation.sendCustomReply") }}
          </button>
        </div>
      </div>

      <div class="mx-auto max-w-3xl rounded-[1.8rem] border border-white/60 bg-white/90 p-3 shadow-[0_24px_60px_-35px_rgba(15,23,42,0.55)] backdrop-blur dark:border-white/10 dark:bg-slate-950/75">
        <div v-if="composerExpanded" class="mb-3 grid gap-3 border-b border-border/60 pb-3 md:grid-cols-[minmax(0,1fr)_minmax(0,1fr)]">
          <div class="space-y-2 rounded-2xl bg-background/75 px-3 py-3">
            <div class="flex items-center gap-2 text-[11px] font-semibold uppercase tracking-[0.18em] text-muted-foreground">
              <Settings2 class="size-3.5" />
              {{ t("conversation.executionMode.title") }}
            </div>
            <select
              v-model="executionMode"
              class="w-full rounded-2xl border border-border bg-background px-3 py-2.5 text-sm"
              :disabled="!canCompose"
            >
              <option
                v-for="option in executionModeOptions"
                :key="option.value"
                :value="option.value"
              >
                {{ t(option.labelKey) }}
              </option>
            </select>
            <p class="text-[11px] text-muted-foreground">{{ t(selectedExecutionMode.summaryKey) }}</p>
          </div>
          <div class="space-y-2 rounded-2xl bg-background/75 px-3 py-3">
            <p class="text-[11px] font-semibold uppercase tracking-[0.18em] text-muted-foreground">
              {{ t("chat.model") }}
            </p>
            <select
              v-if="modelOptions?.length"
              v-model="model"
              class="w-full rounded-2xl border border-border bg-background px-3 py-2.5 text-sm"
              :disabled="!canCompose"
            >
              <option value="">{{ t("conversation.optionalModel") }}</option>
              <option
                v-for="option in modelOptions"
                :key="option.value"
                :value="option.value"
              >
                {{ option.label }}
              </option>
            </select>
            <input
              v-else
              v-model="model"
              type="text"
              class="w-full rounded-2xl border border-border bg-background px-3 py-2.5 text-sm"
              :disabled="!canCompose"
              :placeholder="t('conversation.optionalModel')"
            />
          </div>
        </div>

        <div class="flex items-end gap-2">
          <button
            class="inline-flex size-11 shrink-0 items-center justify-center rounded-full border border-border bg-background text-foreground disabled:cursor-not-allowed disabled:opacity-50"
            :disabled="!canCompose"
            @click="composerExpanded = !composerExpanded"
          >
            <component :is="composerExpanded ? Settings2 : Plus" class="size-5" />
          </button>
          <div class="min-w-0 flex-1 rounded-[1.6rem] border border-border bg-background px-4 py-2.5">
            <textarea
              ref="promptInput"
              v-model="prompt"
              rows="1"
              class="max-h-36 min-h-[1.75rem] w-full resize-none bg-transparent text-sm leading-6 outline-none"
              :disabled="!canCompose"
              :placeholder="canCompose ? t('conversation.promptPlaceholder') : t('conversation.disabledPlaceholder')"
              @keydown="handlePromptKeydown"
              @input="resizePrompt"
            />
          </div>
          <button
            class="inline-flex size-11 shrink-0 items-center justify-center rounded-full bg-primary text-primary-foreground disabled:cursor-not-allowed disabled:opacity-50"
            :disabled="!canCompose || !prompt.trim()"
            @click="submitPrompt"
          >
            <Send class="size-5" />
          </button>
        </div>
      </div>
    </div>
  </section>
</template>
