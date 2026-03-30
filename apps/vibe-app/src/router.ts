import { createRouter, createWebHashHistory } from "vue-router"
import type { RouteRecordRaw } from "vue-router"
import AppShellView from "./views/AppShellView.vue"
import HomeView from "./views/HomeView.vue"
import ProjectWorkspaceView from "./views/ProjectWorkspaceView.vue"
import SettingsView from "./views/SettingsView.vue"

const routes: RouteRecordRaw[] = [
  {
    path: "/",
    component: AppShellView,
    meta: {
      titleKey: "app.title"
    },
    children: [
      {
        path: "",
        name: "home",
        component: HomeView,
        meta: {
          titleKey: "app.title"
        }
      },
      {
        path: "projects/:deviceId/:projectPath(.*)",
        name: "project-workspace",
        component: ProjectWorkspaceView,
        meta: {
          titleKey: "app.title"
        }
      },
      {
        path: "projects",
        redirect: {
          name: "home"
        }
      },
      {
        path: "settings",
        name: "settings",
        component: SettingsView,
        meta: {
          titleKey: "app.title"
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
