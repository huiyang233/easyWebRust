use serde::{Deserialize, Serialize};
use std::fs;

///
/// ## 读取配置和添加配置
/// 如果需要添加配置将结构体添加至Config中。
/// CONFIG有全局变量，可以直接使用。
///
#[derive(Deserialize, Serialize, Debug, Clone,Default)]
pub struct Config {
    pub server: Server,
    pub db_config: DBConfig,
    pub redis_url: String,
    pub sms_config: SmsConfig,
    pub pay:PayConfig,
    pub wx_config:MpWxConfig,
    pub file:FileConfig,
    pub black_switch:Option<bool>,
    pub log_switch:Option<bool>
}

#[derive(Deserialize, Serialize, Debug, Clone,Default)]
pub struct FileConfig{
    pub file_path: String,
    pub url: String,

}

#[derive(Deserialize, Serialize, Debug, Clone,Default)]
pub struct MpWxConfig{
    pub app_id: String,
    pub secret: String,

}
#[derive(Deserialize, Serialize, Debug, Clone,Default)]
pub struct DBConfig{
    pub db_url: String,
    pub max_pool:Option<u32>,
    pub min_pool:Option<u32>,
}

#[derive(Deserialize, Serialize, Debug, Clone,Default)]
pub struct SmsConfig{
    pub app_key: String,
    pub app_secret: String,
    pub sign_name: String,
    pub region_id: String,
}
#[derive(Deserialize, Serialize, Debug, Clone,Default)]
pub struct Server{
    pub port: u16
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("config.yaml")?;
    let yaml: Config = serde_yaml::from_str(&contents)?;
    Ok(yaml)
}

#[derive(Deserialize, Serialize, Debug, Clone,Default)]
pub struct PayConfig{
    pub wechat: Option<WeChatPayConfig>,
}

#[derive(Deserialize, Serialize, Debug, Clone,Default)]
pub struct WeChatPayConfig{
    pub app_id: String,
    pub private_key: String,
    pub mch_id: String,
    pub serial_no: String,
    pub v3_key: String,
    pub notify_url: String,
}

// impl From<WeChatPayConfig> for WechatPay{
//     fn from(value: WeChatPayConfig) -> Self {
//         WechatPay::new(value.app_id,value.mch_id,value.private_key,value.serial_no,value.v3_key,value.notify_url)
//     }
// }
