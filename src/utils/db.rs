use crate::model::result::{PageDto, ResultError, WebResultPage};
use crate::{get_sqlx_db, REDIS_POOL};
use deadpool_redis::PoolError;
use redis::{AsyncCommands, RedisResult};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Number, Value};
use sqlx::postgres::{PgArguments, PgRow};
use sqlx::{
    Arguments, Column, Encode, Error, FromRow, Postgres, Row,
    Transaction, Type, TypeInfo,
};
use std::marker::PhantomData;
use std::ops::Deref;
use tokio_stream::StreamExt;
use tracing::{error, info};

pub struct Executor {}
pub struct PageBuilder<E> {
    sql: String,
    count_value: PgArguments,
    value: PgArguments,
    page_dto: PageDto,
    _phantom: PhantomData<E>,
}
impl<E> PageBuilder<E> {
    pub fn new(page_dto: PageDto) -> Self {
        Self {
            page_dto,
            sql: String::new(),
            count_value: PgArguments::default(),
            value: PgArguments::default(),
            _phantom: Default::default(),
        }
    }

    pub fn new_sql(page_dto: PageDto, sql: &str) -> Self {
        Self {
            page_dto,
            sql: sql.to_string(),
            count_value: PgArguments::default(),
            value: PgArguments::default(),
            _phantom: Default::default(),
        }
    }
    pub fn push_sql(&mut self, sql: &str) {
        self.sql.push_str(sql);
    }

    pub fn and_like<'q, T>(&mut self, name: & str,value: T)
    where
        T: Encode<'q, Postgres> + Type<Postgres> + 'q + Clone,{
        self.sql.push_str(" and ");
        self.sql.push_str(name);
        self.sql.push_str(" like CONCAT('%', ?, '%')");
        self.bind(value)
    }

    pub fn bind_value<'q, T>(mut self, data: T) -> Self
    where
        T: Encode<'q, Postgres> + Type<Postgres> + 'q + Clone,
    {
        match self.value.add(data.clone()) {
            Ok(_) => {}
            Err(e) => {
                error!("errpr:{}", e);
            }
        };
        match self.count_value.add(data) {
            Ok(_) => {}
            Err(e) => {
                error!("errpr:{}", e);
            }
        }
        self
    }
    pub fn bind<'q, T>(&mut self, data: T)
    where
        T: Encode<'q, Postgres> + Type<Postgres> + 'q + Clone,
    {
        match self.value.add(data.clone()) {
            Ok(_) => {}
            Err(e) => {
                error!("errpr:{}", e);
            }
        };
        match self.count_value.add(data) {
            Ok(_) => {}
            Err(e) => {
                error!("errpr:{}", e);
            }
        }
    }

    pub async fn build_page(mut self) -> Result<WebResultPage<E>, Error>
    where
        E: for<'r> FromRow<'r, PgRow> + Send + Sync + Unpin,
    {
        let count_query = String::from(format!("select count(*) from ({})", self.sql));
        let count_query = DB::sql_marks(count_query);
        info!("count Sql:{}", count_query);
        let count_query_builder =
            sqlx::query_scalar_with::<_, i64, _>(&count_query, self.count_value);

        let total = count_query_builder.fetch_one(&get_sqlx_db()).await?;
        if total < 1 {
            return Ok(WebResultPage {
                page_data: vec![],
                page: 0,
                total,
                page_size: 0,
            });
        }
        self.sql.push_str(r" LIMIT ? OFFSET ?");
        let sql = DB::sql_marks(self.sql);
        info!("select Sql:{}", sql);

        let query_builder = sqlx::query_as_with::<_, E, _>(&sql, self.value);
        let offset = self.page_dto.get_limit();
        let query_builder = query_builder.bind(self.page_dto.page_size).bind(offset);

        let x = query_builder.fetch_all(&get_sqlx_db()).await?;
        Ok(WebResultPage {
            page_data: x,
            page: self.page_dto.page,
            total,
            page_size: self.page_dto.page_size,
        })
    }
}
pub struct QueryBuilder<E> {
    sql: String,
    value: PgArguments,
    _phantom: PhantomData<E>,
}
impl QueryBuilder<Executor> {
    pub async fn execute(self) -> Result<u64, Error> {
        let sql = DB::sql_marks(self.sql);
        info!("select Sql:{}", sql);
        let query_builder = sqlx::query_with::<_, _>(&sql, self.value);
        let i = query_builder.execute(&get_sqlx_db()).await?.rows_affected();
        Ok(i)
    }

