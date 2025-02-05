use crate::SERVER_CONFIG;
use base64::engine::general_purpose;
use base64::Engine;
use chrono::Local;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha1::Sha1;
use reqwest::Client;
use salvo::async_trait;
use std::collections::HashMap;
use std::error::Error;
use std::ops::{Add, Deref};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use tracing::log::info;


#[async_trait]
trait SmsInterface {
    async fn send_sms(&self,phone_number:String,code:String)->bool;
}


#[derive(Debug, Clone)]
pub struct AliSms{
    pub access_key_id: String,
    pub access_key_secret: String,
    pub sign_name: String,
    pub domain: Option<String>,
    pub region_id: Option<String>,
    pub version: Option<String>,
}


impl AliSms {
    fn new (app_key:String,app_secret:String,sign_name:String,region:String)->AliSms{
        AliSms{
            access_key_id: app_key.clone(),
            access_key_secret: app_secret.clone(),
            sign_name: sign_name.clone(),
            domain: None,
            region_id: Some(region.clone()),
            version: None,
        }
    }

    pub async fn ali_send_sms(
        &self,
        phone_numbers: &str,
        template_code: &str,
        template_param: Option<HashMap<&str, &str>>,
    ) -> Result<(), Box<dyn Error>> {
        let template_param = template_param.map(|p| serde_json::to_string(&p).unwrap());
        let mut data = HashMap::new();
        data.insert("PhoneNumbers", phone_numbers);
        data.insert("TemplateCode", template_code);
        data.insert("SignName", &self.sign_name);
        if let Some(param) = &template_param {
            data.insert("TemplateParam", param.deref());
        }
        let response = self.request("SendSms", &data).await?;
        if response.get("Code") != Some(&"OK".to_string()) {
            return Err(response.get("Message").unwrap_or(&"failed to send sms".to_string()).clone().into());
        }

        Ok(())
    }

    async fn request(&self, action: &str, data: &HashMap<&str, &str>) -> Result<HashMap<String, String>, Box<dyn Error>> {
        let domain = &self.domain();
        let version = &self.version();
        let access_key_id = &self.access_key_id();
        let mut api_params = HashMap::new();
        let signature_nonce = self.generate_nonce();
        let timestamp = self.generate_timestamp();
        api_params.insert("SignatureMethod", "HMAC-SHA1");
        api_params.insert("SignatureNonce", &*signature_nonce);
        api_params.insert("SignatureVersion", "1.0");
        api_params.insert("Timestamp", &*timestamp);
        api_params.insert("Format", "JSON");
        api_params.insert("Version", version);
        api_params.insert("AccessKeyId", access_key_id);
        api_params.insert("Action", action);
        for (key, value) in data {
            api_params.insert(key, value);
        }
        let query_string = self.encode_params(&api_params);
        let signature = self.generate_signature(&query_string);
        let url = format!("{}://{}/?Signature={}&{}", "https", domain, self.url_encode(&signature), query_string);
        let client = Client::new();
        let response = client.get(&url).send().await?.json::<HashMap<String, String>>().await?;
        Ok(response)
    }
    fn url_encode(&self, input: &str) -> String {
        urlencoding::encode(input).replace("+", "%20").replace("*", "%2A").replace("%7E", "~")
    }


    fn generate_signature(&self, query_string: &str) -> String {
        let string_to_sign = format!("GET&%2F&{}", self.url_encode(query_string));
        let mut hmac = Hmac::new(Sha1::new(), self.access_key_secret.clone().add("&").as_bytes());
        hmac.input(string_to_sign.as_bytes());
        let result = hmac.result();
        general_purpose::STANDARD.encode(&result.code())
    }
    fn encode_params(&self, params: &HashMap<&str, &str>) -> String {
        let mut sorted_params: Vec<(&&str, &&str)> = params.iter().collect();
        sorted_params.sort_by(|a, b| a.0.cmp(b.0));
        let query_string: String = sorted_params.iter()
            .map(|(k, v)| format!("{}={}", self.url_encode(k), self.url_encode(v)))
            .collect::<Vec<String>>()
            .join("&");
        query_string
    }
    fn generate_timestamp(&self) -> String {
        let now = SystemTime::now();
        let since_epoch = now.duration_since(UNIX_EPOCH).unwrap();
        let datetime = chrono::DateTime::<chrono::Utc>::from(SystemTime::UNIX_EPOCH + since_epoch);
        datetime.format("%Y-%m-%dT%H:%M:%SZ").to_string()
    }
    fn generate_nonce(&self) -> String {
        format!("{}{}", Local::now().timestamp() as u64, rand::random::<u8>())
    }
    fn domain(&self) -> String {
        self.domain.clone().unwrap_or("dysmsapi.aliyuncs.com".to_string())
    }
    fn version(&self) -> String {
        self.version.clone().unwrap_or("2017-05-25".to_string())
    }
    fn access_key_id(&self) -> String {
        self.access_key_id.clone()
    }
}

#[async_trait]
impl SmsInterface for AliSms{
    async fn send_sms(&self,phone_number:String,code:String)->bool{
        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("code", &code);
        let res = self.ali_send_sms(&phone_number, "SMS_10000000", Some(params)).await;
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
