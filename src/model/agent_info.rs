use crate::utils::serialize::deserialize_bool_option;
use crate::utils::serialize::deserialize_id;
use crate::utils::serialize::serialize_datetime;
use crate::utils::serialize::serialize_id;
use rbatis::rbdc::db::ExecResult;
use rbatis::rbdc::DateTime;
use rbatis::{crud, impl_select_page, sql, RBatis};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Deserialize, Serialize, Debug, Clone,Default)]
pub struct AgentInfo {
    pub id: u64,
    pub name: String,
    pub city_id: u64,
    pub owner: Option<u64>,
    #[serde(deserialize_with = "deserialize_bool_option")]
    pub id_del: Option<bool>,
    pub create_by: String,
    pub update_by: String,
    pub create_time: DateTime,
    pub update_time: DateTime,
}

impl AgentInfo {
    #[sql("update agent_info set is_del = true where id = ?")]
    pub async fn delete_by_id(rb: &RBatis, id: &u64) -> rbatis::Result<ExecResult> {
        impled!()
    }

    #[sql("select * from agent_info where is_del=false and id = ? limit 1")]
    pub async fn select_by_id(rb: &RBatis,id: &u64) -> rbatis::Result<Option<AgentInfo>> {
        impled!()
    }

    #[sql("select * from agent_info where is_del=false and name = ? limit 1")]
    pub async fn select_by_name(rb: &RBatis,name: String) -> rbatis::Result<Option<AgentInfo>> {
        impled!()
    }
}

crud!(AgentInfo{});
impl_select_page!( AgentInfo{select_page(item:AgentInfoPageReq) =>"
      where is_del = false
     if item.name != null && item.name != '':
       ` and item.name like CONCAT('%', #{item.name}, '%') `"
});


#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct AgentInfoEditDto {
    #[serde(deserialize_with = "deserialize_id")]
    pub id: u64,
    pub name: String,
    #[serde(deserialize_with = "deserialize_id")]
    pub city_id: u64,
    pub owner: Option<u64>,
    pub id_del: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
// 序列化的时候才转换驼峰
#[serde(rename_all(deserialize = "camelCase"))]
pub struct AgentInfoAddDto {
    pub name: String,
    #[serde(deserialize_with = "deserialize_id")]
    pub city_id: u64,
    pub owner: Option<u64>,
}

impl From<AgentInfo> for AgentInfoVo {
    fn from(data: AgentInfo) -> Self {
        Self {
            id: data.id,
            name: data.name,
            city_id: data.city_id,
            owner: data.owner,
            id_del: data.id_del,
            create_by: data.create_by,
            update_by: data.update_by,
            create_time: data.create_time,
            update_time: data.update_time,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct AgentInfoVo {
    #[serde(serialize_with = "serialize_id")]
    pub id: u64,
    pub name: String,
    pub city_id: u64,
    pub owner: Option<u64>,
    pub id_del: Option<bool>,
    pub create_by: String,
    pub update_by: String,
    #[serde(serialize_with = "serialize_datetime")]
    pub create_time: DateTime,
    #[serde(serialize_with = "serialize_datetime")]
    pub update_time: DateTime,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct AgentInfoPageReq {
    pub name: Option<String>,
}