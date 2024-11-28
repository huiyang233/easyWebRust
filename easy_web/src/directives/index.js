/**********************************
 * @Author: Ronnie Zhang
 * @LastEditor: Ronnie Zhang
 * @LastEditTime: 2023/12/05 21:23:01
 * @Email: zclzone@outlook.com
 * Copyright © 2023 Ronnie Zhang(大脸怪) | https://isme.top
 **********************************/

import {usePermissionStore} from '@/store'

const permission = {
  mounted(el, binding) {
    console.log("binding.value",binding.value,el)
    const usePermission = usePermissionStore()
    if(!usePermission.hasPermissions(binding.value)){
      el.remove()
    }
  },
}

const anyPermission = {
  mounted(el, binding) {
    const usePermission = usePermissionStore()
    console.log("binding.value",binding.value)
    if(!usePermission.hasAnyPermissions(binding.value)){
      el.remove()
    }
  },
}


export function setupDirectives(app) {
  app.directive('permission', permission)
  app.directive('any-permission', anyPermission)

}
