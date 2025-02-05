
<template>
  <CommonPage>
    <template #action>
      <n-button v-permission="['user:add']" type="primary" @click="handleAdd()">
        <i class="i-material-symbols:add mr-4 text-18" />
        创建新用户
      </n-button>
    </template>
    <AppCard bordered bg="#fafafc dark:black" class="mb-30 min-h-60 rounded-4">
      <n-form class="flex justify-between p-16" @submit.prevent="handleSearch()">
        <n-space wrap :size="[32, 16]">
          <MeQueryItem label="用户名" :label-width="50">
            <n-input type="text" v-model:value="queryItems.userName" placeholder="请输入用户名" clearable />
          </MeQueryItem>

          <MeQueryItem label="手机号" :label-width="50">
            <n-input type="text" v-model:value="queryItems.phoneNumber" placeholder="请输入手机号" clearable />
          </MeQueryItem>
        </n-space>
        <div class="flex-shrink-0">
          <n-button ghost type="primary" @click="handleReset">
            <i class="i-fe:rotate-ccw mr-4" />
            重置
          </n-button>
          <n-button attr-type="submit" class="ml-20" type="primary">
            <i class="i-fe:search mr-4" />
            搜索
          </n-button>
        </div>
      </n-form>
    </AppCard>
    <x-n-data-table :remote="true" :loading="loading" :pagination="pagination" :data="tableData" :scroll-x="800"  @update:page="onPageChange">
      <x-n-data-table-column width="150" fixed="left"  key="userName" title="用户名" />
      <x-n-data-table-column width="150" key="name" title="姓名" />
      <x-n-data-table-column width="150" key="phoneNumber" title="手机号" />
      <x-n-data-table-column ellipsis width="300" key="roles" title="角色">
        <template #render-cell="{ rowData }">
          <n-tag class="mr-4" :bordered="false" v-for="role in rowData.roles" :key="role.id" type="success">{{ role.name }}</n-tag>
        </template>
      </x-n-data-table-column>
      <x-n-data-table-column v-if="usePermission.hasPermissions(['user:update'])" width="150" key="enable" title="是否启用">
        <template #render-cell="{ rowData }">
          <n-switch :disabled="rowData.id==1" :rubber-band="false" type="small" :loading="rowData.enableLoading" :value="rowData.enable" @update-value="handleEnable(rowData)" />
        </template>
      </x-n-data-table-column>
      <x-n-data-table-column v-if="usePermission.hasPermissions(['user:update'])" width="150" key="isSuperAdmin" title="超级管理员">
        <template #render-cell="{ rowData }">
          <n-switch :disabled="rowData.id==1" :rubber-band="false" type="small" :loading="rowData.isSuperAdminLoading" :value="rowData.isSuperAdmin" @update-value="handleIsSuperAdmin(rowData)" />
        </template>
      </x-n-data-table-column>

      <x-n-data-table-column width="200" key="createTime" title="创建时间" />

      <x-n-data-table-column v-if="usePermission.hasAnyPermissions(['user:del','user:update'])" width="200" align="right"  fixed="right" key="actions" title="操作">
        <template #render-cell="{ column, rowData, rowIndex }">
          <n-button v-permission="['user:update']" :disabled="rowData.id==1" size="small" type="primary" @click="handleEdit(rowData)">
            <i class="i-material-symbols:edit-outline text-14 mr-4"></i>
            编辑
          </n-button>
          <n-button v-permission="['user:del']" :disabled="rowData.id==1" size="small" style="margin-left: 12px" type="error" @click="handleDelete(rowData.id)">
            <i class="i-material-symbols:delete-outline text-14 mr-4"></i>
            删除</n-button>
        </template>
      </x-n-data-table-column>
    </x-n-data-table>

    <n-modal
      v-model:show="modal.show"
      title="确认"
      style="width: 720px"
      :bordered="false"
      size="huge"
      :preset="undefined"
      class="modal-box"
      width="520px"
    >
      <n-card :closable="true" @close="close()">
        <template #header>
          <header class="modal-header">{{ modal.title }}</header>
        </template>
        <n-form  ref="modalFormRef" label-placement="left" label-align="left" :label-width="100" :model="modal.form">
          <n-form-item  label="用户名" path="userName" :rule="{
          required: true,
          message: '请输入用户名',
          trigger: ['input', 'blur'],
          }">
            <n-input v-model:value="modal.form.userName" :disabled="modal.type===2" />
          </n-form-item>
          <n-form-item v-if="modal.type===1" label="密码" path="password" :rule="{
          required: true,
          message: '请输入密码',
          trigger: ['input', 'blur'],
          }">
            <n-input v-model:value="modal.form.password" />
          </n-form-item>
          <n-form-item v-if="modal.type===1" label="状态" path="enable">
            <n-switch v-model:value="modal.form.enable">
              <template #checked>启用</template>
              <template #unchecked>停用</template>
            </n-switch>
          </n-form-item>
          <n-form-item v-if="modal.type===1" label="超级管理员" path="isSuperAdmin">
            <n-switch v-model:value="modal.form.isSuperAdmin">
              <template #checked>是</template>
              <template #unchecked>否</template>
            </n-switch>
          </n-form-item>
          <n-form-item label="姓名" path="name" :rule="{
          required: true,
          message: '请输入姓名',
          trigger: ['input', 'blur'],
          }">
            <n-input v-model:value="modal.form.name" />
          </n-form-item>
          <n-form-item label="手机号" path="phoneNumber" :rule="{
          required: true,
          message: '请输入手机号',
          trigger: ['input', 'blur'],
          }">
            <n-input v-model:value="modal.form.phoneNumber" />
          </n-form-item>


          <n-form-item label="角色" path="roleIds">
            <n-transfer :source-filterable="true" v-model:value="modal.form.roleIds" :options="roles" />


            <!-- <n-select v-model:value="modal.form.roleIds" :options="roles" label-field="name" value-field="id" clearable
                      filterable multiple /> -->


          </n-form-item>
        </n-form>
        <template #footer>
          <footer class="flex justify-end">
            <n-button @click="modal.show=false">
              取消
            </n-button>
            <n-button :loading="modal.loading" @click="handleSubmit()" type="primary" class="ml-20">
              保存
            </n-button>
          </footer>
        </template>
      </n-card>
    </n-modal>
  </CommonPage>
