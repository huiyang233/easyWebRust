use crate::api::admin::black_list_api::init_black_list_router;
use crate::api::admin::log_api::init_sys_log_router;
use crate::api::admin::login_api::init_login_router;
use crate::api::admin::permission_api::init_sys_permission_router;
use crate::api::admin::role_api::init_sys_role_router;
use crate::api::admin::user_api::init_user_router;
use salvo::Router;

mod user_api;
mod role_api;
mod permission_api;
mod login_api;

mod log_api;
mod black_list_api;


pub fn init_admin_router() ->Router {
    Router::new()
        .push(init_user_router())
        .push(init_sys_role_router())
        .push(init_sys_permission_router())
        .push(init_login_router())
        .push(init_sys_log_router())
        .push(init_black_list_router())
}