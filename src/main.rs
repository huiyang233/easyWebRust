use crate::api::init_router;
use crate::config::config::load_config;
use crate::middleware::log::log;
use crate::task::scheduler::init_scheduler;
use crate::task::sms_task::SmsServer;
use deadpool_redis::{Config, Runtime};
use idgen::IDGen;
use lazy_static::lazy_static;
use rbatis::RBatis;
use rbdc_mysql::MysqlDriver;
use salvo::cors::{Any, Cors};
use salvo::prelude::*;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

mod model;
mod auth;
mod utils;
mod api;
mod service;
mod middleware;
mod config;
mod task;

lazy_static! {
    static ref SERVER_CONFIG:config::config::Config = load_config().unwrap();
    // 数据库
    static ref RB:RBatis = RBatis::new();
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
}

#[tokio::main]
async fn main() {
    // _guard必须贯穿整个主流程
    let _guard = utils::log::init_log();
    // 定时任务初始化
    init_scheduler().await;
    // 设置端口
    RB.init(MysqlDriver{},&SERVER_CONFIG.db_url).unwrap();
    // 初始化路由
    let router = init_router();

    let cors_handler = Cors::new()
        .allow_origin(Any)
        .allow_credentials(false)
        .allow_methods(Any)
        .allow_headers(Any)
        .into_handler();
    // 初始化服务
    let service = Service::new(router).hoop(log).hoop(cors_handler);

    //// 证书
    // let cert = include_bytes!("../certs/cert.pem").to_vec();
    // let key = include_bytes!("../certs/key.pem").to_vec();
    // let config = RustlsConfig::new(Keycert::new().cert(cert.as_slice()).key(key.as_slice()));


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
