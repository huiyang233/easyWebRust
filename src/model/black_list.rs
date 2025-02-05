use crate::model::result::{PageDto, WebResultPage};
use crate::utils::db::{Executor, PageBuilder, QueryBuilder};
use crate::utils::serialize::deserialize_id;
use crate::utils::serialize::serialize_id;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone,Default,FromRow)]
pub struct BlackList {
    pub id: i64,
    pub ip: String,
    pub create_time: DateTime<Utc>,
    pub is_del: bool,
    pub reason: String,
    pub ban_time: Option<DateTime<Utc>>
}


#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct BlackListEditDto {
    #[serde(deserialize_with = "deserialize_id")]
    pub id: i64,
    pub ip: String,
    pub reason: String,
    pub ban_time: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
// 序列化的时候才转换驼峰
#[serde(rename_all(deserialize = "camelCase"))]
pub struct BlackListAddDto {
    pub ip: String,
    pub reason: String,
    pub ban_time: Option<DateTime<Utc>>,
}

impl From<BlackList> for BlackListVo {
    fn from(data: BlackList) -> Self {
        Self {
            id: data.id,
            ip: data.ip,
            create_time: data.create_time,
            is_del: data.is_del,
            reason: data.reason,
            ban_time: data.ban_time,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct BlackListVo {
    #[serde(serialize_with = "serialize_id")]
    pub id: i64,
    pub ip: String,
    pub create_time: DateTime<Utc>,
    pub is_del: bool,
    pub reason: String,
    pub ban_time: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Serialize, Debug, Clone,Validate)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct BlackListPageReq {
    #[validate(length(min = 1, message = "最小长度必须是1"))]
    pub ip: Option<String>,
}

impl BlackList {


    pub async fn update_by_id(item:&BlackList) -> Result<u64, Error> {
        let mut builder = QueryBuilder::<Executor>::new_sql("update black_list set  ");

        builder.push_sql(" ip = ? ,");
        builder.bind_value(&item.ip);

        builder.push_sql(" create_time = ? ,");
        builder.bind_value(item.create_time);

        builder.push_sql(" is_del = ? ,");
        builder.bind_value(item.is_del);

        builder.push_sql(" reason = ? ,");
        builder.bind_value(&item.reason);
        if let Some(ban_time) =item.ban_time {
            builder.push_sql(" ban_time = ? ,");
            builder.bind_value(ban_time);
        }


        builder.trim();
        builder.push_sql(" where id = ? ");
        builder.bind(item.id).execute().await
    }

    pub async fn insert(item:&BlackList) -> Result<u64, Error> {
        let builder =  QueryBuilder::<Executor>::new_sql("insert into black_list (id,ip,create_time,is_del,reason,ban_time) values (?,?,?,?,?,?) ")
            .bind(item.id)
            .bind(&item.ip)
            .bind(item.create_time)
            .bind(item.is_del)
            .bind(&item.reason)
            .bind(item.ban_time)
            ;
        builder.execute().await
    }

    pub async fn insert_batch(item:&Vec<BlackList>) -> Result<u64, Error> {
        let mut builder = QueryBuilder::<Executor>::new_sql("insert into black_list (id,ip,create_time,is_del,reason,ban_time) values ");
        for x in item {
            builder.push_sql("(?,?,?,?,?,?),");
            builder = builder
                .bind(x.id)
                .bind(&x.ip)
                .bind(x.create_time)
                .bind(x.is_del)
                .bind(&x.reason)
                .bind(x.ban_time)
            ;
        }
        builder.trim();
        builder.push_sql(" ON conflict(id) DO UPDATE set ip=excluded.ip, reason=excluded.reason,ban_time=excluded.ban_time");
        builder.execute().await
    }

    pub async fn select_by_id(id: &i64) -> Result<Option<BlackList>, Error> {
        QueryBuilder::<BlackList>::new_sql("select * from black_list where is_del=false and id = ? limit 1")
            .bind(id)
            .fetch_optional().await
    }

    pub async fn delete_by_ip(ip: &str) -> Result<u64, Error> {
        QueryBuilder::<Executor>::new_sql("update black_list set is_del = true where ip = ?")
            .bind(ip)
            .execute().await
    }

    pub async fn select_all() -> Result<Vec<BlackList>, Error> {
        QueryBuilder::<BlackList>::new_sql("select * from black_list where is_del=false ")
            .fetch_all().await
    }

    pub async fn select_by_name(name: String) -> Result<Option<BlackList>,Error> {
        QueryBuilder::<BlackList>::new_sql("select * from black_list where is_del=false and name = ? limit 1")
            .bind(name)
            .fetch_optional().await
    }


    pub async fn select_page(item:BlackListPageReq,page_dto: PageDto)->Result<WebResultPage<BlackList>,Error>{
        let mut builder = PageBuilder::<BlackList>::
        new_sql(page_dto,"select * from black_list where is_del=false ");
        if let Some(ip) = item.ip {
            builder.push_sql(" and ip like CONCAT('%', ?, '%') ");
            builder.bind(ip);
        }
        builder.push_sql(" order by create_time desc ");
        builder.build_page().await
    }
}