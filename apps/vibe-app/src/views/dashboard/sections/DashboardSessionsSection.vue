<script setup lang="ts">
import { useI18n } from "vue-i18n"
import {
  Activity,
  BellRing,
  FolderCode,
  GitBranch,
  RefreshCw,
  Server,
  Sparkles
} from "lucide-vue-next"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle
} from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { ScrollArea } from "@/components/ui/scroll-area"
import { Separator } from "@/components/ui/separator"
import { Textarea } from "@/components/ui/textarea"
import { useDashboardController } from "@/views/dashboard/controller"

const { t } = useI18n()
const dashboard = useDashboardController()

const {
  TASK_SCOPE_OPTIONS,
  TASK_STATUS_OPTIONS,
  activityItems,
  activities,
  aiSessions,
  activitySeverityClass,
  appConfig,
  appName,
  canCancel,
  canPreviewGitChangedFile,
  canSubmit,
  currentClientKind,
  deviceSessionCount,
  deviceShellCount,
  devices,
  draft,
  eventKindClass,
  eventState,
  formatAuthRequirement,
  formatConnectionState,
  formatControlClient,
  formatDevicePresence,
  formatEventKind,
  formatGitDrift,
  formatGitFileStatus,
  formatProviderSummary,
  formatScopeOption,
  formatTaskStatus,
  formatTaskStatusOption,
  formatTimestamp,
  gitError,
  gitInspect,
  gitLoading,
  health,
  locale,
  localeOptions,
  localizedErrorMessage,
  nativeSelectClass,
  openGitChangedFile,
  openWorkspaceEntry,
  providerBadgeClass,
  refreshGitInspect,
  refreshWorkspace,
  relayAccessTokenInput,
  relayInput,
  relayPlaceholder,
  selectedDevice,
  selectedDeviceAvailableProviderCount,
  selectedDeviceAvailableProviders,
  selectedDevicePreviewCount,
  selectedDeviceSessionCount,
  selectedDeviceShellCount,
  selectedDeviceSupportsGitInspect,
  selectedDeviceSupportsWorkspace,
  selectedDeviceUnavailableProviders,
  selectedDeviceWorkingRoot,
  selectedStateClass,
  selectedTask,
  selectedTaskDetail,
  selectedWorkspacePath,
  sessionEventCounts,
  sessionLaunchState,
  showGitInspect,
  showMobileRelayHint,
  statusBadgeClass,
  store,
  supervisionSummary,
  switchLocale,
  switchTheme,
  taskScope,
  taskStatusClass,
  taskStatusFilter,
  tasks,
  themeMode,
  themeOptions,
  unreadActivityCount,
  visibleChangedFileCount,
  workspaceError,
  workspaceListing,
  workspaceLoading,
  workspacePreview,
  workspacePreviewLoading,
  navigateWorkspaceUp
} = dashboard
</script>

