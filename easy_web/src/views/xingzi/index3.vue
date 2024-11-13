
<template>
  <CommonPage>
    <AppCard bordered bg="#fafafc dark:black" class="mb-30 min-h-60 rounded-4">
      <n-form class="flex justify-between p-16" @submit.prevent="handleSearch()">
        <n-space wrap :size="[32, 16]">
          <MeQueryItem label="项目名" :label-width="50">
            <n-input type="text" v-model:value="queryItems.itemName" clearable />
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
      <x-n-data-table-column width="150" fixed="left"  key="itemName" title="项目名" />
      <x-n-data-table-column width="150" key="formula" title="公式" />
      <x-n-data-table-column width="200" align="center" fixed="right" key="actions" title="操作">
        <template #render-cell="{ column, rowData, rowIndex }">
          <n-button :disabled="rowData.id==1" size="small" type="primary" @click="edit(rowData)">
            <i  class="i-material-symbols:edit-outline text-14 mr-4"></i>
            编辑
          </n-button>
            <n-button v-permission="['user:del']" :disabled="rowData.id==1" size="small" style="margin-left: 12px" type="error" @click="handleDelete(rowData.id)">
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
const router = useRouter()

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

function edit(row){
  router.push('/xingzi/item/edit')
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
    tableData.value = [{ id: 1, itemName: '基本工资', formula: '7000'},
                        { id: 2, itemName: '全额公积金', formula: '-12%' },
                         { id: 3, itemName: '定额7000%5公积金', formula: '-350' },
                         { id: 4, itemName: '养老保险', formula: '-500' },
                          { id: 4, itemName: '医保', formula: '-500' },
                         { id: 4, itemName: '奖金', formula: '+1000' },
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
