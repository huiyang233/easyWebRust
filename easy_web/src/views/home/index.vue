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
    
      <n-card class="w-100%">
        <div class="font-size-24 font-bold">
          你好, {{ userStore.name ?? userStore.username }}
        </div>
       
        <p class="opacity-60">
        </p>
      </n-card>
    </div>
   
    <n-card class="mt-12" title="用户登录数" segmented>
      <VChart :option="trendOption" :init-options="{ height: 400 }" autoresize />
    </n-card>
  </AppPage>
</template>

<script setup>
import { throttle } from '@/utils'
import { useUserStore } from '@/store'
import * as echarts from 'echarts/core'
import { TooltipComponent, GridComponent, LegendComponent } from 'echarts/components'
import { BarChart, LineChart, PieChart } from 'echarts/charts'
import { UniversalTransition } from 'echarts/features'
import { CanvasRenderer } from 'echarts/renderers'
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
                        data: [200, 320, 520, 550, 600, 805, 888, 950, 1300, 2503, 2702, 2712],
                      },
                    ],
                  })



const initCaptcha = throttle(async() => {
  const {flag,data} = await api.getLoginCountData()
  console.log(flag,data)
  if (!flag){
    $message.error('获取用户登录数图标', { key: 'getLoginCountData' })
    return
  }
  trendOption.value.xAxis[0].data = data.map(item => item.login_date)
  trendOption.value.series[0].data = data.map(item => item.login_count)
  console.log(trendOption.value)
  await api.getUserCount()

}, 500)

initCaptcha()

const message = $message
</script>
