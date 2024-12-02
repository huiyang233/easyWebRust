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
     if item.user_name != null && item.user_name != '':
       ` and user_name like CONCAT('%', #{item.user_name}, '%') `
     if item.log_type != null && item.log_type != '':
       ` and log_type = #{item.log_type}`
      if item.start_time != null && item.start_time != '':
       ` and create_time >= #{item.start_time}`
      if item.end_time != null && item.end_time != '':
       ` and create_time <= #{item.end_time}`
     "



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

use validator::Validate;

#[derive(Deserialize, Serialize, Validate, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct SysLogPageReq {
    pub user_name: Option<String>,
    #[validate(range(min = 1, max = 4, message = "日志类型只能是1-4"))]
    pub log_type: Option<i32>,
    pub start_time: Option<DateTime>,
    pub end_time: Option<DateTime>,
}