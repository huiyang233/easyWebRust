use std::ops::Deref;

use lazy_static::lazy_static;
use rbatis::rbdc::DateTime;
use rbatis::{Page, PageRequest};
use salvo::{Depot, Request};

use crate::auth::auth_check::AuthCheck;
use crate::model::merchant_info::{MerchantInfo, MerchantInfoAddDto, MerchantInfoEditDto, MerchantInfoPageReq, MerchantInfoVo};
use crate::model::result::{Http, HttpPage, PageDto, ResultError, WebResult};
use crate::utils::db::Redis;
use crate::{ID_WORKER, RB};

lazy_static! {
    static ref MERCHANT_INFO_CACHI: Redis<MerchantInfo> = Redis::<MerchantInfo>::new("MerchantInfo");
}
pub struct MerchantInfoService;
impl MerchantInfoService{
    pub async fn select_by_id(id: u64) -> Option<MerchantInfo> {
        let option = MERCHANT_INFO_CACHI.get(id.to_string().as_str()).await;
        match option {
            None => {
                let merchant_info = MerchantInfo::select_by_id(RB.deref(),&id).await;
                match merchant_info {
                    Ok(merchant_info) => {
                        match merchant_info {
                            None => {None}
                            Some(merchant_info) => {
                                MERCHANT_INFO_CACHI.set_minute(id.to_string().as_str(), merchant_info.clone(),10).await.ok();
                                Some(merchant_info)
                            }
                        }
                    }
                    Err(_) => { None }
                }
            }
            Some(merchant_info) => {Some(merchant_info)}
        }
    }

    pub async fn get_merchant_info_details(req: &mut Request)->Http<MerchantInfoVo>{
        let id = req.param::<u64>("id");
        match id {
            None => {
                Err(ResultError::param_error("id不能为空".to_string()))
            }
            Some(id) => {
                match MerchantInfoService::select_by_id(id).await {
                    None => {
                        Err(ResultError::resource_not_found("merchant_info".to_string()))
                    }
                    Some(user) => {
                        Ok(WebResult::success(MerchantInfoVo::from(user)))
                    }
                }

            }
        }

    }

    pub async fn get_merchant_info_by_page(req: &mut Request)->HttpPage<MerchantInfoVo>{
        let page_dto = req.parse_queries::<PageDto>().unwrap_or_else(|_| PageDto{ page: 0, page_size: 10 });
        let page_request = PageRequest::from(page_dto);
        let item:MerchantInfoPageReq = req.parse_queries()?;
        let page = MerchantInfo::select_page(RB.deref(), &page_request, item).await?;
        let page_vo = Page::<MerchantInfoVo>::from(page);
        Ok(WebResult::success_page(page_vo))
    }

    pub async fn delete_merchant_info_by_id(req: &mut Request)->Http<String> {
        let id = req.param::<u64>("id");
        match id {
            None => {
                Err(ResultError::param_error("id不能为空".to_string()))
            }
            Some(id) => {
                MerchantInfo::delete_by_id(RB.deref(), &id).await?;
                MERCHANT_INFO_CACHI.remove(id.to_string().as_str()).await.ok();
                Ok(WebResult::success_none())
            }
        }
    }

    pub async fn edit_merchant_info_by_id(req: &mut Request,depot: &mut Depot)->Http<String> {
        let id = req.param::<u64>("id");
        let id =match id {
            None => {
                return Err(ResultError::param_error("id不能为空".to_string()));
            }
            Some(id) => { id }
        };
        let dto = req.parse_json::<MerchantInfoEditDto>().await?;
        let current_user =depot.get_user()?;
        let option = MerchantInfoService::select_by_id(id).await;
        let mut database_data = match option {
            None => {
                return Err(ResultError::resource_not_found("merchant_info".to_string()));
            }
            Some(data) => {data}
        };
        database_data.name = dto.name;
        database_data.audit_status = dto.audit_status;
        database_data.owner = dto.owner;
        database_data.agent_id = dto.agent_id;
        database_data.city_id = dto.city_id;
        database_data.update_by = Option::from(current_user.user_name.clone());
        database_data.update_time = Option::from(DateTime::now());
        MERCHANT_INFO_CACHI.set_minute(database_data.id.to_string().as_str(), database_data.clone(),10).await.ok();
        MerchantInfo::update_by_column(RB.deref(), &database_data, "id").await?;
        Ok(WebResult::success_none())
    }

    pub async fn add_merchant_info_by_id(req: &mut Request,depot: &mut Depot)->Http<String> {
        let current_user = depot.get_user()?;
        let dto = req.parse_json::<MerchantInfoAddDto>().await?;

        let option = MerchantInfo::select_by_name(RB.deref(),dto.name.clone()).await?;
        if option.is_some() {
            return Err(ResultError::resource_exists("merchant_info".to_string()))
        }
        let mut database_data = MerchantInfo::default();
        database_data.id = ID_WORKER.new_id();
        database_data.name = Some(dto.name);
        database_data.audit_status = Some(dto.audit_status);
        database_data.owner = Some(dto.owner);
        database_data.agent_id = Some(dto.agent_id);
        database_data.city_id = Some(dto.city_id);
        database_data.is_del = Option::from(false);
        database_data.create_by = Option::from(current_user.user_name.clone());
        database_data.update_by = Option::from(current_user.user_name.clone());
        database_data.create_time = Option::from(DateTime::now());
        database_data.update_time = Option::from(DateTime::now());
        MerchantInfo::insert(RB.deref(), &database_data).await?;
        Ok(WebResult::success(database_data.id.to_string()))
    }
}