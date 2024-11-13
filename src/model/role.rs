use rbatis::{impl_insert, impl_select_page, impl_update, RBatis, sql};
use rbatis::executor::Executor;
use rbatis::rbdc::DateTime;
use rbatis::rbdc::db::ExecResult;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::model::permission::SysPermissionVo;
use crate::utils::serialize::deserialize_bool;
use crate::utils::serialize::deserialize_id;
use crate::utils::serialize::deserialize_vec_id_option;
use crate::utils::serialize::serialize_bool;
use crate::utils::serialize::serialize_datetime;
use crate::utils::serialize::serialize_id;

#[derive(Deserialize, Serialize, Debug, Clone,Default)]
pub struct SysRole {
    pub id: u64,
    pub create_by: String,
    pub create_time: DateTime,
    #[serde(deserialize_with = "deserialize_bool",serialize_with = "serialize_bool")]
    pub is_del: bool,
    pub update_by: String,
    pub update_time: DateTime,
    pub name: String
}

impl SysRole {
    #[sql("update sys_role set is_del = true where id = ?")]
    pub async fn delete_by_id(rb: &RBatis, id: &u64) -> rbatis::Result<ExecResult> {
        impled!()
    }

    #[sql("select * from sys_role where is_del=false and id = ? limit 1")]
    pub async fn select_by_id(rb: &RBatis,id: &u64) -> rbatis::Result<Option<SysRole>> {
        impled!()
    }

    #[sql("select * from sys_role where is_del=false and name = ? limit 1")]
    pub async fn select_by_name(rb: &RBatis,name: String) -> rbatis::Result<Option<SysRole>> {
        impled!()
    }

    #[sql("select sr.* from sys_role sr,sys_user_role sur where sur.user_id = ?  \
    and sr.is_del=false and sr.id = sur.role_id ")]
    pub async fn select_by_user_id(rb: &RBatis,user_id: &u64) -> rbatis::Result<Vec<SysRole>>{
        impled!()
    }

    #[sql("delete from sys_user_role where user_id = ?")]
    pub async fn delete_role_by_user_id(rb: &dyn Executor, user_id: &u64) -> rbatis::Result<ExecResult> {
        impled!()
    }

    #[sql("select * from sys_role where is_del= false")]
    pub async fn select_all(rb: &dyn Executor) -> rbatis::Result<Vec<SysRole>> {
        impled!()
    }




    pub async fn insert_list(rb: &dyn Executor, list: &[u64],user_id:&u64) -> rbatis::Result<ExecResult> {
        let mut sql = String::with_capacity(1084usize);
        let mut args = Vec::with_capacity(2usize);
        sql.push_str("insert into sys_user_role(role_id,user_id) values");
        for ref role_id in list {
            args.push(rbs::to_value(role_id ).unwrap_or_default());
            args.push(rbs::to_value(user_id).unwrap_or_default());
            sql.push_str("(?,?)");
            sql.push_str(",");
        }
        sql = sql.trim_end_matches(",").to_string();
        rb.exec(&sql, args).await
    }


}
// crud!(SysRole{});
impl_insert!(SysRole{});
impl_update!(SysRole{});
impl_select_page!( SysRole{select_page(name:Option<String>) =>"
      where is_del = false
     if name != null && name != '':
       ` and name like CONCAT('%', #{name}, '%') `"
});


#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct SysRoleEditDto {
    #[serde(deserialize_with = "deserialize_id")]
    pub id: u64,
    pub name: Option<String>,
    #[serde(default,deserialize_with = "deserialize_vec_id_option")]
    pub permission_ids:Option<Vec<u64>>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
// 序列化的时候才转换驼峰
#[serde(rename_all(deserialize = "camelCase"))]
pub struct SysRoleAddDto {
    pub name: String,
    #[serde(default,deserialize_with = "deserialize_vec_id_option")]
    pub permission_ids:Option<Vec<u64>>
}

impl From<SysRole> for SysRoleVo {
    fn from(data: SysRole) -> Self {
        Self {
            id: data.id,
            create_by: data.create_by,
            create_time: data.create_time,
            name: data.name,
            permissions: vec![],
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct SysRoleVo {
    #[serde(serialize_with = "serialize_id")]
    pub id: u64,
    pub create_by: String,
    #[serde(serialize_with = "serialize_datetime")]
    pub create_time: DateTime,
    pub name: String,
    pub permissions: Vec<SysPermissionVo>,
}