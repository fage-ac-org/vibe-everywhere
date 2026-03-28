<script setup lang="ts">
import { useI18n } from "vue-i18n"
import { Eye, RefreshCw, TerminalSquare } from "lucide-vue-next"
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
import { Textarea } from "@/components/ui/textarea"
import { useDashboardController } from "@/views/dashboard/controller"

const { t } = useI18n()
const dashboard = useDashboardController()

const {
  PORT_FORWARD_STATUS_OPTIONS,
  SHELL_STATUS_OPTIONS,
  TASK_SCOPE_OPTIONS,
  buildPreviewUrl,
  canClosePortForward,
  canCloseShell,
  canCreatePortForward,
  canOpenShell,
  canSendShellInput,
  formatConnectionState,
  formatEndpoint,
  formatPortForwardProtocol,
  formatPortForwardStatus,
  formatPortForwardStatusOption,
  formatPortForwardTransport,
  formatScopeOption,
  formatShellStatus,
  formatShellStatusOption,
  formatStreamLabel,
  formatTimestamp,
  portForwardDraft,
  portForwardScope,
  portForwardStatusClass,
  portForwardStatusFilter,
  portForwards,
  nativeSelectClass,
  selectedDevice,
  selectedDeviceSupportsShell,
  selectedPortForward,
  selectedPreviewIsReady,
  selectedPreviewLoopbackWarning,
  selectedPreviewUrl,
  selectedShellSession,
  selectedShellSessionDetail,
  selectedDeviceWorkingRoot,
  selectedStateClass,
  shellDraft,
  shellScope,
  shellSessions,
  shellSocketState,
  shellStatusClass,
  shellStatusFilter,
  shellTimeline,
  showPreviewTools,
  showShellTools,
  store,
  terminalStreamClass,
  visiblePortForwards,
  visibleShellSessions
} = dashboard
</script>

