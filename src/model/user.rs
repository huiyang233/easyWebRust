use crate::impl_page;
use crate::model::role::SysRoleVo;
use crate::utils::db::{Executor, PageBuilder, QueryBuilder};
use crate::utils::serialize::deserialize_id;
use crate::utils::serialize::deserialize_vec_id_option;
use crate::utils::serialize::serialize_id;
use chrono::{DateTime, Utc};
use derive::CURD;
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, Postgres, Transaction};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone,Default,FromRow,CURD)]
pub struct SysUser {
    #[curd(pk)]
    pub id: i64,
    pub name: String,
    pub user_name: String,
    pub phone_number: Option<String>,
    pub password: String,
    pub enable: bool,
    pub gender: Option<i32>,
    #[curd(logic_del)]
    pub is_del: bool,
    pub is_super_admin: bool,
    pub create_time: DateTime<Utc>,
    pub update_time: DateTime<Utc>,
    pub create_by: String,
    pub update_by: String,
    pub avatar: Option<String>,
    pub wx_open_id:Option<String>
}



#[derive(Deserialize, Serialize, Debug, Clone)]
// 序列化的时候才转换驼峰
#[serde(rename_all(deserialize = "camelCase"))]
pub struct UserEditDto {
    #[serde(deserialize_with = "deserialize_id")]
    pub id: i64,
    pub name: Option<String>,
    pub user_name: Option<String>,
    pub phone_number: Option<String>,
    pub password: Option<String>,
    pub enable: Option<bool>,
    pub gender: Option<i32>,
    pub is_super_admin: Option<bool>,
    pub avatar: Option<String>,
    #[serde(default,deserialize_with = "deserialize_vec_id_option")]
    pub role_ids:Option<Vec<i64>>
}

#[derive(Deserialize, Serialize, Debug, Clone,Validate)]
// 序列化的时候才转换驼峰
#[serde(rename_all(deserialize = "camelCase"))]
pub struct UserAddDto {
    pub name: String,
    #[validate(length(min = 7,max = 20, message = "用户名最小长度是7最大长度是20"))]
    pub user_name: String,
    pub phone_number: Option<String>,
    #[validate(length(min = 8,max = 20, message = "密码长度不能小于8位，不能超过20位"))]
    pub password: String,
    pub enable: bool,
    pub gender: Option<i32>,
    pub is_super_admin: bool,
    pub avatar: Option<String>,
    #[serde(default,deserialize_with = "deserialize_vec_id_option")]
    pub role_ids:Option<Vec<i64>>
}

// 添加修改密码的结构体包括老密码和新密码
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ChangePasswordDto {
    pub old_password: Option<String>,
    pub new_password: Option<String>,
}

// 修改用户信息的结构体
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ChangeUserInfoDto {
    pub name: Option<String>,
    pub phone_number: Option<String>,
    pub gender: Option<i32>,
    pub is_super_admin: Option<bool>,
    pub avatar: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct WxUserVo {
    #[serde(serialize_with = "serialize_id")]
    pub id: i64,
    pub name: String,
    pub user_name: String,
    pub phone_number: Option<String>,
    pub gender: Option<i32>,
    pub avatar_url: Option<String>,

}

impl From<SysUser> for SysUserVo {
    fn from(sys_user: SysUser) -> Self {
        Self{
            id: sys_user.id,
            name: sys_user.name,
            user_name: sys_user.user_name,
            phone_number: sys_user.phone_number,
            gender: sys_user.gender,
            is_super_admin: sys_user.is_super_admin,
            avatar: sys_user.avatar,
            enable: sys_user.enable,
            create_time: sys_user.create_time,
            roles: vec![],
        }
    }
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct SysUserVo {
    #[serde(serialize_with = "serialize_id")]
    pub id: i64,
    pub name: String,
    pub user_name: String,
    pub phone_number: Option<String>,
    pub gender: Option<i32>,
    pub is_super_admin: bool,
    pub avatar: Option<String>,
    pub enable: bool,
    pub create_time: DateTime<Utc>,
    pub roles: Vec<SysRoleVo>,
}


#[derive(Deserialize, Serialize, Debug, Clone,Validate)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct SysUserPageReq {
    pub user_name: Option<String>,
    pub phone_number: Option<String>,

}

pub struct SysUserRole{}
impl SysUserRole {
    pub async fn insert_list(transaction: &mut Transaction<'_, Postgres>,role_ids: &[i64],user_id:&i64) -> Result<u64, Error> {
        let mut builder = QueryBuilder::<Executor>::new_sql("insert into sys_user_role(role_id,user_id) values ");
        for role_id in role_ids {
            builder.push_sql("(?,?),");
            builder = builder
                .bind(role_id)
                .bind(&user_id)

            ;
        }
        builder.trim();
        builder.transaction_execute(transaction).await
    }

    pub async fn delete_role_by_user_id(transaction: &mut Transaction<'_, Postgres>,user_id: &i64) -> Result<u64, Error> {
        QueryBuilder::<Executor>::new_sql("delete from sys_user_role where user_id = ?")
            .bind(user_id)
            .transaction_execute(transaction).await
    }

}



impl SysUser {

    pub async fn select_by_wx_open_id(wx_open_id: &str) -> Result<Option<SysUser>, Error> {
        QueryBuilder::<SysUser>::new_sql("select * from sys_user where wx_open_id = ?")
            .bind(wx_open_id)
            .fetch_optional().await
    }

    pub async fn select_by_user_name(user_name: &str) -> Result<Option<SysUser>,Error> {
        QueryBuilder::<SysUser>::new_sql("select * from sys_user where is_del=false and user_name = ? limit 1")
            .bind(user_name)
            .fetch_optional().await
    }

    pub async fn change_password(password: &str,user_id:&i64) -> Result<u64,Error> {
        QueryBuilder::<Executor>::new_sql("update sys_user set password = ? where id = ?")
            .bind(password)
            .bind(user_id)
            .execute().await
    }

    pub async fn get_count() -> Result<i64,Error> {
        QueryBuilder::<i64>::new_sql("select count(*) from sys_user where is_del = false ")
            .scalar_fetch_one().await
    }

    // pub async fn select_page(item:SysUserPageReq,page_dto: PageDto)->Result<WebResultPage<SysUser>,Error>{
    //     let mut builder = PageBuilder::<SysUser>::
    //     new_sql(page_dto,"select * from sys_user where is_del=false ");
    //     if let Some(name) = item.name {
    //         builder.push_sql(" and name like CONCAT('%', ?, '%') ");
    //         builder.bind(name);
    //     }
    //
    //     if let Some(phone_number) = item.phone_number {
    //         builder.push_sql(" and phone_number like CONCAT('%', ?, '%') ");
    //         builder.bind(phone_number);
    //     }
    //     builder.push_sql(" order by create_time desc ");
    //     builder.build_page().await
    // }
}

impl_page!(SysUser{select_page(req:&SysUserPageReq)=>|builder:&mut PageBuilder<SysUser>|{

     if let Some(user_name) = &req.user_name {
        builder.and_like("user_name",user_name);
    }

    if let Some(phone_number) = &req.phone_number {
        builder.and_like("phone_number",phone_number)
    }
    builder.push_sql(" order by create_time asc");
}});