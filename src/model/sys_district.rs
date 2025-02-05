use crate::model::result::{PageDto, WebResultPage};
use crate::utils::db::{PageBuilder, QueryBuilder};
use derive::CURD;
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow};
use validator::Validate;

#[derive(Debug, Clone,Default,FromRow,CURD)]
pub struct SysDistrict {
    #[curd(pk)]
    pub id: i32,
    pub name: String,
    pub parent_id: i32,
    pub initial: String,
    pub initials: String,
    pub pinyin: String,
    pub suffix: String,
    pub code: String,
    pub sort: i32
}


#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct SysDistrictEditDto {
    pub id: i32,
    pub name: String,
    pub parent_id: i32,
    pub initial: String,
    pub initials: String,
    pub pinyin: String,
    pub suffix: String,
    pub code: String,
    pub sort: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
// 序列化的时候才转换驼峰
#[serde(rename_all(deserialize = "camelCase"))]
pub struct SysDistrictAddDto {
    pub name: String,
    pub parent_id: i32,
    pub initial: String,
    pub initials: String,
    pub pinyin: String,
    pub suffix: String,
    pub code: String,
    pub sort: i32,
}

impl From<SysDistrict> for SysDistrictVo {
    fn from(data: SysDistrict) -> Self {
        Self {
            id: data.id,
            name: data.name,
            parent_id: data.parent_id,
            initial: data.initial,
            initials: data.initials,
            pinyin: data.pinyin,
            suffix: data.suffix,
            code: data.code,
            sort: data.sort,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct SysDistrictVo {
    pub id: i32,
    pub name: String,
    pub parent_id: i32,
    pub initial: String,
    pub initials: String,
    pub pinyin: String,
    pub suffix: String,
    pub code: String,
    pub sort: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone,Validate)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct SysDistrictPageReq {
    pub parent_id: Option<i32>,
}

impl SysDistrict {
    pub async fn select_by_code(code: &String) -> Result<Option<SysDistrict>, Error> {
        QueryBuilder::<SysDistrict>::new_sql("select * from sys_district where code = ? limit 1")
            .bind(code)
            .fetch_optional().await
    }


    pub async fn select_by_name(name: String) -> Result<Option<SysDistrict>,Error> {
        QueryBuilder::<SysDistrict>::new_sql("select * from sys_district where name = ? limit 1")
            .bind(name)
            .fetch_optional().await
    }

    pub async fn select_page(item:SysDistrictPageReq,page_dto: PageDto)->Result<WebResultPage<SysDistrict>,Error>{
        let mut builder = PageBuilder::<SysDistrict>::
        new_sql(page_dto,"select * from sys_district where 1=1 ");
        if let Some(parent_id) = item.parent_id {
            builder.push_sql(" and parent_id = ? ");
            builder.bind(parent_id);
        }else{
            builder.push_sql(" and parent_id = 0 ");
        }
        builder.push_sql(" order by sort asc ");
        builder.build_page().await
    }
}