<template>
  <section class="space-y-4">
    <div class="space-y-2">
      <Badge variant="outline" class="border-border/70 bg-background/55 text-foreground">
        {{ t("dashboard.advanced.badge") }}
      </Badge>
      <div class="space-y-1">
        <h2 class="text-2xl font-semibold tracking-tight text-foreground">
          {{ t("dashboard.advanced.title") }}
        </h2>
        <p class="text-sm text-muted-foreground">{{ t("dashboard.advanced.description") }}</p>
      </div>
    </div>

    <Card
      v-if="showPreviewTools"
      class="border-border/70 bg-card/80 shadow-xl backdrop-blur-xl"
    >
      <CardContent class="p-4">
        <div class="flex flex-col gap-4 lg:flex-row lg:items-end lg:justify-between">
          <div class="space-y-1">
            <p class="text-sm font-medium text-foreground">
              {{ t("dashboard.preview.launchTitle") }}
            </p>
            <p class="text-sm text-muted-foreground">
              {{ t("dashboard.preview.launchDescription") }}
            </p>
          </div>

          <div class="grid gap-3 md:grid-cols-[1.2fr_1fr_auto] lg:min-w-[38rem]">
            <Input
              v-model="portForwardDraft.targetHost"
              :placeholder="t('dashboard.placeholders.targetHost')"
            />
            <Input
              v-model="portForwardDraft.targetPort"
              inputmode="numeric"
              :placeholder="t('dashboard.placeholders.targetPort')"
            />
            <Button :disabled="!canCreatePortForward" @click="store.createPortForward">
              {{ t("dashboard.preview.open") }}
            </Button>
          </div>
        </div>
      </CardContent>
    </Card>

    <div v-if="showShellTools || showPreviewTools" class="grid gap-4 2xl:grid-cols-2">
      <Card
        v-if="showShellTools"
        class="border-border/70 bg-card/80 shadow-xl backdrop-blur-xl"
      >
        <CardHeader class="space-y-4">
          <div class="flex flex-col gap-3 lg:flex-row lg:items-start lg:justify-between">
            <div class="space-y-1">
              <CardTitle class="flex items-center gap-2 text-foreground">
                <TerminalSquare class="size-4 text-violet-300" />
                {{ t("dashboard.terminal.title") }}
              </CardTitle>
              <CardDescription>
                {{
                  t("dashboard.terminal.visibleSummary", {
                    visible: visibleShellSessions.length,
                    total: shellSessions.length,
                    state: formatConnectionState(shellSocketState)
                  })
                }}
              </CardDescription>
            </div>
            <Button
              variant="outline"
              size="sm"
              class="border-border/70 bg-background/55 text-foreground hover:bg-accent/70"
              @click="store.refreshShellSessionsFromPoll"
            >
              <RefreshCw class="size-4" />
              {{ t("common.refresh") }}
            </Button>
          </div>

          <div class="grid gap-3 md:grid-cols-2 xl:grid-cols-[1fr_1fr_1.2fr_auto_auto]">
            <label class="grid gap-2">
              <span class="text-xs font-medium uppercase tracking-[0.16em] text-muted-foreground">
                {{ t("common.scope") }}
              </span>
              <select v-model="shellScope" :class="nativeSelectClass">
                <option v-for="option in TASK_SCOPE_OPTIONS" :key="option" :value="option">
                  {{ formatScopeOption(option) }}
                </option>
              </select>
            </label>

            <label class="grid gap-2">
              <span class="text-xs font-medium uppercase tracking-[0.16em] text-muted-foreground">
                {{ t("common.status") }}
              </span>
              <select v-model="shellStatusFilter" :class="nativeSelectClass">
                <option v-for="option in SHELL_STATUS_OPTIONS" :key="option" :value="option">
                  {{ formatShellStatusOption(option) }}
                </option>
              </select>
            </label>

            <label class="grid gap-2">
              <span class="text-xs font-medium uppercase tracking-[0.16em] text-muted-foreground">
                {{ t("dashboard.fields.terminalCwd") }}
              </span>
              <Input
                v-model="shellDraft.cwd"
                :placeholder="t('dashboard.placeholders.terminalCwd')"
              />
            </label>

            <Button class="self-end" :disabled="!canOpenShell" @click="store.createShellSession">
              {{ t("dashboard.terminal.open") }}
            </Button>

            <Button
              variant="outline"
              class="self-end border-border/70 bg-background/55 text-foreground hover:bg-accent/70"
              :disabled="!canCloseShell"
              @click="store.closeSelectedShellSession"
            >
              {{ t("common.close") }}
            </Button>
          </div>

          <p v-if="selectedDevice && !selectedDeviceSupportsShell" class="text-sm text-muted-foreground">
            {{ t("dashboard.terminal.noCapability") }}
          </p>
        </CardHeader>

        <CardContent class="pt-0">
          <div class="grid gap-4 xl:grid-cols-[260px_minmax(0,1fr)]">
            <div class="space-y-3">
              <p v-if="!visibleShellSessions.length" class="text-sm text-muted-foreground">
                {{ t("dashboard.terminal.empty") }}
              </p>

              <ScrollArea class="h-[28rem] pr-3">
                <div class="space-y-3">
                  <button
                    v-for="session in visibleShellSessions"
                    :key="session.id"
                    type="button"
                    class="w-full rounded-2xl border p-4 text-left transition-colors"
                    :class="selectedStateClass(selectedShellSession?.id === session.id)"
                    @click="store.selectShellSession(session.id)"
                  >
                    <div class="flex items-start justify-between gap-3">
                      <p class="font-medium text-foreground">{{ session.deviceId }}</p>
                      <Badge variant="outline" :class="shellStatusClass(session.status)">
                        {{ formatShellStatus(session.status, session.closeRequested) }}
                      </Badge>
                    </div>
                    <p class="mt-3 font-mono text-xs text-muted-foreground">
                      {{ session.cwd ?? t("common.useAgentWorkingRoot") }}
                    </p>
                    <p class="mt-2 text-xs text-muted-foreground">
                      {{ formatTimestamp(session.startedAtEpochMs ?? session.createdAtEpochMs) }}
                    </p>
                  </button>
                </div>
              </ScrollArea>
            </div>

            <div class="rounded-2xl border border-border/70 bg-background/60">
              <div
                v-if="selectedShellSession"
                class="flex flex-col gap-2 border-b border-border/70 px-4 py-4 sm:flex-row sm:items-start sm:justify-between"
              >
                <div>
                  <p class="font-medium text-foreground">
                    {{ t("dashboard.terminal.detailTitle", { id: selectedShellSession.id }) }}
                  </p>
                  <p class="text-sm text-muted-foreground">
                    {{
                      t("dashboard.terminal.detailSummary", {
                        status: formatShellStatus(
                          selectedShellSession.status,
                          selectedShellSession.closeRequested
                        ),
                        time: formatTimestamp(
                          selectedShellSession.startedAtEpochMs ??
                            selectedShellSession.createdAtEpochMs
                        )
                      })
                    }}
                  </p>
                </div>
                <p class="font-mono text-xs text-muted-foreground">
                  {{ selectedShellSession.cwd ?? selectedDeviceWorkingRoot }}
                </p>
              </div>

              <ScrollArea class="h-[22rem] px-4 py-4">
                <div v-if="selectedShellSessionDetail" class="space-y-3">
                  <div
                    v-if="!shellTimeline.length"
                    class="rounded-2xl border border-dashed border-border/70 bg-background/55 p-4 text-sm text-muted-foreground"
                  >
                    {{ t("dashboard.terminal.waiting") }}
                  </div>

                  <div
                    v-for="entry in shellTimeline"
                    :key="entry.key"
                    class="rounded-2xl border p-4"
                    :class="terminalStreamClass(entry.stream)"
                  >
                    <div class="mb-3 flex flex-col gap-2 sm:flex-row sm:items-center sm:justify-between">
                      <Badge variant="outline" class="border-current/30 bg-transparent text-current">
                        {{ formatStreamLabel(entry.stream) }}
                      </Badge>
                      <span class="text-xs text-muted-foreground">
                        {{ formatTimestamp(entry.timestampEpochMs) }}
                      </span>
                    </div>
                    <pre class="font-mono text-sm text-foreground">{{ entry.data }}</pre>
                  </div>
                </div>

                <div
                  v-else
                  class="rounded-2xl border border-dashed border-border/70 bg-background/55 p-4 text-sm text-muted-foreground"
                >
                  {{ t("dashboard.terminal.select") }}
                </div>
              </ScrollArea>

              <div class="border-t border-border/70 p-4">
                <div class="space-y-3">
                  <Textarea
                    v-model="shellDraft.input"
                    class="min-h-28"
                    :placeholder="t('dashboard.placeholders.terminalInput')"
                    :disabled="!selectedShellSession || selectedShellSession.closeRequested"
                  />
                  <div class="flex flex-col gap-3 lg:flex-row lg:items-center lg:justify-between">
                    <p class="text-sm text-muted-foreground">{{ t("dashboard.terminal.note") }}</p>
                    <Button :disabled="!canSendShellInput" @click="store.submitShellInput">
                      {{ t("common.sendCommand") }}
                    </Button>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </CardContent>
      </Card>

      <Card
        v-if="showPreviewTools"
        class="border-border/70 bg-card/80 shadow-xl backdrop-blur-xl"
      >
        <CardHeader class="space-y-4">
          <div class="flex flex-col gap-3 lg:flex-row lg:items-start lg:justify-between">
            <div class="space-y-1">
              <CardTitle class="flex items-center gap-2 text-foreground">
                <Eye class="size-4 text-cyan-300" />
                {{ t("dashboard.preview.title") }}
              </CardTitle>
              <CardDescription>
                {{
                  t("dashboard.preview.visibleSummary", {
                    visible: visiblePortForwards.length,
                    total: portForwards.length
                  })
                }}
              </CardDescription>
              <p class="text-sm text-muted-foreground">
                {{ t("dashboard.preview.description") }}
              </p>
            </div>
            <Button
              variant="outline"
              size="sm"
              class="border-border/70 bg-background/55 text-foreground hover:bg-accent/70"
              @click="store.refreshPortForwardsFromPoll"
            >
              <RefreshCw class="size-4" />
              {{ t("common.refresh") }}
            </Button>
          </div>

          <div class="grid gap-3 md:grid-cols-2 xl:grid-cols-[1fr_1fr_auto]">
            <label class="grid gap-2">
              <span class="text-xs font-medium uppercase tracking-[0.16em] text-muted-foreground">
                {{ t("common.scope") }}
              </span>
              <select v-model="portForwardScope" :class="nativeSelectClass">
                <option v-for="option in TASK_SCOPE_OPTIONS" :key="option" :value="option">
                  {{ formatScopeOption(option) }}
                </option>
              </select>
            </label>

            <label class="grid gap-2">
              <span class="text-xs font-medium uppercase tracking-[0.16em] text-muted-foreground">
                {{ t("common.status") }}
              </span>
              <select v-model="portForwardStatusFilter" :class="nativeSelectClass">
                <option v-for="option in PORT_FORWARD_STATUS_OPTIONS" :key="option" :value="option">
                  {{ formatPortForwardStatusOption(option) }}
                </option>
              </select>
            </label>

            <Button
              variant="outline"
              class="self-end border-border/70 bg-background/55 text-foreground hover:bg-accent/70"
              :disabled="!canClosePortForward"
              @click="store.closeSelectedPortForward"
            >
              {{ t("common.close") }}
            </Button>
          </div>

          <p class="text-sm text-muted-foreground">
            {{ t("dashboard.preview.serviceHint") }}
          </p>
        </CardHeader>

        <CardContent class="pt-0">
          <div class="grid gap-4 xl:grid-cols-[260px_minmax(0,1fr)]">
            <div class="space-y-3">
              <p v-if="!visiblePortForwards.length" class="text-sm text-muted-foreground">
                {{ t("dashboard.preview.empty") }}
              </p>

              <ScrollArea class="h-[28rem] pr-3">
                <div class="space-y-3">
                  <button
                    v-for="forward in visiblePortForwards"
                    :key="forward.id"
                    type="button"
                    class="w-full rounded-2xl border p-4 text-left transition-colors"
                    :class="selectedStateClass(selectedPortForward?.id === forward.id)"
                    @click="store.selectPortForward(forward.id)"
                  >
                    <div class="flex items-start justify-between gap-3">
                      <p class="font-medium text-foreground">{{ forward.deviceId }}</p>
                      <Badge variant="outline" :class="portForwardStatusClass(forward.status)">
                        {{ formatPortForwardStatus(forward.status) }}
                      </Badge>
                    </div>
                    <p class="mt-3 font-mono text-xs text-foreground">
                      {{ buildPreviewUrl(forward) }}
                    </p>
                    <p class="mt-1 text-xs text-muted-foreground">
                      {{
                        t("dashboard.preview.serviceTarget", {
                          host: forward.targetHost,
                          port: forward.targetPort
                        })
                      }}
                    </p>
                    <p class="mt-2 text-xs text-muted-foreground">
                      {{ formatTimestamp(forward.startedAtEpochMs ?? forward.createdAtEpochMs) }}
                    </p>
                  </button>
                </div>
              </ScrollArea>
            </div>

            <div class="rounded-2xl border border-border/70 bg-background/60 p-4">
              <div v-if="selectedPortForward" class="space-y-4">
                <div class="flex flex-col gap-2 sm:flex-row sm:items-start sm:justify-between">
                  <div>
                    <p class="font-medium text-foreground">
                      {{ t("dashboard.preview.detailTitle", { id: selectedPortForward.id }) }}
                    </p>
                    <p class="text-sm text-muted-foreground">
                      {{ formatPortForwardStatus(selectedPortForward.status) }}
                    </p>
                  </div>
                  <p class="font-mono text-xs text-muted-foreground">{{ selectedPortForward.deviceId }}</p>
                </div>

                <div class="rounded-2xl border border-cyan-500/25 bg-cyan-500/10 p-4">
                  <p class="text-xs uppercase tracking-[0.16em] text-cyan-700 dark:text-cyan-100">
                    {{ t("dashboard.preview.previewUrl") }}
                  </p>
                  <p class="mt-2 font-mono text-sm text-foreground">
                    {{ selectedPreviewUrl }}
                  </p>
                  <p class="mt-3 text-sm text-muted-foreground">
                    {{
                      selectedPreviewIsReady
                        ? t("dashboard.preview.ready")
                        : t("dashboard.preview.waiting")
                    }}
                  </p>
                  <div class="mt-4 flex flex-wrap gap-2">
                    <a
                      :href="selectedPreviewUrl"
                      target="_blank"
                      rel="noreferrer"
                      class="inline-flex h-9 items-center justify-center rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground shadow-xs transition hover:bg-primary/90"
                      :class="!selectedPreviewIsReady ? 'pointer-events-none opacity-50' : ''"
                      :aria-disabled="!selectedPreviewIsReady"
                      :tabindex="selectedPreviewIsReady ? 0 : -1"
                    >
                      {{ t("dashboard.preview.openLink") }}
                    </a>
                  </div>
                </div>

                <div
                  v-if="selectedPreviewLoopbackWarning"
                  class="rounded-2xl border border-amber-500/30 bg-amber-500/10 p-4 text-sm text-amber-700 dark:text-amber-100"
                >
                  {{ t("dashboard.preview.mobileWarning") }}
                </div>

                <div class="grid gap-3 md:grid-cols-3">
                  <div class="rounded-2xl border border-border/70 bg-background/55 p-4">
                    <p class="text-xs uppercase tracking-[0.16em] text-muted-foreground">
                      {{ t("common.created") }}
                    </p>
                    <p class="mt-2 text-xs text-foreground">
                      {{ formatTimestamp(selectedPortForward.createdAtEpochMs) }}
                    </p>
                  </div>
                  <div class="rounded-2xl border border-border/70 bg-background/55 p-4">
                    <p class="text-xs uppercase tracking-[0.16em] text-muted-foreground">
                      {{ t("common.started") }}
                    </p>
                    <p class="mt-2 text-xs text-foreground">
                      {{ formatTimestamp(selectedPortForward.startedAtEpochMs) }}
                    </p>
                  </div>
                  <div class="rounded-2xl border border-border/70 bg-background/55 p-4">
                    <p class="text-xs uppercase tracking-[0.16em] text-muted-foreground">
                      {{ t("common.finished") }}
                    </p>
                    <p class="mt-2 text-xs text-foreground">
                      {{ formatTimestamp(selectedPortForward.finishedAtEpochMs) }}
                    </p>
                  </div>
                </div>

                <div class="rounded-2xl border border-border/70 bg-background/55 p-4">
                  <div class="space-y-1">
                    <p class="text-sm font-medium text-foreground">
                      {{ t("dashboard.preview.advancedTitle") }}
                    </p>
                    <p class="text-sm text-muted-foreground">
                      {{ t("dashboard.preview.advancedDescription") }}
                    </p>
                  </div>

                  <div class="mt-4 grid gap-3 md:grid-cols-4">
                    <div class="rounded-2xl border border-border/70 bg-background/70 p-4">
                      <p class="text-xs uppercase tracking-[0.16em] text-muted-foreground">
                        {{ t("common.protocol") }}
                      </p>
                      <p class="mt-2 text-sm font-medium text-foreground">
                        {{ formatPortForwardProtocol(selectedPortForward.protocol) }}
                      </p>
                    </div>
                    <div class="rounded-2xl border border-border/70 bg-background/70 p-4">
                      <p class="text-xs uppercase tracking-[0.16em] text-muted-foreground">
                        {{ t("dashboard.preview.relayEndpoint") }}
                      </p>
                      <p class="mt-2 font-mono text-xs text-foreground">
                        {{ formatEndpoint(selectedPortForward.relayHost, selectedPortForward.relayPort) }}
                      </p>
                    </div>
                    <div class="rounded-2xl border border-border/70 bg-background/70 p-4">
                      <p class="text-xs uppercase tracking-[0.16em] text-muted-foreground">
                        {{ t("dashboard.preview.targetEndpoint") }}
                      </p>
                      <p class="mt-2 font-mono text-xs text-foreground">
                        {{ formatEndpoint(selectedPortForward.targetHost, selectedPortForward.targetPort) }}
                      </p>
                    </div>
                    <div class="rounded-2xl border border-border/70 bg-background/70 p-4">
                      <p class="text-xs uppercase tracking-[0.16em] text-muted-foreground">
                        {{ t("dashboard.preview.transportLabel") }}
                      </p>
                      <p class="mt-2 text-sm font-medium text-foreground">
                        {{ formatPortForwardTransport(selectedPortForward.transport) }}
                      </p>
                    </div>
                  </div>
                </div>

                <div
                  v-if="selectedPortForward.error"
                  class="rounded-2xl border border-rose-500/30 bg-rose-500/10 p-4 text-sm text-rose-700 dark:text-rose-100"
                >
                  {{ selectedPortForward.error }}
                </div>
              </div>

              <div
                v-else
                class="rounded-2xl border border-dashed border-border/70 bg-background/55 p-4 text-sm text-muted-foreground"
              >
                {{ t("dashboard.preview.select") }}
              </div>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>

    <Card
      v-else
      class="border-border/70 bg-card/80 shadow-xl backdrop-blur-xl"
    >
      <CardContent class="flex min-h-[18rem] items-center justify-center p-6">
        <p class="max-w-md text-center text-sm leading-6 text-muted-foreground">
          {{ t("dashboard.advanced.empty") }}
        </p>
      </CardContent>
    </Card>
  </section>
</template>