    pub async fn transaction_execute(
        self,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<u64, Error> {
        let sql = DB::sql_marks(self.sql);
        info!("select Sql:{}", sql);
        let query_builder = sqlx::query_with::<_, _>(&sql, self.value);
        let i = query_builder
            .execute(&mut **transaction)
            .await?
            .rows_affected();
        Ok(i)
    }

    pub async fn transaction(
        self,
        db: impl sqlx::Executor<'_, Database = Postgres>,
    ) -> Result<u64, Error> {
        let sql = DB::sql_marks(self.sql);
        info!("select Sql:{}", sql);
        let query_builder = sqlx::query_with::<_, _>(&sql, self.value);
        let i = query_builder.execute(db).await?.rows_affected();
        Ok(i)
    }
}
impl QueryBuilder<Vec<String>> {
    pub async fn fetch_all_str(self) -> Result<Vec<String>, Error> {
        let sql = DB::sql_marks(self.sql);
        info!("select Sql:{}", sql);
        let query_builder = sqlx::query_with(&sql, self.value);

        let mut rows = query_builder.fetch(&get_sqlx_db());
        let mut vec = vec![];
        while let Some(row) = rows.try_next().await? {
            let string: String = row.try_get(0)?;
            vec.push(string)
        }
        Ok(vec)
    }
}

impl QueryBuilder<Value> {
    pub async fn fetch_value(self) -> Result<Value, Error> {
        let sql = DB::sql_marks(self.sql);
        info!("select Sql:{}", sql);
        info!("select Sql:{}", sql);
        let query_builder = sqlx::query_with(&sql, self.value);

        let mut rows = query_builder.fetch(&get_sqlx_db());

        let mut vec = vec![];
        while let Some(row) = rows.try_next().await? {
            let mut hash_map = Map::<String, Value>::new();
            for x in row.columns() {
                let name = x.name();
                match x.type_info().name() {
                    "TEXT" => {
                        let value = row.try_get::<String, _>(name)?;
                        hash_map.insert(name.to_string(), Value::String(value));
                    }
                    "INT8" => {
                        let value = row.try_get::<i64, _>(name)?;
                        hash_map.insert(name.to_string(), Value::Number(Number::from(value)));
                    }
                    _ => {}
                }
            }
            vec.push(Value::Object(hash_map));
        }
        Ok(Value::Array(vec))
    }
}

impl<E> QueryBuilder<E> {
    pub fn new() -> Self {
        Self {
            sql: String::new(),
            value: PgArguments::default(),
            _phantom: Default::default(),
        }
    }
    pub fn new_sql(sql: &'static str) -> Self {
        Self {
            sql: sql.to_string(),
            value: PgArguments::default(),
            _phantom: Default::default(),
        }
    }

    pub fn push_sql(&mut self, sql: &str) {
        self.sql.push_str(sql);
    }

    pub fn trim(&mut self) {
        let sql = self.sql.trim_end_matches(",").to_string();
        self.sql = sql;
    }

    pub fn bind_value<'q, T>(&mut self, data: T)
    where
        T: Encode<'q, Postgres> + Type<Postgres> + 'q,
    {
        match self.value.add(data) {
            Ok(_) => {}
            Err(e) => {
                error!("errpr:{}", e);
            }
        }
    }

    pub fn bind<'q, T>(mut self, data: T) -> Self
    where
        T: Encode<'q, Postgres> + Type<Postgres> + 'q,
    {
        match self.value.add(data) {
            Ok(_) => {}
            Err(e) => {
                error!("errpr:{}", e);
            }
        }
        self
    }

