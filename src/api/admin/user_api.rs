use salvo::{handler, Depot, Request, Router};

use crate::auth::auth_check::{auth_check, AuthCheck};
use crate::model::result::{Http, HttpPage};
use crate::model::user::SysUserVo;
use crate::service::user_service::UserService;

pub fn init_user_router() -> Router{
    Router::new()
        .push(
            Router::with_path("user")
                .hoop(auth_check)
                .get(get_user_by_page)
                .push(Router::new().path("{id}").get(get_user_details)),
        )
        .push(
            Router::with_path("user")
                .hoop(auth_check)
                .post(add_user_by_id)
                .push(Router::new().path("{id}").patch(edit_user_by_id).delete(delete_user_by_id)),
        )
}

#[handler]
pub async fn delete_user_by_id(req: &mut Request, depot: &mut Depot) -> Http<String> {
    depot.check_permission(&["user","user:del"]).await?;
    UserService::delete_user_by_id(req).await
}

#[handler]
pub async fn add_user_by_id(req: &mut Request, depot: &mut Depot) -> Http<String> {
    depot.check_permission(&["user","user:add"]).await?;
    UserService::add_user_by_id(req,depot).await
}

#[handler]
pub async fn edit_user_by_id(req: &mut Request, depot: &mut Depot) -> Http<String> {
    depot.check_permission(&["user","user:update"]).await?;
    UserService::edit_user_by_id(req,depot).await
}


#[handler]
pub async fn get_user_details(req: &mut Request,depot: &mut Depot) -> Http<SysUserVo> {
    depot.check_permission(&["user"]).await?;
    UserService::get_user_details(req).await
}

#[handler]
pub async fn get_user_by_page(req: &mut Request,depot: &mut Depot) -> HttpPage<SysUserVo>{
    depot.check_permission(&["user"]).await?;
    UserService::get_user_by_page(req).await
}
