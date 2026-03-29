<script setup lang="ts">
import { useI18n } from "vue-i18n";
import {
  Archive,
  Bot,
  LoaderCircle,
  Send,
  Sparkles,
  UserRound,
} from "lucide-vue-next";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Textarea } from "@/components/ui/textarea";
import type {
  ConversationInputOption,
  ConversationInputRequest,
  ConversationRecord,
  TaskRecord,
} from "@/types";
import type { TranscriptTurn } from "@/views/dashboard/session/types";

const props = defineProps<{
  selectedConversation: ConversationRecord | null;
  currentConversationTask: TaskRecord | null;
  transcriptTurns: TranscriptTurn[];
  activeThreadTitle: string;
  activeThreadSummary: string;
  currentDeviceLabel: string;
  currentProviderLabel: string;
  currentProtocolLabel: string;
  currentWorkingDirectory: string;
  currentModelLabel: string;
  prompt: string;
  title: string;
  model: string;
  canSendPrompt: boolean;
  pendingInputRequest: ConversationInputRequest | null;
  pendingOptionId: string | null;
  pendingText: string;
  showPendingTextInput: boolean;
  canSubmitPendingText: boolean;
  customInputSentinel: string;
  formatTimestamp: (value: number) => string;
  formatTaskStatus: (value: string) => string;
  formatExecutionProtocol: (value: string) => string;
  taskStatusClass: (value: string) => string;
}>();

const emit = defineEmits<{
  (event: "update:prompt", value: string): void;
  (event: "update:title", value: string): void;
  (event: "update:model", value: string): void;
  (event: "submitPrompt"): void;
  (event: "archiveConversation"): void;
  (event: "choosePendingOption", value: ConversationInputOption): void;
  (event: "choosePendingCustomInput"): void;
  (event: "updatePendingText", value: string): void;
  (event: "submitPendingInput"): void;
}>();

const { t } = useI18n();

function assistantFallbackText(turn: TranscriptTurn) {
  switch (turn.detail.task.status) {
    case "waiting_input":
      return t("dashboard.chat.waitingInput");
    case "failed":
      return t("dashboard.chat.traceOnlyFailed");
    case "succeeded":
      return t("dashboard.chat.traceOnlyCompleted");
    case "canceled":
      return t("dashboard.chat.traceOnlyCanceled");
    default:
      return t("dashboard.chat.generating");
  }
}
</script>

