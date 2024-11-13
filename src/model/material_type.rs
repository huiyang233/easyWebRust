use rbatis::{crud, impl_select_page, RBatis, sql};
use rbatis::rbdc::db::ExecResult;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::utils::serialize::deserialize_bool;
use crate::utils::serialize::deserialize_id;
use crate::utils::serialize::serialize_id;

#[derive(Deserialize, Serialize, Debug, Clone,Default)]
pub struct MaterialType {
    pub id: u64,
    pub name: String,
    #[serde(deserialize_with = "deserialize_bool")]
    pub is_del: bool,
}

impl MaterialType {
    #[sql("update material_type set is_del = true where id = ?")]
    pub async fn delete_by_id(rb: &RBatis, id: &u64) -> rbatis::Result<ExecResult> {
        impled!()
    }

    #[sql("select * from material_type where is_del=false and id = ? limit 1")]
    pub async fn select_by_id(rb: &RBatis,id: &u64) -> rbatis::Result<Option<MaterialType>> {
        impled!()
    }

    #[sql("select * from material_type where is_del=false and name = ? limit 1")]
    pub async fn select_by_name(rb: &RBatis,name: String) -> rbatis::Result<Option<MaterialType>> {
        impled!()
    }
}

crud!(MaterialType{});
impl_select_page!( MaterialType{select_page(name:Option<String>) =>"
      where is_del = false
     if name != null && name != '':
       ` and name like CONCAT('%', #{name}, '%') `"
});


#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct MaterialTypeEditDto {
    #[serde(deserialize_with = "deserialize_id")]
    pub id: u64,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
// 序列化的时候才转换驼峰
#[serde(rename_all(deserialize = "camelCase"))]
pub struct MaterialTypeAddDto {
    pub name: String,
}

impl From<MaterialType> for MaterialTypeVo {
    fn from(data: MaterialType) -> Self {
        Self {
            id: data.id,
            name: data.name,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct MaterialTypeVo {
    #[serde(serialize_with = "serialize_id")]
    pub id: u64,
    pub name: String,
}