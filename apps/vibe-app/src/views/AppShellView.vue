<script setup lang="ts">
import { onMounted, onUnmounted } from "vue";
import { RouterView } from "vue-router";
import { useAppStore } from "@/stores/app";

const store = useAppStore();

onMounted(async () => {
  if (!store.isBootstrapping && !store.lastRefreshEpochMs) {
    await store.bootstrap();
  }
});

onUnmounted(() => {
  store.stopAutoRefresh();
});
</script>

<template>
  <div class="min-h-screen bg-[radial-gradient(circle_at_top,rgba(249,115,22,0.14),transparent_28%),linear-gradient(180deg,#fffdf8_0%,#f4f4f0_100%)] text-foreground dark:bg-[radial-gradient(circle_at_top,rgba(249,115,22,0.12),transparent_24%),linear-gradient(180deg,#111215_0%,#09090b_100%)]">
    <RouterView />
  </div>
</template>
