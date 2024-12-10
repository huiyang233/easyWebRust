use crate::utils::serialize::deserialize_bool;
use crate::utils::serialize::deserialize_date_option;
use crate::utils::serialize::deserialize_id;
use crate::utils::serialize::serialize_bool;
use crate::utils::serialize::serialize_datetime;
use crate::utils::serialize::serialize_id;
use crate::utils::serialize::serialize_option_datetime;
use rbatis::executor::Executor;
use rbatis::rbdc::db::ExecResult;
use rbatis::rbdc::DateTime;
use rbatis::{crud, impl_insert, impl_select_page, sql, RBatis};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Deserialize, Serialize, Debug, Clone,Default)]
pub struct BlackListConfig {
    pub id: u64,
    pub ban_time: u64,
    pub interval: u64,
    pub visit_count: u64,
}

impl BlackListConfig {

    #[sql("select * from black_list_config where id = ? limit 1")]
    pub async fn select_by_id(rb: &RBatis,id: &u64) -> rbatis::Result<Option<BlackListConfig>> {
        impled!()
    }

}

crud!(BlackListConfig{});


#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct BlackListConfigEditDto {
    #[serde(deserialize_with = "deserialize_id")]
    pub id: u64,
    pub ban_time: u64,
    pub interval: u64,
    pub visit_count: u64,
}



impl From<BlackListConfig> for BlackListConfigVo {
    fn from(data: BlackListConfig) -> Self {
        Self {
            id: data.id,
            ban_time: data.ban_time,
            interval: data.interval,
            visit_count: data.visit_count,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct BlackListConfigVo {
    #[serde(serialize_with = "serialize_id")]
    pub id: u64,
    pub ban_time: u64,
    pub interval: u64,
    pub visit_count: u64,
}




#[derive(Deserialize, Serialize, Debug, Clone,Default)]
pub struct BlackList {
    pub id: u64,
    pub ip: String,
    pub create_time: DateTime,
    #[serde(deserialize_with = "deserialize_bool",serialize_with = "serialize_bool")]
    pub is_del: bool,
    pub reason: String,
    pub ban_time:Option<DateTime>,
}

impl BlackList {
    #[sql("update black_list set is_del = true where ip = ?")]
    pub async fn delete_by_ip(rb: &RBatis, ip: &str) -> rbatis::Result<ExecResult> {
        impled!()
    }

    #[sql("select * from black_list where is_del=false and id = ? limit 1")]
    pub async fn select_by_id(rb: &RBatis,id: &u64) -> rbatis::Result<Option<BlackList>> {
        impled!()
    }

    #[sql("select * from black_list where is_del=false")]
    pub async fn select_all(rb: &RBatis) -> rbatis::Result<Vec<BlackList>> {
        impled!()
    }

    pub async fn insert_list(rb: &dyn Executor, list: &[BlackList]) -> rbatis::Result<ExecResult> {
        let mut sql = String::with_capacity(1084usize);
        let mut args = Vec::with_capacity(2usize);
        sql.push_str("replace into black_list(id,ip,create_time,is_del,reason,ban_time) values");
        for ref black in list {
            args.push(rbs::to_value(&black.id ).unwrap_or_default());
            args.push(rbs::to_value(&black.ip).unwrap_or_default());
            args.push(rbs::to_value(&black.create_time).unwrap_or_default());
            args.push(rbs::to_value(&black.is_del).unwrap_or_default());
            args.push(rbs::to_value(&black.reason).unwrap_or_default());
            args.push(rbs::to_value(&black.ban_time).unwrap_or_default());
            sql.push_str("(?,?,?,?,?,?)");
            sql.push_str(",");
        }
        sql = sql.trim_end_matches(",").to_string();
        rb.exec(&sql, args).await
    }
}



impl_insert!(BlackList{});
impl_select_page!( BlackList{select_page(item:BlackListPageReq) =>"
      where is_del = false
     if item.ip != null && item.ip != '':
       ` and ip like CONCAT('%', #{item.ip}, '%') `"
});


#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct BlackListEditDto {
    #[serde(deserialize_with = "deserialize_id")]
    pub id: u64,
    pub ip: String,
    pub reason: String,
    #[serde(deserialize_with = "deserialize_date_option")]
    pub ban_time:Option<DateTime>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
// 序列化的时候才转换驼峰
#[serde(rename_all(deserialize = "camelCase"))]
pub struct BlackListAddDto {
    pub ip: String,
    pub reason: String,
    #[serde(deserialize_with = "deserialize_date_option")]
    pub ban_time:Option<DateTime>,
}

impl From<BlackList> for BlackListVo {
    fn from(data: BlackList) -> Self {
        Self {
            id: data.id,
            ip: data.ip,
            create_time: data.create_time,
            reason: data.reason,
            ban_time: data.ban_time,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct BlackListVo {
    #[serde(serialize_with = "serialize_id")]
    pub id: u64,
    pub ip: String,
    #[serde(serialize_with = "serialize_datetime")]
    pub create_time: DateTime,
    pub reason: String,
    #[serde(serialize_with = "serialize_option_datetime")]
    pub ban_time:Option<DateTime>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct BlackListPageReq {
    pub ip: Option<String>,
}