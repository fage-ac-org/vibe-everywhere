import type { AppConfig } from "../types";

const RELAY_STORAGE_KEY = "vibe.remote.relay.baseUrl";
const RELAY_ACCESS_TOKEN_STORAGE_KEY = "vibe.remote.relay.accessToken";

export async function resolveInitialRelayBaseUrl(): Promise<string> {
  const saved = window.localStorage.getItem(RELAY_STORAGE_KEY);
  if (saved) {
    return saved;
  }

  const tauriConfig = await loadTauriConfig();
  if (tauriConfig?.defaultRelayBaseUrl) {
    return tauriConfig.defaultRelayBaseUrl;
  }

  return import.meta.env.VITE_RELAY_BASE_URL ?? "";
}

export function resolveInitialRelayAccessToken(): string {
  const saved = window.localStorage.getItem(RELAY_ACCESS_TOKEN_STORAGE_KEY);
  if (saved) {
    return saved;
  }

  return import.meta.env.VITE_RELAY_ACCESS_TOKEN ?? "";
}

export async function loadTauriConfig(): Promise<AppConfig | null> {
  try {
    const { invoke } = await import("@tauri-apps/api/core");
    return await invoke<AppConfig>("app_config");
  } catch {
    return null;
  }
}

export function persistRelayBaseUrl(baseUrl: string) {
  window.localStorage.setItem(RELAY_STORAGE_KEY, baseUrl);
}

export function persistRelayAccessToken(accessToken: string) {
  window.localStorage.setItem(RELAY_ACCESS_TOKEN_STORAGE_KEY, accessToken);
}

export function buildApiUrl(baseUrl: string, path: string): string {
  if (!baseUrl) {
    return path;
  }

  return `${baseUrl}${path}`;
}

export function buildEventStreamUrl(baseUrl: string, accessToken: string): string {
  const url = buildApiUrl(baseUrl, "/api/events/stream");
  if (!accessToken.trim()) {
    return url;
  }

  const separator = url.includes("?") ? "&" : "?";
  return `${url}${separator}access_token=${encodeURIComponent(accessToken.trim())}`;
}

export function buildWebSocketUrl(
  baseUrl: string,
  path: string,
  accessToken: string
): string {
  const rawUrl = buildApiUrl(baseUrl, path);
  const url = new URL(rawUrl, window.location.origin);
  url.protocol = url.protocol === "https:" ? "wss:" : "ws:";
  if (accessToken.trim()) {
    url.searchParams.set("access_token", accessToken.trim());
  }
  return url.toString();
}
