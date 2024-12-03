use crate::utils::serialize::serialize_id;
use crate::utils::serialize::serialize_option_datetime;
use rbatis::rbdc::DateTime;
use rbatis::{crud, impl_select_page};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Deserialize, Serialize, Debug, Clone,Default)]
pub struct RequestLog {
    pub id: u64,
    pub ip: String,
    pub uri: String,
    pub method:String,
    pub duration:u64,
    pub user_id: Option<u64>,
    pub headers: Option<String>,
    pub query: Option<String>,
    pub create_time: Option<DateTime>,
}


crud!(RequestLog{});
impl_select_page!( RequestLog{select_page(item:RequestLogPageReq) =>"
      where is_del = false
     if item.name != null && item.name != '':
       ` and item.name like CONCAT('%', #{item.name}, '%') `"
});




#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct RequestLogVo {
    #[serde(serialize_with = "serialize_id")]
    pub id: u64,
    pub uri: String,
    pub user_id: Option<u64>,
    pub haders: Option<String>,
    pub req_body: Option<String>,
    pub query: Option<String>,
    pub res_bodu: Option<String>,
    #[serde(serialize_with = "serialize_option_datetime")]
    pub create_time: Option<DateTime>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct RequestLogPageReq {
    pub name: Option<String>,
}