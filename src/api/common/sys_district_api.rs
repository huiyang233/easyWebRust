use crate::model::result::{Http, HttpPage, WebResult};
use crate::model::sys_district::SysDistrictVo;
use crate::model::user::SysUser;
use crate::service::sys_district_service::SysDistrictService;
use salvo::{handler, Request, Router};
use salvo_core::Depot;

pub fn init_sys_district_router() -> Router{
    Router::new()
        // 如果查询不需要登录请删除 hoop，权限检查也记得一并删除。
        .push(
            Router::with_path("sys_district")
                .get(get_sys_district_by_page)
        ) .push(
        Router::with_path("test")
            .get(test)
    )
}

#[handler]
pub async fn get_sys_district_by_page(req: &mut Request) ->HttpPage<SysDistrictVo>{
    SysDistrictService::get_sys_district_by_page(req).await
}

#[handler]
pub async fn test(req: &mut Request,depot: &mut Depot) ->Http<SysUser>{
    Ok(WebResult::success(SysUser::default()))
}

