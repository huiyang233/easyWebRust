use crate::auth::auth_check::{auth_check, AuthCheck};
use crate::model::log::SysLogVo;
use crate::model::result::{Http, HttpPage};
use crate::service::log_server::SysLogService;
use crate::service::report_service::SysReport;
use rbs::Value;
use salvo::{handler, Depot, Request, Router};

pub fn init_sys_log_router() -> Router{
    Router::new()
        // 如果查询不需要登录请删除 hoop，权限检查也记得一并删除。
        .push(
            Router::with_path("sys_log")
                .hoop(auth_check)
                .get(get_sys_log_by_page)
                .push(Router::new().path("<id>").get(get_sys_log_details)),
        ).push(
        Router::with_path("report")
            .get(select_login_count_by_seven_day)
    )
}

#[handler]
pub async fn get_sys_log_by_page(req: &mut Request,depot: &mut Depot) ->HttpPage<SysLogVo>{
    depot.check_permission(&["sys_log"]).await?;
    SysLogService::get_sys_log_by_page(req).await
}

#[handler]
pub async fn select_login_count_by_seven_day(req: &mut Request,depot: &mut Depot) ->Http<Value>{
    SysReport::select_login_count_by_seven_day().await
}




#[handler]
pub async fn get_sys_log_details(req: &mut Request,depot: &mut Depot) -> Http<SysLogVo> {
    depot.check_permission(&["sys_log"]).await?;
    SysLogService::get_sys_log_details(req).await
}



