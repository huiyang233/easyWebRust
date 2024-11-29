use crate::utils::serialize::serialize_datetime;
use crate::utils::serialize::serialize_id;
use rbatis::rbdc::DateTime;
use rbatis::{crud, impl_select_page, sql, RBatis};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Deserialize, Serialize, Debug, Clone,Default)]
pub struct SysLog {
    pub id: u64,
    pub name: String,
    pub log_type: i32,
    pub description: String,
    pub user_name: String,
    pub ip: String,
    pub create_time: DateTime,
}

impl SysLog {
    #[sql("select * from sys_log where id = ? limit 1")]
    pub async fn select_by_id(rb: &RBatis,id: &u64) -> rbatis::Result<Option<SysLog>> {
        impled!()
    }

    #[sql("select * from sys_log where name = ? limit 1")]
    pub async fn select_by_name(rb: &RBatis,name: String) -> rbatis::Result<Option<SysLog>> {
        impled!()
    }
}

crud!(SysLog{});
impl_select_page!( SysLog{select_page(item:SysLogPageReq) =>"
      where 1=1
     if item.name != null && item.name != '':
       ` and item.name like CONCAT('%', #{item.name}, '%') `"
});


impl From<SysLog> for SysLogVo {
    fn from(data: SysLog) -> Self {
        Self {
            id: data.id,
            name: data.name,
            log_type: data.log_type,
            description: data.description,
            user_name: data.user_name,
            ip: data.ip,
            create_time: data.create_time,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct SysLogVo {
    #[serde(serialize_with = "serialize_id")]
    pub id: u64,
    pub name: String,
    pub log_type: i32,
    pub description: String,
    pub user_name: String,
    pub ip: String,
    #[serde(serialize_with = "serialize_datetime")]
    pub create_time: DateTime,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct SysLogPageReq {
    pub name: Option<String>,
}