<script setup lang="ts">
import { useI18n } from "vue-i18n"
import { Activity, Laptop2, Server, ShieldCheck, Sparkles } from "lucide-vue-next"
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
import { useDashboardController } from "@/views/dashboard/controller"

const { t } = useI18n()
const dashboard = useDashboardController()

const {
  activePlatformCapability,
  appConfig,
  appName,
  auditRecords,
  auditTrail,
  auditOutcomeClass,
  currentActor,
  currentClientKind,
  deploymentDocsUrl,
  eventState,
  formatAuditAction,
  formatAuditOutcome,
  formatAuthMode,
  formatAuthRequirement,
  formatConnectionState,
  formatControlClient,
  formatDeploymentMode,
  formatNotificationChannel,
  formatStorageKind,
  formatTimestamp,
  formatUserRole,
  health,
  locale,
  localeOptions,
  platformCapabilityStateClass,
  portForwards,
  relayAccessTokenInput,
  relayInput,
  relayPlaceholder,
  shellSessions,
  showGovernanceSurface,
  store,
  tasks,
  themeMode,
  themeOptions,
  switchLocale,
  switchTheme
} = dashboard
</script>

<template>
  <section class="space-y-4">
    <Card class="overflow-hidden border-border/70 bg-card/85 text-foreground shadow-2xl backdrop-blur-xl">
      <CardContent class="grid gap-6 p-6 lg:grid-cols-[1.35fr_1fr]">
        <div class="space-y-4">
          <Badge variant="outline" class="border-sky-500/30 bg-sky-500/12 text-sky-700 dark:text-sky-100">
            <Sparkles class="size-3.5" />
            {{ t("dashboard.heroBadge") }}
          </Badge>
          <div class="space-y-3">
            <h1 class="text-4xl font-semibold tracking-tight text-foreground md:text-6xl">
              {{ appName }}
            </h1>
            <p class="max-w-3xl text-base leading-7 text-muted-foreground md:text-lg">
              {{ t("dashboard.heroDescription") }}
            </p>
          </div>
        </div>

        <div class="rounded-3xl border border-amber-400/20 bg-gradient-to-b from-amber-400/12 via-background/70 to-transparent p-5 shadow-inner">
          <div class="mb-4 flex items-center justify-between gap-3">
            <Badge variant="outline" class="border-amber-400/30 bg-amber-400/12 text-amber-700 dark:text-amber-100">
              <Activity class="size-3.5" />
              SSE {{ formatConnectionState(eventState) }}
            </Badge>
            <span class="text-xs uppercase tracking-[0.18em] text-muted-foreground">
              {{ t("dashboard.relayTitle") }}
            </span>
          </div>

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

            <Separator class="bg-border/70" />

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
                  {{ t("dashboard.stats.advancedTools") }}
                </p>
                <p class="mt-2 text-2xl font-semibold text-foreground">
                  {{ shellSessions.length + portForwards.length }}
                </p>
              </div>
            </div>
          </div>
        </div>
      </CardContent>
    </Card>

    <section class="grid gap-4 2xl:grid-cols-[minmax(0,1.05fr)_minmax(0,0.95fr)]">
      <Card class="border-border/70 bg-card/80 shadow-xl backdrop-blur-xl">
        <CardHeader class="space-y-1">
          <CardTitle class="flex items-center gap-2 text-foreground">
            <Server class="size-4 text-emerald-300" />
            {{ t("dashboard.deployment.title") }}
          </CardTitle>
          <CardDescription>{{ t("dashboard.deployment.summary") }}</CardDescription>
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
                  {{ appConfig?.deployment.relayPublicOrigin || relayInput || t("common.pending") }}
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
