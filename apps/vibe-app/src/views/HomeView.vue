<script setup lang="ts">
import { computed, watch } from "vue";
import { useRouter } from "vue-router";
import { useI18n } from "vue-i18n";
import { buildProjectRouteParam } from "@/lib/projects";
import { useAppStore } from "@/stores/app";

const store = useAppStore();
const router = useRouter();
const { t } = useI18n();

const launchProject = computed(() => store.recentProjects[0] ?? store.projectSummaries[0] ?? null);

function openProject(deviceId: string, cwd: string | null) {
  store.markProjectVisited(deviceId, cwd);
  void router.replace({
    name: "project-workspace",
    params: {
      deviceId,
      projectPath: buildProjectRouteParam(cwd)
    }
  });
}

watch(launchProject, (value) => {
  if (value) {
    openProject(value.deviceId, value.cwd);
  }
}, { immediate: true });
</script>

<template>
  <section class="mx-auto flex min-h-screen max-w-3xl items-center px-6 py-10">
    <div class="w-full rounded-[2rem] border border-white/60 bg-white/85 p-8 shadow-[0_32px_90px_-45px_rgba(15,23,42,0.5)] backdrop-blur dark:border-white/10 dark:bg-slate-950/60">
      <div class="space-y-3">
        <p class="text-xs font-semibold uppercase tracking-[0.26em] text-muted-foreground">
          {{ t("dashboard.badge") }}
        </p>
        <h1 class="text-3xl font-semibold">
          {{
            store.relayBaseUrl
              ? launchProject
                ? t("home.openingChat")
                : t("home.emptyProjectsTitle")
              : t("home.emptyServerTitle")
          }}
        </h1>
        <p class="text-sm text-muted-foreground">
          {{
            store.relayBaseUrl
              ? launchProject
                ? `${launchProject.deviceName} · ${launchProject.pathLabel}`
                : t("home.emptyProjectsSummary")
              : t("home.emptyServerSummary")
          }}
        </p>
      </div>

      <div class="mt-8 flex flex-wrap gap-3">
        <button
          v-if="launchProject"
          class="rounded-full bg-primary px-5 py-3 text-sm font-medium text-primary-foreground"
          @click="openProject(launchProject.deviceId, launchProject.cwd)"
        >
          {{ t("common.openProject") }}
        </button>
        <button
          class="rounded-full border border-border px-5 py-3 text-sm font-medium"
          @click="router.push({ name: 'settings' })"
        >
          {{ t("nav.settings") }}
        </button>
      </div>
    </div>
  </section>
</template>
