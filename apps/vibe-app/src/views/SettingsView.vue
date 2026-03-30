<script setup lang="ts">
import { computed } from "vue";
import { useI18n } from "vue-i18n";
import StatusBadge from "@/components/common/StatusBadge.vue";
import { getSupportedLocales, setAppLocale } from "@/lib/i18n";
import { getSupportedThemeModes, setThemeMode, useTheme } from "@/lib/theme";
import { useAppStore } from "@/stores/app";

const store = useAppStore();
const { themeMode } = useTheme();
const { t } = useI18n();
const locales = getSupportedLocales();
const themeModes = getSupportedThemeModes();
const currentLocale = computed(() => document.documentElement.lang || "en");

async function save() {
  await store.saveRelaySettings();
}
</script>

<template>
  <section class="space-y-5">
    <div class="rounded-[1.8rem] border border-white/55 bg-white/80 p-5 backdrop-blur dark:border-white/10 dark:bg-slate-950/55">
      <StatusBadge>{{ t("settings.badge") }}</StatusBadge>
      <h2 class="mt-3 text-xl font-semibold">{{ t("settings.title") }}</h2>
      <p class="mt-2 max-w-2xl text-sm text-muted-foreground">
        {{ t("settings.summary") }}
      </p>
    </div>

    <section class="grid gap-5 xl:grid-cols-[1.1fr_0.9fr]">
      <article class="rounded-[1.6rem] border border-white/55 bg-white/80 p-5 backdrop-blur dark:border-white/10 dark:bg-slate-950/55">
        <h3 class="text-lg font-semibold">{{ t("settings.serverTitle") }}</h3>
        <p class="mt-2 text-sm text-muted-foreground">
          {{ t("settings.serverSummary") }}
        </p>
        <div class="mt-5 space-y-4">
          <label class="block space-y-2 text-sm">
            <span class="font-medium">{{ t("settings.relayUrl") }}</span>
            <input
              v-model="store.relayBaseUrlInput"
              type="url"
              class="w-full rounded-2xl border border-border bg-background px-4 py-3"
              placeholder="https://relay.example.com"
            />
          </label>
          <label class="block space-y-2 text-sm">
            <span class="font-medium">{{ t("settings.accessToken") }}</span>
            <input
              v-model="store.relayAccessTokenInput"
              type="password"
              class="w-full rounded-2xl border border-border bg-background px-4 py-3"
              :placeholder="t('settings.accessTokenPlaceholder')"
            />
          </label>
          <button class="rounded-full bg-primary px-4 py-2.5 text-sm font-medium text-primary-foreground" @click="save">
            {{ t("settings.save") }}
          </button>
          <p class="text-xs text-muted-foreground">
            {{ t("settings.currentServer", { value: store.relayBaseUrl || t("settings.notConfigured") }) }}
          </p>
        </div>
      </article>

      <div class="space-y-5">
        <article class="rounded-[1.6rem] border border-white/55 bg-white/80 p-5 backdrop-blur dark:border-white/10 dark:bg-slate-950/55">
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

        <article class="rounded-[1.6rem] border border-white/55 bg-white/80 p-5 backdrop-blur dark:border-white/10 dark:bg-slate-950/55">
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
    </section>
  </section>
</template>
