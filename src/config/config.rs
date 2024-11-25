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
    pub db_url: String,
    pub redis_url: String,
    pub sms_config: SmsConfig,
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