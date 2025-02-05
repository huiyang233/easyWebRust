use crate::api::init_router;
use crate::config::config::load_config;
use crate::middleware::blacklist::BlackListMid;
use crate::middleware::log::log;
use crate::task::scheduler::init_scheduler;
use crate::task::sms_task::SmsServer;
use deadpool_redis::{Config, Runtime};
use idgen::IDGen;
use lazy_static::lazy_static;
use salvo::cors::{Any, Cors};
use salvo::prelude::*;
use sqlx::postgres::PgPoolOptions;
use sqlx::Postgres;
use std::sync::LazyLock;
use tokio::sync::OnceCell;
// use wechat_pay_rust_sdk::pay::WechatPay;

mod model;
mod auth;
mod utils;
mod api;
mod service;
mod middleware;
mod config;
mod task;
mod validate;

lazy_static! {
    // static ref SERVER_CONFIG:config::config::Config = load_config().unwrap();
    // 雪花id生成器
    static ref ID_WORKER:IDGen = IDGen::new(127);
    // Redis
    static ref REDIS_POOL:deadpool_redis::Pool = {
        // 初始化Redis连接池
        let cfg = Config::from_url(&SERVER_CONFIG.redis_url);
        cfg.create_pool(Some(Runtime::Tokio1)).unwrap()
    };
    // 短信服务
    static ref SMS_SERVER:SmsServer= SmsServer::new();
    // static ref WECHAT_PAY:WechatPay = WechatPay::from(SERVER_CONFIG.pay.wechat.clone().unwrap());
}



static DB_POOL: OnceCell<sqlx::Pool<Postgres>> = OnceCell::const_new();
static SERVER_CONFIG:LazyLock<config::config::Config> = LazyLock::new(|| {
    load_config().unwrap()
});
async fn init_db_pool() -> sqlx::Pool<Postgres> {
    PgPoolOptions::new()
        // 默认最大
        .max_connections(SERVER_CONFIG.db_config.max_pool.unwrap_or(10))
        .min_connections(SERVER_CONFIG.db_config.min_pool.unwrap_or(0))
        .connect(&SERVER_CONFIG.db_config.db_url)
        .await
        .unwrap()
}

pub fn get_sqlx_db() -> sqlx::Pool<Postgres> {
    DB_POOL.get().unwrap().clone()
}


#[tokio::main]
async fn main() {
    // _guard必须贯穿整个主流程
    let _guard = utils::log::init_log();
    // 定时任务初始化
    init_scheduler().await;
    // 初始化数据库连接池
    DB_POOL.get_or_init(init_db_pool).await;
    // 初始化路由
    let router = init_router();

    let cors_handler = Cors::new()
        .allow_origin(Any)
        .allow_credentials(false)
        .allow_methods(Any)
        .allow_headers(Any)
        .into_handler();
    let black = BlackListMid::new(60*60);

    // 初始化服务
    let service = Service::new(router)
        .hoop(log)
        .hoop(black)
        .hoop(cors_handler);

    //// 证书
    // let cert = include_bytes!("../certs/cert.pem").to_vec();
    // let key = include_bytes!("../certs/key.pem").to_vec();
    // let config = RustlsConfig::new(Keycert::new().cert(cert.as_slice()).key(key.as_slice()));

    // 微信支付
    // let params = MicroParams::new(
    //     "测试支付1分",
    //     "1243243",
    //     1.into(),
    //     "open_id".into()
    // );
    // let result = WECHAT_PAY.micro_pay(params).await;

    // 设置端口
    let listener = TcpListener::new(("0.0.0.0", SERVER_CONFIG.server.port)).bind().await;

    //// https
    // let listener = TcpListener::new("0.0.0.0:5443")
    //     .rustls(config)
    //     .join(listener)
    //     .bind()
    //     .await;

    //// http3
    // let listener = QuinnListener::new(config, ("127.0.0.1", 5800))
    //     .join(listener)
    //     .bind()
    //     .await;
    // 启动服务并且
    Server::new(listener).serve(service).await;

}