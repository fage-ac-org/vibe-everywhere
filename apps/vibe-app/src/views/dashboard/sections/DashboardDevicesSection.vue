<script setup lang="ts">
import { computed } from "vue"
import { useI18n } from "vue-i18n"
import { Activity, FolderCode, Laptop2, RefreshCw, Server, ShieldCheck, Sparkles } from "lucide-vue-next"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle
} from "@/components/ui/card"
import { ScrollArea } from "@/components/ui/scroll-area"
import { Separator } from "@/components/ui/separator"
import { useDashboardController } from "@/views/dashboard/controller"

const { t } = useI18n()
const dashboard = useDashboardController()

const {
  activePlatformCapability,
  appConfig,
  auditRecords,
  auditTrail,
  auditOutcomeClass,
  currentActor,
  currentClientKind,
  deploymentDocsUrl,
  devicePreviewCount,
  deviceSessionCount,
  deviceShellCount,
  devices,
  formatAuditAction,
  formatAuditOutcome,
  formatAuthMode,
  formatControlClient,
  formatDevicePresence,
  formatDeploymentMode,
  formatNotificationChannel,
  formatProviderSummary,
  formatStorageKind,
  formatTimestamp,
  formatUserRole,
  health,
  platformCapabilityStateClass,
  providerBadgeClass,
  selectedDevice,
  selectedDeviceAvailableProviderCount,
  selectedDevicePreviewCount,
  selectedDeviceSessionCount,
  selectedDeviceShellCount,
  selectedDeviceWorkingRoot,
  selectedStateClass,
  showGovernanceSurface,
  statusBadgeClass,
  store
} = dashboard

const selectedDeviceCapabilities = computed(() => selectedDevice.value?.capabilities ?? [])
const overlayModeLabel = computed(() => formatRuntimeValue(selectedDevice.value?.overlay.mode))
const overlayStateLabel = computed(() => formatRuntimeValue(selectedDevice.value?.overlay.state))
const overlayRelayLabel = computed(
  () => selectedDevice.value?.overlay.relayUrl ?? t("common.pending")
)
const selectedDeviceCurrentTask = computed(
  () => selectedDevice.value?.currentTaskId ?? t("common.pending")
)

function formatRuntimeValue(value?: string | null) {
  if (!value) {
    return t("common.pending")
  }

  return value.replaceAll("_", " ")
}
</script>

