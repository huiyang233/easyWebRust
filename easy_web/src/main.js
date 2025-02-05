/**********************************
 * @Description: 入口文件
 * @FilePath: main.js
 * @Author: Ronnie Zhang
 * @LastEditor: Ronnie Zhang
 * @LastEditTime: 2023/12/04 22:41:32
 * @Email: zclzone@outlook.com
 * Copyright © 2023 Ronnie Zhang(大脸怪) | https://isme.top
 **********************************/

import '@/styles/reset.css'
import '@/styles/global.scss'
import 'uno.css'
import '@wangeditor/editor/dist/css/style.css' // 引入 css

import { createApp } from 'vue'
import App from './App.vue'
import { setupRouter } from './router'
import { setupStore } from './store'
import { setupNaiveDiscreteApi } from './utils'
import { setupDirectives } from './directives'
import { XNDataTable } from '@skit/x.naive-ui';


async function bootstrap() {
  const app = createApp(App)
  setupStore(app)
  setupNaiveDiscreteApi()
  await setupRouter(app)
  app.use(XNDataTable);
  app.mount('#app')
  setupDirectives(app)
}

bootstrap()
