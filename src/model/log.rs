use crate::model::result::{PageDto, WebResultPage};
use crate::utils::db::{Executor, PageBuilder, QueryBuilder};
use crate::utils::serialize::serialize_id;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_repr::Serialize_repr;
use sqlx::{Error, FromRow, Type};
use validator::Validate;

#[derive(Deserialize, Serialize_repr,Debug, Type, Clone, Copy,Default)]
#[repr(i32)]
pub enum LogType {
    UserLogin=1,
    #[default]
    UnKnow=999,
}

#[derive(Deserialize, Serialize, Debug, Clone,FromRow,Default)]
pub struct SysLog {
    pub id: i64,
    pub name: String,
    pub log_type: LogType,
    pub description: String,
    pub user_name: String,
    pub ip: String,
    pub create_time: DateTime<Utc>
}

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
    pub id: i64,
    pub name: String,
    pub log_type: LogType,
    pub description: String,
    pub user_name: String,
    pub ip: String,
    pub create_time: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Validate, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct SysLogPageReq {
    pub user_name: Option<String>,
    #[validate(range(min = 1, max = 4, message = "日志类型只能是1-4"))]
    pub log_type: Option<i32>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
}

impl SysLog {

    pub async fn update_by_id(item:&SysLog) -> Result<u64, Error> {
        let mut builder = QueryBuilder::<Executor>::new_sql("update sys_log set  ");

        builder.push_sql(" name = ? ,");
        builder.bind_value(&item.name);

        builder.push_sql(" log_type = ? ,");
        builder.bind_value(item.log_type);

        builder.push_sql(" description = ? ,");
        builder.bind_value(&item.description);

        builder.push_sql(" user_name = ? ,");
        builder.bind_value(&item.user_name);

        builder.push_sql(" ip = ? ,");
        builder.bind_value(&item.ip);

        builder.push_sql(" create_time = ? ,");
        builder.bind_value(item.create_time);


        builder.trim();
        builder.push_sql(" where id = ? ");
        builder.bind(item.id).execute().await
    }

    pub async fn insert(item:&SysLog) -> Result<u64, Error> {
        let builder =  QueryBuilder::<Executor>::new_sql("insert into sys_log (id,name,log_type,description,user_name,ip,create_time) values (?,?,?,?,?,?,?) ")
            .bind(item.id)
            .bind(&item.name)
            .bind(item.log_type)
            .bind(&item.description)
            .bind(&item.user_name)
            .bind(&item.ip)
            .bind(item.create_time)
            ;
        builder.execute().await
    }

    pub async fn insert_batch(item:&Vec<SysLog>) -> Result<u64, Error> {
        let mut sql = "insert into sys_log () values ".to_string();
        let mut builder = QueryBuilder::<Executor>::new();
        for x in item {
            sql.push_str("(?,?,?,?,?,?,?)");
            sql.push_str(",");
            builder = builder
                .bind(x.id)
                .bind(&x.name)
                .bind(x.log_type)
                .bind(&x.description)
                .bind(&x.user_name)
                .bind(&x.ip)
                .bind(x.create_time)
            ;
        }
        sql = sql.trim_end_matches(",").to_string();
        builder.push_sql(&sql);
        builder.execute().await
    }

    pub async fn select_by_id(id: &i64) -> Result<Option<SysLog>, Error> {
        QueryBuilder::<SysLog>::new_sql("select * from sys_log where id = ? limit 1")
            .bind(id)
            .fetch_optional().await
    }


    pub async fn select_by_name(name: String) -> Result<Option<SysLog>,Error> {
        QueryBuilder::<SysLog>::new_sql("select * from sys_log where name = ? limit 1")
            .bind(name)
            .fetch_optional().await
    }

    pub async fn select_page(item:SysLogPageReq,page_dto: PageDto)->Result<WebResultPage<SysLog>,Error>{
        let mut builder = PageBuilder::<SysLog>::
        new_sql(page_dto,"select * from sys_log where 1=1 ");
        if let Some(name) = item.user_name {
            builder.push_sql(" and user_name like CONCAT('%', ?, '%') ");
            builder.bind(name);
        }
        if let Some(start_time) = item.start_time {
            builder.push_sql(" and create_time >= ? ");
            builder.bind(start_time);
        }
        if let Some(end_time) = item.end_time {
            builder.push_sql(" and create_time >= ? ");
            builder.bind(end_time);
        }

        if let Some(log_type) = item.log_type {
            builder.push_sql(" and log_type =  ? ");
            builder.bind(log_type);
        }

        builder.push_sql(" order by create_time desc ");
        builder.build_page().await
    }
}
