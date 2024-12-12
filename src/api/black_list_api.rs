use salvo::{handler, Depot, Request, Router};

use crate::auth::auth_check::auth_check;
use crate::model::black_list::{BlackListConfigVo, BlackListVo};
use crate::model::result::{Http, HttpPage};
use crate::service::black_list_service::BlackListService;

pub fn init_black_list_router() -> Router{
    Router::new()
        .hoop(auth_check)
        // 如果查询不需要登录请删除 hoop，权限检查也记得一并删除。
        .push(
            Router::with_path("black_list")
                .post(add_black_list_by_id)
                .get(get_black_list_by_page)
                .push(Router::new().path("<ip>")
                    .get(get_black_list_details)
                    .patch(edit_black_list_by_id)
                    .delete(delete_black_list_by_ip)
                )

        )
        .push(Router::new().path("/black_config")
            .get(get_black_config)
            .patch(edit_black_config)
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



#[handler]
pub async fn edit_black_config(req: &mut Request, depot: &mut Depot) -> Http<String> {
    // depot.check_permission(&["black_list","black_list::update"]).await?;
    BlackListService::edit_black_config(req,depot).await
}

#[handler]
pub async fn get_black_config(req: &mut Request, depot: &mut Depot) -> Http<BlackListConfigVo> {
    // depot.check_permission(&["black_list","black_list::update"]).await?;
    BlackListService::get_black_config().await
}
