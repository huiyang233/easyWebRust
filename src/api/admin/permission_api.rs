use salvo::{handler, Depot, Request, Router};

use crate::auth::auth_check::{auth_check, AuthCheck};
use crate::model::permission::{SysPermissionVo, Tree};
use crate::model::result::{Http, HttpPage};
use crate::service::permission_service::SysPermissionService;

pub fn init_sys_permission_router() -> Router{
    Router::new()
        .push(
            Router::with_path("permission")
                .hoop(auth_check)
                .get(get_sys_permission_by_page)
                .push(Router::new().path("tree").get(tree))
                .push(Router::new().path("self").get(get_self_permission))
                .push(Router::new().path("{id}").get(get_sys_permission_details))


        )
        .push(
            Router::with_path("permission")
                .hoop(auth_check)
                .post(add_sys_permission_by_id)
                .push(Router::new().path("{id}")
                    .patch(edit_sys_permission_by_id)
                    .delete(delete_sys_permission_by_id))
        )
}

#[handler]
pub async fn tree(depot: &mut Depot) ->Http<Vec<Tree>>{
    depot.check_any_permission(&["permission","role:add","role:update","role"]).await?;
    SysPermissionService::tree().await
}

#[handler]
pub async fn get_sys_permission_by_page(req: &mut Request,depot: &mut Depot) -> HttpPage<SysPermissionVo>{
    depot.check_permission(&["permission"]).await?;
    SysPermissionService::get_sys_permission_by_page(req).await
}

#[handler]
pub async fn get_self_permission(depot: &mut Depot) -> Http<Vec<SysPermissionVo>>{
    SysPermissionService::get_self_permission(depot).await
}


#[handler]
pub async fn get_sys_permission_details(req: &mut Request,depot: &mut Depot) -> Http<SysPermissionVo> {
    depot.check_permission(&["permission"]).await?;
    SysPermissionService::get_sys_permission_details(req).await
}


#[handler]
pub async fn delete_sys_permission_by_id(depot: &mut Depot) -> Http<String> {
    depot.check_permission(&["permission","permission:del"]).await?;
    SysPermissionService::delete_sys_permission_by_id().await
}

#[handler]
pub async fn add_sys_permission_by_id(req: &mut Request, depot: &mut Depot) -> Http<String> {
    depot.check_permission(&["permission","permission:add"]).await?;
    SysPermissionService::add_sys_permission_by_id(req,depot).await
}


#[handler]
pub async fn edit_sys_permission_by_id(req: &mut Request, depot: &mut Depot) -> Http<String> {
    depot.check_permission(&["permission","permission:update"]).await?;
    SysPermissionService::edit_sys_permission_by_id(req,depot).await
}




