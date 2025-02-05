use crate::model::request_log::RequestLog;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use tracing::info;


pub struct RequestLogTask{
    tx:Sender<RequestLog>
}

impl RequestLogTask{
    pub async fn send(&self,message: RequestLog){
        self.tx.send(message).await.unwrap();
    }

    pub fn new()->RequestLogTask{
        let (tx, mut rx) =  mpsc::channel::<RequestLog>(1000);
        let mut buffer: Vec<RequestLog> = Vec::new();
        info!("保存请求日志任务启动");
        tokio::spawn(async move {
            loop {
                let size = rx.recv_many(&mut buffer, 100).await;
                if size > 0 {
                    RequestLog::insert_batch(&buffer).await.ok();
                    buffer.clear();
                }
            }
        });
        RequestLogTask{tx:tx.clone()}
    }
}
