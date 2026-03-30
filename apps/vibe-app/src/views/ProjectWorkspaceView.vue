<script setup lang="ts">
import { computed, onMounted, watch } from "vue";
import { useRoute } from "vue-router";
import { useI18n } from "vue-i18n";
import StatusBadge from "@/components/common/StatusBadge.vue";
import { formatRelativeTime } from "@/lib/format";
import ProjectChangesPanel from "@/features/project/ProjectChangesPanel.vue";
import ProjectConversationPanel from "@/features/project/ProjectConversationPanel.vue";
import ProjectFilesPanel from "@/features/project/ProjectFilesPanel.vue";
import ProjectLogsPanel from "@/features/project/ProjectLogsPanel.vue";
import { useProjectWorkspace, type ProjectTab } from "@/features/project/useProjectWorkspace";
import { useAppStore } from "@/stores/app";

const route = useRoute();
const store = useAppStore();
const { t } = useI18n();
const deviceId = computed(() => String(route.params.deviceId));
const projectPath = computed(() => route.params.projectPath);
const workspace = useProjectWorkspace(deviceId, projectPath);
const {
  project,
  conversations,
  activeConversationId,
  conversationDetail,
  gitInspect,
  gitDiff,
  activeDiffRepoPath,
  workspace: workspaceBrowse,
  filePreview,
  activeTab,
  errorMessage
} = workspace;

const tabs: { id: ProjectTab; key: string }[] = [
  { id: "conversation", key: "workspace.tabs.conversation" },
  { id: "changes", key: "workspace.tabs.changes" },
  { id: "files", key: "workspace.tabs.files" },
  { id: "logs", key: "workspace.tabs.logs" }
];

const projectAvailabilityTone = computed(() => {
  if (project.value?.availabilityState === "offline") {
    return "muted" as const;
  }
  if (
    project.value?.availabilityState === "unreachable" ||
    project.value?.availabilityState === "history_only"
  ) {
    return "warning" as const;
  }
  return "success" as const;
});

async function restoreConversationFromRoute() {
  const routeConversationId =
    typeof route.query.conversationId === "string" && route.query.conversationId.trim()
      ? route.query.conversationId.trim()
      : null;
  if (!routeConversationId) {
    return;
  }

  const match = conversations.value.find(
    (conversation) => conversation.id === routeConversationId
  );
  if (!match || activeConversationId.value === routeConversationId) {
    return;
  }

  await workspace.selectConversation(routeConversationId);
}

function restoreTabFromRoute() {
  const routeTab =
    typeof route.query.tab === "string" && tabs.some((tab) => tab.id === route.query.tab)
      ? (route.query.tab as ProjectTab)
      : null;
  if (!routeTab) {
    return;
  }

  activeTab.value = routeTab;
}

onMounted(async () => {
  store.markProjectVisited(deviceId.value, workspace.cwd.value);
  await workspace.refreshProject();
  restoreTabFromRoute();
  await restoreConversationFromRoute();
});

watch(
  () => [route.params.deviceId, route.params.projectPath],
  async () => {
    store.markProjectVisited(deviceId.value, workspace.cwd.value);
    await workspace.refreshProject();
    restoreTabFromRoute();
    await restoreConversationFromRoute();
  }
);

watch(
  () => [route.query.conversationId, route.query.tab],
  async () => {
    restoreTabFromRoute();
    await restoreConversationFromRoute();
  }
);
</script>

