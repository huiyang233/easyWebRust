use crate::model::result::Http;
use crate::service::login_service::{LoginService, WxLoginResultVo};
use salvo::{handler, Request, Router};


pub fn init_u_login_router() -> Router{
    Router::with_path("wx_login").post(wx_login)
        .push(Router::with_path("wx_logout").post(logout))
}

#[handler]
pub async fn wx_login(req: &mut Request) -> Http<WxLoginResultVo> {
    LoginService::wx_login(req).await
}

#[handler]
pub async fn logout(req: &mut Request) -> Http<String> {
    LoginService::logout(req).await
}