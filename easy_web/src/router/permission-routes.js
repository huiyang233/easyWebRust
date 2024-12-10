export const permissionRoutes = [
  {
    name: 'Home',
    path: '/',
    component: () => import('@/views/home/index.vue'),
    meta: {
      title: '首页',
      show: true,
      icon: 'i-fe:home'
    },
    keepAlive: true,
  },
  {
    name: 'userInfo',
    path: '/profile',
    component: () => import('@/views/profile/index.vue'),
    meta: {
      title: '个人资料',
      icon: 'i-fe:user'
    }
  },
  {
    name: 'SystemManagement',
    path: '/systemManagement',
    permission: ['user', 'permission', 'role'],
    meta: {
      title: '系统管理',
      icon: 'i-fe:grid',
      show: true
    },
    children: [
      {
        name: 'UserManagement',
        path: '/systemManagement/userManagement',
        permission: ['user'],
        component: () => import('@/views/pms/user/index.vue'),
        meta: {
          title: '用户管理',
          icon: 'i-fe:user',
          show: true,
          keepAlive: null,
        },
      },
      {
        name: 'RoleManagement',
        path: '/systemManagement/roleManagement',
        permission: ['role'],
        component: () => import('@/views/pms/role/index.vue'),
        meta: {
          title: '角色管理',
          icon: 'i-fe:users',
          show: true,
          keepAlive: null,
          btns: [{
            code: 'AddRole',
            name: '新建角色'
          }]
        }

      },
      {
        name: 'PermissionManagement',
        path: '/systemManagement/permissionManagement',
        permission: ['log'],
        component: () => import('@/views/pms/permission/index.vue'),
        meta: {
          title: '权限管理',
          icon: 'i-fe:key',
          show: true,
          keepAlive: null,
        }

      },
      {
        name: 'LogManagement',
        path: '/systemManagement/logManagement',
        permission: ['log'],
        component: () => import('@/views/pms/log/index.vue'),
        meta: {
          title: '日志管理',
          icon: 'i-fe:edit',
          show: true,
          keepAlive: null,
        }
      },
      {
        name: 'BlackListManagement',
        path: '/systemManagement/blackListManagement',
        permission: ['log'],
        component: () => import('@/views/pms/black-list/index.vue'),
        meta: {
          title: '黑名单管理',
          icon: 'i-fe:book',
          show: true,
          keepAlive: null,
        }

      }
    ]
  }
]