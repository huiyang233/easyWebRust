/**********************************
 * @Author: Ronnie Zhang
 * @LastEditor: Ronnie Zhang
 * @LastEditTime: 2023/12/05 21:25:07
 * @Email: zclzone@outlook.com
 * Copyright © 2023 Ronnie Zhang(大脸怪) | https://isme.top
 **********************************/

import {useAuthStore, usePermissionStore} from '@/store'

const WHITE_LIST = ['/login', '/404']
export function createPermissionGuard(router) {
  router.beforeEach(async (to) => {
    const authStore = useAuthStore()
    const permissionStore = usePermissionStore()
    const token = authStore.accessToken

    /** 没有token */
    if (!token) {
      if (WHITE_LIST.includes(to.path)) return true
      return { path: '/login', query: { ...to.query, redirect: to.path } }
    }

    // 有token的情况
    if (to.path === '/login') return { path: '/' }
    if (WHITE_LIST.includes(to.path)) return true

    const routes = router.getRoutes()
    if (routes.find((route) => route.name === to.name)) {
      return true
    }else{
      return { name: '404' }
    }
  })
}
