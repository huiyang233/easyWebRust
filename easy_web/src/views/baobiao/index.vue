<!--------------------------------
 - @Author: Ronnie Zhang
 - @LastEditor: Ronnie Zhang
 - @LastEditTime: 2023/12/05 21:28:22
 - @Email: zclzone@outlook.com
 - Copyright © 2023 Ronnie Zhang(大脸怪) | https://isme.top
 --------------------------------->

<template>
  <AppPage show-footer>
    <n-card class="mt-6" title="员工签到趋势" segmented>
      <VChart :option="trendOption" :init-options="{ height: 400 }" autoresize />
    </n-card>
  </AppPage>
</template>

<script setup>
import { useUserStore } from '@/store'
import * as echarts from 'echarts/core'
import { TooltipComponent, GridComponent, LegendComponent } from 'echarts/components'
import { BarChart, LineChart, PieChart } from 'echarts/charts'
import { UniversalTransition } from 'echarts/features'
import { CanvasRenderer } from 'echarts/renderers'
import VChart from 'vue-echarts'

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

const trendOption = {
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
    data: ['签到数'],
  },
  xAxis: [
    {
      type: 'category',
      data: ['1月', '2月', '3月', '4月', '5月', '6月', '7月', '8月', '9月', '10月', '11月', '12月'],
      axisPointer: {
        type: 'shadow',
      },
    },
  ],
  yAxis: [
    {
      type: 'value',
      min: 0,

      interval: 500,
      axisLabel: {
        formatter: '{value}',
      },

    },
  ],
  series: [
    {
      name: '签到数',
      type: 'line',
      data: [0, 0, 0, 0, 0, 0, 0, 12, 16, 0, 0, 0],
    },

  ],
}

const skillOption = {
  tooltip: {
    trigger: 'item',
    formatter({ name, value }) {
      return `${name} ${value}%`
    },
  },
  legend: {
    left: 'center',
  },
  series: [
    {
      top: '12%',
      type: 'pie',
      radius: ['35%', '90%'],
      avoidLabelOverlap: true,
      itemStyle: {
        borderRadius: 10,
        borderColor: '#fff',
        borderWidth: 2,
      },
      label: {
        show: false,
        position: 'center',
      },
      emphasis: {
        label: {
          show: true,
          fontSize: 36,
          fontWeight: 'bold',
        },
      },
      labelLine: {
        show: false,
      },
      data: [
        { value: 38.5, name: 'Vue' },
        { value: 37.0, name: 'JavaScript' },
        { value: 6.5, name: 'CSS' },
        { value: 6.2, name: 'HTML' },
        { value: 1.8, name: 'Other' },
      ],
    },
  ],
}

const message = $message
</script>
