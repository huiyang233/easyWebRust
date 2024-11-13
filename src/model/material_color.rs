use rbatis::{crud, impl_select_page, RBatis, sql};
use rbatis::rbdc::db::ExecResult;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::utils::serialize::deserialize_bool;
use crate::utils::serialize::deserialize_id;
use crate::utils::serialize::serialize_bool;
use crate::utils::serialize::serialize_id;

#[derive(Deserialize, Serialize, Debug, Clone,Default)]
pub struct MaterialColor {
    pub id: u64,
    pub name: String,
    pub color: String,
    #[serde(deserialize_with = "deserialize_bool",serialize_with = "serialize_bool")]
    pub is_del: bool,
}

impl MaterialColor {
    #[sql("update material_color set is_del = true where id = ?")]
    pub async fn delete_by_id(rb: &RBatis, id: &u64) -> rbatis::Result<ExecResult> {
        impled!()
    }

    #[sql("select * from material_color where is_del=false and id = ? limit 1")]
    pub async fn select_by_id(rb: &RBatis,id: &u64) -> rbatis::Result<Option<MaterialColor>> {
        impled!()
    }

    #[sql("select * from material_color where is_del=false and name = ? limit 1")]
    pub async fn select_by_name(rb: &RBatis,name: String) -> rbatis::Result<Option<MaterialColor>> {
        impled!()
    }
}

crud!(MaterialColor{});
impl_select_page!( MaterialColor{select_page(name:Option<String>) =>"
      where is_del = false
     if name != null && name != '':
       ` and name like CONCAT('%', #{name}, '%') `"
});


#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct MaterialColorEditDto {
    #[serde(deserialize_with = "deserialize_id")]
    pub id: u64,
    pub name: String,
    pub color: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
// 序列化的时候才转换驼峰
#[serde(rename_all(deserialize = "camelCase"))]
pub struct MaterialColorAddDto {
    pub name: String,
    pub color: String,
}

impl From<MaterialColor> for MaterialColorVo {
    fn from(data: MaterialColor) -> Self {
        Self {
            id: data.id,
            name: data.name,
            color: data.color,
            is_del: data.is_del,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct MaterialColorVo {
    #[serde(serialize_with = "serialize_id")]
    pub id: u64,
    pub name: String,
    pub color: String,
    pub is_del: bool,
}