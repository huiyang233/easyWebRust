use crate::model::request_log::RequestLog;
use crate::model::result::ResultError;
use crate::task::request_log_task::RequestLogTask;
use crate::ID_WORKER;
use lazy_static::lazy_static;
use rbatis::rbdc::DateTime;
use salvo::{handler, Depot, FlowCtrl, Request, Response};
use std::time::Instant;

///
/// 打印请求和响应日志中间件
///
///

lazy_static! {
     static ref SMS_REQUEST_LOG_TASK:RequestLogTask= RequestLogTask::new();
}

#[handler]
pub async fn log(req: &mut Request, depot: &mut Depot, res: &mut Response, ctrl: &mut FlowCtrl)->Result<(),ResultError> {

    let now = Instant::now();
    ctrl.call_next(req, depot, res).await;
    let duration = now.elapsed();

    // info!("之前req:{:?}",req);
    // info!("之后res:{:?}",res.body);
    let user_id = req.headers_mut().get("user_id").map(|user_id|{
            let x = user_id.to_str().unwrap_or_default();
            let user_id_u64: u64 = x.parse().unwrap_or_else(|_| 0u64);
            user_id_u64
        }
    );


    SMS_REQUEST_LOG_TASK.send(RequestLog {
        id: ID_WORKER.new_id(),
        ip: req.remote_addr().to_string().replace("socket://",""),
        uri: req.uri().to_string(),
        method: req.method().to_string(),
        duration: duration.as_millis() as u64,
        user_id,
        headers: Some(format!("{:?}",req.headers())),
        query: Some(format!("{:?}",req.queries())),
        create_time: Some(DateTime::now()),
    }).await;
    Ok(())
}

