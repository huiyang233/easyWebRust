use crate::model::result::{PageDto, WebResultPage};
use crate::utils::db::{PageBuilder, QueryBuilder};
use crate::utils::serialize::deserialize_id;
use crate::utils::serialize::serialize_id;
use chrono::{DateTime, Utc};
use derive::CURD;
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, Type};
use validator::Validate;

#[derive(Debug, Type, Clone, Copy, PartialEq, Default)]
#[repr(i32)]
// 1=本地 2=云
pub enum SaveType {
    #[default]
    Local = 1,
    Cloud = 2,
}

#[derive(Debug, Clone,Default,FromRow,CURD)]
pub struct SysFile {
    #[curd(pk)]
    pub id: i64,
    pub create_by: String,
    pub create_time: DateTime<Utc>,
    pub update_by: String,
    pub update_time: DateTime<Utc>,
    pub file_path: String,
    pub name: String,
    pub size: String,
    pub suffix: String,
    pub save_type: SaveType,
    pub url_path: String,
    pub md5: String,
    #[curd(logic_del)]
    pub is_del: bool
}


#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct SysFileEditDto {
    #[serde(deserialize_with = "deserialize_id")]
    pub id: i64,
    pub file_path: String,
    pub name: String,
    pub size: String,
    pub suffix: String,
    pub save_type: i32,
    pub url_path: String,
    pub md5: String,
}

#[derive(Deserialize, Debug, Clone)]
// 序列化的时候才转换驼峰
#[serde(rename_all(deserialize = "camelCase"))]
pub struct SysFileAddDto {
    pub file_path: String,
    pub name: String,
    pub size: String,
    pub suffix: String,
    pub save_type: i32,
    pub url_path: String,
    pub md5: String,
}

impl From<SysFile> for SysFileVo {
    fn from(data: SysFile) -> Self {
        Self {
            id: data.id,
            url_path: data.url_path,
        }
    }
}

#[derive( Serialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct SysFileVo {
    #[serde(serialize_with = "serialize_id")]
    pub id: i64,
    pub url_path: String,
}

#[derive(Deserialize, Debug, Clone,Validate)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct SysFilePageReq {
    #[validate(length(min = 1, message = "最小长度必须是1"))]
    pub name: Option<String>,
}

impl SysFile {
    pub async fn select_by_name(name: String) -> Result<Option<SysFile>,Error> {
        QueryBuilder::<SysFile>::new_sql("select * from sys_file where is_del=false and name = ? limit 1")
            .bind(name)
            .fetch_optional().await
    }

    pub async fn select_page(item:SysFilePageReq,page_dto: PageDto)->Result<WebResultPage<SysFile>,Error>{
        let mut builder = PageBuilder::<SysFile>::
        new_sql(page_dto,"select * from sys_file where is_del=false ");
        if let Some(name) = item.name {
            builder.and_like("name",name);
        }
        builder.push_sql(" order by create_time desc ");
        builder.build_page().await
    }
}