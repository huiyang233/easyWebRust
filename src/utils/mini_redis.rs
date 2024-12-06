use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::Arc;
use std::time::Duration;

use lazy_static::lazy_static;
use serde::{de, Deserialize, Serialize};
use serde_json::error::Result;
use tokio::runtime::Runtime;
use tokio::sync::{Mutex, RwLock};
use tracing::{error, info};

///
/// 本地的Redis
///

lazy_static! {
    static ref MINI_REDIS_DATA:Arc<RwLock<HashMap<String,Data>>> =
    Arc::new(RwLock::new(HashMap::<String,Data>::new()));
}


// 扫描过期KEY的间隔时间(单位秒)
const SCAN_OUT_TIME_DATA_TIME:u64 = 10;
// 保存数据的间隔时间(单位秒)
const SAVE_DATA_TIME:u64 = 10;

#[derive(Debug)]
struct Data{
    data:String,
    out_time:u64,
}
pub struct MiniRedis<T>{
    phantom: PhantomData<T>,
    name: String,
}

pub struct MiniRedisInit{}
impl MiniRedisInit{
    pub fn init(){
        MiniRedisInit::scan_out_time_data();
    }

    fn scan_out_time_data(){
        info!("[MINI Redis] 开始扫描过期数据");
        tokio::spawn(async move {
            loop {
                {
                    let mut vec = vec![];
                    let mut map = MINI_REDIS_DATA.read().await;
                    for (key, data) in map.iter() {
                        if data.out_time!=0 {
                            if data.out_time < chrono::Local::now().timestamp_millis() as u64 {
                                vec.push(key.clone())
                            }
                        }

                    }
                    drop(map);
                    let mut guard = MINI_REDIS_DATA.write().await;
                    for key in vec {
                        info!("[MINI Redis] 删除过期数据{}",key);
                        guard.remove(&key);
                    }
                    drop(guard);
                }
                tokio::time::sleep(Duration::from_secs(SCAN_OUT_TIME_DATA_TIME)).await;
            }
        });
    }
}

// 在线程中主动清理MINI_REDIS_DATA中过期的key

impl<T:Serialize+ for<'de> Deserialize<'de>> MiniRedis<T>{
    pub fn new(name:&str)->MiniRedis<T>{
        info!("[MINI Redis] 初始化{}成功",name);
        MiniRedis{ phantom: Default::default(), name:name.to_string() }
    }

    pub async fn set_second(&self,key:&str,value:T,out_time:u64){
        self.set_millis(key,value,out_time*1000).await;
    }

    pub async fn set_minute(&self,key:&str,value:T,out_time:u64){
        self.set_second(key,value,out_time*60).await;
    }

    pub async fn set(&self,key:&str,value:T){

        let mut map = MINI_REDIS_DATA.write().await;
        match map.get_mut(&format!("{}:{}",self.name.to_string() ,key)) {
            None => {
                let data = Data{
                    data:serde_json::to_string(&value).unwrap(),
                    out_time:0,
                };
                MINI_REDIS_DATA.write().await.insert(format!("{}:{}",self.name.to_string() ,key),data);
            }
            Some(data) => {
                data.data = serde_json::to_string(&value).unwrap();

            }
        }


    }

    pub async fn set_millis(&self,key:&str,value:T,out_time:u64){
        let data = Data{
            data:serde_json::to_string(&value).unwrap(),
            out_time:(chrono::Local::now().timestamp_millis() as u64)+out_time,
        };
        MINI_REDIS_DATA.write().await.insert(format!("{}:{}",self.name.to_string() ,key),data);
    }

    pub async fn remove(&self, key: &str){
        MINI_REDIS_DATA.write().await.remove(&format!("{}:{}",self.name.to_string() ,key));
    }

    // 延长key超时时间
    pub async fn extend_out_time(&self, key: &str, out_time: u64) {
        let mut map = MINI_REDIS_DATA.write().await;
        match map.get_mut(&format!("{}:{}",self.name.to_string() ,key)) {
            None => {}
            Some(data) => {
                if data.out_time!=0 {
                    data.out_time = (chrono::Local::now().timestamp_millis() as u64) + out_time;
                }
            }
        }
    }

    // 获取剩余时间
    pub async fn get_expire(&self, key: &str) -> Option<u64> {
        let map = MINI_REDIS_DATA.read().await;
        match map.get(&format!("{}:{}",self.name.to_string() ,key)) {
            None => {
                None
            }
            Some(data) => {
                if data.out_time!=0 {
                    let time = data.out_time - chrono::Local::now().timestamp_millis() as u64;
                    Some(time)
                }else{
                    Some(0)
                }
            }
        }
    }
    pub async fn get(&self, key: &str) -> Option<T> {
        // 锁定一次并在整个函数中复用
        let map = MINI_REDIS_DATA.read().await;
        match map.get(&format!("{}:{}",self.name.to_string() ,key)) {
            None => {
                None
            }
            Some(data) => {
                // 检查并删除过期项，此时已经持有必要的锁
                if data.out_time!=0 {
                    if data.out_time < chrono::Local::now().timestamp_millis() as u64 {
                        // 删除锁
                        drop(map);
                        MINI_REDIS_DATA.write().await.remove(&format!("{}:{}",self.name.to_string() ,key));
                        return None;
                    }
                }

                // 反序列化数据，直接使用ok()转换Result以避免unwrap导致的panic
                let result = match serde_json::from_str(&data.data) {
                    Ok(result) => Some(result),
                    Err(e) => {
                        error!("error:{:?}",e);
                        None
                    }, // 或者可以记录错误、返回特定错误类型等
                };
                result

            }
        }
    }

}