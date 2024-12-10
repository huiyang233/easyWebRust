use salvo::{handler, Depot, Request, Router};

use crate::auth::auth_check::auth_check;
use crate::model::black_list::BlackListVo;
use crate::model::result::{Http, HttpPage};
use crate::service::black_list_service::BlackListService;

pub fn init_black_list_router() -> Router{
    Router::new()
        // 如果查询不需要登录请删除 hoop，权限检查也记得一并删除。
        .push(
            Router::with_path("black_list")
                .hoop(auth_check)
                .get(get_black_list_by_page)
                .push(Router::new().path("<id>").get(get_black_list_details)),
        )
        .push(
            Router::with_path("black_list")
                .hoop(auth_check)
                .post(add_black_list_by_id)
                .push(Router::new().path("<ip>").patch(edit_black_list_by_id).delete(delete_black_list_by_ip)),
        )
}

#[handler]
pub async fn get_black_list_by_page(req: &mut Request,depot: &mut Depot) ->HttpPage<BlackListVo>{
    // depot.check_permission(&["black_list"]).await?;
    BlackListService::get_black_list_by_page(req).await
}

#[handler]
pub async fn get_black_list_details(req: &mut Request,depot: &mut Depot) -> Http<BlackListVo> {
    // depot.check_permission(&["black_list"]).await?;
    BlackListService::get_black_list_details(req).await
}


#[handler]
pub async fn delete_black_list_by_ip(req: &mut Request,depot: &mut Depot) -> Http<String> {
    // depot.check_permission(&["black_list","black_list::delete"]).await?;
    BlackListService::delete_black_list_by_ip(req).await
}

#[handler]
pub async fn add_black_list_by_id(req: &mut Request, depot: &mut Depot) -> Http<String> {
    // depot.check_permission(&["black_list","black_list::add"]).await?;
    BlackListService::add_black_list_by_id(req,depot).await
}

#[handler]
pub async fn edit_black_list_by_id(req: &mut Request, depot: &mut Depot) -> Http<String> {
    // depot.check_permission(&["black_list","black_list::update"]).await?;
    BlackListService::edit_black_list_by_id(req,depot).await
}


