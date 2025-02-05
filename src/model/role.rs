use crate::model::permission::SysPermissionVo;
use crate::model::result::{PageDto, WebResultPage};
use crate::utils::db::{PageBuilder, QueryBuilder};
use crate::utils::serialize::deserialize_id;
use crate::utils::serialize::deserialize_vec_id_option;
use crate::utils::serialize::serialize_id;
use chrono::{DateTime, Utc};
use derive::CURD;
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, Postgres, Transaction};

#[derive(Deserialize,Serialize,Debug, Clone,Default,FromRow,CURD)]
pub struct SysRole {
    #[curd(pk)]
    pub id: i64,
    pub create_by: String,
    pub create_time: DateTime<Utc>,
    #[curd(logic_del)]
    pub is_del: bool,
    pub update_by: String,
    pub update_time: DateTime<Utc>,
    pub name: String
}

pub struct SysRolePermission{}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct SysRoleEditDto {
    #[serde(deserialize_with = "deserialize_id")]
    pub id: i64,
    pub name: Option<String>,
    #[serde(default,deserialize_with = "deserialize_vec_id_option")]
    pub permission_ids:Option<Vec<i64>>
}

#[derive(Deserialize, Debug, Clone)]
// 序列化的时候才转换驼峰
#[serde(rename_all(deserialize = "camelCase"))]
pub struct SysRoleAddDto {
    pub name: String,
    #[serde(default,deserialize_with = "deserialize_vec_id_option")]
    pub permission_ids:Option<Vec<i64>>
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct SysRoleVo {
    #[serde(serialize_with = "serialize_id")]
    pub id: i64,
    pub create_by: String,
    pub create_time: DateTime<Utc>,
    pub name: String,
    pub permissions: Vec<SysPermissionVo>,
}


#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct SysRolePageReq {
    pub name: Option<String>,
}


impl SysRolePermission {

    pub async fn insert_list(transaction: &mut Transaction<'_, Postgres>,permission_ids: &[i64],role_id:&i64) -> Result<u64, Error> {
        let mut builder = QueryBuilder::<crate::utils::db::Executor>::new_sql("insert into sys_permission_role(permission_id,role_id) values ");
        for permission in permission_ids {
            builder.push_sql("(?,?),");
            builder = builder
                .bind(role_id)
                .bind(&permission)

            ;
        }
        builder.trim();
        builder.transaction_execute(transaction).await
    }

    pub async fn delete_permission_by_role_id(transaction: &mut Transaction<'_, Postgres>,role_id: &i64) -> Result<u64, Error> {
        QueryBuilder::<crate::utils::db::Executor>::new_sql("delete from sys_permission_role where role_id = ?")
            .bind(role_id)
            .transaction_execute(transaction).await
    }

}

impl SysRole {

    pub async fn select_by_name(name: String) -> Result<Option<SysRole>,Error> {
        QueryBuilder::<SysRole>::new_sql("select * from sys_role where is_del=false and name = ? limit 1")
            .bind(name)
            .fetch_optional().await
    }

    pub async fn select_by_user_id(user_id: &i64) -> Result<Vec<SysRole>,Error> {
        QueryBuilder::<SysRole>::new_sql(r"select sr.* from sys_role sr,sys_user_role sur where sur.user_id = ?
     and sr.is_del=false and sr.id = sur.role_id")
            .bind(user_id)
            .fetch_all().await
    }

    pub async fn select_all() -> Result<Vec<SysRole>,Error> {
        QueryBuilder::<SysRole>::new_sql(r"select * from sys_role where is_del= false")
            .fetch_all().await
    }


    pub async fn select_page(item:SysRolePageReq,page_dto: PageDto)->Result<WebResultPage<SysRole>,Error>{
        let mut builder = PageBuilder::<SysRole>::
        new_sql(page_dto,"select * from sys_role where is_del=false ");
        if let Some(name) = item.name {
            builder.push_sql(" and name like CONCAT('%', ?, '%') ");
            builder.bind(name);
        }
        builder.push_sql(" order by create_time desc ");
        builder.build_page().await
    }


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
