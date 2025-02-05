use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use tokio::sync::{ RwLock};
use tracing::{info};

///
/// 本地的Redis
///


// 扫描过期KEY的间隔时间(单位秒)
const SCAN_OUT_TIME_DATA_TIME:u64 = 10;

#[derive(Debug)]
struct Data<T>{
    data:T,
    out_time:u64,
}
#[derive(Debug)]
pub struct MiniRedis<T>{
    data: Arc<RwLock<HashMap<String,Data<T>>>>,
    name: String,
}



// 在线程中主动清理MINI_REDIS_DATA中过期的key

impl<T:Serialize+ for<'de> Deserialize<'de> + Send + Sync + 'static> MiniRedis<T>{
    pub fn new(name:&str)->MiniRedis<T>{
        info!("[MINI Redis] 初始化{}成功", name);
        let redis = MiniRedis {
            data: Arc::new(RwLock::new(HashMap::<String, Data<T>>::new())),
            name: name.to_string(),
        };
        let redis_clone = Arc::clone(&redis.data);
        tokio::spawn(async move {
            Self::scan_out_time_data(redis_clone).await;
        });
        redis
    }
    async fn scan_out_time_data(data: Arc<RwLock<HashMap<String, Data<T>>>>) {
        // info!("[MINI Redis] 开始扫描过期数据");
        loop {
            {
                let mut vec = vec![];
                let map = data.read().await;
                for (key, data) in map.iter() {
                    if data.out_time != 0 {
                        if data.out_time < chrono::Local::now().timestamp_millis() as u64 {
                            vec.push(key.clone());
                        }
                    }
                }
                drop(map);
                let mut guard = data.write().await;
                for key in vec {
                    info!("[MINI Redis] 删除过期数据{}", key);
                    guard.remove(&key);
                }
                drop(guard);
            }
            tokio::time::sleep(Duration::from_secs(SCAN_OUT_TIME_DATA_TIME)).await;
        }
    }

    pub async fn set_second(&self,key:&str,value:T,out_time:u64){
        self.set_millis(key,value,out_time*1000).await;
    }

    pub async fn set_minute(&self,key:&str,value:T,out_time:u64){
        self.set_second(key,value,out_time*60).await;
    }

    pub async fn set(&self,key:&str,value:T){

        let mut map = self.data.write().await;
        match map.get_mut(&format!("{}:{}",self.name.to_string() ,key)) {
            None => {
                let data = Data{
                    data:value,
                    out_time:0,
                };
                map.insert(format!("{}:{}",self.name.to_string() ,key),data);
            }
            Some(data) => {
                data.data = value;

            }
        }


    }

    pub async fn set_millis(&self,key:&str,value:T,out_time:u64){
        let data = Data{
            data:value,
            out_time:(chrono::Local::now().timestamp_millis() as u64)+out_time,
        };
        self.data.write().await.insert(format!("{}:{}",self.name.to_string() ,key),data);
    }

    pub async fn remove(&self, key: &str){
        self.data.write().await.remove(&format!("{}:{}",self.name.to_string() ,key));
    }

    // 延长key超时时间
    pub async fn extend_out_time(&self, key: &str, out_time: u64) {
        let mut map = self.data.write().await;
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
        let map = self.data.read().await;
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

    pub async fn keys(&self)->Vec<String>{
        let map = self.data.read().await;
        let mut vec = vec![];
        for (key, _) in map.iter() {
            vec.push(key.clone());
        }
        vec
    }
    pub async fn get(&self, key: &str) -> Option<T> where
        T: Clone, {
        // 锁定一次并在整个函数中复用
        let map = self.data.read().await;
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
                        self.data.write().await.remove(&format!("{}:{}",self.name.to_string() ,key));
                        return None;
                    }
                }
                let t = data.data.clone();
                Some(t)
            }
        }
    }

}