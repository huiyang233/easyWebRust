use rbatis::{crud, impl_select_page, RBatis, sql};
use rbatis::executor::Executor;
use rbatis::rbdc::DateTime;
use rbatis::rbdc::db::ExecResult;
use serde::{Deserialize, Serialize};

use crate::utils::serialize::deserialize_id;
use crate::utils::serialize::serialize_datetime;
use crate::utils::serialize::serialize_id;

#[derive(Deserialize, Serialize, Debug, Clone, Default, Eq, PartialEq,Hash)]
pub struct SysPermission {
    pub id: u64,
    pub create_by: String,
    pub create_time: DateTime,
    pub update_by: String,
    pub update_time: DateTime,
    pub name: String,
    pub value: String,
    pub p_id: u64,
}

impl SysPermission {
    #[sql("select * from sys_permission where id = ? limit 1")]
    pub async fn select_by_id(rb: &RBatis,id: &u64) -> rbatis::Result<Option<SysPermission>> {
        impled!()
    }

    #[sql("select * from sys_permission where name = ? limit 1")]
    pub async fn select_by_name(rb: &RBatis,name: String) -> rbatis::Result<Option<SysPermission>> {
        impled!()
    }

    #[sql("select * from sys_permission sp,sys_permission_role spr,sys_user_role sur where \
     sp.id = spr.permission_id and spr.role_id = sur.role_id and sur.user_id = ?")]
    pub async fn select_value_list_by_user_id(rb: &RBatis,user_id: u64) -> rbatis::Result<Vec<SysPermission>> {
        impled!()
    }

    #[sql("select * from sys_permission sp,sys_permission_role spr where \
     sp.id = spr.permission_id and spr.role_id = ?")]
    pub async fn select_by_role_id(rb: &RBatis,role_id: u64) -> rbatis::Result<Vec<SysPermission>> {
        impled!()
    }

    #[sql("select * from sys_permission sp,sys_permission_role spr where \
     sp.id = spr.permission_id and spr.role_id in (?)")]
    pub async fn select_by_role_ids(rb: &RBatis,role_ids: &[u64]) -> rbatis::Result<Vec<SysPermission>> {
        impled!()
    }



    #[sql("delete from sys_permission_role where role_id = ?")]
    pub async fn delete_permission_by_role_id(rb: &dyn Executor, role_id: &u64) -> rbatis::Result<ExecResult> {
        impled!()
    }


    pub async fn insert_list(rb: &dyn Executor, list: &[u64], role_id: &u64) -> rbatis::Result<ExecResult> {
        let mut sql = String::with_capacity(1084usize);
        let mut args = Vec::with_capacity(2usize);
        sql.push_str("insert into sys_permission_role(permission_id,role_id) values");
        for permission_id in list {
            args.push(rbs::to_value(permission_id).unwrap_or_default());
            args.push(rbs::to_value(role_id ).unwrap_or_default());
            sql.push_str("(?,?)");
            sql.push_str(",");
        }
        sql = sql.trim_end_matches(",").to_string();
        rb.exec(&sql, args).await
    }
}

crud!(SysPermission{});
impl_select_page!( SysPermission{select_page(name:Option<String>) =>"
      where 1=1
     if name != null && name != '':
       ` and name like CONCAT('%', #{name}, '%') `"
});


#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct SysPermissionEditDto {
    #[serde(deserialize_with = "deserialize_id")]
    pub id: u64,
    pub name: String,
    pub value: String,
    #[serde(deserialize_with = "deserialize_id")]
    pub p_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
// 序列化的时候才转换驼峰
#[serde(rename_all(deserialize = "camelCase"))]
pub struct SysPermissionAddDto {
    pub name: String,
    pub value: String,
    #[serde(deserialize_with = "deserialize_id")]
    pub p_id: u64,
}

impl From<SysPermission> for SysPermissionVo {
    fn from(data: SysPermission) -> Self {
        Self {
            id: data.id,
            create_by: data.create_by,
            create_time: data.create_time,
            name: data.name,
            value: data.value,
            p_id: data.p_id,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct SysPermissionVo {
    #[serde(serialize_with = "serialize_id")]
    pub id: u64,
    pub create_by: String,
    #[serde(serialize_with = "serialize_datetime")]
    pub create_time: DateTime,
    pub name: String,
    pub value: String,
    pub p_id: u64,
}

#[derive(Deserialize, Serialize,Debug, Clone)]
pub struct Tree{
    #[serde(serialize_with = "serialize_id")]
    pub id: u64,
    pub name:String,
    pub parent_id:u64,
    pub children:Option<Vec<Tree>>
}
impl Tree{
    pub fn build_tree_nodes(items: &[SysPermission], parent_id: u64, ) -> Vec<Tree> {
        items
            .iter()
            .filter(|item| item.p_id == parent_id)
            .map(|item| {
                let mut tree_node = Tree {
                    id: item.id,
                    name: item.name.clone(),
                    parent_id: item.p_id,
                    children: None,
                };

                // 递归构建子节点
                let children = Tree::build_tree_nodes(items, item.id);
                if !children.is_empty() {
                    tree_node.children = Some(children);
                }

                tree_node
            })
            .collect()
    }

}