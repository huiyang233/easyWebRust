use salvo::Router;
use crate::api::admin::init_admin_router;
use crate::api::common::init_common_router;
use crate::api::front_end::init_front_end_router;

mod admin;

mod front_end;
mod common;

///
/// ## 所有路由初始化
///
pub fn init_router() -> Router {
    Router::new()
        .push(init_common_router())
        .push(
            Router::with_path("admin").push(init_admin_router())
        )
        .push(
            Router::with_path("api").push(init_front_end_router())
        )
}