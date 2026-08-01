import { createRouter, createWebHistory } from 'vue-router'
import welcomeView from '@/views/welcome/welcomeView.vue'
import loginView from '@/views/welcome/loginView.vue'
import registerView from '@/views/welcome/registerView.vue'
import mainPanelView from '@/views/panel/mainPanelView.vue'

import fileExplorerView from '@/views/panel/views/fileExplorerView.vue'
import sharedFilesView from '@/views/panel/views/sharedFilesView.vue'
import accountView from '@/views/panel/views/accountView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL || '/'),
  routes: [
    {
      path: "",
      name: "welcome",
      component: welcomeView,
    },
    {
      path: "/login",
      name: "login",
      component: loginView,
    },
    {
      path: "/register",
      name: "register",
      component: registerView,
    },
    {
      path: "/panel",
      name: "panel",
      component: mainPanelView,
      children: [
        {
          path: "storage/:token?",
          name: "storage",
          component: fileExplorerView,
        },
        {
          path: "shared",
          name: "shared",
          component: sharedFilesView,
        },
        {
          path: "account",
          name: "account",
          component: accountView,
        }
      ]
    },
    {
      path: '/:pathMatch(.*)*',
      component: mainPanelView,
    }
  ],
})

export default router
