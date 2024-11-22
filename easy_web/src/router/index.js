/**********************************
 * @Author: Ronnie Zhang
 * @LastEditor: Ronnie Zhang
 * @LastEditTime: 2023/12/05 21:25:23
 * @Email: zclzone@outlook.com
 * Copyright © 2023 Ronnie Zhang(大脸怪) | https://isme.top
 **********************************/

import { createRouter, createWebHistory, createWebHashHistory } from 'vue-router'
import { setupRouterGuards } from './guards'
import { useAuthStore, usePermissionStore, useUserStore } from '@/store'
import { basicRoutes } from './basic-routes'

export const router = createRouter({
  history:
    import.meta.env.VITE_USE_HASH === 'true'
      ? createWebHashHistory(import.meta.env.VITE_PUBLIC_PATH || '/')
      : createWebHistory(import.meta.env.VITE_PUBLIC_PATH || '/'),
  routes: basicRoutes,
  scrollBehavior: () => ({ left: 0, top: 0 }),
})

export async function setupRouter(app) {
  try {
    await initUserAndPermissions()
  } catch (error) {
    console.error('🚀 初始化失败', error)
  }
  setupRouterGuards(router)
  app.use(router)
}

export async function initUserAndPermissions(data) {
  const permissionStore = usePermissionStore()
  const userStore = useUserStore()
  const authStore = useAuthStore()

  if (!authStore.accessToken) {
    const route = unref(router.currentRoute)
    if (route.path !== '/login' && route.path !== '/frontEnd') {
      router.replace({
        path: '/login',
        query: route.query,
      })
    }
    return
  }
  if(data){
    userStore.setUser(data.user)
    userStore.setRole(data.roles)
    permissionStore.setPermissions(data.permissions);
  }
  permissionStore.initPermissions()
  permissionStore.accessRoutes.forEach((route) => {
    console.log("accessRoutes",route)
    !router.hasRoute(route.name) && router.addRoute(route)
  })
}