</template>

<script setup>
import {NButton, NSwitch, NTag} from 'naive-ui'
import api from './api'
import {XNDataTable, XNDataTableColumn} from '@skit/x.naive-ui'
import {AppCard, CommonPage, MeQueryItem} from '@/components/index.js'
import {usePermissionStore} from '@/store'

defineOptions({ name: 'UserManagement' })

const usePermission = usePermissionStore()

const roles = ref([])
const tableData = ref([])
const loading = ref(false)
const pagination = reactive({ page: 1, pageSize: 10 })
const queryItems = ref({})
const modalFormRef= ref(null)
const modal = ref({
  show:false,
  form:{
  },
  title:"",
  //type 1添加 2编辑
  type:2,
  loading:false
})

onMounted(()=>{
  api.getAllRoles().then(({ data = [] }) => (roles.value = data.map(res=> ({label:res.name,value:res.id}))))
});



function handleSearch() {
  pagination.page = 1
  handleQuery()
}

function handleEdit(row){
  modal.value.type=2
  modal.value.form = { ...row }
  modal.value.form.roleIds =  modal.value.form.roles.map(res=> res.id)
  modal.value.show = true
  modal.value.title="编辑用户"
}

function handleAdd(){
  modal.value.type=1
  modal.value.form = { isSuperAdmin:false,enable:false }
  modal.value.form.roleIds =  []
  modal.value.show = true
  modal.value.title="添加用户"
}


async function handleSubmit(){
  await modalFormRef.value?.validate()
  modal.value.loading = true
  if(modal.value.type===1){
    try{
      await api.create(modal.value.form)
      modal.value.show = false
      $message.success('添加成功')
      modal.value.loading = false
      handleSearch()
    }catch(error){
      $message.error('添加失败')
      modal.value.loading = false
    }
  }else{
    try{
     await api.update(modal.value.form)
      $message.success('更新成功')
      modal.value.show = false
      modal.value.loading = false
      handleSearch()
    }catch(error){
      $message.error('更新失败')
      modal.value.loading = false
    }
  }
}

function close() {
  modal.value.show = false
}

function onPageChange(currentPage) {
  pagination.page = currentPage
  handleQuery()
}

async function handleReset() {
  for (const key in queryItems.value) {
    queryItems.value[key] = null
  }
  await nextTick()
  pagination.page = 1
  handleQuery()
}

async function handleQuery() {
  try {
    loading.value = true
    let paginationParams = {}
    // 如果非分页模式或者使用前端分页,则无需传分页参数
    paginationParams = { page: pagination.page, pageSize: pagination.pageSize }
    const { data } = await api.read({
      ...queryItems.value,
      ...paginationParams,
    })
    tableData.value = data?.pageData || data
    pagination.itemCount = data.total ?? data.length
  } catch (error) {
    tableData.value = []
    pagination.itemCount = 0
  } finally {
    loading.value = false
  }
}

async function handleEnable(row) {
  row.enableLoading = true
  try {
    await api.update({ id: row.id, enable: !row.enable })
    row.enableLoading = false
    $message.success('操作成功')
    row.enable = !row.enable
  } catch (error) {
    row.enableLoading = false
  }
}
async function handleIsSuperAdmin(row) {
  row.isSuperAdminLoading = true
  try {
    await api.update({ id: row.id, isSuperAdmin: !row.isSuperAdmin })
    row.isSuperAdminLoading = false
    $message.success('操作成功')
    row.isSuperAdmin = !row.isSuperAdmin
  } catch (error) {
    row.isSuperAdminLoading = false
  }
}

function handleDelete(id, confirmOptions) {
  if (!id && id !== 0) return
  const d = $dialog.warning({
    content: '确定删除？',
    title: '提示',
    positiveText: '确定',
    negativeText: '取消',
    async onPositiveClick() {
      try {
        d.loading = true
        const data = await api.delete(id)
        $message.success('删除成功')
        d.loading = false
        handleQuery()
      } catch (error) {
        d.loading = false
      }
    },
    ...confirmOptions,
  })
}



handleQuery()





</script>
