use crate::model::black_list::BlackList;
use crate::model::request_log::RequestLog;
use crate::RB;
use std::ops::Deref;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use tracing::info;


pub struct BlackListInsertTask{
    tx:Sender<BlackList>,
    del_tx:Sender<String>
}

impl BlackListInsertTask {
    pub async fn send(&self,message: BlackList){
        self.tx.send(message).await.unwrap();
    }

    pub async fn send_del(&self,message: String){
        self.del_tx.send(message).await.unwrap();
    }

    pub fn new()-> BlackListInsertTask {
        let (tx, mut rx) =  mpsc::channel::<BlackList>(10);
        let (del_tx, mut del_rx) =  mpsc::channel::<String>(10);
        let mut buffer: Vec<BlackList> = Vec::new();
        info!("保存黑名单任务启动");
        tokio::spawn(async move {
            loop {
                let size = rx.recv_many(&mut buffer, 100).await;
                if size > 0 {
                    BlackList::insert_batch(RB.deref(), &buffer, buffer.len() as u64).await.ok();
                    buffer.clear();
                }
            }
        });

        tokio::spawn(async move {
            while let Some(message) = del_rx.recv().await {
                BlackList::delete_by_ip(RB.deref(), message.as_str()).await.ok();
            }
        });

        BlackListInsertTask {tx:tx.clone(), del_tx: del_tx.clone() }
    }
}
