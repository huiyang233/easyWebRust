use salvo::Router;
use crate::api::common::file_api::init_file_router;
use crate::api::common::sys_district_api::init_sys_district_router;

mod file_api;
mod sys_district_api;

pub fn init_common_router() ->Router {
    Router::new()
        .push(init_file_router())
        .push(init_sys_district_router())
}