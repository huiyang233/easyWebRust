use salvo::{handler, Depot, Request, Router};

use crate::auth::auth_check::auth_check;
use crate::model::result::Http;
use crate::service::login_service::{CaptchaVo, LoginResultVo, LoginService};
use crate::service::user_service::UserService;

pub fn init_login_router() -> Router{
    Router::new()
        .push(
            Router::with_path("login")
                .post(login)
                .push(Router::with_path("verificationCode").get(get_verification_code))
                .push(Router::with_path("sendSmsVerificationCode").post(send_sms_verification_code))


        )
        .push(
            Router::with_path("login")
                .hoop(auth_check)
                .push(Router::with_path("logout").post(logout))
                .push(Router::with_path("changePassword").post(change_password))
        )
}

#[handler]
pub async fn login(req: &mut Request) -> Http<LoginResultVo> {
    LoginService::login(req).await
}



#[handler]
pub async fn get_verification_code() -> Http<CaptchaVo> {
    LoginService::get_verification_code().await
}

#[handler]
pub async fn logout(req: &mut Request) -> Http<String> {
    LoginService::logout(req).await
}

#[handler]
pub async fn change_password(req: &mut Request, depot: &mut Depot) -> Http<String> {
    UserService::change_password(req, depot).await
}

#[handler]
pub async fn send_sms_verification_code(req: &mut Request) -> Http<String> {
    LoginService::send_sms_verification_code(req).await
}





