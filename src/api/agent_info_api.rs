use salvo::{handler, Depot, Request, Router};

use crate::auth::auth_check::{auth_check, AuthCheck};
use crate::model::agent_info::AgentInfoVo;
use crate::model::result::{Http, HttpPage};
use crate::service::agent_info_service::AgentInfoService;

pub fn init_agent_info_router() -> Router{
    Router::new()
        // 如果查询不需要登录请删除 hoop，权限检查也记得一并删除。
        .push(
            Router::with_path("agent_info")
                .hoop(auth_check)
                .get(get_agent_info_by_page)
                .push(Router::new().path("<id>").get(get_agent_info_details)),
        )
        .push(
            Router::with_path("agent_info")
                .hoop(auth_check)
                .post(add_agent_info_by_id)
                .push(Router::new().path("<id>").patch(edit_agent_info_by_id).delete(delete_agent_info_by_id)),
        )
}

#[handler]
pub async fn get_agent_info_by_page(req: &mut Request,depot: &mut Depot) ->HttpPage<AgentInfoVo>{
    depot.check_permission(&["agent_info"]).await?;
    AgentInfoService::get_agent_info_by_page(req).await
}

#[handler]
pub async fn get_agent_info_details(req: &mut Request,depot: &mut Depot) -> Http<AgentInfoVo> {
    depot.check_permission(&["agent_info"]).await?;
    AgentInfoService::get_agent_info_details(req).await
}


#[handler]
pub async fn delete_agent_info_by_id(req: &mut Request,depot: &mut Depot) -> Http<String> {
    depot.check_permission(&["agent_info","agent_info::delete"]).await?;
    AgentInfoService::delete_agent_info_by_id(req).await
}

#[handler]
pub async fn add_agent_info_by_id(req: &mut Request, depot: &mut Depot) -> Http<String> {
    depot.check_permission(&["agent_info","agent_info::add"]).await?;
    AgentInfoService::add_agent_info_by_id(req,depot).await
}

#[handler]
pub async fn edit_agent_info_by_id(req: &mut Request, depot: &mut Depot) -> Http<String> {
    depot.check_permission(&["agent_info","agent_info::update"]).await?;
    AgentInfoService::edit_agent_info_by_id(req,depot).await
}


