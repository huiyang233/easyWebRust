use crate::model::result::ResultError;
use salvo::{handler, Depot, FlowCtrl, Request, Response};

///
/// 打印请求和响应日志中间件
///
#[handler]
pub async fn log(req: &mut Request, depot: &mut Depot, res: &mut Response, ctrl: &mut FlowCtrl)->Result<(),ResultError> {
    // info!("之前req:{:?}",req.headers());
    ctrl.call_next(req, depot, res).await;
    // info!("之后res:{:?}",res.body);
    Ok(())
}