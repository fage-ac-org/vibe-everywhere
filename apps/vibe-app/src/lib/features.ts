import type { AppConfig } from "@/types";

export const APP_FEATURE_FLAGS = {
  relayShellSessions: "relay_shell_sessions",
  relayTcpForwardingControlPlane: "relay_tcp_forwarding_control_plane",
  sessionGitInspect: "session_git_inspect",
  governanceAuditConsole: "governance_audit_console"
} as const;

export type AppFeatureFlag =
  (typeof APP_FEATURE_FLAGS)[keyof typeof APP_FEATURE_FLAGS];

export function hasAppFeatureFlag(
  config: AppConfig | null | undefined,
  flag: AppFeatureFlag
) {
  return Boolean(config?.featureFlags.includes(flag));
}
