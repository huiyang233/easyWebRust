## 🎯项目结构
    easyWebRust
    - easy_web 前端文件
    - sql 数据库文件
    - fonts 生成验证码用到的字体文件
    - src rust源码
        - api rest接口
        - auth 权限相关内容
        - config 配置文件读取
        - middleware 中间件
        - model 数据模型
        - service 服务层
        - task 定时任务/队列任务
        - utils 工具包

## ⚡️如何启动
#### 后端
1. 执行sql目录中的sql文件
2. 修改config.toml配置文件
3. cargo run

#### 前端
1. 安装node >= 20+
2. cd到easy_web目录下
3. npm install
4. npm run dev
