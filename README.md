## 简介
#### easyWebRust 是一个基于Rust开发的WEB脚手架，你可以基于这个项目继续开发你需要的内容。
## 后端部分
#### Web框架使用[salvo](https://github.com/salvo-rs/salvo)
#### 数据库使用[sqlx](https://github.com/launchbadge/sqlx)(Pg数据库)和[redis-rs](https://github.com/redis-rs/redis-rs)
## 前端部分
#### 基于[Vue Naive Admin](https://github.com/zclzone/vue-naive-admin)开发;修改权限地方的部分代码。

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

## 后端部分功能介绍
#### 增删改查
```rust
// 添加注解CURD 使用#[curd(pk)]添加到主键上；如果需要逻辑删除，使用#[curd(logic_del)]
#[derive(FromRow,CURD,Default)]
pub struct SysUser {
    #[curd(pk)]
    pub id: i64,
    pub name: String,
    #[curd(logic_del)]
    pub is_del: bool,
}
fn test_curd(){
    let user = SysUser::default();
    user.insert(&get_sqlx_db()).await?;
    user.name = "hello".to_string();
    user.update(&get_sqlx_db()).await?;
    // 也可以使用update_by_col进行更新；枚举的名字是struct的名字+Field
    // user.update_by_col(&get_sqlx_db(),SysUserField::Id).await?;
    user.delete(&get_sqlx_db()).await?;
}
fn test_transaction(){
    let user = SysUser::default();
    let mut tx = get_sqlx_db().begin().await?;
    user.insert(&mut *tx).await?;
    user.name = "hello".to_string();
    user.update(&mut *tx).await?;
    user.delete(&mut *tx).await?;
    tx.commit().await?;
}


```
#### 分页查询
```rust
// impl_page会自动帮你添加PageDto参数所以不用写
impl_page!(SysUser{select_page(req:&SysUserPageReq)=>|builder:&mut PageBuilder<SysUser>|{

     if let Some(name) = &req.name {
        builder.and_like("name",name);
    }

    if let Some(phone_number) = &req.phone_number {
        builder.and_like("phone_number",phone_number)
    }
    builder.push_sql(" order by create_time asc");
}});
// 调用
// SysUser::select_page(page_dto,&item).await?;
```
#### 自定义SQL
```rust
impl SysUser {
    // 支持宏，目前只支持添加了FromRow的结构体使用支持Option和Vec
    #[select(sql="select * from sys_user where is_del=false and id = #{user_id}")]
    pub async fn select_by_user(user_id: &i64) -> Result<Option<SysUser>, Error> {

    }

    #[select(sql="select * from sys_user where is_del=false and user_name = #{user_name}")]
    pub async fn select_by_user(user_name: &str) -> Result<Vec<SysUser>, Error> {

    }
    
    // 可以使用封装的QueryBuilder，也可以使用原生的SQLx
    pub async fn get_count() -> Result<i64,Error> {
        QueryBuilder::<i64>::new_sql("select count(*) from sys_user where is_del = false ")
            .scalar_fetch_one().await
    }

    pub async fn select_by_user_name(user_name: &str) -> Result<Option<SysUser>,Error> {
        QueryBuilder::<SysUser>::new_sql("select * from sys_user where is_del=false and user_name = ? limit 1")
            .bind(user_name)
            .fetch_optional().await
    }

    pub async fn select_page(page_dto: PageDto,item:SysUserPageReq)->Result<WebResultPage<SysUser>,Error>{
        let mut builder = PageBuilder::<SysUser>::
        new_sql(page_dto,"select * from sys_user where is_del=false ");
        if let Some(name) = item.name {
            builder.push_sql(" and name like CONCAT('%', ?, '%') ");
            builder.bind(name);
        }

        if let Some(phone_number) = item.phone_number {
            builder.push_sql(" and phone_number like CONCAT('%', ?, '%') ");
            builder.bind(phone_number);
        }
        builder.push_sql(" order by create_time desc ");
        builder.build_page().await
    }
}


```

#### Redis
```rust
// 定义一个静态的Redis<T>
lazy_static! {
    static ref SYS_USER_CACHI: Redis<SysUser> = Redis::<SysUser>::new("SysUser");
}
// 更多方法详见Redis工具类
// SYS_USER_CACHI.set_minute(id.to_string().as_str(), &sys_user,10).await.ok();
```


## 版权说明

本项目使用 `MIT协议`，默认授权给任何人，被授权人可免费地无限制的使用、复制、修改、合并、发布、发行、再许可、售卖本软件拷贝、并有权向被供应人授予同等的权利，但必须满足以下条件:

- 复制、修改和发行本项目代码需包含原作者的版权及许可信息，包括但不限于文件头注释、协议等

简单来说，作者只想保留版权，没有任何其他限制。

## 