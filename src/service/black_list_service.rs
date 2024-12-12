use std::ops::Deref;

use rbatis::{Page, PageRequest};
use salvo::{Depot, Request};

use crate::auth::auth_check::AuthCheck;
use crate::middleware::blacklist::BlackListMid;
use crate::model::black_list::{BlackList, BlackListAddDto, BlackListConfig, BlackListConfigEditDto, BlackListConfigVo, BlackListEditDto, BlackListPageReq, BlackListVo};
use crate::model::result::{Http, HttpPage, PageDto, ResultError, WebResult};
use crate::RB;


pub struct BlackListService;
impl BlackListService{
    pub async fn select_by_id(id: u64) -> Option<BlackList> {
        let black_list = BlackList::select_by_id(RB.deref(),&id).await;
        match black_list {
            Ok(black_list) => {
                match black_list {
                    None => {None}
                    Some(black_list) => {
                        Some(black_list)
                    }
                }
            }
            Err(_) => { None }
        }
    }

    pub async fn get_black_list_details(req: &mut Request)->Http<BlackListVo>{
        let id = req.param::<u64>("id");
        match id {
            None => {
                Err(ResultError::param_error("id不能为空".to_string()))
            }
            Some(id) => {
                match BlackListService::select_by_id(id).await {
                    None => {
                        Err(ResultError::resource_not_found("black_list".to_string()))
                    }
                    Some(user) => {
                        Ok(WebResult::success(BlackListVo::from(user)))
                    }
                }

            }
        }

    }

    pub async fn get_black_list_by_page(req: &mut Request)->HttpPage<BlackListVo>{
        let page_dto = req.parse_queries::<PageDto>().unwrap_or_else(|_| PageDto{ page: 0, page_size: 10 });
        let page_request = PageRequest::from(page_dto);
        let item:BlackListPageReq = req.parse_queries()?;
        let page = BlackList::select_page(RB.deref(), &page_request, item).await?;
        let page_vo = Page::<BlackListVo>::from(page);
        Ok(WebResult::success_page(page_vo))
    }

    pub async fn delete_black_list_by_ip(req: &mut Request)->Http<String> {
        let ip = req.param::<String>("ip");
        match ip {
            None => {
                Err(ResultError::param_error("ip不能为空".to_string()))
            }
            Some(ip) => {
                BlackListMid::del_black_list(ip).await;
                Ok(WebResult::success_none())
            }
        }
    }



    pub async fn edit_black_list_by_id(req: &mut Request,depot: &mut Depot)->Http<String> {
        let id = req.param::<u64>("id");
        let id =match id {
            None => {
                return Err(ResultError::param_error("id不能为空".to_string()));
            }
            Some(id) => { id }
        };
        let dto = req.parse_json::<BlackListEditDto>().await?;
        BlackListMid::update_black_list_date_time(id,dto.ip,dto.ban_time,dto.reason).await;
        Ok(WebResult::success_none())
    }

    pub async fn add_black_list_by_id(req: &mut Request,depot: &mut Depot)->Http<String> {
        let current_user = depot.get_user()?;
        let dto = req.parse_json::<BlackListAddDto>().await?;
        let i = BlackListMid::add_black_list_date_time(dto.ip, dto.ban_time,dto.reason).await;
        Ok(WebResult::success(i.to_string()))
    }

    pub async fn edit_black_config(req: &mut Request,depot: &mut Depot)->Http<String> {
        let dto = req.parse_json::<BlackListConfigEditDto>().await?;
        BlackListConfig::update_by_id(RB.deref(), &dto.ban_time, &dto.interval, &dto.visit_count,&1).await?;
        BlackListMid::update_black_config(dto.ban_time,dto.interval,dto.visit_count).await;
        Ok(WebResult::success_none())
    }

    pub async fn get_black_config()->Http<BlackListConfigVo> {
        let option = BlackListConfig::select_by_id(RB.deref(), &1).await?;
        match option {
            None => {
                Err(ResultError::resource_not_found("black_list_config".to_string()))
            }
            Some(user) => {
                Ok(WebResult::success(BlackListConfigVo::from(user)))
            }
        }
    }
}


