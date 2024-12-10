use crate::model::black_list::{BlackList, BlackListConfig};
use crate::model::result::ResultError;
use crate::task::black_list_task::BlackListInsertTask;
use crate::utils::mini_redis::MiniRedis;
use crate::{ID_WORKER, RB};
use lazy_static::lazy_static;
use rbatis::rbatis_codegen::ops::AsProxy;
use rbatis::rbdc::DateTime;
use salvo::prelude::Json;
use salvo::{async_trait, Depot, FlowCtrl, Handler, Request, Response};
use std::ops::{Add, Deref};
use std::sync::Arc;
use std::time::Duration;
use tracing::{error, info};

lazy_static! {
    static ref BLACK_LIST_TASK:BlackListInsertTask= BlackListInsertTask::new();
    static ref BLACK_LIST:MiniRedis<String> = MiniRedis::new("blackList");
}

pub struct BlackListMid{
    // 存的是ip
    pub cache: MiniRedis<u64>,
    // 禁封时间 0=永久 >0=xx秒
    pub ban_time: u64,
    // 秒，间隔
    pub interval: u64,
    // 每访问的次数
    pub visit_count: u64,
    pub sync_interval:u64,
}
///
///
/// 黑名单功能
/// 比如 `BlackList::new(60, 60, 10)` = 每分钟访问数超过10个，封禁60秒
///
///
impl BlackListMid {
    pub fn new(ban_time: u64, interval: u64, visit_count: u64) -> Self {
        let mut list = Self {
            cache: MiniRedis::new("cache"),
            ban_time,
            interval,
            visit_count,
            sync_interval: 60,
        };
        BlackListMid::sync_black_list(list.sync_interval);
        list
    }
    fn sync_black_list(sync_interval:u64){
       tokio::spawn(async move {
           loop {
               let result = BlackList::select_all(RB.deref()).await;
               match result {
                   Ok(x) => {
                       for x in x {
                           if x.ban_time.is_some() {
                               let time = x.ban_time.unwrap();
                               let i = time.unix_timestamp();
                               let i = i - DateTime::now().unix_timestamp();
                               if i > 0 {
                                   BLACK_LIST.set_second(x.ip.as_str(), x.ip.clone(), i.u64()).await;
                                   continue;
                               }else{
                                   BLACK_LIST_TASK.send_del(x.ip).await;
                                   continue;
                               }

                           }else {
                               BLACK_LIST.set(x.ip.as_str(), String::new()).await;
                           }

                       }
                   }
                   Err(e) => {
                       error!("同步黑名单异常:{}", e);
                   }
               }
               info!("同步完成:{:?}",BLACK_LIST.keys().await);
               tokio::time::sleep(Duration::from_secs(sync_interval)).await;
           }
       });
   }

    pub async fn add_black_list(ip:String,ban_time:u64,reason:String)->u64{
        let id = ID_WORKER.new_id();
        if ban_time==0 {
            BLACK_LIST.set(ip.as_str(), "".to_string()).await;
            BLACK_LIST_TASK.send(BlackList{
                id: id,
                ip:ip.clone(),
                create_time: DateTime::now(),
                is_del: false,
                reason,
                ban_time: None,
            }).await;

        }else{
            BLACK_LIST.set_second(ip.as_str(), "".to_string(), ban_time).await;
            BLACK_LIST_TASK.send(BlackList{
                id: id,
                ip:ip.clone(),
                create_time: DateTime::now(),
                is_del: false,
                reason,
                ban_time: Some(DateTime::now().add(Duration::from_secs(ban_time))),
            }).await;
        }
        id

    }

    pub async fn add_black_list_date_time(ip:String,ban_time:Option<DateTime>,reason:String)->u64{
        let id = ID_WORKER.new_id();
        match ban_time {
            None => {
                BLACK_LIST.set(ip.as_str(), "".to_string()).await;
                BLACK_LIST_TASK.send(BlackList{
                    id: id,
                    ip:ip.clone(),
                    create_time: DateTime::now(),
                    is_del: false,
                    reason,
                    ban_time: None,
                }).await;
            }
            Some(ban_time) => {
                let ban_time_second = ban_time.unix_timestamp() - DateTime::now().unix_timestamp();
                if ban_time_second<1 {
                    return 0;
                }
                BLACK_LIST.set_second(ip.as_str(), "".to_string(), ban_time_second as u64).await;
                BLACK_LIST_TASK.send(BlackList{
                    id: id,
                    ip:ip.clone(),
                    create_time: DateTime::now(),
                    is_del: false,
                    reason,
                    ban_time: Some(ban_time),
                }).await;
            }
        }

        id

    }

    pub async fn update_black_list_date_time(id:u64,ip:String,ban_time:Option<DateTime>,reason:String){
        match ban_time {
            None => {
                BLACK_LIST.set(ip.as_str(), "".to_string()).await;
                BLACK_LIST_TASK.send(BlackList{
                    id,
                    ip:ip.clone(),
                    create_time: DateTime::now(),
                    is_del: false,
                    reason,
                    ban_time: None,
                }).await;
            }
            Some(ban_time) => {
                let ban_time_second = ban_time.unix_timestamp() - DateTime::now().unix_timestamp();
                if ban_time_second<1 {
                    return;
                }
                BLACK_LIST.set_second(ip.as_str(), "".to_string(), ban_time_second as u64).await;
                BLACK_LIST_TASK.send(BlackList{
                    id,
                    ip:ip.clone(),
                    create_time: DateTime::now(),
                    is_del: false,
                    reason,
                    ban_time: Some(ban_time),
                }).await;
            }
        }


    }


    pub async fn del_black_list(ip:String){
        BLACK_LIST.remove(ip.as_str()).await;
        BLACK_LIST_TASK.send_del(ip).await;
    }

}

#[async_trait]
impl Handler for BlackListMid {
    async fn handle(&self, req: &mut Request, _depot: &mut Depot, res: &mut Response, ctrl: &mut FlowCtrl) {
        let ip = req.remote_addr().clone().into_std().unwrap().ip().to_string();
        let option = BLACK_LIST.get(ip.as_str()).await;
        if option.is_some() {
            ctrl.skip_rest();
            // let string = format!("访问太频繁，请稍后再试;{:?}ms 后才能访问", self.black_list.get_expire(ip.as_str()).await);
            res.render(Json(ResultError::new(403, "访问太频繁，请稍后再试".to_string())));
            return;
        }

        let x = self.cache.get(ip.as_str()).await;
        match x {
            None => {
                self.cache.set_second(ip.as_str(), 1, self.interval).await;
            }
            Some(mut number) => {
                number = number + 1;
                info!("ip:{},number:{}", ip, number);
                self.cache.set(ip.as_str(), number).await;
                if number >= self.visit_count {
                    // 黑名单拦截
                    self.cache.remove(ip.as_str()).await;
                    BlackListMid::add_black_list(ip,self.ban_time,"自动".to_string()).await;
                }
            }
        }
    }
}