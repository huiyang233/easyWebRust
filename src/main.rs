use crate::api::init_router;
use crate::auth::auth_check::AuthCheck;
use crate::config::config::load_config;
use crate::middleware::log::log;
use crate::task::sms_task::SmsServer;
use deadpool_redis::{Config, Runtime};
use idgen::IDGen;
use lazy_static::lazy_static;
use rbatis::RBatis;
use rbdc_mysql::MysqlDriver;
use redis::AsyncCommands;
use salvo::conn::Acceptor;
use salvo::prelude::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::ops::Deref;

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
        let cfg = Config::from_url(&SERVER_CONFIG.redis_url);
        cfg.create_pool(Some(Runtime::Tokio1)).unwrap()
    };
    // 短信服务
    static ref SMS_SERVER:SmsServer= SmsServer::new();
}


#[tokio::main]
async fn main() {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_thread_ids(true)
        .init();
    // 设置端口
    let database_url = &SERVER_CONFIG.db_url;
    RB.init(MysqlDriver{},&database_url).unwrap();
    // 初始化路由
    let router = init_router();
    // 初始化服务
    let service = Service::new(router).hoop(log);
    let acceptor = TcpListener::new(("0.0.0.0", SERVER_CONFIG.server.port)).bind().await;
    // 启动服务并且
    Server::new(acceptor).serve(service).await;
}
