use salvo::Router;

use crate::api::file_api::init_file_router;
use crate::api::login_api::init_login_router;
use crate::api::permission_api::init_sys_permission_router;
use crate::api::role_api::init_sys_role_router;
use crate::api::user_api::init_user_router;

mod user_api;
mod role_api;
mod permission_api;
mod login_api;
mod file_api;

pub fn init_router() ->Router {
    Router::new()
        .push(init_user_router())
        .push(init_sys_role_router())
        .push(init_sys_permission_router())
        .push(init_login_router())
        .push(init_file_router())
}