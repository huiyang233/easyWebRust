<!--------------------------------
 - @Author: Ronnie Zhang
 - @LastEditor: Ronnie Zhang
 - @LastEditTime: 2023/12/05 21:29:38
 - @Email: zclzone@outlook.com
 - Copyright © 2023 Ronnie Zhang(大脸怪) | https://isme.top
 --------------------------------->

<template>
  <CommonPage>
    <template #action>
      <n-button v-permission="['role:add']" type="primary" @click="handleAdd()">
        <i class="i-material-symbols:add mr-4 text-18" />
        新增角色
      </n-button>
    </template>

    <AppCard bordered bg="#fafafc dark:black" class="mb-30 min-h-60 rounded-4">
      <n-form class="flex justify-between p-16" @submit.prevent="handleSearch()">
        <n-space wrap :size="[32, 16]">
          <MeQueryItem label="角色名" :label-width="50">
            <n-input v-model:value="queryItems.name" type="text" placeholder="请输入角色名" clearable />
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

    <x-n-data-table :remote="true" :loading="loading" :pagination="pagination" :data="tableData"  @update:page="onPageChange">
      <x-n-data-table-column max-width="100" key="name" title="角色名" />
      <x-n-data-table-column v-if="usePermission.hasAnyPermissions(['role:del','role:update'])" max-width="100" fixed="right" align="right"  key="actions" title="操作">
        <template #render-cell="{ column, rowData, rowIndex }">
          <n-button v-permission="['role:update']" :disabled="rowData.id==1" size="small" type="primary" @click="handleEdit(rowData)">
            <i  class="i-material-symbols:edit-outline text-14 mr-4"></i>
            编辑
          </n-button>
          <n-button v-permission="['role:del']" :disabled="rowData.id==1" size="small" style="margin-left: 12px" type="error" @click="handleDelete(rowData.id)">
            <i  class="i-material-symbols:delete-outline text-14 mr-4"></i>
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
          <n-form-item
            label="角色名"
            path="name"
            :rule="{
            required: true,
            message: '请输入角色名',
            trigger: ['input', 'blur'],
          }"
          >
            <n-input v-model:value="modal.form.name" />
          </n-form-item>
          <n-form-item label="权限" path="permissionIds">
            <CascadeTree
              v-model:value="modal.form.permissionIds"
              :tree-data="permissionTree"
              label-field="name"
              key-field="id"
              class="cus-scroll max-h-200 w-full"
            />
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
import {NButton} from 'naive-ui'
import {AppCard, MeQueryItem} from '@/components'
import api from './api'
import CascadeTree from './components/CascadeTree.vue'
import {CommonPage} from '@/components/index.js'
import {XNDataTable, XNDataTableColumn} from '@skit/x.naive-ui'
import {usePermissionStore} from '@/store'

defineOptions({ name: 'RoleManagement' })

const usePermission = usePermissionStore()


const tableData = ref([])
const loading = ref(false)
const pagination = reactive({ page: 1, pageSize: 10 })
const queryItems = ref({})
const modalFormRef= ref(null)
const modal = ref({
  show:false,
  form:{},
  title:"",
  //type 1添加 2编辑
  type:2,
  loading:false
})

function handleSearch() {
  pagination.page = 1
  handleQuery()
}

function handleEdit(row){
  modal.value.type=2
  modal.value.form = { ...row }
  modal.value.form.permissionIds =  modal.value.form.permissions.map(res=> res.id)
  modal.value.show = true
  modal.value.title="编辑角色"
}

function handleAdd(){
  modal.value.type=1
  modal.value.form = { }
  modal.value.form.permissionIds =  []
  modal.value.show = true
  modal.value.title="添加角色"
}


async function handleSubmit(){
  await modalFormRef.value?.validate()
  modal.value.loading = true
  if(modal.value.type===1){
    try {
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
    try {
      await api.update(modal.value.form)
      $message.success('更新成功')
      modal.value.show = false
      modal.value.loading = false
      handleSearch()

    }catch(error){
      $message.error('添加失败')
      // modal.value.show = false
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

// async function handleEnable(row) {
//   row.enableLoading = true
//   try {
//     await api.update({ id: row.id, enable: !row.enable })
//     row.enableLoading = false
//     $message.success('操作成功')
//     row.enable = !row.enable
//   } catch (error) {
//     row.enableLoading = false
//   }
// }
// async function handleIsSuperAdmin(row) {
//   row.isSuperAdminLoading = true
//   try {
//     await api.update({ id: row.id, isSuperAdmin: !row.isSuperAdmin })
//     row.isSuperAdminLoading = false
//     $message.success('操作成功')
//     row.isSuperAdmin = !row.isSuperAdmin
//   } catch (error) {
//     row.isSuperAdminLoading = false
//   }
// }

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

const permissionTree = ref([])
onMounted(() => {

  api.getAllPermissionTree().then(({ data = [] }) => (permissionTree.value = data))
  handleQuery()
})

</script>
