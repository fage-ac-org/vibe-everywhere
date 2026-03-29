<script setup lang="ts">
import { computed } from "vue";
import { RouterLink } from "vue-router";
import { useI18n } from "vue-i18n";
import {
  FolderCode,
  FolderTree,
  Home,
  LoaderCircle,
  MessageSquarePlus,
  RefreshCw,
  Server,
  Settings2,
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
import type {
  DeviceRecord,
  WorkspaceEntry,
  WorkspaceFilePreviewResponse,
} from "@/types";
import type { ConversationListItem } from "@/views/dashboard/session/types";

const props = defineProps<{
  relayInput: string;
  relayAccessTokenInput: string;
  relayPlaceholder: string;
  connectionStateLabel: string;
  selectedDeviceId: string | null;
  selectedDeviceName: string;
  devices: DeviceRecord[];
  nativeSelectClass: string;
  availableProviders: { kind: string; label: string }[];
  selectedProvider: string;
  projectFolder: string;
  projectFolderPlaceholder: string;
  setupStateLabel: string;
  selectedConversationId: string | null;
  conversationItems: ConversationListItem[];
  workspaceLoading: boolean;
  workspaceError: string;
  workspacePath: string;
  workspaceRootPath: string;
  workspaceEntries: WorkspaceEntry[];
  workspacePreview: WorkspaceFilePreviewResponse | null;
  workspacePreviewLoading: boolean;
  selectedWorkspacePath: string | null;
}>();

const emit = defineEmits<{
  (event: "update:relayInput", value: string): void;
  (event: "update:relayAccessTokenInput", value: string): void;
  (event: "connectRelay"): void;
  (event: "selectDevice", value: string): void;
  (event: "chooseProvider", value: string): void;
  (event: "updateProjectFolder", value: string): void;
  (event: "startNewConversation"): void;
  (event: "selectConversation", value: string): void;
  (event: "refreshWorkspace"): void;
  (event: "navigateWorkspaceUp"): void;
  (event: "navigateWorkspaceRoot"): void;
  (event: "openWorkspaceEntry", value: WorkspaceEntry): void;
}>();

const { t } = useI18n();
const hasWorkspaceEntries = computed(() => props.workspaceEntries.length > 0);

function handleDeviceChange(event: Event) {
  emit("selectDevice", (event.target as HTMLSelectElement).value);
}
</script>

<template>
  <div
    class="grid gap-4 xl:sticky xl:top-4 xl:h-[calc(100vh-2rem)] xl:grid-rows-[auto_minmax(0,1fr)_minmax(0,1.1fr)]"
  >
    <Card class="border-border/70 bg-card/85 shadow-xl backdrop-blur-xl">
      <CardHeader class="space-y-2">
        <Badge
          variant="outline"
          class="w-fit border-amber-500/30 bg-amber-500/10 text-amber-700 dark:text-amber-100"
        >
          {{ t("dashboard.chat.liveBadge") }}
        </Badge>
        <div class="space-y-1">
          <CardTitle class="text-base">{{
            t("dashboard.chat.sidebarTitle")
          }}</CardTitle>
          <CardDescription>{{
            t("dashboard.chat.sidebarSummary")
          }}</CardDescription>
        </div>
      </CardHeader>
      <CardContent class="space-y-4 pt-0">
        <div class="grid gap-3">
          <div class="rounded-2xl border border-border/70 bg-background/55 p-3">
            <p
              class="text-[11px] uppercase tracking-[0.18em] text-muted-foreground"
            >
              {{ t("dashboard.shell.connectionState") }}
            </p>
            <p class="mt-2 text-sm font-medium text-foreground">
              {{ connectionStateLabel }}
            </p>
          </div>
          <div class="rounded-2xl border border-border/70 bg-background/55 p-3">
            <p
              class="text-[11px] uppercase tracking-[0.18em] text-muted-foreground"
            >
              {{ t("common.status") }}
            </p>
            <p class="mt-2 text-sm font-medium text-foreground">
              {{ setupStateLabel }}
            </p>
          </div>
        </div>

        <div class="space-y-2">
          <label
            class="text-[11px] font-medium uppercase tracking-[0.18em] text-muted-foreground"
          >
            {{ t("dashboard.relayBaseUrl") }}
          </label>
          <div class="flex gap-2">
            <Input
              :model-value="relayInput"
              :placeholder="relayPlaceholder"
              class="border-border/70 bg-background/75"
              @update:model-value="emit('update:relayInput', String($event))"
            />
            <Button
              type="button"
              class="shrink-0"
              @click="emit('connectRelay')"
            >
              <Server class="size-4" />
            </Button>
          </div>
        </div>

        <div class="space-y-2">
          <label
            class="text-[11px] font-medium uppercase tracking-[0.18em] text-muted-foreground"
          >
            {{ t("dashboard.fields.accessToken") }}
          </label>
          <Input
            :model-value="relayAccessTokenInput"
            type="password"
            :placeholder="t('common.optionalAccessToken')"
            class="border-border/70 bg-background/75"
            @update:model-value="
              emit('update:relayAccessTokenInput', String($event))
            "
          />
        </div>

        <div class="space-y-2">
          <label
            class="text-[11px] font-medium uppercase tracking-[0.18em] text-muted-foreground"
          >
            {{ t("dashboard.shell.selectedDevice") }}
          </label>
          <select
            :value="selectedDeviceId ?? ''"
            :class="nativeSelectClass"
            @change="handleDeviceChange"
          >
            <option value="" disabled>
              {{ t("dashboard.shell.noDeviceSelected") }}
            </option>
            <option
              v-for="device in devices"
              :key="device.id"
              :value="device.id"
            >
              {{ device.name }}
            </option>
          </select>
          <p class="text-xs text-muted-foreground">{{ selectedDeviceName }}</p>
        </div>

        <div class="space-y-2">
          <label
            class="text-[11px] font-medium uppercase tracking-[0.18em] text-muted-foreground"
          >
            {{ t("common.provider") }}
          </label>
          <div class="flex flex-wrap gap-2">
            <Button
              v-for="provider in availableProviders"
              :key="provider.kind"
              type="button"
              variant="outline"
              size="sm"
              :class="
                selectedProvider === provider.kind
                  ? 'border-sky-500/40 bg-sky-500/12 text-sky-900 dark:text-sky-100'
                  : 'border-border/70 bg-background/55 text-foreground hover:bg-accent/40'
              "
              @click="emit('chooseProvider', provider.kind)"
            >
              {{ provider.label }}
            </Button>
          </div>
        </div>

        <div class="space-y-2">
          <label
            class="text-[11px] font-medium uppercase tracking-[0.18em] text-muted-foreground"
          >
            {{ t("dashboard.chat.projectFolder") }}
          </label>
          <Input
            :model-value="projectFolder"
            :placeholder="projectFolderPlaceholder"
            class="border-border/70 bg-background/75"
            @update:model-value="emit('updateProjectFolder', String($event))"
          />
          <p class="text-xs text-muted-foreground">
            {{ t("dashboard.chat.projectFolderSummary") }}
          </p>
        </div>

        <div class="grid grid-cols-2 gap-2">
          <Button
            type="button"
            variant="outline"
            @click="emit('startNewConversation')"
          >
            <MessageSquarePlus class="size-4" />
            {{ t("dashboard.chat.newConversation") }}
          </Button>
          <Button
            type="button"
            variant="outline"
            @click="emit('refreshWorkspace')"
          >
            <RefreshCw class="size-4" />
            {{ t("common.refresh") }}
          </Button>
        </div>

        <div class="grid grid-cols-2 gap-2">
          <RouterLink
            :to="{ name: 'dashboard-devices' }"
            class="inline-flex items-center justify-center gap-2 rounded-xl border border-border/70 bg-background/55 px-3 py-2 text-sm text-foreground transition hover:bg-accent/40"
          >
            <Settings2 class="size-4" />
            {{ t("dashboard.chat.devicesEntry") }}
          </RouterLink>
          <RouterLink
            :to="{ name: 'dashboard-advanced' }"
            class="inline-flex items-center justify-center gap-2 rounded-xl border border-border/70 bg-background/55 px-3 py-2 text-sm text-foreground transition hover:bg-accent/40"
          >
            <FolderCode class="size-4" />
            {{ t("dashboard.nav.advanced") }}
          </RouterLink>
        </div>
      </CardContent>
    </Card>

    <Card
      class="min-h-0 border-border/70 bg-card/80 shadow-xl backdrop-blur-xl"
    >
      <CardHeader class="space-y-1">
        <div class="flex items-center justify-between gap-3">
          <div>
            <CardTitle class="text-base">{{
              t("dashboard.chat.historyTitle")
            }}</CardTitle>
            <CardDescription>{{
              t("dashboard.chat.historySummary")
            }}</CardDescription>
          </div>
          <Badge
            variant="outline"
            class="border-border/70 bg-background/60 text-foreground"
          >
            {{ conversationItems.length }}
          </Badge>
        </div>
      </CardHeader>
      <CardContent class="min-h-0 pt-0">
        <ScrollArea class="h-[18rem] xl:h-full">
          <div class="space-y-2 pr-3">
            <button
              type="button"
              class="w-full rounded-2xl border border-dashed border-border/70 bg-background/45 px-4 py-3 text-left transition hover:bg-accent/35"
              :class="
                !selectedConversationId ? 'border-primary/50 bg-primary/10' : ''
              "
              @click="emit('startNewConversation')"
            >
              <p class="text-sm font-medium text-foreground">
                {{ t("dashboard.chat.startBlank") }}
              </p>
              <p class="mt-1 text-xs leading-5 text-muted-foreground">
                {{ t("dashboard.chat.startBlankSummary") }}
              </p>
            </button>

            <button
              v-for="conversation in conversationItems"
              :key="conversation.id"
              type="button"
              class="w-full rounded-2xl border px-4 py-3 text-left transition"
              :class="
                selectedConversationId === conversation.id
                  ? 'border-primary/50 bg-primary/12'
                  : 'border-border/70 bg-background/55 hover:bg-accent/35'
              "
              @click="emit('selectConversation', conversation.id)"
            >
              <div class="flex items-start justify-between gap-3">
                <div class="min-w-0">
                  <p class="truncate text-sm font-medium text-foreground">
                    {{ conversation.title }}
                  </p>
                  <p class="mt-1 truncate text-xs text-muted-foreground">
                    {{ conversation.meta }}
                  </p>
                </div>
                <Badge
                  variant="outline"
                  class="shrink-0"
                  :class="conversation.statusClass"
                >
                  {{ conversation.status }}
                </Badge>
              </div>
              <p class="mt-3 text-xs text-muted-foreground">
                {{ conversation.updatedAt }}
              </p>
            </button>
          </div>
        </ScrollArea>
      </CardContent>
    </Card>

    <Card
      class="min-h-0 border-border/70 bg-card/80 shadow-xl backdrop-blur-xl"
    >
      <CardHeader class="space-y-1">
        <div class="flex items-center justify-between gap-3">
          <div>
            <CardTitle class="flex items-center gap-2 text-base">
              <FolderTree class="size-4" />
              {{ t("dashboard.workspace.browser.title") }}
            </CardTitle>
            <CardDescription>{{
              t("dashboard.chat.workspaceBrowserSummary")
            }}</CardDescription>
          </div>
          <Badge
            variant="outline"
            class="border-border/70 bg-background/60 text-foreground"
          >
            {{ workspaceEntries.length }}
          </Badge>
        </div>
      </CardHeader>
      <CardContent
        class="grid min-h-0 gap-3 pt-0 xl:grid-rows-[auto_minmax(0,1fr)_auto]"
      >
        <div class="grid grid-cols-3 gap-2">
          <Button
            type="button"
            variant="outline"
            size="sm"
            @click="emit('navigateWorkspaceUp')"
          >
            <Home class="size-4 rotate-180" />
            {{ t("dashboard.workspace.browser.up") }}
          </Button>
          <Button
            type="button"
            variant="outline"
            size="sm"
            @click="emit('navigateWorkspaceRoot')"
          >
            <Home class="size-4" />
            {{ t("dashboard.workspace.browser.root") }}
          </Button>
          <Button
            type="button"
            variant="outline"
            size="sm"
            @click="emit('refreshWorkspace')"
          >
            <RefreshCw class="size-4" />
            {{ t("common.refresh") }}
          </Button>
        </div>

        <div
          class="min-h-0 rounded-2xl border border-border/70 bg-background/45"
        >
          <div class="border-b border-border/70 px-4 py-3">
            <p
              class="text-[11px] uppercase tracking-[0.18em] text-muted-foreground"
            >
              {{ t("dashboard.workspace.browser.path") }}
            </p>
            <p
              class="mt-2 truncate text-sm text-foreground"
              :title="workspacePath || workspaceRootPath"
            >
              {{ workspacePath || workspaceRootPath }}
            </p>
          </div>

          <div
            v-if="workspaceLoading"
            class="flex items-center gap-2 px-4 py-4 text-sm text-muted-foreground"
          >
            <LoaderCircle class="size-4 animate-spin" />
            {{ t("dashboard.workspace.browser.loading") }}
          </div>
          <p
            v-else-if="workspaceError"
            class="px-4 py-4 text-sm text-rose-600 dark:text-rose-200"
          >
            {{ workspaceError }}
          </p>
          <ScrollArea v-else class="h-[15rem] xl:h-full">
            <div class="space-y-1 p-2">
              <button
                v-for="entry in workspaceEntries"
                :key="entry.path"
                type="button"
                class="flex w-full items-center justify-between gap-3 rounded-xl px-3 py-2 text-left transition"
                :class="
                  selectedWorkspacePath === entry.path
                    ? 'bg-primary/12 text-primary'
                    : 'hover:bg-accent/35'
                "
                @click="emit('openWorkspaceEntry', entry)"
              >
                <div class="min-w-0">
                  <p class="truncate text-sm font-medium">
                    {{ entry.name }}
                  </p>
                  <p class="truncate text-xs text-muted-foreground">
                    {{ entry.path }}
                  </p>
                </div>
                <Badge
                  variant="outline"
                  class="shrink-0 border-border/70 bg-background/60 text-foreground"
                >
                  {{ t(`dashboard.workspace.browser.kind.${entry.kind}`) }}
                </Badge>
              </button>
              <p
                v-if="!hasWorkspaceEntries"
                class="px-2 py-3 text-sm text-muted-foreground"
              >
                {{ t("dashboard.workspace.browser.empty") }}
              </p>
            </div>
          </ScrollArea>
        </div>

        <div class="rounded-2xl border border-border/70 bg-background/55 p-4">
          <div
            v-if="workspacePreviewLoading"
            class="flex items-center gap-2 text-sm text-muted-foreground"
          >
            <LoaderCircle class="size-4 animate-spin" />
            {{ t("dashboard.workspace.browser.previewLoading") }}
          </div>
          <template v-else-if="workspacePreview">
            <p class="truncate text-sm font-medium text-foreground">
              {{ workspacePreview.path }}
            </p>
            <p
              v-if="workspacePreview.kind === 'binary'"
              class="mt-2 text-sm text-muted-foreground"
            >
              {{ t("dashboard.workspace.browser.binaryNotice") }}
            </p>
            <template v-else>
              <p class="mt-1 text-xs text-muted-foreground">
                {{ t("dashboard.workspace.browser.lines") }} ·
                {{ workspacePreview.totalLines ?? 0 }}
              </p>
              <pre
                class="mt-3 max-h-48 overflow-auto whitespace-pre-wrap rounded-xl bg-slate-950 p-3 text-xs leading-6 text-slate-100 dark:bg-slate-900"
                >{{ workspacePreview.content }}</pre
              >
            </template>
          </template>
          <p v-else class="text-sm text-muted-foreground">
            {{ t("dashboard.workspace.browser.previewEmpty") }}
          </p>
        </div>
      </CardContent>
    </Card>
  </div>
</template>
