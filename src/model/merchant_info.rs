use crate::utils::serialize::deserialize_bool_option;
use crate::utils::serialize::deserialize_id;
use crate::utils::serialize::serialize_id;
use crate::utils::serialize::serialize_option_datetime;
use rbatis::rbdc::db::ExecResult;
use rbatis::rbdc::DateTime;
use rbatis::{crud, impl_select_page, sql, RBatis};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Deserialize, Serialize, Debug, Clone,Default)]
pub struct MerchantInfo {
    pub id: u64,
    pub name: Option<String>,
    pub audit_status: Option<i32>,
    pub owner: Option<u64>,
    pub agent_id: Option<u64>,
    pub city_id: Option<u64>,
    #[serde(deserialize_with = "deserialize_bool_option")]
    pub is_del: Option<bool>,
    pub create_by: Option<String>,
    pub update_by: Option<String>,
    pub create_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
}

impl MerchantInfo {
    #[sql("update merchant_info set is_del = true where id = ?")]
    pub async fn delete_by_id(rb: &RBatis, id: &u64) -> rbatis::Result<ExecResult> {
        impled!()
    }

    #[sql("select * from merchant_info where is_del=false and id = ? limit 1")]
    pub async fn select_by_id(rb: &RBatis,id: &u64) -> rbatis::Result<Option<MerchantInfo>> {
        impled!()
    }

    #[sql("select * from merchant_info where is_del=false and name = ? limit 1")]
    pub async fn select_by_name(rb: &RBatis,name: String) -> rbatis::Result<Option<MerchantInfo>> {
        impled!()
    }
}

crud!(MerchantInfo{});
impl_select_page!( MerchantInfo{select_page(item:MerchantInfoPageReq) =>"
      where is_del = false
     if item.name != null && item.name != '':
       ` and item.name like CONCAT('%', #{item.name}, '%') `"
});


#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct MerchantInfoEditDto {
    #[serde(deserialize_with = "deserialize_id")]
    pub id: u64,
    pub name: Option<String>,
    pub audit_status: Option<i32>,
    pub owner: Option<u64>,
    pub agent_id: Option<u64>,
    pub city_id: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
// 序列化的时候才转换驼峰
#[serde(rename_all(deserialize = "camelCase"))]
pub struct MerchantInfoAddDto {
    pub name: String,
    pub audit_status: i32,
    pub owner: u64,
    pub agent_id: u64,
    pub city_id: u64,
}

impl From<MerchantInfo> for MerchantInfoVo {
    fn from(data: MerchantInfo) -> Self {
        Self {
            id: data.id,
            name: data.name,
            audit_status: data.audit_status,
            owner: data.owner,
            agent_id: data.agent_id,
            city_id: data.city_id,
            is_del: data.is_del,
            create_by: data.create_by,
            update_by: data.update_by,
            create_time: data.create_time,
            update_time: data.update_time,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct MerchantInfoVo {
    #[serde(serialize_with = "serialize_id")]
    pub id: u64,
    pub name: Option<String>,
    pub audit_status: Option<i32>,
    pub owner: Option<u64>,
    pub agent_id: Option<u64>,
    pub city_id: Option<u64>,
    pub is_del: Option<bool>,
    pub create_by: Option<String>,
    pub update_by: Option<String>,
    #[serde(serialize_with = "serialize_option_datetime")]
    pub create_time: Option<DateTime>,
    #[serde(serialize_with = "serialize_option_datetime")]
    pub update_time: Option<DateTime>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct MerchantInfoPageReq {
    pub name: Option<String>,
}