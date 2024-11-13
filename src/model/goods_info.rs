use rbatis::{crud, impl_select_page, RBatis, sql};
use rbatis::rbdc::DateTime;
use rbatis::rbdc::db::ExecResult;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::utils::serialize::deserialize_bool;
use crate::utils::serialize::deserialize_id;
use crate::utils::serialize::serialize_bool;
use crate::utils::serialize::serialize_datetime;
use crate::utils::serialize::serialize_id;

#[derive(Deserialize, Serialize, Debug, Clone,Default)]
pub struct GoodsInfo {
    pub id: u64,
    #[serde(deserialize_with = "deserialize_bool",serialize_with = "serialize_bool")]
    pub is_del: bool,
    pub create_time: DateTime,
    pub update_time: DateTime,
    pub create_by: String,
    pub update_by: String,
    pub name: String,
    pub category_id: Option<u64>,
    pub main_image: Option<String>,
    pub description: Option<String>,
    pub status: i32,
}

impl GoodsInfo {
    #[sql("update goods_info set is_del = true where id = ?")]
    pub async fn delete_by_id(rb: &RBatis, id: &u64) -> rbatis::Result<ExecResult> {
        impled!()
    }

    #[sql("select * from goods_info where is_del=false and id = ? limit 1")]
    pub async fn select_by_id(rb: &RBatis,id: &u64) -> rbatis::Result<Option<GoodsInfo>> {
        impled!()
    }

    #[sql("select * from goods_info where is_del=false and name = ? limit 1")]
    pub async fn select_by_name(rb: &RBatis,name: String) -> rbatis::Result<Option<GoodsInfo>> {
        impled!()
    }
}

crud!(GoodsInfo{});
impl_select_page!( GoodsInfo{select_page(name:Option<String>) =>"
      where is_del = false
     if name != null && name != '':
       ` and name like CONCAT('%', #{name}, '%') `"
});


#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct GoodsInfoEditDto {
    #[serde(deserialize_with = "deserialize_id")]
    pub id: u64,
    pub name: String,
    pub category_id: Option<u64>,
    pub main_image: Option<String>,
    pub description: Option<String>,
    pub status: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
// 序列化的时候才转换驼峰
#[serde(rename_all(deserialize = "camelCase"))]
pub struct GoodsInfoAddDto {
    pub name: String,
    pub category_id: Option<u64>,
    pub main_image: Option<String>,
    pub description: Option<String>,
    pub status: i32,
}

impl From<GoodsInfo> for GoodsInfoVo {
    fn from(data: GoodsInfo) -> Self {
        Self {
            id: data.id,
            create_time: data.create_time,
            update_time: data.update_time,
            create_by: data.create_by,
            update_by: data.update_by,
            name: data.name,
            category_id: data.category_id,
            main_image: data.main_image,
            description: data.description,
            status: data.status,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct GoodsInfoVo {
    #[serde(serialize_with = "serialize_id")]
    pub id: u64,
    #[serde(serialize_with = "serialize_datetime")]
    pub create_time: DateTime,
    #[serde(serialize_with = "serialize_datetime")]
    pub update_time: DateTime,
    pub create_by: String,
    pub update_by: String,
    pub name: String,
    pub category_id: Option<u64>,
    pub main_image: Option<String>,
    pub description: Option<String>,
    pub status: i32,
}