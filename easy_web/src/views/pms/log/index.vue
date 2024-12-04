<template>
    <CommonPage>
      <AppCard bordered bg="#fafafc dark:black" class="mb-30 min-h-60 rounded-4">
        <n-form class="flex justify-between p-16" @submit.prevent="handleSearch()">
          <n-space wrap :size="[32, 16]">
            <MeQueryItem label="事件类型" :label-width="60">
                <n-select
                    v-model:value="queryItems.logType"
                    :options="selectOptions"
                    clearable
                />
            </MeQueryItem>

            <MeQueryItem label="操作人" :label-width="60">
                 <n-input type="text" v-model:value="queryItems.userName" placeholder="请输入操作人" clearable />
            </MeQueryItem>

            <MeQueryItem label="时间" :label-width="60" :contentWidth="390">
                <n-date-picker :value-format="'yyyy-MM-dd HH:mm:ss'" v-model:formatted-value="queryItems.time" type="datetimerange" clearable />
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
      <x-n-data-table :loading="loading" :pagination="pagination" :data="tableData" :scroll-x="800"  @update:page="onPageChange">
         
          <x-n-data-table-column width="150"   key="name" title="事件名" >
          </x-n-data-table-column>
          <x-n-data-table-column width="150"   key="logType" title="事件类型" >
            <template #render-cell="{ rowData }">
                <n-tag v-if="rowData.logType==1"  :bordered="false" type="success">用户登录</n-tag>
                <n-tag v-else-if="rowData.logType==2" :bordered="false" type="success">删除资源</n-tag>
                <n-tag v-else-if="rowData.logType==3"  :bordered="false" type="success">更新资源</n-tag>
                <n-tag v-else-if="rowData.logType==4"  :bordered="false" type="success">其他</n-tag>
                <n-tag v-else="rowData.logType==4"  :bordered="false" type="success">未知</n-tag>
            </template>
          </x-n-data-table-column>
          <x-n-data-table-column width="150"  key="description" title="简介" >
          </x-n-data-table-column>
          <x-n-data-table-column width="150"  key="userName" title="操作人" >
          </x-n-data-table-column>
          <x-n-data-table-column width="150"  key="ip" title="IP地址" >
          </x-n-data-table-column>
          <x-n-data-table-column width="150"  key="createTime" title="创建时间" >
          </x-n-data-table-column>
      </x-n-data-table>
  
    </CommonPage>
  </template>
  
  <script setup>
  import { NButton} from 'naive-ui'
  import api from './api'
  import { XNDataTable, XNDataTableColumn } from '@skit/x.naive-ui'
  import { AppCard, CommonPage, MeQueryItem } from '@/components/index.js'
  
  defineOptions({ name: 'SysLog' })

  const selectOptions = [
        {
          label: "用户登录",
          value: 1
        },
        {
          label: "删除资源",
          value: 2
        },
        {
          label: "更新资源",
          value: 3
        },
        {
          label: "其他",
          value: 4
        },
      ]
  
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
      let time = queryItems.value.time
      paginationParams = { page: pagination.page, pageSize: pagination.pageSize }
      if(queryItems.value.time){
        queryItems.value.startTime=queryItems.value.time[0]
        queryItems.value.endTime=queryItems.value.time[1]
        queryItems.value.time=undefined
      }
      const { data } = await api.read({
        ...queryItems.value,
        ...paginationParams,
      })
      queryItems.value.startTime=undefined
      queryItems.value.endTime=undefined
       queryItems.value.time=time
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