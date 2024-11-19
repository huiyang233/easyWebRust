use salvo::{handler, Depot, Request, Router};

use crate::auth::auth_check::{auth_check, AuthCheck};
use crate::model::merchant_info::MerchantInfoVo;
use crate::model::result::{Http, HttpPage};
use crate::service::merchant_info_service::MerchantInfoService;

pub fn init_merchant_info_router() -> Router{
    Router::new()
        // 如果查询不需要登录请删除 hoop，权限检查也记得一并删除。
        .push(
            Router::with_path("merchant_info")
                .hoop(auth_check)
                .get(get_merchant_info_by_page)
                .push(Router::new().path("<id>").get(get_merchant_info_details)),
        )
        .push(
            Router::with_path("merchant_info")
                .hoop(auth_check)
                .post(add_merchant_info_by_id)
                .push(Router::new().path("<id>").patch(edit_merchant_info_by_id).delete(delete_merchant_info_by_id)),
        )
}

#[handler]
pub async fn get_merchant_info_by_page(req: &mut Request,depot: &mut Depot) ->HttpPage<MerchantInfoVo>{
    depot.check_permission(&["merchant_info"]).await?;
    MerchantInfoService::get_merchant_info_by_page(req).await
}

#[handler]
pub async fn get_merchant_info_details(req: &mut Request,depot: &mut Depot) -> Http<MerchantInfoVo> {
    depot.check_permission(&["merchant_info"]).await?;
    MerchantInfoService::get_merchant_info_details(req).await
}


#[handler]
pub async fn delete_merchant_info_by_id(req: &mut Request,depot: &mut Depot) -> Http<String> {
    depot.check_permission(&["merchant_info","merchant_info::delete"]).await?;
    MerchantInfoService::delete_merchant_info_by_id(req).await
}

#[handler]
pub async fn add_merchant_info_by_id(req: &mut Request, depot: &mut Depot) -> Http<String> {
    depot.check_permission(&["merchant_info","merchant_info::add"]).await?;
    MerchantInfoService::add_merchant_info_by_id(req,depot).await
}

#[handler]
pub async fn edit_merchant_info_by_id(req: &mut Request, depot: &mut Depot) -> Http<String> {
    depot.check_permission(&["merchant_info","merchant_info::update"]).await?;
    MerchantInfoService::edit_merchant_info_by_id(req,depot).await
}


