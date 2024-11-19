use std::ops::Deref;

use lazy_static::lazy_static;
use rbatis::rbdc::DateTime;
use rbatis::{Page, PageRequest};
use salvo::{Depot, Request};

use crate::auth::auth_check::AuthCheck;
use crate::model::agent_info::{AgentInfo, AgentInfoAddDto, AgentInfoEditDto, AgentInfoPageReq, AgentInfoVo};
use crate::model::result::{Http, HttpPage, PageDto, ResultError, WebResult};
use crate::utils::db::Redis;
use crate::{ID_WORKER, RB};

lazy_static! {
    static ref AGENT_INFO_CACHI: Redis<AgentInfo> = Redis::<AgentInfo>::new("AgentInfo");
}
pub struct AgentInfoService;
impl AgentInfoService{
    pub async fn select_by_id(id: u64) -> Option<AgentInfo> {
        let option = AGENT_INFO_CACHI.get(id.to_string().as_str()).await;
        match option {
            None => {
                let agent_info = AgentInfo::select_by_id(RB.deref(),&id).await;
                match agent_info {
                    Ok(agent_info) => {
                        match agent_info {
                            None => {None}
                            Some(agent_info) => {
                                AGENT_INFO_CACHI.set_minute(id.to_string().as_str(), agent_info.clone(),10).await.ok();
                                Some(agent_info)
                            }
                        }
                    }
                    Err(_) => { None }
                }
            }
            Some(agent_info) => {Some(agent_info)}
        }
    }

    pub async fn get_agent_info_details(req: &mut Request)->Http<AgentInfoVo>{
        let id = req.param::<u64>("id");
        match id {
            None => {
                Err(ResultError::param_error("id不能为空".to_string()))
            }
            Some(id) => {
                match AgentInfoService::select_by_id(id).await {
                    None => {
                        return Err(ResultError::resource_not_found("agent_info".to_string()));
                    }
                    Some(user) => {
                        Ok(WebResult::success(AgentInfoVo::from(user)))
                    }
                }

            }
        }

    }

    pub async fn get_agent_info_by_page(req: &mut Request)->HttpPage<AgentInfoVo>{
        let page_dto = req.parse_queries::<PageDto>().unwrap_or_else(|_| PageDto{ page: 0, page_size: 10 });
        let page_request = PageRequest::from(page_dto);
        let item: AgentInfoPageReq = req.parse_queries()?;
        let page = AgentInfo::select_page(RB.deref(), &page_request, item).await?;
        let page_vo = Page::<AgentInfoVo>::from(page);
        Ok(WebResult::success_page(page_vo))
    }

    pub async fn delete_agent_info_by_id(req: &mut Request)->Http<String> {
        let id = req.param::<u64>("id");
        match id {
            None => {
                Err(ResultError::param_error("id不能为空".to_string()))
            }
            Some(id) => {
                AgentInfo::delete_by_id(RB.deref(), &id).await?;
                AGENT_INFO_CACHI.remove(id.to_string().as_str()).await.ok();
                Ok(WebResult::success_none())
            }
        }
    }

    pub async fn edit_agent_info_by_id(req: &mut Request,depot: &mut Depot)->Http<String> {
        let id = req.param::<u64>("id");
        let id =match id {
            None => {
                return Err(ResultError::param_error("id不能为空".to_string()));
            }
            Some(id) => { id }
        };
        let dto = req.parse_json::<AgentInfoEditDto>().await?;
        let current_user =depot.get_user()?;
        let option = AgentInfoService::select_by_id(id).await;
        let mut database_data = match option {
            None => {
                return Err(ResultError::resource_not_found("agent_info".to_string()));
            }
            Some(data) => {data}
        };
        database_data.name = dto.name;
        database_data.city_id = dto.city_id;
        database_data.owner = dto.owner;
        database_data.id_del = dto.id_del;
        database_data.update_by = current_user.user_name.clone();
        database_data.update_time = DateTime::now();
        AGENT_INFO_CACHI.set_minute(database_data.id.to_string().as_str(), database_data.clone(),10).await.ok();
        AgentInfo::update_by_column(RB.deref(), &database_data, "id").await?;
        Ok(WebResult::success_none())
    }

    pub async fn add_agent_info_by_id(req: &mut Request,depot: &mut Depot)->Http<String> {
        let current_user = depot.get_user()?;
        let dto = req.parse_json::<AgentInfoAddDto>().await?;
        let option = AgentInfo::select_by_name(RB.deref(),dto.name.clone()).await?;
        if option.is_some() {
            return Err(ResultError::resource_exists("agent_info".to_string()))
        }
        let mut database_data = AgentInfo::default();
        database_data.id = ID_WORKER.new_id();
        database_data.name = dto.name;
        database_data.city_id = dto.city_id;
        database_data.owner = dto.owner;
        database_data.id_del = Some(false);
        database_data.create_by = current_user.user_name.clone();
        database_data.update_by = current_user.user_name.clone();
        database_data.create_time = DateTime::now();
        database_data.update_time = DateTime::now();
        AgentInfo::insert(RB.deref(), &database_data).await?;
        Ok(WebResult::success(database_data.id.to_string()))
    }
}