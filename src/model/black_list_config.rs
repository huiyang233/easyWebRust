use crate::utils::db::{Executor, QueryBuilder};
use crate::utils::serialize::deserialize_id;
use crate::utils::serialize::serialize_id;
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow};

#[derive(Deserialize, Serialize, Debug, Clone,Default,FromRow)]
pub struct BlackListConfig {
    pub id: i64,
    pub ban_time: i32,
    pub interval: i32,
    pub visit_count: i32
}


#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct BlackListConfigEditDto {
    #[serde(deserialize_with = "deserialize_id")]
    pub id: i64,
    pub ban_time: i32,
    pub interval: i32,
    pub visit_count: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
// 序列化的时候才转换驼峰
#[serde(rename_all(deserialize = "camelCase"))]
pub struct BlackListConfigAddDto {
    pub ban_time: i32,
    pub interval: i32,
    pub visit_count: i32,
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
    pub id: i64,
    pub ban_time: i32,
    pub interval: i32,
    pub visit_count: i32,
}



impl BlackListConfig {

    pub async fn update_by_id(item:&BlackListConfig) -> Result<u64, Error> {
        let mut builder = QueryBuilder::<Executor>::new_sql("update black_list_config set  ");

        builder.push_sql(" ban_time = ? ,");
        builder.bind_value(item.ban_time);

        builder.push_sql(" interval = ? ,");
        builder.bind_value(item.interval);

        builder.push_sql(" visit_count = ? ,");
        builder.bind_value(item.visit_count);


        builder.trim();
        builder.push_sql(" where id = ? ");
        builder.bind(item.id).execute().await
    }

    pub async fn insert(item:&BlackListConfig) -> Result<u64, Error> {
        let builder =  QueryBuilder::<Executor>::new_sql("insert into black_list_config (id,ban_time,interval,visit_count) values (?,?,?,?) ")
            .bind(item.id)
            .bind(item.ban_time)
            .bind(item.interval)
            .bind(item.visit_count)
            ;
        builder.execute().await
    }

    pub async fn insert_batch(item:&Vec<BlackListConfig>) -> Result<u64, Error> {
        let mut sql = "insert into black_list_config () values ".to_string();
        let mut builder = QueryBuilder::<Executor>::new();
        for x in item {
            sql.push_str("(?,?,?,?)");
            sql.push_str(",");
            builder = builder
                .bind(x.id)
                .bind(x.ban_time)
                .bind(x.interval)
                .bind(x.visit_count)
            ;
        }
        sql = sql.trim_end_matches(",").to_string();
        builder.push_sql(&sql);
        builder.execute().await
    }

    pub async fn select_by_id(id: &i64) -> Result<Option<BlackListConfig>, Error> {
        QueryBuilder::<BlackListConfig>::new_sql("select * from black_list_config where id = ? limit 1")
            .bind(id)
            .fetch_optional().await
    }



}