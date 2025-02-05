use crate::utils::db::{Executor, QueryBuilder};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow};

#[derive(Deserialize, Serialize, Debug, Clone,Default,FromRow)]
pub struct RequestLog {
    pub id: i64,
    pub uri: String,
    pub method: String,
    pub duration: i64,
    pub ip: String,
    pub user_id: Option<i64>,
    pub headers: String,
    pub query: String,
    pub create_time: DateTime<Utc>
}

impl RequestLog {

    pub async fn insert_batch(item:&Vec<RequestLog>) -> Result<u64, Error> {
        let mut builder = QueryBuilder::<Executor>::new_sql("insert into request_log (id,uri,method,duration,ip,user_id,headers,query,create_time) values ");
        for x in item {
            builder.push_sql("(?,?,?,?,?,?,?,?,?),");
            builder = builder
                .bind(x.id)
                .bind(&x.uri)
                .bind(&x.method)
                .bind(x.duration)
                .bind(&x.ip)
                .bind(x.user_id)
                .bind(&x.headers)
                .bind(&x.query)
                .bind(x.create_time)
            ;
        }
        builder.trim();
        builder.execute().await
    }





}