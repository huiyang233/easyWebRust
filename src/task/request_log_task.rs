use crate::model::request_log::RequestLog;
use crate::{RB, SERVER_CONFIG};
use std::ops::Deref;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use tracing::info;


pub struct RequestLogTask{
    tx:Sender<RequestLog>
}

impl RequestLogTask{
    pub async fn send_sms(&self,message: RequestLog){
        self.tx.send(message).await.unwrap();
    }

    pub fn new()->RequestLogTask{
        let (tx, mut rx) =  mpsc::channel::<RequestLog>(100);
        let mut buffer: Vec<RequestLog> = Vec::with_capacity(100);
        info!("请求日志模块初始化成功");
        tokio::spawn(async move {
            loop {
                let size = rx.recv_many(&mut buffer, 100).await;
                info!("收到日志:{}",size);
                if size > 0 {
                    RequestLog::insert_batch(RB.deref(), &buffer, buffer.len() as u64).await.ok();
                    buffer.clear();
                }
            }
        });
        RequestLogTask{tx:tx.clone()}
    }
}