<template>
  <section class="space-y-4">
    <Card class="overflow-hidden border-border/70 bg-card/85 text-foreground shadow-2xl backdrop-blur-xl">
      <CardContent class="grid gap-6 p-6 xl:grid-cols-[1.2fr_1fr]">
        <div class="space-y-5">
          <Badge variant="outline" class="border-sky-500/30 bg-sky-500/12 text-sky-700 dark:text-sky-100">
            <Sparkles class="size-3.5" />
            {{ t("dashboard.sessions.primaryBadge") }}
          </Badge>

          <div class="space-y-3">
            <h2 class="text-4xl font-semibold tracking-tight text-foreground md:text-5xl">
              {{ appName }}
            </h2>
            <p class="max-w-3xl text-base leading-7 text-muted-foreground md:text-lg">
              {{ t("dashboard.sessions.primaryDescription") }}
            </p>
          </div>

          <div class="flex flex-wrap gap-2">
            <Badge variant="outline" class="border-border/70 bg-background/55 text-foreground">
              1. {{ t("dashboard.sessions.steps.connect") }}
            </Badge>
            <Badge variant="outline" class="border-border/70 bg-background/55 text-foreground">
              2. {{ t("dashboard.sessions.steps.chooseDevice") }}
            </Badge>
            <Badge variant="outline" class="border-border/70 bg-background/55 text-foreground">
              3. {{ t("dashboard.sessions.steps.start") }}
            </Badge>
            <Badge variant="outline" class="border-border/70 bg-background/55 text-foreground">
              4. {{ t("dashboard.sessions.steps.review") }}
            </Badge>
          </div>

          <div class="grid gap-3 sm:grid-cols-3">
            <div class="rounded-2xl border border-border/70 bg-background/55 p-4">
              <p class="text-xs uppercase tracking-[0.16em] text-muted-foreground">
                {{ t("dashboard.shell.connectionState") }}
              </p>
              <div class="mt-2 flex items-center gap-2 text-sm font-medium text-foreground">
                <Activity class="size-4 text-amber-400" />
                {{ formatConnectionState(eventState) }}
              </div>
            </div>
            <div class="rounded-2xl border border-border/70 bg-background/55 p-4">
              <p class="text-xs uppercase tracking-[0.16em] text-muted-foreground">
                {{ t("dashboard.shell.selectedDevice") }}
              </p>
              <p class="mt-2 text-sm font-medium text-foreground">
                {{ selectedDevice?.name ?? t("dashboard.shell.noDeviceSelected") }}
              </p>
            </div>
            <div class="rounded-2xl border border-border/70 bg-background/55 p-4">
              <p class="text-xs uppercase tracking-[0.16em] text-muted-foreground">
                {{ t("dashboard.deployment.currentClient") }}
              </p>
              <p class="mt-2 text-sm font-medium text-foreground">
                {{ formatControlClient(currentClientKind) }}
              </p>
            </div>
          </div>
        </div>

        <div class="space-y-4 rounded-3xl border border-amber-400/20 bg-gradient-to-b from-amber-400/12 via-background/70 to-transparent p-5 shadow-inner">
          <div class="space-y-3">
            <div class="space-y-2">
              <label class="text-xs font-medium uppercase tracking-[0.18em] text-muted-foreground">
                {{ t("dashboard.relayBaseUrl") }}
              </label>
              <div class="flex flex-col gap-2 sm:flex-row">
                <Input
                  v-model="relayInput"
                  :placeholder="relayPlaceholder"
                  class="border-border/70 bg-background/70 text-foreground placeholder:text-muted-foreground"
                />
                <Button class="sm:min-w-28" @click="store.applyRelayBaseUrl">
                  {{ t("common.connect") }}
                </Button>
              </div>
            </div>

            <div class="space-y-2">
              <label class="text-xs font-medium uppercase tracking-[0.18em] text-muted-foreground">
                {{ t("dashboard.fields.accessToken") }}
              </label>
              <div class="flex flex-col gap-2 sm:flex-row sm:items-center">
                <Input
                  v-model="relayAccessTokenInput"
                  type="password"
                  :placeholder="t('common.optionalAccessToken')"
                  class="border-border/70 bg-background/70 text-foreground placeholder:text-muted-foreground"
                />
                <span class="text-xs text-muted-foreground">
                  {{ formatAuthRequirement(appConfig?.requiresAuth) }}
                </span>
              </div>
            </div>

            <div v-if="showMobileRelayHint" class="rounded-2xl border border-amber-500/30 bg-amber-500/10 p-4 text-sm text-amber-700 dark:text-amber-100">
              {{ t("dashboard.mobileRemoteHint") }}
            </div>
          </div>

          <Separator class="bg-border/70" />

          <div class="grid gap-4 lg:grid-cols-2">
            <div class="space-y-2">
              <label class="text-xs font-medium uppercase tracking-[0.18em] text-muted-foreground">
                {{ t("locale.label") }}
              </label>
              <div class="flex flex-wrap gap-2">
                <Button
                  v-for="option in localeOptions"
                  :key="option.value"
                  type="button"
                  variant="outline"
                  size="sm"
                  :class="
                    option.value === locale
                      ? 'border-sky-400/40 bg-sky-400/15 text-sky-900 hover:bg-sky-400/20 dark:text-white'
                      : 'border-border/70 bg-background/55 text-foreground hover:bg-accent/70'
                  "
                  @click="switchLocale(option.value)"
                >
                  {{ option.label }}
                </Button>
              </div>
            </div>

            <div class="space-y-2">
              <label class="text-xs font-medium uppercase tracking-[0.18em] text-muted-foreground">
                {{ t("theme.label") }}
              </label>
              <div class="flex flex-wrap gap-2">
                <Button
                  v-for="option in themeOptions"
                  :key="option.value"
                  type="button"
                  variant="outline"
                  size="sm"
                  :class="
                    option.value === themeMode
                      ? 'border-emerald-400/40 bg-emerald-400/15 text-emerald-900 hover:bg-emerald-400/20 dark:text-emerald-50'
                      : 'border-border/70 bg-background/55 text-foreground hover:bg-accent/70'
                  "
                  @click="switchTheme(option.value)"
                >
                  {{ option.label }}
                </Button>
              </div>
            </div>
          </div>

          <div class="grid grid-cols-2 gap-3 text-sm sm:grid-cols-4">
            <div class="rounded-2xl border border-border/70 bg-background/55 p-3">
              <p class="text-xs uppercase tracking-[0.18em] text-muted-foreground">
                {{ t("dashboard.stats.onlineDevices") }}
              </p>
              <p class="mt-2 text-2xl font-semibold text-foreground">
                {{ health?.onlineDeviceCount ?? 0 }}
              </p>
            </div>
            <div class="rounded-2xl border border-border/70 bg-background/55 p-3">
              <p class="text-xs uppercase tracking-[0.18em] text-muted-foreground">
                {{ t("dashboard.stats.devices") }}
              </p>
              <p class="mt-2 text-2xl font-semibold text-foreground">
                {{ health?.deviceCount ?? 0 }}
              </p>
            </div>
            <div class="rounded-2xl border border-border/70 bg-background/55 p-3">
              <p class="text-xs uppercase tracking-[0.18em] text-muted-foreground">
                {{ t("dashboard.stats.aiSessions") }}
              </p>
              <p class="mt-2 text-2xl font-semibold text-foreground">
                {{ tasks.length }}
              </p>
            </div>
            <div class="rounded-2xl border border-border/70 bg-background/55 p-3">
              <p class="text-xs uppercase tracking-[0.18em] text-muted-foreground">
                {{ t("dashboard.stats.unreadActivity") }}
              </p>
              <p class="mt-2 text-2xl font-semibold text-foreground">
                {{ unreadActivityCount }}
              </p>
            </div>
          </div>
        </div>
      </CardContent>
    </Card>

    <Card
      v-if="localizedErrorMessage"
      class="border-rose-500/25 bg-rose-500/10 text-rose-700 shadow-none dark:text-rose-100"
    >
      <CardContent class="p-4">
        <p class="text-sm">{{ localizedErrorMessage }}</p>
      </CardContent>
    </Card>

    <section class="grid gap-4 xl:grid-cols-[340px_minmax(0,1fr)]">
      <div class="space-y-4">
        <Card class="border-border/70 bg-card/80 shadow-xl backdrop-blur-xl">
          <CardHeader class="flex flex-row items-start justify-between gap-4 space-y-0">
            <div class="space-y-1">
              <CardTitle class="flex items-center gap-2 text-foreground">
                <Server class="size-4 text-sky-300" />
                {{ t("dashboard.sessions.devicePickerTitle") }}
              </CardTitle>
              <CardDescription>
                {{ t("dashboard.sessions.devicePickerDescription") }}
              </CardDescription>
            </div>
            <Button
              variant="outline"
              size="sm"
              class="border-border/70 bg-background/55 text-foreground hover:bg-accent/70"
              @click="store.reloadAll"
            >
              <RefreshCw class="size-4" />
              {{ t("common.refresh") }}
            </Button>
          </CardHeader>
          <CardContent class="pt-0">
            <ScrollArea class="h-[26rem] pr-3">
              <div class="space-y-3">
                <button
                  v-for="device in devices"
                  :key="device.id"
                  type="button"
                  class="w-full rounded-2xl border p-4 text-left transition-colors"
                  :class="selectedStateClass(selectedDevice?.id === device.id)"
                  @click="store.selectDevice(device.id)"
                >
                  <div class="flex items-start justify-between gap-3">
                    <div class="space-y-1">
                      <p class="font-medium text-foreground">{{ device.name }}</p>
                      <p class="text-sm text-muted-foreground">
                        {{ device.platform }} · {{ device.metadata.arch }}
                      </p>
                    </div>
                    <Badge
                      variant="outline"
                      :class="device.online ? statusBadgeClass('online') : statusBadgeClass('offline')"
                    >
                      {{ formatDevicePresence(device.online) }}
                    </Badge>
                  </div>

                  <p class="mt-3 font-mono text-xs text-muted-foreground">
                    {{ device.metadata.workingRoot ?? t("common.useAgentWorkingRoot") }}
                  </p>

                  <div class="mt-3 flex flex-wrap gap-2">
                    <Badge variant="outline" class="border-border/70 bg-background/55 text-foreground">
                      {{ t("dashboard.devices.sessions", { count: deviceSessionCount(device.id) }) }}
                    </Badge>
                    <Badge variant="outline" class="border-border/70 bg-background/55 text-foreground">
                      {{ t("dashboard.devices.terminals", { count: deviceShellCount(device.id) }) }}
                    </Badge>
                  </div>

                  <div class="mt-3 flex flex-wrap gap-2">
                    <Badge
                      v-for="provider in device.providers"
                      :key="provider.kind"
                      variant="outline"
                      :class="providerBadgeClass(provider.available)"
                    >
                      {{ formatProviderSummary(provider.kind, provider.executionProtocol) }}
                    </Badge>
                  </div>
                </button>
              </div>
            </ScrollArea>
          </CardContent>
        </Card>

        <Card class="border-border/70 bg-card/80 shadow-xl backdrop-blur-xl">
          <CardHeader class="space-y-4">
            <div class="flex flex-row items-start justify-between gap-4 space-y-0">
              <div class="space-y-1">
                <CardTitle class="flex items-center gap-2 text-foreground">
                  <Sparkles class="size-4 text-amber-300" />
                  {{ t("dashboard.sessions.recentTitle") }}
                </CardTitle>
                <CardDescription>
                  {{
                    t("dashboard.sessions.visibleSummary", {
                      visible: aiSessions.length,
                      total: tasks.length
                    })
                  }}
                </CardDescription>
              </div>
              <Button
                variant="outline"
                size="sm"
                class="border-border/70 bg-background/55 text-foreground hover:bg-accent/70"
                :disabled="!canCancel"
                @click="store.cancelSelectedTask"
              >
                {{ t("common.cancel") }}
              </Button>
            </div>

            <div class="grid gap-3 sm:grid-cols-2">
              <label class="grid gap-2">
                <span class="text-xs font-medium uppercase tracking-[0.16em] text-muted-foreground">
                  {{ t("common.scope") }}
                </span>
                <select v-model="taskScope" :class="nativeSelectClass">
                  <option v-for="option in TASK_SCOPE_OPTIONS" :key="option" :value="option">
                    {{ formatScopeOption(option) }}
                  </option>
                </select>
              </label>

              <label class="grid gap-2">
                <span class="text-xs font-medium uppercase tracking-[0.16em] text-muted-foreground">
                  {{ t("common.status") }}
                </span>
                <select v-model="taskStatusFilter" :class="nativeSelectClass">
                  <option v-for="option in TASK_STATUS_OPTIONS" :key="option" :value="option">
                    {{ formatTaskStatusOption(option) }}
                  </option>
                </select>
              </label>
            </div>
          </CardHeader>
          <CardContent class="pt-0">
            <p v-if="!aiSessions.length" class="mb-3 text-sm text-muted-foreground">
              {{ t("dashboard.sessions.empty") }}
            </p>
            <ScrollArea class="h-[22rem] pr-3">
              <div class="space-y-3">
                <button
                  v-for="task in aiSessions"
                  :key="task.id"
                  type="button"
                  class="w-full rounded-2xl border p-4 text-left transition-colors"
                  :class="selectedStateClass(selectedTask?.id === task.id)"
                  @click="store.selectTask(task.id)"
                >
                  <div class="flex items-start justify-between gap-3">
                    <div class="space-y-1">
                      <p class="font-medium text-foreground">{{ task.title }}</p>
                      <p class="text-sm text-muted-foreground">
                        {{ formatProviderSummary(task.provider, task.executionProtocol) }}
                      </p>
                    </div>
                    <Badge variant="outline" :class="taskStatusClass(task.status)">
                      {{ formatTaskStatus(task.status) }}
                    </Badge>
                  </div>

                  <p class="mt-3 font-mono text-xs text-muted-foreground">
                    {{ task.cwd ?? t("common.useAgentWorkingRoot") }}
                  </p>
                  <p class="mt-2 text-xs text-muted-foreground">
                    {{ formatTimestamp(task.startedAtEpochMs ?? task.createdAtEpochMs) }}
                  </p>
                </button>
              </div>
            </ScrollArea>
          </CardContent>
        </Card>

        <Card class="border-border/70 bg-card/80 shadow-xl backdrop-blur-xl">
          <CardHeader class="flex flex-row items-start justify-between gap-4 space-y-0">
            <div class="space-y-1">
              <CardTitle class="flex items-center gap-2 text-foreground">
                <BellRing class="size-4 text-sky-300" />
                {{ t("dashboard.activity.title") }}
              </CardTitle>
              <CardDescription>
                {{
                  t("dashboard.activity.summary", {
                    unread: unreadActivityCount,
                    total: activities.length
                  })
                }}
              </CardDescription>
            </div>
            <Button
              variant="outline"
              size="sm"
              class="border-border/70 bg-background/55 text-foreground hover:bg-accent/70"
              :disabled="!activities.length"
              @click="store.markAllActivitiesRead"
            >
              {{ t("dashboard.activity.markAllRead") }}
            </Button>
          </CardHeader>
          <CardContent class="pt-0">
            <div
              v-if="!activityItems.length"
              class="rounded-2xl border border-dashed border-border/70 bg-background/55 p-4 text-sm text-muted-foreground"
            >
              {{ t("dashboard.activity.empty") }}
            </div>
            <ScrollArea v-else class="h-[18rem] pr-3">
              <div class="space-y-3">
                <button
                  v-for="activity in activityItems"
                  :key="activity.id"
                  type="button"
                  class="w-full rounded-2xl border border-border/70 bg-background/55 p-4 text-left transition hover:bg-accent/30"
                  @click="store.openActivity(activity.id)"
                >
                  <div class="flex flex-col gap-3">
                    <div class="flex flex-wrap items-center gap-2">
                      <Badge variant="outline" :class="activitySeverityClass(activity.severity)">
                        {{ activity.title }}
                      </Badge>
                      <Badge
                        v-if="activity.unread"
                        variant="outline"
                        class="border-sky-500/30 bg-sky-500/10 text-sky-700 dark:text-sky-100"
                      >
                        {{ t("dashboard.activity.unread") }}
                      </Badge>
                    </div>
                    <p class="text-sm text-foreground">{{ activity.description }}</p>
                    <span class="text-xs text-muted-foreground">
                      {{ formatTimestamp(activity.timestampEpochMs) }}
                    </span>
                  </div>
                </button>
              </div>
            </ScrollArea>
          </CardContent>
        </Card>
      </div>

      <div class="space-y-4">
        <Card class="border-border/70 bg-card/80 shadow-xl backdrop-blur-xl">
          <CardHeader class="space-y-4">
            <div class="flex flex-col gap-3 lg:flex-row lg:items-start lg:justify-between">
              <div class="space-y-1">
                <CardTitle class="flex items-center gap-2 text-foreground">
                  <FolderCode class="size-4 text-emerald-300" />
                  {{ t("dashboard.sessions.launchTitle") }}
                </CardTitle>
                <CardDescription>{{ t("dashboard.sessions.launchDescription") }}</CardDescription>
              </div>
              <Button :disabled="!canSubmit" @click="store.submitTask">
                {{ t("dashboard.sessions.start") }}
              </Button>
            </div>

            <div v-if="selectedDevice" class="grid gap-3 md:grid-cols-3">
              <div class="rounded-2xl border border-border/70 bg-background/55 p-4">
                <p class="text-xs uppercase tracking-[0.16em] text-muted-foreground">
                  {{ t("dashboard.workspace.device") }}
                </p>
                <p class="mt-2 text-sm font-medium text-foreground">{{ selectedDevice.name }}</p>
                <p class="mt-1 text-xs text-muted-foreground">
                  {{ selectedDevice.platform }} · {{ selectedDevice.metadata.arch }}
                </p>
              </div>
              <div class="rounded-2xl border border-border/70 bg-background/55 p-4">
                <p class="text-xs uppercase tracking-[0.16em] text-muted-foreground">
                  {{ t("dashboard.workspace.workingRoot") }}
                </p>
                <p class="mt-2 font-mono text-xs text-foreground">{{ selectedDeviceWorkingRoot }}</p>
                <p class="mt-1 text-xs text-muted-foreground">
                  {{ t("dashboard.workspace.relativePathHint") }}
                </p>
              </div>
              <div class="rounded-2xl border border-border/70 bg-background/55 p-4">
                <p class="text-xs uppercase tracking-[0.16em] text-muted-foreground">
                  {{ t("dashboard.workspace.deviceCapacity") }}
                </p>
                <p class="mt-2 text-sm font-medium text-foreground">
                  {{
                    t("dashboard.workspace.providers", {
                      count: selectedDeviceAvailableProviderCount
                    })
                  }}
                </p>
                <p class="mt-1 text-xs text-muted-foreground">
                  {{
                    t("dashboard.workspace.capacitySummary", {
                      sessions: selectedDeviceSessionCount,
                      terminals: selectedDeviceShellCount,
                      previews: selectedDevicePreviewCount
                    })
                  }}
                </p>
              </div>
            </div>
          </CardHeader>

          <CardContent class="space-y-5 pt-0">
            <div
              class="rounded-2xl border p-4 text-sm"
              :class="
                sessionLaunchState === 'ready'
                  ? 'border-emerald-500/30 bg-emerald-500/10 text-emerald-700 dark:text-emerald-100'
                  : 'border-amber-500/30 bg-amber-500/10 text-amber-700 dark:text-amber-100'
              "
            >
              {{
                t(`dashboard.sessions.launchState.${sessionLaunchState}`, {
                  device: selectedDevice?.name ?? t('dashboard.shell.noDeviceSelected')
                })
              }}
            </div>

            <div v-if="selectedDeviceAvailableProviders.length" class="flex flex-wrap gap-2">
              <Badge
                v-for="provider in selectedDeviceAvailableProviders"
                :key="provider.kind"
                variant="outline"
                class="border-sky-500/30 bg-sky-500/12 text-sky-700 dark:text-sky-100"
              >
                {{ formatProviderSummary(provider.kind, provider.executionProtocol) }}
              </Badge>
            </div>

            <div
              v-if="selectedDeviceUnavailableProviders.length"
              class="rounded-2xl border border-amber-500/30 bg-amber-500/10 p-4"
            >
              <p class="text-sm font-medium text-amber-700 dark:text-amber-100">
                {{ t("dashboard.sessions.providerIssuesTitle") }}
              </p>
              <div class="mt-3 space-y-3">
                <div
                  v-for="provider in selectedDeviceUnavailableProviders"
                  :key="provider.kind"
                  class="rounded-2xl border border-amber-500/20 bg-background/40 p-3"
                >
                  <p class="text-sm font-medium text-foreground">
                    {{ formatProviderSummary(provider.kind, provider.executionProtocol) }}
                  </p>
                  <p class="mt-1 text-xs text-muted-foreground">
                    {{ provider.error ?? t("dashboard.devices.providerVersionPending") }}
                  </p>
                </div>
              </div>
            </div>

            <div class="grid gap-4 md:grid-cols-2">
              <label class="grid gap-2">
                <span class="text-xs font-medium uppercase tracking-[0.16em] text-muted-foreground">
                  {{ t("dashboard.fields.provider") }}
                </span>
                <select v-model="draft.provider" :class="nativeSelectClass">
                  <option disabled value="">{{ t("dashboard.placeholders.selectProvider") }}</option>
                  <option
                    v-for="provider in store.availableProviders"
                    :key="provider.kind"
                    :value="provider.kind"
                  >
                    {{ provider.label }}
                  </option>
                </select>
              </label>

              <label class="grid gap-2">
                <span class="text-xs font-medium uppercase tracking-[0.16em] text-muted-foreground">
                  {{ t("dashboard.fields.title") }}
                </span>
                <Input v-model="draft.title" :placeholder="t('dashboard.placeholders.sessionTitle')" />
              </label>

              <label class="grid gap-2">
                <span class="text-xs font-medium uppercase tracking-[0.16em] text-muted-foreground">
                  {{ t("dashboard.fields.sessionCwd") }}
                </span>
                <Input v-model="draft.cwd" :placeholder="t('dashboard.placeholders.sessionCwd')" />
              </label>

              <label class="grid gap-2">
                <span class="text-xs font-medium uppercase tracking-[0.16em] text-muted-foreground">
                  {{ t("dashboard.fields.model") }}
                </span>
                <Input v-model="draft.model" :placeholder="t('dashboard.placeholders.model')" />
              </label>

              <label class="grid gap-2 md:col-span-2">
                <span class="text-xs font-medium uppercase tracking-[0.16em] text-muted-foreground">
                  {{ t("dashboard.fields.prompt") }}
                </span>
                <Textarea
                  v-model="draft.prompt"
                  class="min-h-44"
                  :placeholder="t('dashboard.placeholders.prompt')"
                />
              </label>
            </div>
          </CardContent>
        </Card>

        <Card
          v-if="selectedTaskDetail"
          class="border-border/70 bg-card/80 shadow-xl backdrop-blur-xl"
        >
          <CardHeader class="space-y-4">
            <div class="flex flex-col gap-3 lg:flex-row lg:items-start lg:justify-between">
              <div class="space-y-1">
                <CardTitle class="flex items-center gap-2 text-foreground">
                  <Sparkles class="size-4 text-amber-300" />
                  {{ t("dashboard.sessions.reviewTitle") }}
                </CardTitle>
                <CardDescription>{{ t("dashboard.sessions.reviewDescription") }}</CardDescription>
              </div>
              <Button
                variant="outline"
                class="border-border/70 bg-background/55 text-foreground hover:bg-accent/70"
                :disabled="!canCancel"
                @click="store.cancelSelectedTask"
              >
                {{ t("dashboard.sessions.cancelSession") }}
              </Button>
            </div>

            <div class="grid gap-3 md:grid-cols-3">
              <div class="rounded-2xl border border-border/70 bg-background/55 p-4">
                <p class="text-xs uppercase tracking-[0.16em] text-muted-foreground">
                  {{ t("common.status") }}
                </p>
                <div class="mt-2 flex items-center gap-2">
                  <Badge variant="outline" :class="taskStatusClass(selectedTaskDetail.task.status)">
                    {{ formatTaskStatus(selectedTaskDetail.task.status) }}
                  </Badge>
                </div>
                <p class="mt-2 text-xs text-muted-foreground">
                  {{ t("common.started") }}
                  {{ formatTimestamp(selectedTaskDetail.task.startedAtEpochMs ?? selectedTaskDetail.task.createdAtEpochMs) }}
                </p>
              </div>

              <div class="rounded-2xl border border-border/70 bg-background/55 p-4">
                <p class="text-xs uppercase tracking-[0.16em] text-muted-foreground">
                  {{ t("dashboard.workspace.workingDirectory") }}
                </p>
                <p class="mt-2 font-mono text-xs text-foreground">
                  {{ selectedTaskDetail.task.cwd ?? selectedDeviceWorkingRoot }}
                </p>
                <p class="mt-2 text-xs text-muted-foreground">
                  {{ formatProviderSummary(selectedTaskDetail.task.provider, selectedTaskDetail.task.executionProtocol) }}
                  · {{ selectedTaskDetail.task.model ?? t("common.defaultModel") }}
                </p>
              </div>

              <div class="rounded-2xl border border-border/70 bg-background/55 p-4">
                <p class="text-xs uppercase tracking-[0.16em] text-muted-foreground">
                  {{ t("dashboard.workspace.sessionMetrics") }}
                </p>
                <p class="mt-2 text-sm font-medium text-foreground">
                  {{
                    t("dashboard.workspace.exitCode", {
                      code: selectedTaskDetail.task.exitCode ?? t("common.pending")
                    })
                  }}
                </p>
                <p class="mt-2 text-xs text-muted-foreground">
                  {{
                    t("dashboard.workspace.eventsSummary", {
                      count: selectedTaskDetail.events.length,
                      deviceId: selectedTaskDetail.task.deviceId
                    })
                  }}
                </p>
              </div>
            </div>
          </CardHeader>

          <CardContent class="space-y-5 pt-0">
            <div class="rounded-2xl border border-sky-500/30 bg-sky-500/10 p-4 text-sm text-sky-700 dark:text-sky-100">
              {{ supervisionSummary }}
            </div>

            <div class="grid gap-3 md:grid-cols-4">
              <div class="rounded-2xl border border-border/70 bg-background/55 p-4">
                <p class="text-xs uppercase tracking-[0.16em] text-muted-foreground">
                  {{ t("dashboard.workspace.supervision.counts.assistant") }}
                </p>
                <p class="mt-2 text-sm font-medium text-foreground">
                  {{ sessionEventCounts.assistant }}
                </p>
              </div>
              <div class="rounded-2xl border border-border/70 bg-background/55 p-4">
                <p class="text-xs uppercase tracking-[0.16em] text-muted-foreground">
                  {{ t("dashboard.workspace.supervision.counts.tool") }}
                </p>
                <p class="mt-2 text-sm font-medium text-foreground">
                  {{ sessionEventCounts.tool }}
                </p>
              </div>
              <div class="rounded-2xl border border-border/70 bg-background/55 p-4">
                <p class="text-xs uppercase tracking-[0.16em] text-muted-foreground">
                  {{ t("dashboard.workspace.supervision.counts.stderr") }}
                </p>
                <p class="mt-2 text-sm font-medium text-foreground">
                  {{ sessionEventCounts.stderr }}
                </p>
              </div>
              <div class="rounded-2xl border border-border/70 bg-background/55 p-4">
                <p class="text-xs uppercase tracking-[0.16em] text-muted-foreground">
                  {{ t("dashboard.workspace.supervision.counts.changed") }}
                </p>
                <p class="mt-2 text-sm font-medium text-foreground">
                  {{ visibleChangedFileCount }}
                </p>
              </div>
            </div>

            <div class="rounded-2xl border border-border/70 bg-background/60 p-4">
              <p class="text-xs font-medium uppercase tracking-[0.16em] text-muted-foreground">
                {{ t("dashboard.workspace.promptTitle") }}
              </p>
              <pre class="mt-3 rounded-xl border border-border/70 bg-background/70 p-4 font-mono text-sm text-foreground">{{ selectedTaskDetail.task.prompt }}</pre>
            </div>

            <div
              v-if="selectedTaskDetail.task.error"
              class="rounded-2xl border border-rose-500/30 bg-rose-500/10 p-4 text-sm text-rose-700 dark:text-rose-100"
            >
              {{ selectedTaskDetail.task.error }}
            </div>
          </CardContent>
        </Card>

        <Card
          v-else
          class="border-border/70 bg-card/80 shadow-xl backdrop-blur-xl"
        >
          <CardContent class="flex min-h-[16rem] items-center justify-center p-6">
            <p class="max-w-md text-center text-sm leading-6 text-muted-foreground">
              {{
                selectedDevice
                  ? t("dashboard.sessions.readySelected")
                  : t("dashboard.sessions.readyEmpty")
              }}
            </p>
          </CardContent>
        </Card>

        <template v-if="selectedDevice">
          <Card v-if="showGitInspect" class="border-border/70 bg-card/80 shadow-xl backdrop-blur-xl">
            <CardHeader class="space-y-4">
              <div class="flex flex-col gap-3 lg:flex-row lg:items-start lg:justify-between">
                <div class="space-y-1">
                  <CardTitle class="flex items-center gap-2 text-foreground">
                    <GitBranch class="size-4 text-emerald-300" />
                    {{ t("dashboard.sessions.resultReviewTitle") }}
                  </CardTitle>
                  <CardDescription>{{ t("dashboard.sessions.resultReviewDescription") }}</CardDescription>
                </div>
                <Button
                  variant="outline"
                  size="sm"
                  class="border-border/70 bg-background/55 text-foreground hover:bg-accent/70"
                  :disabled="gitLoading"
                  @click="refreshGitInspect"
                >
                  <RefreshCw class="size-4" />
                  {{ t("common.refresh") }}
                </Button>
              </div>
            </CardHeader>

            <CardContent class="pt-0">
              <div
                v-if="!selectedDeviceSupportsGitInspect"
                class="rounded-2xl border border-dashed border-border/70 bg-background/60 p-4 text-sm text-muted-foreground"
              >
                {{ t("dashboard.workspace.git.unsupported") }}
              </div>

              <div v-else class="space-y-4">
                <div
                  v-if="gitError"
                  class="rounded-2xl border border-rose-500/30 bg-rose-500/10 p-4 text-sm text-rose-700 dark:text-rose-100"
                >
                  {{ gitError }}
                </div>

                <div
                  v-if="gitLoading"
                  class="rounded-2xl border border-dashed border-border/70 bg-background/55 p-4 text-sm text-muted-foreground"
                >
                  {{ t("dashboard.workspace.git.loading") }}
                </div>

                <div
                  v-else-if="!gitInspect"
                  class="rounded-2xl border border-dashed border-border/70 bg-background/55 p-4 text-sm text-muted-foreground"
                >
                  {{ t("dashboard.workspace.git.empty") }}
                </div>

                <div v-else class="space-y-4">
                  <div class="grid gap-3 md:grid-cols-2 xl:grid-cols-4">
                    <div class="rounded-2xl border border-border/70 bg-background/55 p-4">
                      <p class="text-xs uppercase tracking-[0.16em] text-muted-foreground">
                        {{ t("dashboard.workspace.git.workspaceRoot") }}
                      </p>
                      <p class="mt-2 font-mono text-xs text-foreground">
                        {{ gitInspect.workspaceRoot }}
                      </p>
                    </div>
                    <div class="rounded-2xl border border-border/70 bg-background/55 p-4">
                      <p class="text-xs uppercase tracking-[0.16em] text-muted-foreground">
                        {{ t("dashboard.workspace.git.repoRoot") }}
                      </p>
                      <p class="mt-2 font-mono text-xs text-foreground">
                        {{ gitInspect.repoRoot ?? t("common.pending") }}
                      </p>
                      <p class="mt-2 text-xs text-muted-foreground">
                        {{ t("dashboard.workspace.git.scopePath") }}:
                        {{ gitInspect.scopePath ?? t("common.pending") }}
                      </p>
                    </div>
                    <div class="rounded-2xl border border-border/70 bg-background/55 p-4">
                      <p class="text-xs uppercase tracking-[0.16em] text-muted-foreground">
                        {{ t("dashboard.workspace.git.branch") }}
                      </p>
                      <p class="mt-2 text-sm font-medium text-foreground">
                        {{ gitInspect.branchName ?? t("common.pending") }}
                      </p>
                    </div>
                    <div class="rounded-2xl border border-border/70 bg-background/55 p-4">
                      <p class="text-xs uppercase tracking-[0.16em] text-muted-foreground">
                        {{ t("dashboard.workspace.git.upstream") }}
                      </p>
                      <p class="mt-2 text-sm font-medium text-foreground">
                        {{ gitInspect.upstreamBranch ?? t("dashboard.workspace.git.noUpstream") }}
                      </p>
                      <p class="mt-2 text-xs text-muted-foreground">
                        {{ t("dashboard.workspace.git.drift") }}:
                        {{ formatGitDrift(gitInspect.aheadCount, gitInspect.behindCount) }}
                      </p>
                    </div>
                  </div>

                  <div
                    v-if="gitInspect.state !== 'ready'"
                    class="rounded-2xl border border-dashed border-border/70 bg-background/55 p-4 text-sm text-muted-foreground"
                  >
                    {{ t(`dashboard.workspace.git.state.${gitInspect.state}`) }}
                  </div>

                  <template v-else>
                    <div class="flex flex-wrap gap-2">
                      <Badge variant="outline" class="border-border/70 bg-background/55 text-foreground">
                        {{ t("dashboard.workspace.git.stats.changedFiles", { count: gitInspect.diffStats.changedFiles }) }}
                      </Badge>
                      <Badge variant="outline" class="border-border/70 bg-background/55 text-foreground">
                        {{ t("dashboard.workspace.git.stats.stagedFiles", { count: gitInspect.diffStats.stagedFiles }) }}
                      </Badge>
                      <Badge variant="outline" class="border-border/70 bg-background/55 text-foreground">
                        {{ t("dashboard.workspace.git.stats.unstagedFiles", { count: gitInspect.diffStats.unstagedFiles }) }}
                      </Badge>
                      <Badge variant="outline" class="border-border/70 bg-background/55 text-foreground">
                        {{ t("dashboard.workspace.git.stats.untrackedFiles", { count: gitInspect.diffStats.untrackedFiles }) }}
                      </Badge>
                      <Badge variant="outline" class="border-border/70 bg-background/55 text-foreground">
                        {{ t("dashboard.workspace.git.stats.conflictedFiles", { count: gitInspect.diffStats.conflictedFiles }) }}
                      </Badge>
                    </div>

                    <div
                      v-if="!gitInspect.changedFiles.length"
                      class="rounded-2xl border border-dashed border-border/70 bg-background/55 p-4 text-sm text-muted-foreground"
                    >
                      {{ t("dashboard.workspace.git.clean") }}
                    </div>

                    <div class="grid gap-4 xl:grid-cols-[minmax(0,1fr)_minmax(0,0.95fr)]">
                      <div class="rounded-2xl border border-border/70 bg-background/55 p-4">
                        <div class="mb-4 flex items-center gap-2">
                          <GitBranch class="size-4 text-emerald-300" />
                          <p class="text-sm font-medium text-foreground">
                            {{ t("dashboard.workspace.git.changedFilesTitle") }}
                          </p>
                        </div>

                        <ScrollArea class="h-[18rem] pr-3">
                          <div class="space-y-3">
                            <div
                              v-for="file in gitInspect.changedFiles"
                              :key="file.repoPath"
                              class="rounded-2xl border border-border/70 bg-background/70 p-4"
                            >
                              <div class="flex items-start justify-between gap-3">
                                <div class="space-y-1">
                                  <p class="text-sm font-medium text-foreground">
                                    {{ file.repoPath }}
                                  </p>
                                  <p class="font-mono text-xs text-muted-foreground">
                                    {{ file.path }}
                                  </p>
                                </div>
                                <Badge variant="outline" class="border-border/70 bg-background/55 text-foreground">
                                  {{ formatGitFileStatus(file.status) }}
                                </Badge>
                              </div>

                              <div class="mt-3 flex items-center justify-end">
                                <Button
                                  v-if="canPreviewGitChangedFile(file)"
                                  variant="outline"
                                  size="sm"
                                  class="border-border/70 bg-background/55 text-foreground hover:bg-accent/70"
                                  @click="openGitChangedFile(file)"
                                >
                                  {{ t("dashboard.workspace.git.preview") }}
                                </Button>
                                <p
                                  v-else-if="file.status === 'deleted'"
                                  class="text-xs text-muted-foreground"
                                >
                                  {{ t("dashboard.workspace.git.deletedPreviewUnavailable") }}
                                </p>
                              </div>
                            </div>
                          </div>
                        </ScrollArea>
                      </div>

                      <div class="rounded-2xl border border-border/70 bg-background/55 p-4">
                        <div class="mb-4 flex items-center gap-2">
                          <Activity class="size-4 text-sky-300" />
                          <p class="text-sm font-medium text-foreground">
                            {{ t("dashboard.workspace.git.recentCommitsTitle") }}
                          </p>
                        </div>

                        <div
                          v-if="!gitInspect.hasCommits"
                          class="rounded-2xl border border-dashed border-border/70 bg-background/70 p-4 text-sm text-muted-foreground"
                        >
                          {{ t("dashboard.workspace.git.noCommits") }}
                        </div>

                        <ScrollArea v-else class="h-[18rem] pr-3">
                          <div class="space-y-3">
                            <div
                              v-for="commit in gitInspect.recentCommits"
                              :key="commit.id"
                              class="rounded-2xl border border-border/70 bg-background/70 p-4"
                            >
                              <div class="flex items-start justify-between gap-3">
                                <Badge variant="outline" class="border-border/70 bg-background/55 text-foreground">
                                  {{ commit.shortId }}
                                </Badge>
                                <span class="text-xs text-muted-foreground">
                                  {{ formatTimestamp(commit.committedAtEpochMs) }}
                                </span>
                              </div>
                              <p class="mt-3 text-sm font-medium text-foreground">
                                {{ commit.summary || commit.shortId }}
                              </p>
                              <p class="mt-2 text-xs text-muted-foreground">
                                {{ commit.authorName }}
                              </p>
                            </div>
                          </div>
                        </ScrollArea>
                      </div>
                    </div>
                  </template>
                </div>
              </div>
            </CardContent>
          </Card>

          <Card class="border-border/70 bg-card/80 shadow-xl backdrop-blur-xl">
            <CardHeader class="space-y-4">
              <div class="flex flex-col gap-3 lg:flex-row lg:items-start lg:justify-between">
                <div class="space-y-1">
                  <CardTitle class="flex items-center gap-2 text-foreground">
                    <Activity class="size-4 text-sky-300" />
                    {{ t("dashboard.sessions.eventStreamTitle") }}
                  </CardTitle>
                  <CardDescription>{{ t("dashboard.sessions.eventStreamDescription") }}</CardDescription>
                </div>
                <Badge
                  v-if="selectedTaskDetail"
                  variant="outline"
                  :class="taskStatusClass(selectedTaskDetail.task.status)"
                >
                  {{ formatTaskStatus(selectedTaskDetail.task.status) }}
                </Badge>
              </div>
            </CardHeader>

            <CardContent class="pt-0">
              <ScrollArea class="h-[24rem] pr-3">
                <div
                  v-if="!selectedTaskDetail?.events.length"
                  class="rounded-2xl border border-dashed border-border/70 bg-background/55 p-4 text-sm text-muted-foreground"
                >
                  {{ t("dashboard.workspace.waitingEvents") }}
                </div>

                <div v-else class="space-y-3">
                  <div
                    v-for="event in selectedTaskDetail.events"
                    :key="`${event.taskId}-${event.seq}`"
                    class="rounded-2xl border p-4"
                    :class="eventKindClass(event.kind)"
                  >
                    <div class="mb-3 flex flex-col gap-2 sm:flex-row sm:items-center sm:justify-between">
                      <Badge variant="outline" class="border-current/30 bg-transparent text-current">
                        {{ formatEventKind(event.kind) }}
                      </Badge>
                      <span class="text-xs text-muted-foreground">
                        {{ formatTimestamp(event.timestampEpochMs) }}
                      </span>
                    </div>
                    <pre class="font-mono text-sm text-foreground">{{ event.message }}</pre>
                  </div>
                </div>
              </ScrollArea>
            </CardContent>
          </Card>

          <Card class="border-border/70 bg-card/80 shadow-xl backdrop-blur-xl">
            <CardHeader class="space-y-4">
              <div class="flex flex-col gap-3 lg:flex-row lg:items-start lg:justify-between">
                <div class="space-y-1">
                  <CardTitle class="flex items-center gap-2 text-foreground">
                    <FolderCode class="size-4 text-emerald-300" />
                    {{ t("dashboard.workspace.browser.title") }}
                  </CardTitle>
                  <CardDescription>
                    {{ t("dashboard.workspace.browser.description") }}
                  </CardDescription>
                </div>
                <div class="flex flex-wrap gap-2">
                  <Button
                    variant="outline"
                    size="sm"
                    class="border-border/70 bg-background/55 text-foreground hover:bg-accent/70"
                    :disabled="!workspaceListing?.parentPath || workspaceLoading"
                    @click="navigateWorkspaceUp"
                  >
                    {{ t("dashboard.workspace.browser.up") }}
                  </Button>
                  <Button
                    variant="outline"
                    size="sm"
                    class="border-border/70 bg-background/55 text-foreground hover:bg-accent/70"
                    :disabled="workspaceLoading"
                    @click="refreshWorkspace"
                  >
                    <RefreshCw class="size-4" />
                    {{ t("common.refresh") }}
                  </Button>
                </div>
              </div>
            </CardHeader>

            <CardContent class="pt-0">
              <div
                v-if="!selectedDeviceSupportsWorkspace"
                class="rounded-2xl border border-dashed border-border/70 bg-background/60 p-4 text-sm text-muted-foreground"
              >
                {{ t("dashboard.workspace.browser.unsupported") }}
              </div>

              <div v-else class="grid gap-4 xl:grid-cols-[300px_minmax(0,1fr)]">
                <div class="rounded-2xl border border-border/70 bg-background/60 p-4">
                  <div class="mb-4 space-y-3">
                    <div class="space-y-1">
                      <p class="text-xs uppercase tracking-[0.16em] text-muted-foreground">
                        {{ t("dashboard.workspace.browser.root") }}
                      </p>
                      <p class="font-mono text-xs text-foreground">
                        {{ workspaceListing?.rootPath ?? selectedDeviceWorkingRoot }}
                      </p>
                    </div>
                    <div class="space-y-1">
                      <p class="text-xs uppercase tracking-[0.16em] text-muted-foreground">
                        {{ t("dashboard.workspace.browser.path") }}
                      </p>
                      <p class="font-mono text-xs text-foreground">
                        {{ workspaceListing?.path ?? selectedDeviceWorkingRoot }}
                      </p>
                    </div>
                    <p class="text-xs text-muted-foreground">
                      {{
                        t("dashboard.workspace.browser.entries", {
                          count: workspaceListing?.entries.length ?? 0
                        })
                      }}
                    </p>
                  </div>

                  <div
                    v-if="workspaceError"
                    class="mb-4 rounded-2xl border border-rose-500/30 bg-rose-500/10 p-4 text-sm text-rose-700 dark:text-rose-100"
                  >
                    {{ workspaceError }}
                  </div>

                  <div
                    v-if="workspaceLoading"
                    class="rounded-2xl border border-dashed border-border/70 bg-background/55 p-4 text-sm text-muted-foreground"
                  >
                    {{ t("dashboard.workspace.browser.loading") }}
                  </div>

                  <div
                    v-else-if="workspaceListing && !workspaceListing.entries.length"
                    class="rounded-2xl border border-dashed border-border/70 bg-background/55 p-4 text-sm text-muted-foreground"
                  >
                    {{ t("dashboard.workspace.browser.empty") }}
                  </div>

                  <ScrollArea v-else class="h-[22rem] pr-3">
                    <div class="space-y-2">
                      <button
                        v-for="entry in workspaceListing?.entries ?? []"
                        :key="entry.path"
                        type="button"
                        class="w-full rounded-2xl border p-3 text-left transition-colors"
                        :class="selectedStateClass(selectedWorkspacePath === entry.path)"
                        @click="openWorkspaceEntry(entry)"
                      >
                        <div class="flex items-start justify-between gap-3">
                          <div class="space-y-1">
                            <p class="text-sm font-medium text-foreground">{{ entry.name }}</p>
                            <p class="font-mono text-xs text-muted-foreground">
                              {{ entry.path }}
                            </p>
                          </div>
                          <Badge variant="outline" class="border-border/70 bg-background/55 text-foreground">
                            {{ t(`dashboard.workspace.browser.kind.${entry.kind}`) }}
                          </Badge>
                        </div>
                        <p
                          v-if="entry.kind === 'file' && entry.sizeBytes !== null"
                          class="mt-2 text-xs text-muted-foreground"
                        >
                          {{ t("dashboard.workspace.browser.size", { size: entry.sizeBytes }) }}
                        </p>
                      </button>
                    </div>
                  </ScrollArea>
                </div>

                <div class="rounded-2xl border border-border/70 bg-background/60 p-4">
                  <div class="mb-4 space-y-2">
                    <p class="text-sm font-medium text-foreground">
                      {{ t("dashboard.workspace.browser.previewTitle") }}
                    </p>
                    <p class="font-mono text-xs text-muted-foreground">
                      {{ workspacePreview?.path ?? selectedWorkspacePath ?? t("common.waiting") }}
                    </p>
                  </div>

                  <div
                    v-if="workspacePreviewLoading"
                    class="rounded-2xl border border-dashed border-border/70 bg-background/55 p-4 text-sm text-muted-foreground"
                  >
                    {{ t("dashboard.workspace.browser.previewLoading") }}
                  </div>

                  <div
                    v-else-if="workspacePreview?.kind === 'binary'"
                    class="rounded-2xl border border-dashed border-border/70 bg-background/55 p-4 text-sm text-muted-foreground"
                  >
                    {{ t("dashboard.workspace.browser.binaryNotice") }}
                  </div>

                  <div
                    v-else-if="workspacePreview?.kind === 'text'"
                    class="space-y-3"
                  >
                    <div class="grid gap-3 md:grid-cols-3">
                      <div class="rounded-2xl border border-border/70 bg-background/55 p-4">
                        <p class="text-xs uppercase tracking-[0.16em] text-muted-foreground">
                          {{ t("dashboard.workspace.browser.root") }}
                        </p>
                        <p class="mt-2 font-mono text-xs text-foreground">
                          {{ workspacePreview.rootPath }}
                        </p>
                      </div>
                      <div class="rounded-2xl border border-border/70 bg-background/55 p-4">
                        <p class="text-xs uppercase tracking-[0.16em] text-muted-foreground">
                          {{ t("dashboard.workspace.browser.sizeLabel") }}
                        </p>
                        <p class="mt-2 text-sm font-medium text-foreground">
                          {{ t("dashboard.workspace.browser.size", { size: workspacePreview.sizeBytes }) }}
                        </p>
                      </div>
                      <div class="rounded-2xl border border-border/70 bg-background/55 p-4">
                        <p class="text-xs uppercase tracking-[0.16em] text-muted-foreground">
                          {{ t("dashboard.workspace.browser.lines") }}
                        </p>
                        <p class="mt-2 text-sm font-medium text-foreground">
                          {{ workspacePreview.totalLines ?? 0 }}
                        </p>
                      </div>
                    </div>

                    <div
                      v-if="workspacePreview.truncated"
                      class="rounded-2xl border border-amber-500/30 bg-amber-500/10 p-4 text-sm text-amber-700 dark:text-amber-100"
                    >
                      {{ t("dashboard.workspace.browser.truncated") }}
                    </div>

                    <pre class="rounded-2xl border border-border/70 bg-background/70 p-4 font-mono text-sm text-foreground">
{{ workspacePreview.content }}
                    </pre>
                  </div>

                  <div
                    v-else
                    class="rounded-2xl border border-dashed border-border/70 bg-background/55 p-4 text-sm text-muted-foreground"
                  >
                    {{ t("dashboard.workspace.browser.previewEmpty") }}
                  </div>
                </div>
              </div>
            </CardContent>
          </Card>
        </template>
      </div>
    </section>
  </section>
</template>