    pub async fn fetch_all(self) -> Result<Vec<E>, Error>
    where
        E: for<'r> FromRow<'r, PgRow> + Send + Sync + Unpin,
    {
        let sql = DB::sql_marks(self.sql);
        info!("select Sql:{}", sql);
        let query_builder = sqlx::query_as_with::<_, E, _>(&sql, self.value);
        let vec = query_builder.fetch_all(&get_sqlx_db()).await?;
        Ok(vec)
    }

    pub async fn fetch_all_no_marks(self) -> Result<Vec<E>, Error>
    where
        E: for<'r> FromRow<'r, PgRow> + Send + Sync + Unpin,
    {

        info!("select Sql:{}", self.sql);
        let query_builder = sqlx::query_as_with::<_, E, _>(&self.sql, self.value);
        let vec = query_builder.fetch_all(&get_sqlx_db()).await?;
        Ok(vec)
    }

    pub async fn scalar_fetch_no_marks(self) -> Result<E, Error>
    where
        (E,): for<'r> FromRow<'r, PgRow>,
        E: Send + Unpin,
    {

        info!("select Sql:{}", self.sql);
        let query_builder = sqlx::query_scalar_with::<_, E, _>(&self.sql, self.value);
        let e = query_builder.fetch_one(&get_sqlx_db()).await?;
        Ok(e)
    }

    pub async fn fetch_optional_no_marks(self) -> Result<Option<E>, Error>
    where
        E: for<'r> FromRow<'r, PgRow> + Send + Sync + Unpin,
    {

        info!("select Sql:{}", self.sql);
        let query_builder = sqlx::query_as_with::<_, E, _>(&self.sql, self.value);
        let e = query_builder.fetch_optional(&get_sqlx_db()).await?;
        Ok(e)
    }
    pub async fn fetch_one_no_marks(self) -> Result<E, Error>
    where
        E: for<'r> FromRow<'r, PgRow> + Send + Sync + Unpin,
    {

        info!("select Sql:{}", self.sql);
        let query_builder = sqlx::query_as_with::<_, E, _>(&self.sql, self.value);
        let e = query_builder.fetch_one(&get_sqlx_db()).await?;
        Ok(e)
    }

    pub async fn scalar_fetch_one(self) -> Result<E, Error>
    where
        (E,): for<'r> FromRow<'r, PgRow>,
        E: Send + Unpin,
    {
        let sql = DB::sql_marks(self.sql);
        info!("select Sql:{}", sql);
        let query_builder = sqlx::query_scalar_with::<_, E, _>(&sql, self.value);
        let e = query_builder.fetch_one(&get_sqlx_db()).await?;
        Ok(e)
    }

    pub async fn fetch_one(self) -> Result<E, Error>
    where
        E: for<'r> FromRow<'r, PgRow> + Send + Sync + Unpin,
    {
        let sql = DB::sql_marks(self.sql);
        info!("select Sql:{}", sql);
        let query_builder = sqlx::query_as_with::<_, E, _>(&sql, self.value);
        let e = query_builder.fetch_one(&get_sqlx_db()).await?;
        Ok(e)
    }

    pub async fn fetch_optional(self) -> Result<Option<E>, Error>
    where
        E: for<'r> FromRow<'r, PgRow> + Send + Sync + Unpin,
    {
        let sql = DB::sql_marks(self.sql);
        info!("select Sql:{}", sql);
        let query_builder = sqlx::query_as_with::<_, E, _>(&sql, self.value);
        let e = query_builder.fetch_optional(&get_sqlx_db()).await?;
        Ok(e)
    }
}

pub struct DB;

impl DB {
    pub fn sql_marks(sql: String) -> String {
        let mut result = String::new();
        let mut count = 1; // 用于计数占位符

        for c in sql.chars() {
            if c == '?' {
                result.push_str(&format!(" ${} ", count));
                count += 1;
            } else {
                result.push(c);
            }
        }
        result
    }
}


pub struct Redis<T> {
    phantom: PhantomData<T>,
    name: String,
}

impl<T: Serialize + for<'de> Deserialize<'de>> Redis<T> {
    pub fn new(name: &str) -> Redis<T> {
        Redis {
            phantom: Default::default(),
            name: name.to_string(),
        }
    }

