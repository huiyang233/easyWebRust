<template>
  <CommonPage>
    <template #action>
      <div class="flex-row">
        <n-button class="mr-10"  v-permission="['black_list:add']"  type="primary" @click="handleAdd()">
          <i class="i-material-symbols:add mr-4 text-18" />
          新增
        </n-button>

        <n-button v-permission="['black_list_config:update']"  type="primary" @click="handleConfig()">
          <i class="i-material-symbols:edit-outline mr-4 text-18" />
          配置黑名单参数
        </n-button>
      </div>
    </template>
    <AppCard bordered bg="#fafafc dark:black" class="mb-30 min-h-60 rounded-4">
      <n-form class="flex justify-between p-16" @submit.prevent="handleSearch()">
        <n-space wrap :size="[32, 16]">
          <MeQueryItem label="IP" :label-width="100">
            <n-input type="text" v-model:value="queryItems.ip" placeholder="请输入IP" clearable />
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

      <x-n-data-table-column width="150"  key="ip" title="IP" >
      </x-n-data-table-column>
      <x-n-data-table-column width="150"  key="createTime" title="封禁日期" >
        <template #render-cell="{ column, rowData, rowIndex }">
          <n-time  :time="new Date(rowData.createTime).time" />
        </template>
      </x-n-data-table-column>

      <x-n-data-table-column width="150" key="reason" title="原因" >
      </x-n-data-table-column>
      <x-n-data-table-column width="150"  key="banTime" title="封禁时间" >
        <template #render-cell="{ column, rowData, rowIndex }">
          <n-tag v-if="rowData.banTime===null" class="mr-4" :bordered="false"  type="error">永久</n-tag>
          <n-time v-else :time="new Date(rowData.banTime).time" />
        </template>
      </x-n-data-table-column>
      <x-n-data-table-column v-if="usePermission.hasAnyPermissions(['black_list:update','black_list:del'])" width="200" align="right" fixed="right" key="actions" title="操作">
        <template  #render-cell="{ column, rowData, rowIndex }">
          <n-button v-permission="['black_list:update']"  size="small" type="primary" @click="handleEdit(rowData)">
            <i  class="i-material-symbols:edit-outline text-14 mr-4"></i>
            编辑
          </n-button>
          <n-button v-permission="['black_list:del']"  size="small" style="margin-left: 12px" type="error" @click="handleDelete(rowData.ip)">
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

          <n-form-item label="IP" path="ip" :rule="{
          required: true,
          message: '请输入IP',
          trigger: ['input', 'blur'],
          }">
            <n-input v-model:value="modal.form.ip" />
          </n-form-item>

          <n-form-item label="禁封时间" path="banTime">
            <n-date-picker v-model:value="modal.form.banTime2" type="datetime" clearable />
          </n-form-item>

          <n-form-item label="说明" path="reason" :rule="{
          required: true,
          message: '请输入说明',
          trigger: ['input', 'blur'],
          }">
            <n-input maxlength="100" show-count type="textarea" v-model:value="modal.form.reason" />
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

    <n-modal
        v-model:show="configModal.show"
        title="确认"
        style="width: 520px"
        :bordered="false"
        size="huge"
        :preset="'card'"
        class="modal-box"
        width="520px"

    >

<!--        <n-card :closable="true" @close="configModal.show = false">-->
          <template #header>
            <header class="modal-header">{{ configModal.title }}</header>


          </template>


      <n-spin :show="configModal.loading">
            <n-form  ref="configModalFormRef" label-placement="left" label-align="left" :label-width="100" :model="configModal.form">
              <n-form-item  label="间隔时间" path="interval">
                <n-input-number class="w-100%" :show-button="false" v-model:value="configModal.form.interval">
                  <template #suffix>
                    秒
                  </template>
                </n-input-number>
              </n-form-item>
              <n-form-item  label="间隔时间阈值" path="visitCount">
                <n-input-number class="w-100%" :show-button="false" v-model:value="configModal.form.visitCount">
                  <template #suffix>
                    个
                  </template>
                </n-input-number>
              </n-form-item>

              <n-alert  class="mb-20" type="warning" closable>
                禁封时间设置0为永久禁封
              </n-alert>

              <n-form-item label="禁封时间" path="banTime">

                <n-input-number class="w-100%" :show-button="false" v-model:value="configModal.form.banTime">
                  <template #suffix>
                    秒
                  </template>
                </n-input-number>
               </n-form-item>
            </n-form>
      </n-spin>

            <template #footer>
              <footer class="flex justify-end">
                <n-button :disabled="configModal.loading" @click="configModal.show=false">
                  取消
                </n-button>
                <n-button  :disabled="configModal.loading" :loading="configModal.buttonLoading" @click="handleConfigSubmit()" type="primary" class="ml-20">
                  保存
                </n-button>
              </footer>
            </template>
<!--      </n-card>-->

    </n-modal>
  </CommonPage>
</template>

<script setup>
import { NButton, NTag } from 'naive-ui'
import api from './api'
import { XNDataTable, XNDataTableColumn } from '@skit/x.naive-ui'
import { AppCard, CommonPage, MeQueryItem } from '@/components/index.js'
import { usePermissionStore } from '@/store'

defineOptions({ name: 'BlackListManagement' })
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

const configModalFormRef= ref(null)
const configModal = ref({
  show:false,
  form:{},
  title:"",
  buttonLoading:false,
  loading:false
})



function handleSearch() {
  pagination.page = 1
  handleQuery()
}

function handleEdit(row){
  modal.value.type=2
  modal.value.form = { ...row }
  modal.value.show = true
  modal.value.title="编辑"
}

async function handleConfig(){
  configModal.value.show = true
  configModal.value.form = { }
  configModal.loading = true
  configModal.value.title="配置黑名单参数"
  try {
    const { data } = await api.getBlackListConfig()
    configModal.loading = false
    configModal.value.form = data;


  }catch (error){
    $message.error('黑名单获取失败')
    configModal.loading = false
    configModal.value.show = false
  }


}
function handleAdd(){
  modal.value.type=1
  modal.value.form = { }
  modal.value.show = true
  modal.value.title="添加"
}


async function handleConfigSubmit(){
  await configModalFormRef.value?.validate()
  configModal.value.loading = true

  try{
    await api.setBlackListConfig(configModal.value.form)
    configModal.value.show = false
    $message.success('保存成功')
    configModal.value.loading = false
  }catch(error){
    $message.error('保存失败')
    configModal.value.loading = false
  }
}

async function handleSubmit(){
  await modalFormRef.value?.validate()
  modal.value.loading = true
  if(modal.value.type===1){
    try{
      modal.value.form.banTime = new Date(modal.value.form.banTime2)
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