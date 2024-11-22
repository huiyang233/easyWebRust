/**********************************
 * @Author: Ronnie Zhang
 * @LastEditor: Ronnie Zhang
 * @LastEditTime: 2023/12/05 21:25:59
 * @Email: zclzone@outlook.com
 * Copyright © 2023 Ronnie Zhang(大脸怪) | https://isme.top
 **********************************/

import { defineStore } from 'pinia'

export const useUserStore = defineStore('user', {
  state: () => ({
    userInfo: null,
    role:null,
  }),
  getters: {
    userId() {
      return this.userInfo?.id
    },
    username() {
      return this.userInfo?.userName
    },
    name() {
      return this.userInfo?.name
    },
    avatar() {
      return this.userInfo?.avatar
    },
    currentRole() {
      return this.userInfo?.currentRole || {}
    },
    roles() {
      return this.userInfo?.roles || []
    },
  },
  actions: {
    setAvatar(avatar){
      this.userInfo.avatar = avatar
    },
    setRole(role) {
      this.role = role
    },
    setUser(user) {
      this.userInfo = user
    },
    resetUser() {
      this.$reset()
    },
  },
  persist: {
    key: 'user',
  },
})
