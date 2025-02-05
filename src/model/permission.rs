use crate::model::result::{PageDto, WebResultPage};
use crate::utils::db::{PageBuilder, QueryBuilder};
use crate::utils::serialize::deserialize_id;
use crate::utils::serialize::serialize_id;
use chrono::{DateTime, Utc};
use derive::CURD;
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow};

#[derive(Deserialize, Serialize, Debug, Clone,Default,Eq, PartialEq,Hash,FromRow,CURD )]
pub struct SysPermission {
    #[curd(pk)]
    pub id: i64,
    pub create_by: String,
    pub create_time: DateTime<Utc>,
    pub update_by: String,
    pub update_time: DateTime<Utc>,
    pub name: String,
    pub value: String,
    pub p_id: i64
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct SysPermissionEditDto {
    #[serde(deserialize_with = "deserialize_id")]
    pub id: i64,
    pub name: String,
    pub value: String,
    #[serde(deserialize_with = "deserialize_id")]
    pub p_id: i64,
}

#[derive(Deserialize, Debug, Clone)]
// 序列化的时候才转换驼峰
#[serde(rename_all(deserialize = "camelCase"))]
pub struct SysPermissionAddDto {
    pub name: String,
    pub value: String,
    #[serde(deserialize_with = "deserialize_id")]
    pub p_id: i64,
}


#[derive(Serialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct SysPermissionVo {
    #[serde(serialize_with = "serialize_id")]
    pub id: i64,
    pub create_by: String,
    pub create_time: DateTime<Utc>,
    pub name: String,
    pub value: String,
    pub p_id: i64,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct SysPermissionPageReq {
    pub name: Option<String>,
}

#[derive( Serialize,Debug, Clone)]
pub struct Tree{
    #[serde(serialize_with = "serialize_id")]
    pub id: i64,
    pub name:String,
    pub parent_id:i64,
    pub children:Option<Vec<Tree>>
}




impl SysPermission {
    pub async fn select_value_list_by_user_id(user_id: &i64) -> Result<Vec<SysPermission>, Error> {
        QueryBuilder::<SysPermission>::new_sql(r"select * from sys_permission sp,sys_permission_role spr,sys_user_role sur where
        sp.id = spr.permission_id and spr.role_id = sur.role_id and sur.user_id = ?")
            .bind(user_id)
            .fetch_all().await
    }

    pub async fn select_by_role_id(role_id: &i64) -> Result<Vec<SysPermission>, Error> {
        QueryBuilder::<SysPermission>::new_sql(r"select * from sys_permission sp,sys_permission_role spr where
     sp.id = spr.permission_id and spr.role_id = ?")
            .bind(role_id)
            .fetch_all().await
    }

    pub async fn select_by_role_ids(role_ids: &[i64]) -> Result<Vec<SysPermission>, Error> {
        QueryBuilder::<SysPermission>::new_sql(r"select * from sys_permission sp,sys_permission_role spr where
     sp.id = spr.permission_id and spr.role_id in (?)")
            .bind(role_ids)
            .fetch_all().await
    }

    pub async fn select_all() -> Result<Vec<SysPermission>, Error> {
        QueryBuilder::<SysPermission>::new_sql("select * from sys_permission ")
            .fetch_all().await
    }

    pub async fn select_by_name(name: String) -> Result<Option<SysPermission>,Error> {
        QueryBuilder::<SysPermission>::new_sql("select * from sys_permission where name = ? limit 1")
            .bind(name)
            .fetch_optional().await
    }

    pub async fn select_page(item:SysPermissionPageReq,page_dto: PageDto)->Result<WebResultPage<SysPermission>,Error>{
        let mut builder = PageBuilder::<SysPermission>::
        new_sql(page_dto,"select * from sys_permission where 1=1 ");
        if let Some(name) = item.name {
            builder.push_sql(" and name like CONCAT('%', ?, '%') ");
            builder.bind(name);
        }
        builder.push_sql(" order by create_time desc ");
        builder.build_page().await
    }
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

impl Tree{
    pub fn build_tree_nodes(items: &[SysPermission], parent_id: i64, ) -> Vec<Tree> {
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