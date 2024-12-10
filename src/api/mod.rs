use crate::api::agent_info_api::init_agent_info_router;
use crate::api::black_list_api::init_black_list_router;
use crate::api::file_api::init_file_router;
use crate::api::log_api::init_sys_log_router;
use crate::api::login_api::init_login_router;
use crate::api::permission_api::init_sys_permission_router;
use crate::api::role_api::init_sys_role_router;
use crate::api::user_api::init_user_router;
use salvo::Router;

mod user_api;
mod role_api;
mod permission_api;
mod login_api;
mod file_api;
mod agent_info_api;
mod merchant_info_api;
mod log_api;
mod black_list_api;

///
/// ## 所有路由初始化
///
pub fn init_router() ->Router {
    Router::new()
        .push(init_user_router())
        .push(init_sys_role_router())
        .push(init_sys_permission_router())
        .push(init_login_router())
        .push(init_file_router())
        .push(init_agent_info_router())
        .push(init_sys_log_router())
        .push(init_black_list_router())
}