<template>
  <section class="space-y-5">
    <div
      class="rounded-[1.9rem] border border-white/55 bg-white/80 p-5 shadow-[0_24px_70px_-35px_rgba(15,23,42,0.55)] backdrop-blur dark:border-white/10 dark:bg-slate-950/55"
    >
      <div class="flex flex-col gap-4 xl:flex-row xl:items-start xl:justify-between">
        <div class="space-y-3">
          <div class="flex flex-wrap items-center gap-2">
            <StatusBadge>{{ t("workspace.badge") }}</StatusBadge>
            <StatusBadge tone="muted">{{ project?.deviceName ?? deviceId }}</StatusBadge>
            <StatusBadge v-if="project" :tone="projectAvailabilityTone">
              {{ t(`projectCard.availability.${project.availabilityState}`) }}
            </StatusBadge>
          </div>
          <div>
            <h1 class="text-2xl font-semibold">{{ project?.title ?? t("workspace.title") }}</h1>
            <p class="mt-2 text-sm text-muted-foreground">
              {{ project?.pathLabel ?? t("workspace.loadingPath") }}
            </p>
          </div>
          <div class="flex flex-wrap gap-3 text-xs text-muted-foreground">
            <span>{{ t("workspace.metrics.topics", { count: project?.conversationCount ?? 0 }) }}</span>
            <span>{{ t("workspace.metrics.running", { count: project?.runningTaskCount ?? 0 }) }}</span>
            <span>{{ t("workspace.metrics.waiting", { count: project?.waitingInputCount ?? 0 }) }}</span>
            <span v-if="project?.branchName">{{ t("workspace.metrics.branch", { value: project.branchName }) }}</span>
            <span>{{ t("workspace.metrics.changedFiles", { count: project?.changedFilesCount ?? 0 }) }}</span>
            <span>{{ t("workspace.metrics.updated", { value: formatRelativeTime(project?.updatedAtEpochMs) }) }}</span>
          </div>
        </div>

        <div class="grid gap-3 text-sm xl:min-w-[260px]">
          <article class="rounded-2xl bg-background/75 p-4">
            <p class="text-xs uppercase tracking-[0.18em] text-muted-foreground">{{ t("workspace.currentState") }}</p>
            <p class="mt-2 font-semibold">
              {{
                project?.failedTaskCount
                  ? t("workspace.state.failed")
                  : project?.waitingInputCount
                    ? t("workspace.state.waiting")
                    : project?.runningTaskCount
                      ? t("workspace.state.running")
                      : t("workspace.state.ready")
              }}
            </p>
          </article>
          <button class="rounded-full border border-border px-4 py-2 text-sm font-medium" @click="workspace.refreshProject">
            {{ t("common.refresh") }}
          </button>
        </div>
      </div>
    </div>

    <div class="flex flex-wrap gap-2">
      <button
        v-for="tab in tabs"
        :key="tab.id"
        class="rounded-full border px-4 py-2 text-sm"
        :class="
          activeTab === tab.id
            ? 'border-primary bg-primary text-primary-foreground'
            : 'border-border bg-background/70'
        "
        @click="activeTab = tab.id"
      >
        {{ t(tab.key) }}
      </button>
    </div>

    <div
      v-if="errorMessage"
      class="rounded-[1.5rem] border border-rose-500/20 bg-rose-500/8 p-4 text-sm text-rose-700 dark:text-rose-300"
    >
      {{ errorMessage }}
    </div>

    <ProjectConversationPanel
      v-if="activeTab === 'conversation'"
      :detail="conversationDetail"
      :conversations="conversations"
      :active-conversation-id="activeConversationId"
      :project-providers="project?.providers"
      :project-title="project?.title"
      @select-conversation="workspace.selectConversation"
      @send-prompt="workspace.sendFollowUp"
      @respond-input="workspace.respondToInput"
      @cancel-task="workspace.cancelTask"
      @open-tab="activeTab = $event"
    />

    <ProjectChangesPanel
      v-else-if="activeTab === 'changes'"
      :git-inspect="gitInspect"
      :git-diff="gitDiff"
      :active-repo-path="activeDiffRepoPath"
      @select-file="workspace.selectChangeFile"
    />

    <ProjectFilesPanel
      v-else-if="activeTab === 'files'"
      :workspace="workspaceBrowse"
      :file-preview="filePreview"
      @open-entry="workspace.openEntry"
    />

    <ProjectLogsPanel v-else :detail="conversationDetail" />
  </section>
</template>
