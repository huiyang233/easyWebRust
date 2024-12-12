<template>
    <CommonPage>
      <AppCard bordered bg="#fafafc dark:black" class="mb-30 min-h-60 rounded-4">
        <n-form class="flex justify-between p-16" @submit.prevent="handleSearch()">
          <n-space wrap :size="[32, 16]">
            <MeQueryItem label="权限名" :label-width="100">
              <n-input type="text" v-model:value="queryItems.name" placeholder="请输入权限名" clearable />
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
          <x-n-data-table-column width="100"  key="name" title="权限名" >
          </x-n-data-table-column>
          <x-n-data-table-column width="100" key="value" title="权限值" >
          </x-n-data-table-column>
      </x-n-data-table>
  
    </CommonPage>
  </template>
  
  <script setup>
  import {NButton} from 'naive-ui'
  import api from './api'
  import {XNDataTable, XNDataTableColumn} from '@skit/x.naive-ui'
  import {AppCard, CommonPage, MeQueryItem} from '@/components/index.js'

  defineOptions({ name: 'PermissionManagement' })
  
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
  
  

  
  
  handleQuery()
  
  
  
  
  
  </script>