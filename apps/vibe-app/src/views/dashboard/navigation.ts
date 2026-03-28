import { markRaw } from "vue"
import { Cable, Server, Sparkles, TerminalSquare } from "lucide-vue-next"

export type DashboardRouteName =
  | "dashboard-sessions"
  | "dashboard-devices"
  | "dashboard-connections"
  | "dashboard-advanced"

export const dashboardSections = [
  {
    routeName: "dashboard-sessions" as const,
    path: "sessions",
    titleKey: "dashboard.nav.sessions",
    descriptionKey: "dashboard.navDescriptions.sessions",
    icon: markRaw(Sparkles)
  },
  {
    routeName: "dashboard-devices" as const,
    path: "devices",
    titleKey: "dashboard.nav.devices",
    descriptionKey: "dashboard.navDescriptions.devices",
    icon: markRaw(Server)
  },
  {
    routeName: "dashboard-connections" as const,
    path: "connections",
    titleKey: "dashboard.nav.connections",
    descriptionKey: "dashboard.navDescriptions.connections",
    icon: markRaw(Cable)
  },
  {
    routeName: "dashboard-advanced" as const,
    path: "advanced",
    titleKey: "dashboard.nav.advanced",
    descriptionKey: "dashboard.navDescriptions.advanced",
    icon: markRaw(TerminalSquare)
  }
]
