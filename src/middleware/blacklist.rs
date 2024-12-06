use crate::utils::mini_redis::MiniRedis;
use salvo::{async_trait, Depot, FlowCtrl, Handler, Request, Response};

struct BlackList{
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
    async fn handle(&self, req: &mut Request, _depot: &mut Depot, res: &mut Response, _ctrl: &mut FlowCtrl) {
        let ip = req.remote_addr().into_std().unwrap().ip().to_string();
        self.black_list.get(ip.as_str());
        let x = self.cache.get(ip.as_str()).await;
        match x {
            None => {
                self.cache.set_second(ip.as_str(), 1,self.interval).await;
            }
            Some(mut number) => {
                number = number + 1;
                self.cache.set(ip.as_str(), number).await;
                if number >= self.visit_count {
                    self.black_list.set_second(ip.as_str(), "".to_string(), self.ban_time).await;
                    // 黑名单拦截
                }
            }
        }
    }
}