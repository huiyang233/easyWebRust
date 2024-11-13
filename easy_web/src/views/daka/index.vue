
<template>
  <CommonPage>
    <AppCard bordered bg="#fafafc dark:black" class="mb-30 min-h-60 rounded-4">
    <n-form class="flex justify-between p-16" @submit.prevent="handleSearch()">

            <div class="flex-shrink-0">
             <n-button ghost type="primary" @click="clockIn">
                   打卡
              </n-button>
            </div>
          </n-form>

    </AppCard>
    <x-n-data-table :loading="loading" :pagination="pagination" :data="tableData" :scroll-x="800"  @update:page="onPageChange">
      <x-n-data-table-column width="150" fixed="left"  key="userName" title="用户名" />
      <x-n-data-table-column width="150" key="name" title="姓名" />
      <x-n-data-table-column width="150" key="phoneNumber" title="手机号" />
       <x-n-data-table-column width="150" key="date" title="日期" />
    </x-n-data-table>

  </CommonPage>
</template>

<script setup>
import { NButton, NSwitch, NTag } from 'naive-ui'
import api from './api'
import { XNDataTable, XNDataTableColumn } from '@skit/x.naive-ui'
import { AppCard, CommonPage, MeQueryItem } from '@/components/index.js'
const router = useRouter()

defineOptions({ name: 'UserMgt' })

const roles = ref([])
const tableData = ref([])
const loading = ref(false)
const pagination = reactive({ page: 1, pageSize: 20 })
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
  router.push('/kaoqing/check')
}



function close() {
  modal.value.show = false
}

function onPageChange(currentPage) {
  pagination.page = currentPage
  handleQuery()
}

async function clockIn() {
  await api.clockIn()
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
   tableData.value = [{ id: 2, userName: '2409931477', name: '杨辉', phoneNumber: '13402152243', date:'2024-10-10',money: '6160'},
         { id: 2, userName: '2409931477', name: '杨辉', phoneNumber: '13402152243', date:'2024-10-09',money: '6160'},
       { id: 2, userName: '2409931477', name: '杨辉', phoneNumber: '13402152243', date:'2024-10-08',money: '6160'},
        { id: 2, userName: '2409931477', name: '杨辉', phoneNumber: '13402152243', date:'2024-10-07',money: '6160'},
        { id: 2, userName: '2409931477', name: '杨辉', phoneNumber: '13402152243', date:'2024-10-06',money: '6160'},
        { id: 2, userName: '2409931477', name: '杨辉', phoneNumber: '13402152243', date:'2024-10-05',money: '6160'},
        { id: 2, userName: '2409931477', name: '杨辉', phoneNumber: '13402152243', date:'2024-10-04',money: '6160'},
        { id: 2, userName: '2409931477', name: '杨辉', phoneNumber: '13402152243', date:'2024-10-03',money: '6160'},
        { id: 2, userName: '2409931477', name: '杨辉', phoneNumber: '13402152243', date:'2024-10-02',money: '6160'},
        { id: 2, userName: '2409931477', name: '杨辉', phoneNumber: '13402152243', date:'2024-10-01',money: '6160'},
        { id: 2, userName: '2409931477', name: '杨辉', phoneNumber: '13402152243', date:'2024-09-30',money: '6160'},
        { id: 2, userName: '2409931477', name: '杨辉', phoneNumber: '13402152243', date:'2024-09-29',money: '6160'},


         ]
    pagination.itemCount = 500
  } catch (error) {
    tableData.value = []
    pagination.itemCount = 500
  } finally {
    loading.value = false
  }
}




handleQuery()





</script>
