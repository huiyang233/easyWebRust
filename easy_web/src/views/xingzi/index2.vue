
<template>
  <CommonPage>
   <template #action>
        <n-button v-permission="['user:add']" type="primary" @click="handleAdd()">
          <i class="i-material-symbols:add mr-4 text-18" />
          新增
        </n-button>
      </template>
    <x-n-data-table :loading="loading" :pagination="pagination" :data="tableData" :scroll-x="800"  @update:page="onPageChange">
      <x-n-data-table-column width="150" key="name" title="姓名" />
      <x-n-data-table-column width="150" key="phoneNumber" title="手机号" />
      <x-n-data-table-column width="150" key="item" title="项目" />
      <x-n-data-table-column width="200" align="center" fixed="right" key="actions" title="操作">
              <template #render-cell="{ column, rowData, rowIndex }">
                <n-button :disabled="rowData.id==1" size="small" type="primary" @click="edit(rowData)">
                  <i  class="i-material-symbols:edit-outline text-14 mr-4"></i>
                  编辑
                </n-button>
                  <n-button :disabled="rowData.id==1" size="small" style="margin-left: 12px" type="error" @click="handleDelete(rowData.id)">
                <i  class="i-material-symbols:delete-outline text-14 mr-4"></i>
                删除</n-button>
              </template>

            </x-n-data-table-column>
    </x-n-data-table>


  </CommonPage>
</template>

<script setup>
import { NButton, NSwitch, NTag } from 'naive-ui'
import api from './api'
import { XNDataTable, XNDataTableColumn } from '@skit/x.naive-ui'
import { AppCard, CommonPage, MeQueryItem } from '@/components/index.js'

defineOptions({ name: 'UserMgt' })

const roles = ref([])
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

function check(row){
  router.push('user/kaoqing/check')
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
    paginationParams = { page: pagination.page, pageSize: pagination.pageSize }
    const { data } = await api.read({
      ...queryItems.value,
      ...paginationParams,
    })
    tableData.value = [{ id: 1, userName: '2409931477', name: '杨辉', phoneNumber: '13402152243', item: '基本工资'},
    { id: 2, userName: '2409931477', name: '杨辉', phoneNumber: '13402152243', item: '养老保险'},
    { id: 3, userName: '2409931477', name: '杨辉', phoneNumber: '13402152243', item: '医保'},
    { id: 3, userName: '2409931477', name: '杨辉', phoneNumber: '13402152243', item: '全额公积金'},
    ]
    pagination.itemCount = data.total ?? data.length
  } catch (error) {
    tableData.value = []
    pagination.itemCount = 0
  } finally {
    loading.value = false
  }
}




handleQuery()





</script>
