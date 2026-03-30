<script setup lang="ts">
import { computed } from "vue";
import { useI18n } from "vue-i18n";
import StatusBadge from "@/components/common/StatusBadge.vue";
import { formatRelativeTime } from "@/lib/format";
import { getSupportedLocales, setAppLocale } from "@/lib/i18n";
import { getSupportedThemeModes, setThemeMode, useTheme } from "@/lib/theme";
import { useAppStore } from "@/stores/app";

const store = useAppStore();
const { themeMode } = useTheme();
const { t } = useI18n();
const locales = getSupportedLocales();
const themeModes = getSupportedThemeModes();
const currentLocale = computed(() => document.documentElement.lang || "en");
const recentProjects = computed(() => store.recentProjects.slice(0, 6));

async function save() {
  await store.saveRelaySettings();
}
</script>

<template>
  <section class="mx-auto flex min-h-screen w-full max-w-6xl flex-col gap-4 px-4 py-4 md:px-6">
    <header class="rounded-[1.6rem] border border-white/60 bg-white/85 px-5 py-5 shadow-[0_20px_60px_-40px_rgba(15,23,42,0.55)] backdrop-blur dark:border-white/10 dark:bg-slate-950/75">
      <div class="flex flex-wrap items-center gap-2">
        <StatusBadge>{{ t("settings.badge") }}</StatusBadge>
        <StatusBadge :tone="store.onlineHostCount ? 'success' : 'muted'">
          {{ t("dashboard.hostsOnline", { count: store.onlineHostCount }) }}
        </StatusBadge>
      </div>
      <h2 class="mt-3 text-2xl font-semibold">{{ t("settings.title") }}</h2>
      <p class="mt-2 text-sm text-muted-foreground">{{ t("settings.summary") }}</p>
    </header>

    <div class="grid gap-4 xl:grid-cols-[minmax(0,1.1fr)_minmax(320px,0.9fr)]">
      <article class="rounded-[1.6rem] border border-white/60 bg-white/85 p-5 backdrop-blur dark:border-white/10 dark:bg-slate-950/75">
        <div class="flex items-center justify-between gap-3">
          <div>
            <h3 class="text-lg font-semibold">{{ t("settings.serverTitle") }}</h3>
            <p class="mt-1 text-sm text-muted-foreground">{{ t("settings.serverSummary") }}</p>
          </div>
          <button class="rounded-full bg-primary px-4 py-2.5 text-sm font-medium text-primary-foreground" @click="save">
            {{ t("settings.save") }}
          </button>
        </div>

        <div class="mt-5 grid gap-3">
          <label class="grid gap-2 text-sm">
            <span class="font-medium">{{ t("settings.relayUrl") }}</span>
            <input
              v-model="store.relayBaseUrlInput"
              type="url"
              class="w-full rounded-2xl border border-border bg-background px-4 py-3"
              placeholder="https://relay.example.com"
            />
          </label>

          <label class="grid gap-2 text-sm">
            <span class="font-medium">{{ t("settings.accessToken") }}</span>
            <input
              v-model="store.relayAccessTokenInput"
              type="password"
              class="w-full rounded-2xl border border-border bg-background px-4 py-3"
              :placeholder="t('settings.accessTokenPlaceholder')"
            />
          </label>

          <div class="rounded-2xl border border-border bg-background/70 px-4 py-3 text-xs text-muted-foreground">
            {{ t("settings.currentServer", { value: store.relayBaseUrl || t("settings.notConfigured") }) }}
          </div>
        </div>
      </article>

      <article class="rounded-[1.6rem] border border-white/60 bg-white/85 p-5 backdrop-blur dark:border-white/10 dark:bg-slate-950/75">
        <h3 class="text-lg font-semibold">{{ t("settings.hostsTitle") }}</h3>
        <div class="mt-4 space-y-2">
          <div
            v-for="host in store.hostSummaries"
            :key="host.device.id"
            class="flex items-center justify-between rounded-2xl border border-border bg-background/70 px-4 py-3"
          >
            <div class="min-w-0">
              <p class="truncate text-sm font-medium">{{ host.device.name }}</p>
              <p class="mt-1 text-xs text-muted-foreground">
                {{ t("common.projectsCount", { count: host.projectCount }) }} ·
                {{ t("dashboard.runningTasks", { count: host.runningTaskCount }) }}
              </p>
            </div>
            <StatusBadge :tone="host.device.online ? 'success' : 'muted'">
              {{ host.device.online ? t("common.online") : t("common.offline") }}
            </StatusBadge>
          </div>
        </div>
      </article>
    </div>

    <div class="grid gap-4 xl:grid-cols-[minmax(0,1.1fr)_minmax(320px,0.9fr)]">
      <article class="rounded-[1.6rem] border border-white/60 bg-white/85 p-5 backdrop-blur dark:border-white/10 dark:bg-slate-950/75">
        <h3 class="text-lg font-semibold">{{ t("settings.projectsTitle") }}</h3>
        <div class="mt-4 space-y-2">
          <div
            v-for="project in recentProjects"
            :key="project.key"
            class="rounded-2xl border border-border bg-background/70 px-4 py-3"
          >
            <div class="flex items-start justify-between gap-3">
              <div class="min-w-0">
                <p class="truncate text-sm font-medium">{{ project.title }}</p>
                <p class="mt-1 truncate text-xs text-muted-foreground">
                  {{ project.deviceName }} · {{ project.pathLabel }}
                </p>
              </div>
              <StatusBadge :tone="project.runningTaskCount ? 'default' : project.failedTaskCount ? 'danger' : 'muted'">
                {{ formatRelativeTime(project.updatedAtEpochMs) }}
              </StatusBadge>
            </div>
          </div>
          <div
            v-if="!recentProjects.length"
            class="rounded-2xl border border-dashed border-border bg-background/70 px-4 py-5 text-sm text-muted-foreground"
          >
            {{ t("settings.projectsEmpty") }}
          </div>
        </div>
      </article>

      <div class="space-y-4">
        <article class="rounded-[1.6rem] border border-white/60 bg-white/85 p-5 backdrop-blur dark:border-white/10 dark:bg-slate-950/75">
          <h3 class="text-lg font-semibold">{{ t("settings.language") }}</h3>
          <div class="mt-4 flex flex-wrap gap-2">
            <button
              v-for="locale in locales"
              :key="locale"
              class="rounded-full border px-4 py-2 text-sm"
              :class="
                currentLocale === locale
                  ? 'border-primary bg-primary text-primary-foreground'
                  : 'border-border'
              "
              @click="setAppLocale(locale)"
            >
              {{ locale }}
            </button>
          </div>
        </article>

        <article class="rounded-[1.6rem] border border-white/60 bg-white/85 p-5 backdrop-blur dark:border-white/10 dark:bg-slate-950/75">
          <h3 class="text-lg font-semibold">{{ t("settings.theme") }}</h3>
          <div class="mt-4 flex flex-wrap gap-2">
            <button
              v-for="mode in themeModes"
              :key="mode"
              class="rounded-full border px-4 py-2 text-sm capitalize"
              :class="
                themeMode === mode
                  ? 'border-primary bg-primary text-primary-foreground'
                  : 'border-border'
              "
              @click="setThemeMode(mode)"
            >
              {{ mode }}
            </button>
          </div>
        </article>
      </div>
    </div>
  </section>
</template>
