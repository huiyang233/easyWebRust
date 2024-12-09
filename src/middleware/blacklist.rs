use crate::model::result::ResultError;
use crate::utils::mini_redis::MiniRedis;
use salvo::prelude::Json;
use salvo::{async_trait, Depot, FlowCtrl, Handler, Request, Response};
use tracing::info;

pub struct BlackList{
    // 存的是ip
    pub cache: MiniRedis<u64>,
    pub black_list: MiniRedis<String>,
    // 禁封时间 秒
    pub ban_time: u64,
    // 秒，间隔
    pub interval: u64,
    // 每访问的次数
    pub visit_count: u64,
}
///
///
/// 黑名单功能
/// 比如 `BlackList::new(60, 60, 10)` = 每分钟访问数超过10个，封禁60秒
///
///
impl BlackList {
    pub fn new(ban_time: u64, interval: u64, visit_count: u64) -> Self {
        let black_list = MiniRedis::new("blackList");
        Self {
            cache:  MiniRedis::new("cache"),
            black_list,
            ban_time,
            interval,
            visit_count,
        }
    }
}

#[async_trait]
impl Handler for BlackList {
    async fn handle(&self, req: &mut Request, _depot: &mut Depot, res: &mut Response, ctrl: &mut FlowCtrl) {
        let ip = req.remote_addr().clone().into_std().unwrap().ip().to_string();
        let option = self.black_list.get(ip.as_str()).await;
        if option.is_some() {
            ctrl.skip_rest();
            // let string = format!("访问太频繁，请稍后再试;{:?}ms 后才能访问", self.black_list.get_expire(ip.as_str()).await);
            res.render(Json(ResultError::new(403, "访问太频繁，请稍后再试".to_string())));
            return;
        }

        let x = self.cache.get(ip.as_str()).await;
        match x {
            None => {
                self.cache.set_second(ip.as_str(), 1,self.interval).await;
            }
            Some(mut number) => {
                number = number + 1;
                info!("ip:{},number:{}", ip, number);
                self.cache.set(ip.as_str(), number).await;
                if number >= self.visit_count {
                    // 黑名单拦截
                    self.black_list.set_second(ip.as_str(), "".to_string(), self.ban_time).await;
                    self.cache.remove(ip.as_str()).await;
                }
            }
        }
    }
}