<template>
  <Card class="border-border/70 bg-card/85 shadow-2xl backdrop-blur-xl">
    <CardContent class="flex min-h-[calc(100vh-8rem)] flex-col p-0">
      <header class="border-b border-border/70 px-5 py-4">
        <div
          class="flex flex-col gap-4 xl:flex-row xl:items-start xl:justify-between"
        >
          <div class="space-y-2">
            <Badge
              variant="outline"
              class="w-fit border-sky-500/30 bg-sky-500/10 text-sky-700 dark:text-sky-100"
            >
              {{
                selectedConversation
                  ? t("dashboard.chat.activeThread")
                  : t("dashboard.chat.newThread")
              }}
            </Badge>
            <div class="space-y-1">
              <h2 class="text-2xl font-semibold tracking-tight text-foreground">
                {{ activeThreadTitle }}
              </h2>
              <p class="text-sm leading-6 text-muted-foreground">
                {{ activeThreadSummary }}
              </p>
            </div>
          </div>

          <div class="flex flex-wrap items-center gap-2">
            <Badge
              variant="outline"
              class="border-border/70 bg-background/60 text-foreground"
            >
              {{ currentDeviceLabel }}
            </Badge>
            <Badge
              variant="outline"
              class="border-border/70 bg-background/60 text-foreground"
            >
              <Sparkles class="mr-1 size-3.5" />
              {{ currentProviderLabel }}
            </Badge>
            <Badge
              variant="outline"
              class="border-border/70 bg-background/60 text-foreground"
            >
              {{ currentProtocolLabel }}
            </Badge>
            <Badge
              v-if="currentConversationTask"
              variant="outline"
              :class="taskStatusClass(currentConversationTask.status)"
            >
              {{ formatTaskStatus(currentConversationTask.status) }}
            </Badge>
            <Button
              v-if="selectedConversation"
              type="button"
              variant="outline"
              size="sm"
              @click="emit('archiveConversation')"
            >
              <Archive class="size-4" />
              {{ t("dashboard.chat.archive") }}
            </Button>
          </div>
        </div>

        <div class="mt-4 grid gap-3 md:grid-cols-3">
          <div class="rounded-2xl border border-border/70 bg-background/55 p-3">
            <p
              class="text-[11px] uppercase tracking-[0.18em] text-muted-foreground"
            >
              {{ t("dashboard.chat.projectFolder") }}
            </p>
            <p
              class="mt-2 truncate text-sm text-foreground"
              :title="currentWorkingDirectory"
            >
              {{ currentWorkingDirectory }}
            </p>
          </div>
          <div class="rounded-2xl border border-border/70 bg-background/55 p-3">
            <p
              class="text-[11px] uppercase tracking-[0.18em] text-muted-foreground"
            >
              {{ t("common.model") }}
            </p>
            <p
              class="mt-2 truncate text-sm text-foreground"
              :title="currentModelLabel"
            >
              {{ currentModelLabel }}
            </p>
          </div>
          <div class="rounded-2xl border border-border/70 bg-background/55 p-3">
            <p
              class="text-[11px] uppercase tracking-[0.18em] text-muted-foreground"
            >
              {{ t("dashboard.chat.latestTurn") }}
            </p>
            <p class="mt-2 text-sm text-foreground">
              {{
                currentConversationTask
                  ? formatTimestamp(currentConversationTask.createdAtEpochMs)
                  : t("common.pending")
              }}
            </p>
          </div>
        </div>
      </header>

      <ScrollArea class="min-h-0 flex-1">
        <div class="space-y-8 px-5 py-6">
          <template v-if="transcriptTurns.length">
            <article
              v-for="turn in transcriptTurns"
              :key="turn.detail.task.id"
              class="space-y-4"
            >
              <div class="flex justify-end">
                <div
                  class="max-w-[88%] rounded-[28px] bg-slate-900 px-5 py-4 text-sm leading-7 text-slate-50 shadow-lg dark:bg-slate-50 dark:text-slate-900"
                >
                  <div class="mb-3 flex items-center justify-between gap-3">
                    <div
                      class="flex items-center gap-2 text-xs uppercase tracking-[0.18em] text-slate-300 dark:text-slate-500"
                    >
                      <UserRound class="size-3.5" />
                      {{ t("dashboard.chat.userTurn") }}
                    </div>
                    <span class="text-xs text-slate-300/80 dark:text-slate-500">
                      {{ formatTimestamp(turn.detail.task.createdAtEpochMs) }}
                    </span>
                  </div>
                  <p class="whitespace-pre-wrap">
                    {{ turn.detail.task.prompt }}
                  </p>
                </div>
              </div>

              <div class="flex gap-3">
                <div
                  class="mt-1 flex size-10 shrink-0 items-center justify-center rounded-2xl bg-amber-500/12 text-amber-700 dark:text-amber-100"
                >
                  <Bot class="size-5" />
                </div>
                <div class="min-w-0 flex-1 space-y-3">
                  <div
                    class="rounded-[28px] border border-border/70 bg-background/70 px-5 py-4 shadow-sm"
                  >
                    <div class="mb-3 flex flex-wrap items-center gap-2">
                      <Badge
                        variant="outline"
                        :class="taskStatusClass(turn.detail.task.status)"
                      >
                        {{ formatTaskStatus(turn.detail.task.status) }}
                      </Badge>
                      <Badge
                        variant="outline"
                        class="border-border/70 bg-background/70 text-foreground"
                      >
                        {{
                          formatExecutionProtocol(
                            turn.detail.task.executionProtocol,
                          )
                        }}
                      </Badge>
                      <Badge
                        v-if="turn.traceEvents.length"
                        variant="outline"
                        class="border-border/70 bg-background/70 text-muted-foreground"
                      >
                        {{ t("dashboard.chat.toolEvents") }}
                        {{ turn.traceEvents.length }}
                      </Badge>
                    </div>

                    <p
                      v-if="turn.assistantText"
                      class="whitespace-pre-wrap text-sm leading-7 text-foreground"
                    >
                      {{ turn.assistantText }}
                    </p>
                    <p v-else class="text-sm leading-7 text-muted-foreground">
                      {{ assistantFallbackText(turn) }}
                    </p>
                  </div>
                </div>
              </div>
            </article>
          </template>

          <div
            v-else
            class="rounded-[32px] border border-dashed border-border/70 bg-gradient-to-br from-amber-500/8 via-background to-sky-500/8 px-6 py-10 text-center"
          >
            <div class="mx-auto max-w-xl space-y-3">
              <Badge
                variant="outline"
                class="border-amber-500/30 bg-amber-500/10 text-amber-700 dark:text-amber-100"
              >
                {{ t("dashboard.chat.emptyBadge") }}
              </Badge>
              <h3 class="text-2xl font-semibold tracking-tight text-foreground">
                {{ t("dashboard.chat.emptyTitle") }}
              </h3>
              <p class="text-sm leading-7 text-muted-foreground">
                {{ t("dashboard.chat.emptySummary") }}
              </p>
            </div>
          </div>
        </div>
      </ScrollArea>

      <div
        v-if="pendingInputRequest"
        class="border-t border-border/70 bg-amber-500/6 px-5 py-4"
      >
        <div
          class="space-y-4 rounded-[28px] border border-amber-500/25 bg-background/85 p-5 shadow-sm"
        >
          <div class="space-y-1">
            <Badge
              variant="outline"
              class="border-amber-500/30 bg-amber-500/12 text-amber-700 dark:text-amber-100"
            >
              {{ t("dashboard.chat.inputRequestBadge") }}
            </Badge>
            <h3 class="text-lg font-semibold text-foreground">
              {{ pendingInputRequest.prompt }}
            </h3>
            <p class="text-sm text-muted-foreground">
              {{ t("dashboard.chat.inputRequestSummary") }}
            </p>
          </div>

          <div
            v-if="pendingInputRequest.options.length"
            class="flex flex-wrap gap-2"
          >
            <button
              v-for="option in pendingInputRequest.options"
              :key="option.id"
              type="button"
              class="rounded-2xl border px-4 py-3 text-left transition"
              :class="
                pendingOptionId === option.id
                  ? 'border-sky-500/40 bg-sky-500/12 text-sky-900 dark:text-sky-100'
                  : 'border-border/70 bg-background/60 text-foreground hover:bg-accent/40'
              "
              @click="emit('choosePendingOption', option)"
            >
              <p class="text-sm font-medium">{{ option.label }}</p>
              <p
                v-if="option.description"
                class="mt-1 text-xs leading-5 text-muted-foreground"
              >
                {{ option.description }}
              </p>
            </button>

            <button
              v-if="pendingInputRequest.allowCustomInput"
              type="button"
              class="rounded-2xl border px-4 py-3 text-left transition"
              :class="
                pendingOptionId === customInputSentinel
                  ? 'border-sky-500/40 bg-sky-500/12 text-sky-900 dark:text-sky-100'
                  : 'border-border/70 bg-background/60 text-foreground hover:bg-accent/40'
              "
              @click="emit('choosePendingCustomInput')"
            >
              <p class="text-sm font-medium">
                {{ t("dashboard.chat.customOption") }}
              </p>
              <p class="mt-1 text-xs leading-5 text-muted-foreground">
                {{ t("dashboard.chat.customOptionSummary") }}
              </p>
            </button>
          </div>

          <div v-if="showPendingTextInput" class="space-y-3">
            <Textarea
              :model-value="pendingText"
              :placeholder="
                pendingInputRequest.customInputPlaceholder ??
                t('dashboard.chat.customInputPlaceholder')
              "
              class="min-h-[120px] border-border/70 bg-background/75"
              @update:model-value="emit('updatePendingText', String($event))"
            />
            <div class="flex justify-end">
              <Button
                type="button"
                :disabled="!canSubmitPendingText"
                @click="emit('submitPendingInput')"
              >
                <Send class="size-4" />
                {{ t("dashboard.chat.submitChoice") }}
              </Button>
            </div>
          </div>
        </div>
      </div>

      <footer class="border-t border-border/70 bg-card/90 px-5 py-4">
        <form class="space-y-3" @submit.prevent="emit('submitPrompt')">
          <div class="grid gap-3 lg:grid-cols-[minmax(0,1fr)_240px]">
            <Textarea
              :model-value="prompt"
              :placeholder="
                selectedConversation
                  ? t('dashboard.chat.replyPlaceholder')
                  : t('dashboard.chat.startPlaceholder')
              "
              class="min-h-[160px] border-border/70 bg-background/75 text-base leading-7"
              @update:model-value="emit('update:prompt', String($event))"
            />

            <div class="space-y-3">
              <div v-if="!selectedConversation" class="space-y-2">
                <label
                  class="text-[11px] font-medium uppercase tracking-[0.18em] text-muted-foreground"
                >
                  {{ t("common.title") }}
                </label>
                <Input
                  :model-value="title"
                  :placeholder="t('dashboard.chat.autoTitleHint')"
                  @update:model-value="emit('update:title', String($event))"
                />
              </div>

              <div class="space-y-2">
                <label
                  class="text-[11px] font-medium uppercase tracking-[0.18em] text-muted-foreground"
                >
                  {{ t("common.model") }}
                </label>
                <Input
                  :model-value="model"
                  :placeholder="t('common.defaultModel')"
                  @update:model-value="emit('update:model', String($event))"
                />
              </div>

              <Button type="submit" class="w-full" :disabled="!canSendPrompt">
                <template v-if="currentConversationTask?.status === 'running'">
                  <LoaderCircle class="size-4 animate-spin" />
                </template>
                <template v-else>
                  <Send class="size-4" />
                </template>
                {{
                  selectedConversation
                    ? t("dashboard.chat.sendReply")
                    : t("dashboard.chat.startConversation")
                }}
              </Button>
            </div>
          </div>
        </form>
      </footer>
    </CardContent>
  </Card>
</template>
