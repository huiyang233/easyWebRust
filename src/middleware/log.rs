use crate::model::result::ResultError;
use salvo::{handler, Depot, FlowCtrl, Request, Response};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone,Default)]
struct Test{
    a:String,
    b:String
}

#[handler]
pub async fn log(req: &mut Request, depot: &mut Depot, res: &mut Response, ctrl: &mut FlowCtrl)->Result<(),ResultError> {
    // info!("之前req:{:?}",req.headers());
    ctrl.call_next(req, depot, res).await;
    // info!("之后res:{:?}",res.body);
    Ok(())
}