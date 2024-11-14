use rbatis::rbdc::db::ExecResult;
use rbatis::rbdc::DateTime;
use rbatis::{crud, impl_select, impl_select_page, sql, RBatis};
use serde::{Deserialize, Serialize};

use crate::model::role::SysRoleVo;
use crate::utils::serialize::deserialize_bool;
use crate::utils::serialize::deserialize_id;
use crate::utils::serialize::deserialize_vec_id_option;
use crate::utils::serialize::serialize_bool;
use crate::utils::serialize::serialize_datetime;
use crate::utils::serialize::serialize_id;

#[derive(Deserialize, Serialize, Debug, Clone,Default)]
pub struct SysUser {
    pub id: u64,
    pub name: Option<String>,
    pub user_name: String,
    pub phone_number: Option<String>,
    pub password: Option<String>,
    #[serde(deserialize_with = "deserialize_bool",serialize_with = "serialize_bool")]
    pub enable: bool,
    pub gender: Option<i32>,
    #[serde(deserialize_with = "deserialize_bool",serialize_with = "serialize_bool")]
    pub is_super_admin: bool,
    pub create_time: DateTime,
    pub create_by: String,
    pub update_time:  DateTime,
    pub update_by: String,
    pub avatar: Option<String>,
    #[serde(deserialize_with = "deserialize_bool",serialize_with = "serialize_bool")]
    pub is_del:bool,
}

impl SysUser {
    #[sql("update sys_user set is_del = true where id = ?")]
    pub async fn delete_by_id(rb: &RBatis, user_id: &u64) -> rbatis::Result<ExecResult> {
        impled!()
    }

    #[sql("update sys_user set password = ? where id = ?")]
    pub async fn change_password(rb: &RBatis,password:&str ,user_id: &u64) -> rbatis::Result<ExecResult> {
        impled!()
    }
}
crud!(SysUser{});
impl_select!(SysUser{select_by_user_name(user_name:&str) -> Option => "`where is_del=false and user_name = #{user_name} limit 1`"});
impl_select!(SysUser{select_by_id(id:&u64) -> Option => "`where is_del=false and id = #{id} limit 1`"});
impl_select_page!(SysUser{select_page_by_user_name(user_name:Option<String>,phone_number:Option<String>) =>"
      where is_del = false
      if user_name != null && user_name != '':
       ` and user_name like CONCAT('%', #{user_name}, '%') `
      if phone_number != null && phone_number != '':
       ` and phone_number like CONCAT('%', #{phone_number}, '%') `

       "
});


#[derive(Deserialize, Serialize, Debug, Clone)]
// 序列化的时候才转换驼峰
#[serde(rename_all(deserialize = "camelCase"))]
pub struct UserEditDto {
    #[serde(deserialize_with = "deserialize_id")]
    pub id: u64,
    pub name: Option<String>,
    pub user_name: Option<String>,
    pub phone_number: Option<String>,
    pub password: Option<String>,
    pub enable: Option<bool>,
    pub gender: Option<i32>,
    pub is_super_admin: Option<bool>,
    pub avatar: Option<String>,
    #[serde(default,deserialize_with = "deserialize_vec_id_option")]
    pub role_ids:Option<Vec<u64>>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
// 序列化的时候才转换驼峰
#[serde(rename_all(deserialize = "camelCase"))]
pub struct UserAddDto {
    pub name: String,
    pub user_name: String,
    pub phone_number: Option<String>,
    pub password: String,
    pub enable: bool,
    pub gender: Option<i32>,
    pub is_super_admin: bool,
    pub avatar: Option<String>,
    #[serde(default,deserialize_with = "deserialize_vec_id_option")]
    pub role_ids:Option<Vec<u64>>
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

impl From<SysUser> for UserVo {
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

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct UserVo {
    #[serde(serialize_with = "serialize_id")]
    pub id: u64,
    pub name: Option<String>,
    pub user_name: String,
    pub phone_number: Option<String>,
    pub gender: Option<i32>,
    pub is_super_admin: bool,
    pub avatar: Option<String>,
    pub enable: bool,
    #[serde(serialize_with = "serialize_datetime")]
    pub create_time: DateTime,
    pub roles: Vec<SysRoleVo>,
}