<template>
  <section class="space-y-4">
    <div class="grid gap-4 md:grid-cols-3">
      <div class="rounded-3xl border border-border/70 bg-card/80 p-5 shadow-xl backdrop-blur-xl">
        <p class="text-xs uppercase tracking-[0.16em] text-muted-foreground">
          {{ t("dashboard.devices.registered", { count: devices.length }) }}
        </p>
        <p class="mt-3 text-3xl font-semibold text-foreground">
          {{ devices.length }}
        </p>
      </div>
      <div class="rounded-3xl border border-border/70 bg-card/80 p-5 shadow-xl backdrop-blur-xl">
        <p class="text-xs uppercase tracking-[0.16em] text-muted-foreground">
          {{ t("dashboard.stats.onlineDevices") }}
        </p>
        <p class="mt-3 text-3xl font-semibold text-foreground">
          {{ health?.onlineDeviceCount ?? 0 }}
        </p>
      </div>
      <div class="rounded-3xl border border-border/70 bg-card/80 p-5 shadow-xl backdrop-blur-xl">
        <p class="text-xs uppercase tracking-[0.16em] text-muted-foreground">
          {{ t("dashboard.devices.availableProviders") }}
        </p>
        <p class="mt-3 text-3xl font-semibold text-foreground">
          {{ selectedDeviceAvailableProviderCount }}
        </p>
      </div>
    </div>

    <section class="grid gap-4 xl:grid-cols-[360px_minmax(0,1fr)]">
      <Card class="border-border/70 bg-card/80 shadow-xl backdrop-blur-xl">
        <CardHeader class="flex flex-row items-start justify-between gap-4 space-y-0">
          <div class="space-y-1">
            <CardTitle class="flex items-center gap-2 text-foreground">
              <Server class="size-4 text-sky-300" />
              {{ t("dashboard.devices.inventoryTitle") }}
            </CardTitle>
            <CardDescription>{{ t("dashboard.devices.inventoryDescription") }}</CardDescription>
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
          <ScrollArea class="h-[34rem] pr-3">
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
                  <Badge variant="outline" class="border-border/70 bg-background/55 text-foreground">
                    {{ t("dashboard.devices.previews", { count: devicePreviewCount(device.id) }) }}
                  </Badge>
                </div>
              </button>
            </div>
          </ScrollArea>
        </CardContent>
      </Card>

      <div v-if="selectedDevice" class="space-y-4">
        <Card class="border-border/70 bg-card/80 shadow-xl backdrop-blur-xl">
          <CardHeader class="space-y-1">
            <div class="flex flex-col gap-3 lg:flex-row lg:items-start lg:justify-between">
              <div class="space-y-1">
                <CardTitle class="flex items-center gap-2 text-foreground">
                  <FolderCode class="size-4 text-emerald-300" />
                  {{ t("dashboard.devices.runtimeTitle") }}
                </CardTitle>
                <CardDescription>{{ t("dashboard.devices.runtimeDescription") }}</CardDescription>
              </div>
              <Badge
                variant="outline"
                :class="selectedDevice.online ? statusBadgeClass('online') : statusBadgeClass('offline')"
              >
                {{ formatDevicePresence(selectedDevice.online) }}
              </Badge>
            </div>
          </CardHeader>
          <CardContent class="grid gap-3 pt-0 md:grid-cols-2 xl:grid-cols-3">
            <div class="rounded-2xl border border-border/70 bg-background/55 p-4">
              <p class="text-xs uppercase tracking-[0.16em] text-muted-foreground">
                {{ t("dashboard.devices.platform") }}
              </p>
              <p class="mt-2 text-sm font-medium text-foreground">
                {{ selectedDevice.platform }}
              </p>
              <p class="mt-2 text-xs text-muted-foreground">
                {{ selectedDevice.metadata.arch }}
              </p>
            </div>
            <div class="rounded-2xl border border-border/70 bg-background/55 p-4">
              <p class="text-xs uppercase tracking-[0.16em] text-muted-foreground">
                {{ t("dashboard.devices.workingRoot") }}
              </p>
              <p class="mt-2 font-mono text-xs text-foreground">
                {{ selectedDeviceWorkingRoot }}
              </p>
            </div>
            <div class="rounded-2xl border border-border/70 bg-background/55 p-4">
              <p class="text-xs uppercase tracking-[0.16em] text-muted-foreground">
                {{ t("dashboard.devices.lastSeen") }}
              </p>
              <p class="mt-2 text-sm font-medium text-foreground">
                {{ formatTimestamp(selectedDevice.lastSeenEpochMs) }}
              </p>
            </div>
            <div class="rounded-2xl border border-border/70 bg-background/55 p-4">
              <p class="text-xs uppercase tracking-[0.16em] text-muted-foreground">
                {{ t("dashboard.devices.currentTask") }}
              </p>
              <p class="mt-2 font-mono text-xs text-foreground">
                {{ selectedDeviceCurrentTask }}
              </p>
            </div>
            <div class="rounded-2xl border border-border/70 bg-background/55 p-4">
              <p class="text-xs uppercase tracking-[0.16em] text-muted-foreground">
                {{ t("dashboard.devices.overlayMode") }}
              </p>
              <p class="mt-2 text-sm font-medium text-foreground">
                {{ overlayModeLabel }}
              </p>
              <p class="mt-2 text-xs text-muted-foreground">
                {{ t("dashboard.devices.overlayState") }}: {{ overlayStateLabel }}
              </p>
            </div>
            <div class="rounded-2xl border border-border/70 bg-background/55 p-4">
              <p class="text-xs uppercase tracking-[0.16em] text-muted-foreground">
                {{ t("dashboard.devices.overlayRelay") }}
              </p>
              <p class="mt-2 font-mono text-xs text-foreground">
                {{ overlayRelayLabel }}
              </p>
            </div>
          </CardContent>
        </Card>

        <Card class="border-border/70 bg-card/80 shadow-xl backdrop-blur-xl">
          <CardHeader class="space-y-1">
            <CardTitle class="flex items-center gap-2 text-foreground">
              <Sparkles class="size-4 text-amber-300" />
              {{ t("dashboard.devices.capabilitiesTitle") }}
            </CardTitle>
            <CardDescription>{{ t("dashboard.devices.capabilitiesDescription") }}</CardDescription>
          </CardHeader>
          <CardContent class="grid gap-4 pt-0 xl:grid-cols-[minmax(0,0.9fr)_minmax(0,1.1fr)]">
            <div class="rounded-2xl border border-border/70 bg-background/55 p-4">
              <p class="mb-3 text-sm font-medium text-foreground">
                {{ t("dashboard.devices.capabilitiesLabel") }}
              </p>
              <div v-if="selectedDeviceCapabilities.length" class="flex flex-wrap gap-2">
                <Badge
                  v-for="capability in selectedDeviceCapabilities"
                  :key="capability"
                  variant="outline"
                  class="border-border/70 bg-background/70 text-foreground"
                >
                  {{ capability }}
                </Badge>
              </div>
              <p v-else class="text-sm text-muted-foreground">
                {{ t("dashboard.devices.capabilitiesEmpty") }}
              </p>
            </div>

            <div class="rounded-2xl border border-border/70 bg-background/55 p-4">
              <div class="mb-4 flex items-center justify-between gap-3">
                <p class="text-sm font-medium text-foreground">
                  {{ t("dashboard.devices.providersTitle") }}
                </p>
                <Badge variant="outline" class="border-border/70 bg-background/70 text-foreground">
                  {{ selectedDevice.providers.length }}
                </Badge>
              </div>

              <div class="space-y-3">
                <div
                  v-for="provider in selectedDevice.providers"
                  :key="provider.kind"
                  class="rounded-2xl border border-border/70 bg-background/70 p-4"
                >
                  <div class="flex flex-col gap-2 sm:flex-row sm:items-start sm:justify-between">
                    <div class="space-y-1">
                      <p class="text-sm font-medium text-foreground">
                        {{ formatProviderSummary(provider.kind, provider.executionProtocol) }}
                      </p>
                      <p class="text-xs text-muted-foreground">
                        {{ provider.command }}
                      </p>
                    </div>
                    <Badge variant="outline" :class="providerBadgeClass(provider.available)">
                      {{
                        provider.available
                          ? t("dashboard.devices.providerAvailable")
                          : t("dashboard.devices.providerUnavailable")
                      }}
                    </Badge>
                  </div>
                  <p class="mt-3 text-xs text-muted-foreground">
                    {{
                      provider.version
                        ? `${t("dashboard.devices.providerVersion")}: ${provider.version}`
                        : t("dashboard.devices.providerVersionPending")
                    }}
                  </p>
                  <p v-if="provider.error" class="mt-2 text-xs text-rose-700 dark:text-rose-100">
                    {{ provider.error }}
                  </p>
                </div>
              </div>
            </div>
          </CardContent>
        </Card>

        <Card class="border-border/70 bg-card/80 shadow-xl backdrop-blur-xl">
          <CardHeader class="space-y-1">
            <CardTitle class="flex items-center gap-2 text-foreground">
              <Activity class="size-4 text-sky-300" />
              {{ t("dashboard.devices.workloadTitle") }}
            </CardTitle>
            <CardDescription>{{ t("dashboard.devices.workloadDescription") }}</CardDescription>
          </CardHeader>
          <CardContent class="grid gap-3 pt-0 md:grid-cols-3">
            <div class="rounded-2xl border border-border/70 bg-background/55 p-4">
              <p class="text-xs uppercase tracking-[0.16em] text-muted-foreground">
                {{ t("dashboard.devices.sessions", { count: selectedDeviceSessionCount }) }}
              </p>
              <p class="mt-2 text-2xl font-semibold text-foreground">
                {{ selectedDeviceSessionCount }}
              </p>
            </div>
            <div class="rounded-2xl border border-border/70 bg-background/55 p-4">
              <p class="text-xs uppercase tracking-[0.16em] text-muted-foreground">
                {{ t("dashboard.devices.terminals", { count: selectedDeviceShellCount }) }}
              </p>
              <p class="mt-2 text-2xl font-semibold text-foreground">
                {{ selectedDeviceShellCount }}
              </p>
            </div>
            <div class="rounded-2xl border border-border/70 bg-background/55 p-4">
              <p class="text-xs uppercase tracking-[0.16em] text-muted-foreground">
                {{ t("dashboard.devices.previews", { count: selectedDevicePreviewCount }) }}
              </p>
              <p class="mt-2 text-2xl font-semibold text-foreground">
                {{ selectedDevicePreviewCount }}
              </p>
            </div>
          </CardContent>
        </Card>
      </div>

      <Card
        v-else
        class="border-border/70 bg-card/80 shadow-xl backdrop-blur-xl"
      >
        <CardContent class="flex min-h-[24rem] items-center justify-center p-6">
          <p class="max-w-md text-center text-sm leading-6 text-muted-foreground">
            {{ t("dashboard.devices.emptySelection") }}
          </p>
        </CardContent>
      </Card>
    </section>

    <section class="grid gap-4 2xl:grid-cols-[minmax(0,1.05fr)_minmax(0,0.95fr)]">
      <Card class="border-border/70 bg-card/80 shadow-xl backdrop-blur-xl">
        <CardHeader class="space-y-1">
          <CardTitle class="flex items-center gap-2 text-foreground">
            <Server class="size-4 text-emerald-300" />
            {{ t("dashboard.deployment.title") }}
          </CardTitle>
          <CardDescription>{{ t("dashboard.devices.managementDescription") }}</CardDescription>
        </CardHeader>
        <CardContent class="grid gap-4 pt-0">
          <div class="grid gap-3 md:grid-cols-2">
            <div class="rounded-2xl border border-border/70 bg-background/55 p-4">
              <p class="text-xs uppercase tracking-[0.16em] text-muted-foreground">
                {{ t("dashboard.deployment.mode") }}
              </p>
              <p class="mt-2 text-sm font-medium text-foreground">
                {{ formatDeploymentMode(appConfig?.deployment.mode) }}
              </p>
            </div>
            <div class="rounded-2xl border border-border/70 bg-background/55 p-4">
              <p class="text-xs uppercase tracking-[0.16em] text-muted-foreground">
                {{ t("dashboard.deployment.authMode") }}
              </p>
              <p class="mt-2 text-sm font-medium text-foreground">
                {{ formatAuthMode(appConfig?.authMode) }}
              </p>
            </div>
            <div class="rounded-2xl border border-border/70 bg-background/55 p-4">
              <p class="text-xs uppercase tracking-[0.16em] text-muted-foreground">
                {{ t("dashboard.deployment.storageKind") }}
              </p>
              <p class="mt-2 text-sm font-medium text-foreground">
                {{ formatStorageKind(appConfig?.storageKind) }}
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

          <div class="rounded-2xl border border-border/70 bg-background/55 p-4">
            <div class="flex flex-col gap-3 lg:flex-row lg:items-start lg:justify-between">
              <div class="space-y-1">
                <p class="text-sm font-medium text-foreground">
                  {{ t("dashboard.deployment.relayOrigin") }}
                </p>
                <p class="font-mono text-xs text-muted-foreground">
                  {{ appConfig?.deployment.relayPublicOrigin || t("common.pending") }}
                </p>
              </div>
              <a
                v-if="deploymentDocsUrl"
                :href="deploymentDocsUrl"
                target="_blank"
                rel="noreferrer"
                class="text-sm text-sky-700 underline decoration-sky-500/50 underline-offset-4 dark:text-sky-100"
              >
                {{ t("dashboard.deployment.documentation") }}
              </a>
            </div>
          </div>

          <Separator class="bg-border/70" />

          <div class="rounded-2xl border border-border/70 bg-background/55 p-4">
            <div class="space-y-1">
              <div class="flex items-center gap-2">
                <Laptop2 class="size-4 text-amber-300" />
                <p class="text-sm font-medium text-foreground">
                  {{ t("dashboard.platform.title") }}
                </p>
              </div>
              <p class="text-xs leading-6 text-muted-foreground">
                {{ t("dashboard.platform.summary") }}
              </p>
            </div>

            <div class="mt-4 rounded-2xl border border-border/70 bg-background/70 p-4">
              <div class="flex items-start justify-between gap-3">
                <div>
                  <p class="text-xs uppercase tracking-[0.16em] text-muted-foreground">
                    {{ t("dashboard.platform.currentClientLabel") }}
                  </p>
                  <p class="mt-2 text-sm font-medium text-foreground">
                    {{ formatControlClient(currentClientKind) }}
                  </p>
                </div>
                <Badge
                  variant="outline"
                  class="border-sky-500/30 bg-sky-500/12 text-sky-700 dark:text-sky-100"
                >
                  {{ t("dashboard.platform.currentlyUsing") }}
                </Badge>
              </div>
              <div class="mt-4 flex flex-wrap gap-2">
                <Badge
                  variant="outline"
                  :class="platformCapabilityStateClass(activePlatformCapability?.mobileOptimized ?? false)"
                >
                  {{
                    activePlatformCapability?.mobileOptimized
                      ? t("dashboard.platform.mobileOptimized")
                      : t("dashboard.platform.desktopOptimized")
                  }}
                </Badge>
                <Badge
                  variant="outline"
                  :class="platformCapabilityStateClass(activePlatformCapability?.supportsSystemNotifications ?? false)"
                >
                  {{
                    activePlatformCapability?.supportsSystemNotifications
                      ? t("dashboard.platform.systemNotifications")
                      : t("dashboard.platform.inAppOnly")
                  }}
                </Badge>
                <Badge
                  variant="outline"
                  :class="platformCapabilityStateClass(activePlatformCapability?.supportsPersistedRuntimeConfig ?? false)"
                >
                  {{
                    activePlatformCapability?.supportsPersistedRuntimeConfig
                      ? t("dashboard.platform.persistedConfig")
                      : t("dashboard.platform.sessionOnlyConfig")
                  }}
                </Badge>
                <Badge
                  variant="outline"
                  :class="platformCapabilityStateClass(activePlatformCapability?.prefersExplicitRemoteRelayUrl ?? false)"
                >
                  {{
                    activePlatformCapability?.prefersExplicitRemoteRelayUrl
                      ? t("dashboard.platform.explicitRelay")
                      : t("dashboard.platform.loopbackFriendly")
                  }}
                </Badge>
              </div>
            </div>
          </div>
        </CardContent>
      </Card>

      <Card
        v-if="showGovernanceSurface"
        class="border-border/70 bg-card/80 shadow-xl backdrop-blur-xl"
      >
        <CardHeader class="space-y-1">
          <CardTitle class="flex items-center gap-2 text-foreground">
            <ShieldCheck class="size-4 text-violet-300" />
            {{ t("dashboard.governance.title") }}
          </CardTitle>
          <CardDescription>{{ t("dashboard.governance.description") }}</CardDescription>
        </CardHeader>
        <CardContent class="grid gap-4 pt-0">
          <div class="grid gap-3 md:grid-cols-2">
            <div class="rounded-2xl border border-border/70 bg-background/55 p-4">
              <p class="text-xs uppercase tracking-[0.16em] text-muted-foreground">
                {{ t("dashboard.governance.tenant") }}
              </p>
              <p class="mt-2 font-mono text-xs text-foreground">
                {{ currentActor?.tenantId ?? t("common.pending") }}
              </p>
            </div>
            <div class="rounded-2xl border border-border/70 bg-background/55 p-4">
              <p class="text-xs uppercase tracking-[0.16em] text-muted-foreground">
                {{ t("dashboard.governance.user") }}
              </p>
              <p class="mt-2 font-mono text-xs text-foreground">
                {{ currentActor?.userId ?? t("common.pending") }}
              </p>
            </div>
            <div class="rounded-2xl border border-border/70 bg-background/55 p-4">
              <p class="text-xs uppercase tracking-[0.16em] text-muted-foreground">
                {{ t("dashboard.governance.role") }}
              </p>
              <p class="mt-2 text-sm font-medium text-foreground">
                {{ currentActor ? formatUserRole(currentActor.role) : t("common.pending") }}
              </p>
            </div>
            <div class="rounded-2xl border border-border/70 bg-background/55 p-4">
              <p class="text-xs uppercase tracking-[0.16em] text-muted-foreground">
                {{ t("dashboard.governance.notificationChannels") }}
              </p>
              <div class="mt-2 flex flex-wrap gap-2">
                <Badge
                  v-for="channel in appConfig?.notificationChannels ?? []"
                  :key="channel"
                  variant="outline"
                  class="border-border/70 bg-background/70 text-foreground"
                >
                  {{ formatNotificationChannel(channel) }}
                </Badge>
              </div>
            </div>
          </div>

          <div class="rounded-2xl border border-border/70 bg-background/55 p-4">
            <div class="mb-4 flex items-center justify-between gap-3">
              <p class="text-sm font-medium text-foreground">
                {{ t("dashboard.governance.auditTitle") }}
              </p>
              <Badge variant="outline" class="border-border/70 bg-background/70 text-foreground">
                {{ auditRecords.length }}
              </Badge>
            </div>

            <div
              v-if="!auditTrail.length"
              class="rounded-2xl border border-dashed border-border/70 bg-background/70 p-4 text-sm text-muted-foreground"
            >
              {{ t("dashboard.governance.auditEmpty") }}
            </div>
            <ScrollArea v-else class="h-[18rem] pr-3">
              <div class="space-y-3">
                <div
                  v-for="record in auditTrail"
                  :key="record.id"
                  class="rounded-2xl border border-border/70 bg-background/70 p-4"
                >
                  <div class="flex flex-col gap-2 sm:flex-row sm:items-start sm:justify-between">
                    <div class="space-y-2">
                      <div class="flex flex-wrap items-center gap-2">
                        <Badge variant="outline" class="border-border/70 bg-background/55 text-foreground">
                          {{ formatAuditAction(record.action) }}
                        </Badge>
                        <Badge variant="outline" :class="auditOutcomeClass(record.outcome)">
                          {{ formatAuditOutcome(record.outcome) }}
                        </Badge>
                      </div>
                      <p class="text-sm text-foreground">
                        {{ record.resourceKind }} · {{ record.resourceId }}
                      </p>
                      <p class="font-mono text-xs text-muted-foreground">
                        {{ record.userId }} · {{ formatUserRole(record.actorRole) }}
                      </p>
                      <p v-if="record.message" class="text-xs text-muted-foreground">
                        {{ record.message }}
                      </p>
                    </div>
                    <span class="text-xs text-muted-foreground">
                      {{ formatTimestamp(record.timestampEpochMs) }}
                    </span>
                  </div>
                </div>
              </div>
            </ScrollArea>
          </div>
        </CardContent>
      </Card>
    </section>
  </section>
</template>
