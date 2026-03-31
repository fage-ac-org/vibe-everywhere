<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue";
import { useRoute, useRouter } from "vue-router";
import { useI18n } from "vue-i18n";
import StatusBadge from "@/components/common/StatusBadge.vue";
import { formatRelativeTime } from "@/lib/format";
import { buildProjectRouteParam } from "@/lib/projects";
import ProjectChangesPanel from "@/features/project/ProjectChangesPanel.vue";
import ProjectConversationPanel from "@/features/project/ProjectConversationPanel.vue";
import ProjectFilesPanel from "@/features/project/ProjectFilesPanel.vue";
import { useProjectWorkspace, type ProjectTab } from "@/features/project/useProjectWorkspace";
import { useAppStore } from "@/stores/app";

const route = useRoute();
const router = useRouter();
const store = useAppStore();
const { t } = useI18n();
const deviceId = computed(() => String(route.params.deviceId));
const projectPath = computed(() => route.params.projectPath);
const workspace = useProjectWorkspace(deviceId, projectPath);
const {
  project,
  conversations,
  activeConversationId,
  isDraftConversation,
  conversationDetail,
  gitInspect,
  gitDiff,
  activeDiffRepoPath,
  workspace: workspaceBrowse,
  filePreview,
  activeTab,
  errorMessage
} = workspace;
const sidebarOpen = ref(false);

const tabs: { id: ProjectTab; key: string }[] = [
  { id: "conversation", key: "workspace.tabs.conversation" },
  { id: "changes", key: "workspace.tabs.changes" },
  { id: "files", key: "workspace.tabs.files" }
];

const hostSummaries = computed(() => store.hostSummaries);
const activeHostProjects = computed(() =>
  store.projectSummaries.filter((entry) => entry.deviceId === deviceId.value)
);
const selectedProjectKey = computed(() => project.value?.key ?? "");
const selectedHostId = computed(() => deviceId.value);
const recentConversationList = computed(() => conversations.value.slice(0, 4));
const isSecondaryPanelOpen = computed(() => activeTab.value !== "conversation");

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

function syncRouteState() {
  const conversationId = activeConversationId.value ?? undefined;
  const tab = activeTab.value !== "conversation" ? activeTab.value : undefined;
  void router.replace({
    query: {
      ...route.query,
      conversationId,
      tab
    }
  });
}

function openProject(targetDeviceId: string, cwd: string | null) {
  store.markProjectVisited(targetDeviceId, cwd);
  sidebarOpen.value = false;
  void router.push({
    name: "project-workspace",
    params: {
      deviceId: targetDeviceId,
      projectPath: buildProjectRouteParam(cwd)
    },
    query: {}
  });
}

function switchHost(targetDeviceId: string) {
  const preferredPath = store.getPreferredProjectPath(targetDeviceId);
  const nextProject =
    store.findProject(targetDeviceId, preferredPath) ??
    store.projectSummaries.find((entry) => entry.deviceId === targetDeviceId) ??
    null;
  if (!nextProject) {
    return;
  }
  openProject(nextProject.deviceId, nextProject.cwd);
}

function switchProject(projectKey: string) {
  const nextProject = store.projectSummaries.find((entry) => entry.key === projectKey);
  if (!nextProject) {
    return;
  }
  openProject(nextProject.deviceId, nextProject.cwd);
}

function selectConversation(conversationId: string) {
  sidebarOpen.value = false;
  activeTab.value = "conversation";
  void workspace.selectConversation(conversationId);
}

function startNewConversation() {
  sidebarOpen.value = false;
  workspace.startNewConversation();
  activeTab.value = "conversation";
}

function onHostChange(event: Event) {
  const value = (event.target as HTMLSelectElement).value;
  switchHost(value);
}

