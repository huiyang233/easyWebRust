use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::Receiver;
use tokio::sync::Mutex;
use tokio::time::sleep;
use tracing::info;

#[derive(Debug, Clone,Default)]
pub struct SMSTask {
    pub phone_number:String,
    pub code:String
}

pub async fn init_sms_task(msg_rx: Arc<Mutex<Receiver<SMSTask>>>) {

    tokio::spawn(async move {
        let mut msg_rx = msg_rx.lock().await;
        while let Some(message) = msg_rx.recv().await {
            info!("接收到消息: {:?}", message);
            // 模拟处理消息的时间
        }
    });
}