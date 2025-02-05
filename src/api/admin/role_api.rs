use salvo::{handler, Depot, Request, Router};

use crate::auth::auth_check::{auth_check, AuthCheck};
use crate::model::result::{Http, HttpPage};
use crate::model::role::SysRoleVo;
use crate::service::role_service::SysRoleService;

pub fn init_sys_role_router() -> Router{
    Router::new()
        .push(
            Router::with_path("role")
                .hoop(auth_check)
                .get(get_sys_role_by_page)
                .push(Router::new().path("{id:num}").get(get_sys_role_details))
                .push(Router::new().path("all").get(get_all_role)),
        )
        .push(
            Router::with_path("role")
                .hoop(auth_check)
                .post(add_sys_role_by_id)
                .push(Router::new().path("{id}").patch(edit_sys_role_by_id).delete(delete_sys_role_by_id))
        )
}

#[handler]
pub async fn delete_sys_role_by_id(req: &mut Request) -> Http<String> {
    SysRoleService::delete_sys_role_by_id(req).await
}

#[handler]
pub async fn add_sys_role_by_id(req: &mut Request, depot: &mut Depot) -> Http<String> {
    SysRoleService::add_sys_role_by_id(req,depot).await
}

#[handler]
pub async fn edit_sys_role_by_id(req: &mut Request, depot: &mut Depot) -> Http<String> {
    SysRoleService::edit_sys_role_by_id(req,depot).await
}


#[handler]
pub async fn get_sys_role_details(req: &mut Request) -> Http<SysRoleVo> {
    SysRoleService::get_sys_role_details(req).await
}

#[handler]
pub async fn get_sys_role_by_page(req: &mut Request,depot: &mut Depot) ->HttpPage<SysRoleVo>{
    depot.check_permission(&["role"]).await?;
    SysRoleService::get_sys_role_by_page(req).await
}

// 查询全部角色
#[handler]
pub async fn get_all_role() -> Http<Vec<SysRoleVo>> {
    SysRoleService::get_all_role().await
}



