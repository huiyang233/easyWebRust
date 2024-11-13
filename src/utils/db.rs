use deadpool_redis::PoolError;
use rbatis::executor::RBatisTxExecutorGuard;
use redis::{AsyncCommands, Connection, RedisError};
use serde::{de, Deserialize, Serialize};
use std::marker::PhantomData;
use std::ops::Deref;
use std::result;
use tracing::error;

use crate::model::result::ResultError;
use crate::{RB, REDIS_POOL};

pub struct  DB;

impl DB{
    pub async fn get_transaction() -> RBatisTxExecutorGuard {
        let tx = RB.deref().acquire_begin().await.unwrap();
        let mut tx = tx.defer_async(|mut tx| async move {
            if !tx.done {
                let r = tx.rollback().await;
                error!("回滚");
                if let Err(e) = r {
                    error!("rollback fail {}", e);
                }
            }
        });
        tx

    }
}

pub struct REDIS;
impl REDIS{

    ///获取连接
    pub async fn get_conn() -> Result<deadpool_redis::Connection, PoolError> {
        REDIS_POOL.deref().get().await
    }
    /// 获取Key
    /// 传入key
    pub async fn get<T:Serialize+ for<'de> Deserialize<'de>>(str: &str) -> Result<Option<T>, ResultError>{
        let mut conn = REDIS::get_conn().await?;
        let result:Option<String> = conn.get(str).await?;
        match result {
            None => {
                Ok(None)
            }
            Some(str) => {
                let result = match serde_json::from_str(&str) {
                    Ok(result) => Ok(Some(result)),
                    Err(e) => {
                        error!("error:{:?}",e);
                        Ok(None)
                    },
                };
                result
            }
        }


    }

    pub async fn set<T:Serialize+ for<'de> Deserialize<'de>>(key: &str, value: T) -> Result<(), ResultError> {
        let mut conn = REDIS::get_conn().await?;
        let value = serde_json::to_string(&value).unwrap();
        conn.set(key, value).await?;
        Ok(())
    }

    /// 设置Key 过期时间是秒
    pub async fn set_second<T: Serialize+ for<'de> Deserialize<'de>>(key: &str, value: T, out_time: u64) -> Result<(), ResultError> {
        let mut conn = REDIS::get_conn().await?;
        let value = serde_json::to_string(&value).unwrap();
        conn.set_ex(key, value, out_time).await?;
        Ok(())
    }

    /// 设置Key 过期时间是分钟
    pub async fn set_minute<T: Serialize+ for<'de> Deserialize<'de>>(key: &str, value: T, out_time: u64) -> Result<(), ResultError> {
        let mut conn = REDIS::get_conn().await?;
        let value = serde_json::to_string(&value).unwrap();
        conn.set_ex(key, value, out_time * 60).await?;
        Ok(())
    }

    /// 删除Key
    pub async fn remove(key: &str) -> Result<(), ResultError> {
        let mut conn = REDIS::get_conn().await?;
        conn.del(key).await?;
        Ok(())
    }


    /// 延长Key的时间
    pub async fn extend_out_time(key: &str, out_time: i64) -> Result<(), ResultError> {
        let mut conn = REDIS::get_conn().await?;
        conn.expire(key, out_time).await?;
        Ok(())
    }



}

pub struct Redis<T>{
    phantom: PhantomData<T>,
    name:String,
}

impl <T:Serialize+ for<'de> Deserialize<'de>> Redis<T>{
    pub fn new(name:&str) -> Redis<T> {
        Redis {
            phantom: Default::default(),
            name:name.to_string(),
        }
    }

    ///获取连接
    pub async fn get_conn(&self) -> Result<deadpool_redis::Connection, PoolError> {
        REDIS_POOL.deref().get().await
    }

    /// 获取Key
    /// 传入key
    pub async fn get(&self,key: &str) -> Result<Option<T>, ResultError>{
        let mut conn = self.get_conn().await?;
        let result:Option<String> = conn.get(format!("{}:{}",self.name ,key)).await?;
        match result {
            None => {
                Ok(None)
            }
            Some(str) => {
                let result = match serde_json::from_str(&str) {
                    Ok(result) => Ok(Some(result)),
                    Err(e) => {
                        error!("error:{:?}",e);
                        Ok(None)
                    },
                };
                result
            }
        }
    }

    pub async fn set(&self,key: &str, value: T) -> Result<(), ResultError> {
        let mut conn =  REDIS::get_conn().await?;
        let value = serde_json::to_string(&value).unwrap();
        conn.set(format!("{}:{}",self.name ,key), value).await?;
        Ok(())
    }

    /// 设置Key 过期时间是秒
    pub async fn set_second(&self,key: &str, value: T, out_time: u64) -> Result<(), ResultError> {
        let mut conn = REDIS::get_conn().await?;
        let value = serde_json::to_string(&value).unwrap();
        conn.set_ex(format!("{}:{}",self.name ,key), value, out_time).await?;
        Ok(())
    }

    /// 设置Key 过期时间是分钟
    pub async fn set_minute(&self,key: &str, value: T, out_time: u64) -> Result<(), ResultError> {
        let mut conn = REDIS::get_conn().await?;
        let value = serde_json::to_string(&value).unwrap();
        conn.set_ex(format!("{}:{}",self.name ,key), value, out_time * 60).await?;
        Ok(())
    }

    /// 删除Key
    pub async fn remove(&self,key: &str) -> Result<(), ResultError> {
        let mut conn = REDIS::get_conn().await?;
        conn.del(format!("{}:{}",self.name ,key)).await?;
        Ok(())
    }

}