function onProjectChange(event: Event) {
  const value = (event.target as HTMLSelectElement).value;
  switchProject(value);
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

watch([activeConversationId, activeTab], () => {
  syncRouteState();
});
</script>

<template>
  <section class="mx-auto flex min-h-screen w-full max-w-[1680px] gap-0 px-0 md:px-4 xl:px-6">
    <div
      v-if="sidebarOpen"
      class="fixed inset-0 z-30 bg-black/35 xl:hidden"
      @click="sidebarOpen = false"
    />

    <aside
      class="fixed inset-y-0 left-0 z-40 flex w-[88vw] max-w-[360px] flex-col border-r border-border/60 bg-[#f4f1ea]/95 px-4 py-5 shadow-[0_28px_80px_-40px_rgba(15,23,42,0.55)] backdrop-blur transition-transform dark:bg-slate-950/95 xl:sticky xl:top-0 xl:z-0 xl:h-screen xl:w-[320px] xl:max-w-none xl:translate-x-0 xl:border xl:border-white/10 xl:shadow-none"
      :class="sidebarOpen ? 'translate-x-0' : '-translate-x-full'"
    >
      <div class="space-y-4">
        <div class="flex items-center justify-between xl:hidden">
          <p class="text-xs font-semibold uppercase tracking-[0.24em] text-muted-foreground">
            {{ t("conversation.history") }}
          </p>
          <button class="rounded-full border border-border px-3 py-1 text-xs" @click="sidebarOpen = false">
            {{ t("common.close") }}
          </button>
        </div>

        <div class="space-y-3 rounded-[1.6rem] border border-white/60 bg-white/80 p-4 dark:border-white/10 dark:bg-slate-900/70">
          <div>
            <p class="text-[11px] font-semibold uppercase tracking-[0.18em] text-muted-foreground">
              {{ t("workspace.currentHost") }}
            </p>
            <select
              :value="selectedHostId"
              class="mt-2 w-full rounded-2xl border border-border bg-background px-3 py-2.5 text-sm"
              @change="onHostChange"
            >
              <option
                v-for="host in hostSummaries"
                :key="host.device.id"
                :value="host.device.id"
              >
                {{ host.device.name }}
              </option>
            </select>
          </div>

          <div>
            <p class="text-[11px] font-semibold uppercase tracking-[0.18em] text-muted-foreground">
              {{ t("workspace.currentProject") }}
            </p>
            <select
              :value="selectedProjectKey"
              class="mt-2 w-full rounded-2xl border border-border bg-background px-3 py-2.5 text-sm"
              @change="onProjectChange"
            >
              <option
                v-for="entry in activeHostProjects"
                :key="entry.key"
                :value="entry.key"
              >
                {{ entry.title }}
              </option>
            </select>
          </div>
        </div>

        <button
          class="flex w-full items-center justify-between rounded-[1.3rem] border border-border bg-background px-4 py-3 text-left text-sm font-medium"
          @click="startNewConversation"
        >
          <span>{{ t("conversation.newChat") }}</span>
          <span class="text-muted-foreground">+</span>
        </button>

        <div class="space-y-2">
          <p class="px-1 text-[11px] font-semibold uppercase tracking-[0.18em] text-muted-foreground">
            {{ t("conversation.recentChats") }}
          </p>
          <button
            v-for="conversation in recentConversationList"
            :key="conversation.id"
            class="w-full rounded-[1.3rem] border px-4 py-3 text-left"
            :class="
              conversation.id === activeConversationId && !isDraftConversation
                ? 'border-primary bg-primary/10'
                : 'border-border bg-background/70'
            "
            @click="selectConversation(conversation.id)"
          >
            <p class="truncate text-sm font-medium">{{ conversation.title }}</p>
            <p class="mt-1 text-xs text-muted-foreground">
              {{ formatRelativeTime(conversation.updatedAtEpochMs) }}
            </p>
          </button>
          <div
            v-if="!recentConversationList.length"
            class="rounded-[1.3rem] border border-dashed border-border bg-background/60 px-4 py-5 text-sm text-muted-foreground"
          >
            {{ t("conversation.emptyTitle") }}
          </div>
        </div>
      </div>

      <div class="mt-auto">
        <button
          class="flex w-full items-center justify-between rounded-[1.3rem] border border-border bg-background px-4 py-3 text-left text-sm font-medium"
          @click="router.push({ name: 'settings' })"
        >
          <span>{{ t("nav.settings") }}</span>
          <span class="text-xs text-muted-foreground">{{ store.activeServerLabel }}</span>
        </button>
      </div>
    </aside>

    <div class="flex min-h-screen min-w-0 flex-1 flex-col px-4 py-4 md:px-6 xl:pl-6">
      <header class="sticky top-0 z-20 rounded-[1.6rem] border border-white/60 bg-white/85 px-4 py-4 shadow-[0_20px_60px_-40px_rgba(15,23,42,0.55)] backdrop-blur dark:border-white/10 dark:bg-slate-950/75">
        <div class="flex items-start justify-between gap-4">
          <div class="min-w-0">
            <div class="flex items-center gap-2">
              <button
                class="rounded-full border border-border px-3 py-2 text-sm xl:hidden"
                @click="sidebarOpen = true"
              >
                {{ t("common.menu") }}
              </button>
              <StatusBadge tone="muted">{{ project?.deviceName ?? deviceId }}</StatusBadge>
              <StatusBadge v-if="project" :tone="projectAvailabilityTone">
                {{ t(`projectCard.availability.${project.availabilityState}`) }}
              </StatusBadge>
            </div>
            <h1 class="mt-3 truncate text-2xl font-semibold">
              {{ project?.title ?? t("workspace.title") }}
            </h1>
            <p class="mt-1 truncate text-sm text-muted-foreground">
              {{ project?.pathLabel ?? t("workspace.loadingPath") }}
            </p>
          </div>

          <div class="flex flex-wrap justify-end gap-2">
            <button
              v-for="tab in tabs.filter((entry) => entry.id !== 'conversation')"
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
            <button class="rounded-full border border-border px-4 py-2 text-sm" @click="() => workspace.refreshProject()">
              {{ t("common.refresh") }}
            </button>
          </div>
        </div>

        <div class="mt-4 flex flex-wrap gap-3 text-xs text-muted-foreground">
          <span>{{ t("workspace.metrics.topics", { count: project?.conversationCount ?? 0 }) }}</span>
          <span>{{ t("workspace.metrics.running", { count: project?.runningTaskCount ?? 0 }) }}</span>
          <span>{{ t("workspace.metrics.waiting", { count: project?.waitingInputCount ?? 0 }) }}</span>
          <span v-if="project?.branchName">{{ t("workspace.metrics.branch", { value: project.branchName }) }}</span>
          <span>{{ t("workspace.metrics.changedFiles", { count: project?.changedFilesCount ?? 0 }) }}</span>
          <span>{{ t("workspace.metrics.updated", { value: formatRelativeTime(project?.updatedAtEpochMs) }) }}</span>
        </div>
      </header>

      <div
        v-if="errorMessage"
        class="mt-4 rounded-[1.4rem] border border-rose-500/20 bg-rose-500/8 p-4 text-sm text-rose-700 dark:text-rose-300"
      >
        {{ errorMessage }}
      </div>

      <ProjectConversationPanel
        class="mt-4"
        :detail="conversationDetail"
        :project-providers="project?.providers"
        :project-title="project?.title"
        :is-draft-conversation="isDraftConversation"
        :is-restoring-conversation="workspace.isRestoringConversation.value"
        @send-prompt="workspace.sendFollowUp"
        @respond-input="workspace.respondToInput"
        @cancel-task="workspace.cancelTask"
        @open-tab="activeTab = $event"
      />
    </div>

    <div
      v-if="isSecondaryPanelOpen"
      class="fixed inset-0 z-50 bg-black/40 px-3 py-3 md:px-6 md:py-6"
      @click.self="activeTab = 'conversation'"
    >
      <div class="mx-auto flex h-full max-w-6xl flex-col overflow-hidden rounded-[1.8rem] border border-white/60 bg-[#f8f7f3] shadow-[0_28px_90px_-45px_rgba(15,23,42,0.6)] dark:border-white/10 dark:bg-slate-950">
        <div class="flex items-center justify-between border-b border-border/70 px-5 py-4">
          <div>
            <p class="text-xs font-semibold uppercase tracking-[0.18em] text-muted-foreground">
              {{ activeTab === 'changes' ? t('workspace.tabs.changes') : t('workspace.tabs.files') }}
            </p>
            <p class="text-sm text-muted-foreground">{{ project?.title ?? t("workspace.title") }}</p>
          </div>
          <button class="rounded-full border border-border px-4 py-2 text-sm" @click="activeTab = 'conversation'">
            {{ t("common.close") }}
          </button>
        </div>

        <div class="flex-1 overflow-auto p-4 md:p-5">
          <ProjectChangesPanel
            v-if="activeTab === 'changes'"
            :git-inspect="gitInspect"
            :git-diff="gitDiff"
            :active-repo-path="activeDiffRepoPath"
            @select-file="workspace.selectChangeFile"
          />

          <ProjectFilesPanel
            v-else
            :workspace="workspaceBrowse"
            :file-preview="filePreview"
            @open-entry="workspace.openEntry"
          />
        </div>
      </div>
    </div>
  </section>
</template>
