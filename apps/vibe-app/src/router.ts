import { createRouter, createWebHashHistory } from "vue-router"
import type { RouteRecordRaw } from "vue-router"
import DashboardView from "./views/DashboardView.vue"
import DashboardAdvancedSection from "./views/dashboard/sections/DashboardAdvancedSection.vue"
import DashboardDevicesSection from "./views/dashboard/sections/DashboardDevicesSection.vue"
import DashboardSessionsSection from "./views/dashboard/sections/DashboardSessionsSection.vue"

const routes: RouteRecordRaw[] = [
  {
    path: "/",
    component: DashboardView,
    meta: {
      titleKey: "app.title"
    },
    children: [
      {
        path: "",
        redirect: {
          name: "dashboard-sessions"
        }
      },
      {
        path: "connections",
        redirect: {
          name: "dashboard-sessions"
        }
      },
      {
        path: "sessions",
        name: "dashboard-sessions",
        component: DashboardSessionsSection,
        meta: {
          titleKey: "dashboard.nav.sessions"
        }
      },
      {
        path: "devices",
        name: "dashboard-devices",
        component: DashboardDevicesSection,
        meta: {
          titleKey: "dashboard.nav.devices"
        }
      },
      {
        path: "advanced",
        name: "dashboard-advanced",
        component: DashboardAdvancedSection,
        meta: {
          titleKey: "dashboard.nav.advanced"
        }
      }
    ]
  }
]

const router = createRouter({
  history: createWebHashHistory(),
  routes,
  scrollBehavior() {
    return { top: 0 }
  }
})

export default router
