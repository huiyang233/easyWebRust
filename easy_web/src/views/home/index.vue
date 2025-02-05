<!--------------------------------
 - @Author: Ronnie Zhang
 - @LastEditor: Ronnie Zhang
 - @LastEditTime: 2023/12/05 21:28:22
 - @Email: zclzone@outlook.com
 - Copyright © 2023 Ronnie Zhang(大脸怪) | https://isme.top
 --------------------------------->

<template>
  <AppPage show-footer>
    <div class="flex">
      <n-card class="w-30% h-120">
        <div class="font-size-16 font-bold">
          你好
        </div>
        <div class="font-size-32 font-bold">
          {{ userStore.name ?? userStore.username }}
        </div>
      </n-card>

      <n-card class="ml-12 h-120">
        <n-row>
          <n-col :span="8">
            <n-statistic label="系统用户数">
              <template #prefix>
                <i class="i-fe:user"/>
              </template>
              {{ sysUserCount }}
            </n-statistic>
          </n-col>
          <n-col :span="8">

            <n-statistic label="今日登录数">
              <template #prefix>
                <i class="i-fe:users"/>
              </template>
              {{ todayLoginUserCount }}
            </n-statistic>
          </n-col>
          <n-col :span="8">

            <n-statistic label="今日活跃用户">
              <template #prefix>
                <i class="i-fe:user"/>
              </template>
              {{ todayActiveUsersCount }}
            </n-statistic>
          </n-col>
        </n-row>

      </n-card>
    </div>

    <n-card class="mt-12" title="用户登录数" segmented>
      <VChart :option="trendOption" :init-options="{ height: 400 }" autoresize/>
    </n-card>
  </AppPage>
</template>

<script setup>
import {throttle} from '@/utils'
import {useUserStore} from '@/store'
import * as echarts from 'echarts/core'
import {GridComponent, LegendComponent, TooltipComponent} from 'echarts/components'
import {BarChart, LineChart, PieChart} from 'echarts/charts'
import {UniversalTransition} from 'echarts/features'
import {CanvasRenderer} from 'echarts/renderers'
import VChart from 'vue-echarts'
import api from './api'


const userStore = useUserStore()

echarts.use([
  TooltipComponent,
  GridComponent,
  LegendComponent,
  BarChart,
  LineChart,
  CanvasRenderer,
  UniversalTransition,
  PieChart,
])

defineOptions({ name: 'Home' })

const trendOption = ref({
  tooltip: {
    trigger: 'axis',
    axisPointer: {
      type: 'cross',
      crossStyle: {
        color: '#999',
      },
    },
  },
  legend: {
    top: '5%',
    data: ['登录数'],
  },
  xAxis: [
    {
      type: 'category',
      data: [],
      axisPointer: {
        type: 'shadow',
      },
    },
  ],
  yAxis: [
    {
      type: 'value',
      interval: 1,
      axisLabel: {
        formatter: '{value}',
      },
    },
  ],
  series: [
    {
      name: '登录数',
      type: 'line',
      data: [],
    },
  ],
})

const todayLoginUserCount = ref(0)
const sysUserCount = ref(0)
const todayActiveUsersCount = ref(0)




const initCaptcha = throttle(async () => {
  const {flag, data} = await api.getLoginCountData()
  console.log(flag, data)
  if (!flag) {
    $message.error('获取用户登录数图标', {key: 'getLoginCountData'})
    return
  }
  trendOption.value.xAxis[0].data = data.map(item => item.login_date)
  trendOption.value.series[0].data = data.map(item => item.login_count)
  console.log(trendOption.value)
  const res2 = await api.getUserCount()
  sysUserCount.value = res2.data
  const res = await api.getTodayLoginCount()
  todayLoginUserCount.value = res.data
  const res3 = await api.getTodayActiveUsersCount()
  todayActiveUsersCount.value = res3.data

}, 500)

onMounted(() => {
  initCaptcha()
})



const message = $message
</script>
