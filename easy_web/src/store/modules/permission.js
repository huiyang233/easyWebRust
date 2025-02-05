/**********************************
 * @Author: Ronnie Zhang
 * @LastEditor: Ronnie Zhang
 * @LastEditTime: 2023/12/05 21:25:47
 * @Email: zclzone@outlook.com
 * Copyright © 2023 Ronnie Zhang(大脸怪) | https://isme.top
 **********************************/

import {defineStore} from 'pinia'
import {permissionRoutes} from '@/router/permission-routes'

export const usePermissionStore = defineStore('permission', {
  state: () => ({
    accessRoutes: [],
    permissions: [],
    menus: [],
  }),
  actions: {
    hasPermissions(requiredPermission){
      if (Array.isArray(requiredPermission)) {
        // 如果是数组，遍历数组并检查每个元素
        for (let i = 0; i < requiredPermission.length;i++){
          if(!this.permissions.some(item => item.value === requiredPermission[i])){
            return false
          }
        }
        return true;
      } else {
        // 如果不是数组，只检查单个值
        return this.permissions.some(item => item.value === requiredPermission)
      }

    },
    hasAnyPermissions(requiredPermissions){
      return requiredPermissions.some(permission => this.hasPermissions(permission))
    },
    setPermissions(permissions) {
      this.permissions = permissions
    },
    /**
     * 初始化权限
     */
    initPermissions(){
      this.accessRoutes.length = 0   
      this.menus = permissionRoutes
      .map((item) => this.getMenuItem(item))
      .filter((item) => !!item)
    },
    /**
     * 添加目录和权限的路由
     * @param {*} route 路由
     * @param {*} parent 递归用的
     * @returns 
     */
    getMenuItem(route, parent) {
      if(route.permission!=null && route.permission.length > 0){
          const exists = this.permissions.some(item => route.permission.includes(item.value ));
          if(!exists){
            return null;
          }
         
      }
      this.accessRoutes.push(route)
      if(!route.meta.show){
        return null
      }
      const menuItem = {
        label: route.meta.title,
        key: route.name,
        path: route.path,
        originPath: route.meta.originPath,
        icon: () => h('i', { class: `${route.meta.icon} text-16` }),
      }
      const children = route.children||[]
      if (children.length) {
        menuItem.children = children
          .map((child) => this.getMenuItem(child, menuItem))
          .filter((item) => !!item)
        if (!menuItem.children.length) delete menuItem.children
      }
      return menuItem
    },
    resetPermission() {
      this.$reset()
    },
  },
  persist: {
    key: 'permissions',
  },
})
