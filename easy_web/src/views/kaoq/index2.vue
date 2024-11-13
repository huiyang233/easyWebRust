
<template>
  <CommonPage>
    <x-n-data-table :loading="loading" :pagination="pagination" :data="tableData" :scroll-x="800"  @update:page="onPageChange">
      <x-n-data-table-column width="150" fixed="left"  key="userName" title="用户名" />
      <x-n-data-table-column width="150" key="name" title="姓名" />
      <x-n-data-table-column width="150" key="phoneNumber" title="手机号" />
      <x-n-data-table-column width="150" key="date" title="考勤时间" />
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
    tableData.value = [{ id: 1, userName: '2409931477', name: '杨辉', phoneNumber: '13402152243', date: '2024-10-11'},
    { id: 2, userName: '2409931477', name: '杨辉', phoneNumber: '13402152243', date: '2024-10-12'},
    { id: 3, userName: '2409931477', name: '杨辉', phoneNumber: '13402152243', date: '2024-10-13'},
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
