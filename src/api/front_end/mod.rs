
mod login;
mod sse_api;

use crate::api::front_end::login::init_u_login_router;
use crate::api::front_end::sse_api::init_u_sse_router;
use salvo::Router;

pub fn init_front_end_router() ->Router {
    Router::new()
        .push(init_u_login_router())
        .push(init_u_sse_router())
}