## 简介
#### easyWebRust 是一个基于Rust开发的WEB脚手架，你可以基于这个项目继续开发你需要的内容。
#### 后端部分基于[salvo](https://github.com/salvo-rs/salvo)开发，使用[rbatis](https://github.com/rbatis/rbatis)作为数据库连接，使用[redis-rs](https://github.com/redis-rs/redis-rs)作为缓存。
#### 前端部分基于[Vue Naive Admin](https://github.com/zclzone/vue-naive-admin)开发;修改权限地方的部分代码。

## 项目结构
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
## 如何启动

#### 安装Rust
[Rust官网](https://www.rust-lang.org/tools/install)
[Rust官方文档](https://forge.rust-lang.org/infra/other-installation-methods.html)


#### 后端
1. 执行sql目录中的sql文件
2. 修改config.toml配置文件
3. cargo run

#### 前端
1. 安装node >= 20+
2. cd到easy_web目录下
3. npm install
4. npm run dev

## 版权说明

本项目使用 `MIT协议`，默认授权给任何人，被授权人可免费地无限制的使用、复制、修改、合并、发布、发行、再许可、售卖本软件拷贝、并有权向被供应人授予同等的权利，但必须满足以下条件:

- 复制、修改和发行本项目代码需包含原作者的版权及许可信息，包括但不限于文件头注释、协议等

简单来说，作者只想保留版权，没有任何其他限制。

## 