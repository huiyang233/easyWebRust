<!--------------------------------
 - @Author: Ronnie Zhang
 - @LastEditor: Ronnie Zhang
 - @LastEditTime: 2023/12/05 21:30:11
 - @Email: zclzone@outlook.com
 - Copyright © 2023 Ronnie Zhang(大脸怪) | https://isme.top
 --------------------------------->

<template>
  <AppPage show-footer>
  

    <n-card class="mt-20" title="个人资料信息">
      <template #header-extra>
        <n-button text type="primary" @click="pwdModalRef.open()">
            <i class="i-fe:key mr-4" />
              修改密码
          </n-button>
        <div class="mr-20"></div>
        <n-button type="primary" text @click="profileModalRef.open()">
          <i class="i-fe:edit mr-4" />
          修改资料
        </n-button>
      </template>

      <n-descriptions label-placement="left" :label-style="{ width: '200px', textAlign: 'center' }" :column="1"
        bordered>
        <n-descriptions-item label="用户名">{{  userStore.username }}</n-descriptions-item>
        <n-descriptions-item label="名字">{{ userStore.userInfo?.name }}</n-descriptions-item>
        <n-descriptions-item label="性别">
          {{ genders.find((item) => item.value === userStore.userInfo?.gender)?.label ?? '未知' }}
        </n-descriptions-item>
        <n-descriptions-item label="手机号">{{ userStore.userInfo?.phoneNumber }}</n-descriptions-item>

      </n-descriptions>
    </n-card>

    <MeModal ref="avatarModalRef" width="420px" title="更改头像" @ok="handleAvatarSave()">
      <n-input v-model:value="newAvatar" />
    </MeModal>

    <MeModal ref="pwdModalRef" title="修改密码" width="420px" @ok="handlePwdSave()">
      <n-form ref="pwdFormRef" :model="pwdForm" label-placement="left" require-mark-placement="left">
        <n-form-item label="原密码" path="oldPassword" :rule="required">
          <n-input v-model:value="pwdForm.oldPassword" type="password" placeholder="请输入原密码" />
        </n-form-item>
        <n-form-item label="新密码" path="newPassword" :rule="required">
          <n-input v-model:value="pwdForm.newPassword" type="password" placeholder="请输入新密码" />
        </n-form-item>
      </n-form>
    </MeModal>

    <MeModal ref="profileModalRef" title="修改资料" width="420px" @ok="handleProfileSave()">
      <n-form ref="profileFormRef" :model="profileForm" label-placement="left">
        <n-form-item label="名字" path="name">
          <n-input v-model:value="profileForm.name" placeholder="请输入昵称" />
        </n-form-item>
        <n-form-item label="性别" path="gender">
          <n-select v-model:value="profileForm.gender" :options="genders" placeholder="请选择性别" />
        </n-form-item>

        <n-form-item label="手机" path="phoneNumber">
          <n-input v-model:value="profileForm.phoneNumber" placeholder="请输入邮箱" />
        </n-form-item>
      </n-form>
    </MeModal>
  </AppPage>
</template>

<script setup>
import { MeModal } from '@/components'
import { useForm, useModal } from '@/composables'
import { useUserStore, useAuthStore } from '@/store'
import { ref } from 'vue'
import api from './api'
import sysApi from '@/api'

const userStore = useUserStore()
const required = {
  required: true,
  message: '此为必填项',
  trigger: ['blur', 'change'],
}

const [pwdModalRef] = useModal()
const [pwdFormRef, pwdForm, pwdValidation] = useForm()

async function handlePwdSave() {
  await pwdValidation()
  await api.changePassword(pwdForm.value)
  $message.success('密码修改成功')
  // refreshUserInfo()
  useAuthStore().logout()
  return true
}

const newAvatar = ref(userStore.avatar)
const [avatarModalRef] = useModal()
async function handleAvatarSave() {
  if (!newAvatar.value) {
    $message.error('请输入头像地址')
    return false
  }
  await api.updateProfile({ avatar: newAvatar.value })
  $message.success('头像修改成功')
  // refreshUserInfo()
  userStore.setAvatar(newAvatar.value)
}

const genders = [
  { label: '保密', value: 2 },
  { label: '男', value: 1 },
  { label: '女', value: 0 },
]
const [profileModalRef] = useModal()
const [profileFormRef, profileForm, profileValidation] = useForm({
  id: userStore.userId,
  nickName: userStore.nickName,
  gender: userStore.userInfo?.gender ?? 0,
  address: userStore.userInfo?.address,
  email: userStore.userInfo?.email,
})

profileForm.value.name = userStore.userInfo.name
profileForm.value.phoneNumber = userStore.userInfo.phoneNumber
profileForm.value.gender = userStore.userInfo.gender

async function handleProfileSave() {
  await profileValidation()
  await api.updateProfile(profileForm.value)
  $message.success('资料修改成功')
  userStore.userInfo.name = profileForm.value.name
  userStore.userInfo.phoneNumber = profileForm.value.phoneNumber
  userStore.userInfo.gender= profileForm.value.gender
}

async function handleFinish(e) {
  const fileInput = document.querySelector('#fileInput');
  if (fileInput.files.length > 0) {
    const file = fileInput.files[0];
    const { data, flag } = await sysApi.uploadImage(file)
    if (flag) {
      newAvatar.value = data.urlPath
      handleAvatarSave()
    }

  }


}
function handleSelectFile() {
  document.querySelector('#fileInput').click();
}

</script>
