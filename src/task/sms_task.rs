use crate::{REDIS_POOL, SERVER_CONFIG};
use futures::StreamExt;
use lazy_static::lazy_static;
use rbatis::async_trait;
use spring_sms::config::AliyunSmsConfig;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::{mpsc, Mutex};
use tokio::time::sleep;
use tracing::info;

#[async_trait]
trait SmsInterface {
    async fn send_sms(&self,phone_number:String,code:String)->bool;
}

#[derive(Debug, Clone)]
pub struct WechatSms{
    pub app_key:String,
    pub app_secret:String,
}

#[async_trait]
impl SmsInterface for WechatSms {
    async fn send_sms(&self,phone_number:String,code:String)->bool{
        // 发送短信
        true
    }
}

#[derive(Debug, Clone)]
pub struct AliSms{
    config:AliyunSmsConfig,

}

impl AliSms {
    fn new (app_key:String,app_secret:String,sign_name:String,region:String)->AliSms{
        AliSms{
            config:AliyunSmsConfig {
                access_key_id: app_key.clone(),
                access_key_secret: app_secret.clone(),
                sign_name: sign_name.clone(),
                domain: None,
                region_id: Some(region.clone()),
                version: None,
            }
        }
    }
}

#[async_trait]
impl SmsInterface for AliSms{
    async fn send_sms(&self,phone_number:String,code:String)->bool{
        let sms_config = spring_sms::config::SmsConfig {
            aliyun: Some(self.config.clone()),
        };
        let sms_client = spring_sms::client::SmsClient::new(sms_config);
        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("code", &code);
        let res = sms_client.send_sms_by_aliyun(&phone_number, "SMS_10000000", Some(params)).await;
        match res {
            Ok(_) => {true}
            Err(_) => {false}
        }
    }
}

#[derive(Debug, Clone,Default)]
pub struct SmsMessage {
    pub phone_number:String,
    pub code:String
}
pub struct SmsServer{
    tx:Sender<SmsMessage>
}
impl SmsServer{
    pub async fn send_sms(&self,message: SmsMessage){
        self.tx.send(message).await.unwrap();
    }


    pub fn new()->SmsServer{
        let config = SERVER_CONFIG.sms_config.clone();
        let sms = AliSms::new(config.app_key, config.app_secret, config.sign_name, config.region_id);
        let (tx, mut rx) =  mpsc::channel::<SmsMessage>(10);
        info!("短信服务初始化成功");
        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                // 处理发送短信任务
                info!("发送短信: {:?}", message);
                let x = sms.send_sms(message.phone_number, message.code).await;
                info!("是否成功: {:?}", x);
            }
        });
        SmsServer{tx:tx.clone()}
    }
}