    ///获取连接
    pub async fn get_conn(&self) -> Result<deadpool_redis::Connection, PoolError> {
        REDIS_POOL.deref().get().await
    }

    /// 获取Key
    /// 传入key
    pub async fn get(&self, key: &str) -> Option<T> {
        let mut conn = match self.get_conn().await {
            Ok(conn) => conn,
            Err(e) => {
                error!("redis error:{}", e);
                return None;
            }
        };
        let result: RedisResult<Option<String>> = conn.get(format!("{}:{}", self.name, key)).await;
        let result = match result {
            Ok(result) => result,
            Err(e) => {
                error!("redis error:{}", e);
                None
            }
        };
        match result {
            None => None,
            Some(str) => {
                let result = match serde_json::from_str(&str) {
                    Ok(result) => Some(result),
                    Err(e) => {
                        error!("error:{:?}", e);
                        None
                    }
                };
                result
            }
        }
    }

    pub async fn set(&self, key: &str, value: &T) -> Result<(), ResultError> {
        let mut conn = self.get_conn().await?;
        let json_value = serde_json::to_string(value)
            .map_err(|e| ResultError::new(40000,e.to_string()))?;

        conn.set(format!("{}:{}", self.name, key), json_value).await?;
        Ok(())
    }

    /// 设置Key 过期时间是秒
    pub async fn set_second(&self, key: &str, value: &T, out_time: u64) -> Result<(), ResultError> {
        let mut conn = self.get_conn().await?;
        let value = serde_json::to_string(value).unwrap();
        conn.set_ex(format!("{}:{}", self.name, key), value, out_time)
            .await?;
        Ok(())
    }

    /// 设置Key 过期时间是分钟
    pub async fn set_minute(&self, key: &str, value: &T, out_time: u64) -> Result<(), ResultError> {
        let mut conn = self.get_conn().await?;
        let value = serde_json::to_string(&value).unwrap();
        conn.set_ex(format!("{}:{}", self.name, key), value, out_time * 60)
            .await?;
        Ok(())
    }

    /// 设置过期时间
    pub async fn extend_out_time(&self, key: &str, out_time: i64) -> Result<(), ResultError> {
        let mut conn = self.get_conn().await?;
        conn.expire(format!("{}:{}", self.name, key), out_time)
            .await?;
        Ok(())
    }

    /// 设置过期时间
    pub async fn extend_out_time_minute(
        &self,
        key: &str,
        out_time: i64,
    ) -> Result<(), ResultError> {
        let mut conn = self.get_conn().await?;
        conn.expire(format!("{}:{}", self.name, key), out_time * 60)
            .await?;
        Ok(())
    }
    /// 获取过期时间
    pub async fn get_expire(&self, key: &str) -> Result<Option<i64>, ResultError> {
        let mut conn = self.get_conn().await?;
        let result = conn.ttl(format!("{}:{}", self.name, key)).await?;
        Ok(result)
    }

    /// 删除Key
    pub async fn remove(&self, key: &str) -> Result<(), ResultError> {
        let mut conn = self.get_conn().await?;
        conn.del(format!("{}:{}", self.name, key)).await?;
        Ok(())
    }
}

#[macro_export]
macro_rules! impl_page {
    ($table:ty{$fn_name:ident($($param_key:ident:$param_type:ty$(,)?)*) => $where_sql:expr}) => {
        impl $table {
            pub async fn $fn_name(
                page_dto: crate::model::result::PageDto,
                $($param_key:$param_type,)*
            ) -> std::result::Result<crate::model::result::WebResultPage::<$table>, sqlx::Error> {
                let mut builder = crate::utils::db::PageBuilder::<$table>::new_sql(page_dto,"SELECT * FROM ");

                builder.push_sql(<$table>::get_table_name());

                builder.push_sql(" where 1=1 ");

                if let Some(logic_del_field) = <$table>::get_logic_del_field(){
                    builder.push_sql(&format!(" and {} = false ",logic_del_field));
                }

                $where_sql(&mut builder);

                builder.build_page().await
            }
        }
    };
}