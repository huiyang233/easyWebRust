use std::ops::Deref;

use crate::model::log::{SysLog, SysLogPageReq, SysLogVo};
use crate::model::result::{Http, HttpPage, PageDto, ResultError, WebResult};
use crate::{ID_WORKER, RB};
use rbatis::rbdc::DateTime;
use rbatis::{Page, PageRequest};
use salvo::Request;
use tracing::info;
use validator::Validate;

pub struct SysLogService;
static LOG_TYPE_LOGIN: i32 = 1;
static LOG_TYPE_DEL: i32 = 2;
static LOG_TYPE_UPDATE: i32 = 3;
static LOG_TYPE_OTHER: i32 = 4;


impl SysLogService {
    pub async fn select_by_id(id: u64) -> Option<SysLog> {
        let sys_log = SysLog::select_by_id(RB.deref(), &id).await;
        match sys_log {
            Ok(sys_log) => {
                match sys_log {
                    None => { None }
                    Some(sys_log) => {
                        Some(sys_log)
                    }
                }
            }
            Err(_) => { None }
        }
    }

    pub async fn get_sys_log_details(req: &mut Request) -> Http<SysLogVo> {
        let id = req.param::<u64>("id");
        match id {
            None => {
                Err(ResultError::param_error("id不能为空".to_string()))
            }
            Some(id) => {
                match SysLogService::select_by_id(id).await {
                    None => {
                        return Err(ResultError::resource_not_found("sys_log".to_string()));
                    }
                    Some(user) => {
                        Ok(WebResult::success(SysLogVo::from(user)))
                    }
                }
            }
        }
    }

    pub async fn get_sys_log_by_page(req: &mut Request) -> HttpPage<SysLogVo> {
        let page_dto = req.parse_queries::<PageDto>().unwrap_or_else(|_| PageDto { page: 0, page_size: 10 });
        let page_request = PageRequest::from(page_dto);
        let item: SysLogPageReq = req.parse_queries()?;
        if let Err(e) = item.validate() {
            return Err(ResultError::param_error(e.to_string()));
        }
        info!("item:{:?}", item);
        let page = SysLog::select_page(RB.deref(), &page_request, item).await?;
        let page_vo = Page::<SysLogVo>::from(page);
        Ok(WebResult::success_page(page_vo))
    }

    pub async fn add_login_log(user_name: String, ip: String){
        SysLogService::add_sys_log("用户登录".to_string(), LOG_TYPE_LOGIN, "登录成功".to_string(), user_name, ip.replace("socket://","")).await.ok();
    }
    pub async fn add_sys_log(name: String, log_type: i32, desc: String, user_name: String, ip: String) -> Result<(), ResultError> {
        let mut database_data = SysLog::default();
        database_data.id = ID_WORKER.new_id();
        database_data.name = name;
        database_data.log_type = log_type;
        database_data.description = desc;
        database_data.user_name = user_name;
        database_data.ip = ip;
        database_data.create_time = DateTime::now();
        SysLog::insert(RB.deref(), &database_data).await?;
        Ok(())
    }
}
