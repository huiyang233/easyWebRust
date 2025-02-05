use crate::middleware::blacklist::BlackListMid;
use crate::model::black_list::{BlackList, BlackListAddDto, BlackListEditDto, BlackListPageReq, BlackListVo};
use crate::model::black_list_config::{BlackListConfig, BlackListConfigEditDto, BlackListConfigVo};
use crate::model::result::{Http, HttpPage, PageDto, ResultError, WebResult, WebResultPage};
use chrono::Local;
use salvo::Request;
use tracing::info;


pub struct BlackListService;
impl BlackListService{
    pub async fn select_by_id(id: i64) -> Option<BlackList> {
        let black_list = BlackList::select_by_id(&id).await;
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
        let id = req.param::<i64>("id");
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
        let page_dto = req.parse_queries::<PageDto>().unwrap_or_else(|_| PageDto{ page: 1, page_size: 10 });

        let item:BlackListPageReq = req.parse_queries()?;
        let page = BlackList::select_page(item,page_dto).await?;
        let page_vo = WebResultPage::<BlackListVo>::from(page);
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



    pub async fn edit_black_list_by_id(req: &mut Request)->Http<String> {
        let dto = req.parse_json::<BlackListEditDto>().await?;
        info!("dto.ban_time:{:?}",dto.ban_time);
        match &dto.ban_time {
            Some(ban_time) => {
                let ban_time_second = ban_time.timestamp() - Local::now().to_utc().timestamp();
                info!("ban_time_second:{}",ban_time_second);
                if ban_time_second<1 {
                    return Err(ResultError::parameter_error("禁封时间不能小于当前时间".to_string()));
                }
            }
            _ => {}
        }

        BlackListMid::update_black_list_date_time(dto.id,dto.ip,dto.ban_time,dto.reason).await;
        Ok(WebResult::success_none())
    }

    pub async fn add_black_list_by_id(req: &mut Request)->Http<String> {
        let dto = req.parse_json::<BlackListAddDto>().await?;
        match &dto.ban_time {
            Some(ban_time) => {
                let ban_time_second = ban_time.timestamp() - Local::now().to_utc().timestamp();
                info!("ban_time_second:{}",ban_time_second);
                if ban_time_second<1 {
                    return Err(ResultError::parameter_error("禁封时间不能小于当前时间".to_string()));
                }
            }
            _ => {}
        }
        let i = BlackListMid::add_black_list_date_time(dto.ip, dto.ban_time,dto.reason).await;
        Ok(WebResult::success(i.to_string()))
    }

    pub async fn edit_black_config(req: &mut Request)->Http<String> {
        let dto = req.parse_json::<BlackListConfigEditDto>().await?;
        let config = BlackListConfig{
            id: 1,
            ban_time: dto.ban_time,
            interval: dto.interval,
            visit_count: dto.visit_count,
        };

        BlackListConfig::update_by_id(&config).await?;
        BlackListMid::update_black_config(dto.ban_time, dto.interval, dto.visit_count).await;
        Ok(WebResult::success_none())
    }

    pub async fn get_black_config()->Http<BlackListConfigVo> {
        let option = BlackListConfig::select_by_id(&1).await?;
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


