use std::ops::Deref;

use crypto::digest::Digest;
use lazy_static::lazy_static;
use rbatis::{Page, PageRequest};
use rbatis::rbdc::DateTime;
use salvo::{Depot, Request};

use crate::{ID_WORKER, RB};
use crate::auth::auth_check::AuthCheck;
use crate::model::permission::{SysPermission, SysPermissionVo};
use crate::model::result::{Http, HttpPage, PageDto, ResultError, WebResult};
use crate::model::role::{SysRole, SysRoleAddDto, SysRoleEditDto, SysRoleVo};
use crate::service::permission_service::{ROLE_PERMISSION_CACHI, SysPermissionService};
use crate::utils::db::DB;
use crate::utils::mini_redis::MiniRedis;
use crate::utils::vec::FromVo;

lazy_static! {
    static ref SYS_ROLE_CACHI: MiniRedis<SysRole> = MiniRedis::<SysRole>::new("SysRole");
    pub static ref USER_SYS_ROLE_CACHI: MiniRedis<Vec<SysRole>> = MiniRedis::<Vec<SysRole>>::new("UserSysRole");
}
pub struct SysRoleService;
impl SysRoleService{
    pub async fn select_by_id(id: u64) -> Option<SysRole> {
        let option = SYS_ROLE_CACHI.get(id.to_string().as_str()).await;
        match option {
            None => {
                let sys_role = SysRole::select_by_id(RB.deref(),&id).await;
                match sys_role {
                    Ok(sys_role) => {
                        match sys_role {
                            None => {None}
                            Some(sys_role) => {
                                SYS_ROLE_CACHI.set(id.to_string().as_str(), sys_role.clone()).await;
                                Some(sys_role)
                            }
                        }
                    }
                    Err(_) => { None }
                }
            }
            Some(sys_role) => {
                SYS_ROLE_CACHI.extend_out_time(id.to_string().as_str(), 60).await;
                Some(sys_role)
            }
        }
    }

    pub async fn select_by_user_id(user_id: u64) -> Option<Vec<SysRole>> {
        let sys_role = SysRole::select_by_user_id(RB.deref(),&user_id).await;
        match sys_role {
            Ok(sys_role) => {
                Some(sys_role)
            }
            Err(_) => { None }
        }
    }

    pub async fn get_sys_role_details(req: &mut Request)->Http<SysRoleVo>{
        let id = req.param::<u64>("id");
        match id {
            None => {
                Err(ResultError::param_error("id不能为空".to_string()))
            }
            Some(id) => {
                match SysRoleService::select_by_id(id).await {
                    None => {
                        return Err(ResultError::resource_not_found("角色".to_string()));
                    }
                    Some(user) => {
                        let mut vo = SysRoleVo::from(user);
                        let permissions = SysPermissionService::select_by_role_id(id).await?;
                        let vec = Vec::<SysPermissionVo>::from_vo(permissions);
                        vo.permissions = vec;
                        Ok(WebResult::success(vo))
                    }
                }

            }
        }

    }

    pub async fn get_sys_role_by_page(req: &mut Request)->HttpPage<SysRoleVo>{
        let page_dto = req.parse_queries::<PageDto>().unwrap_or_else(|_| PageDto{ page: 0, page_size: 10 });
        let page_request = PageRequest::from(page_dto);
        let name = req.query::<String>("name");
        let page = SysRole::select_page(RB.deref(), &page_request, name).await?;
        let mut page_vo = Page::<SysRoleVo>::from(page);
        for x in &mut page_vo.records {
            let option = SysPermissionService::select_by_role_id(x.id).await;
            let vec = Vec::<SysPermissionVo>::from_vo(option.unwrap());
            x.permissions = vec;
        }

        Ok(WebResult::success_page(page_vo))
    }

    pub async fn delete_sys_role_by_id(req: &mut Request)->Http<String> {
        let id = req.param::<u64>("id");
        match id {
            None => {
                Err(ResultError::param_error("id不能为空".to_string()))
            }
            Some(id) => {
                SysRole::delete_by_id(RB.deref(), &id).await?;
                SYS_ROLE_CACHI.remove(id.to_string().as_str()).await;
                Ok(WebResult::success_none())
            }
        }
    }

    pub async fn edit_sys_role_by_id(req: &mut Request,depot: &mut Depot)->Http<String> {
        let id = req.param::<u64>("id");
        let id =match id {
            None => {
                return Err(ResultError::param_error("id不能为空".to_string()));
            }
            Some(id) => { id }
        };
        let dto = req.parse_json::<SysRoleEditDto>().await?;
        let current_user =depot.get_user()?;
        let option = SysRoleService::select_by_id(id).await;
        let mut database_data = match option {
            None => {
                return Err(ResultError::resource_not_found("角色".to_string()));
            }
            Some(data) => {data}
        };

        let permission_ids= dto.permission_ids.unwrap_or_else(|| {
            vec![]
        });

        let mut permission_vec = vec![];
        for x in &permission_ids {
            let role = SysPermissionService::select_by_id(x.clone()).await;
            match role {
                None => {
                    return Err(ResultError::resource_not_found(format!("权限id:{}",x)));
                }
                Some(role) => {permission_vec.push(role);}
            }
        }

        database_data.update_by = current_user.user_name.clone();
        database_data.update_time = DateTime::now();
        dto.name.map(|x|{
            database_data.name = x;
        });
        let mut tx = DB::get_transaction().await;
        if !permission_vec.is_empty() {
            SysPermission::delete_permission_by_role_id(&mut tx,&database_data.id).await?;
            SysPermission::insert_list(&mut tx, &permission_ids, &database_data.id).await?;
        }
        SysRole::update_by_column(&mut tx, &database_data, "id").await?;
        tx.commit().await?;
        if !permission_vec.is_empty() {
            ROLE_PERMISSION_CACHI.set(database_data.id.to_string().as_str(), permission_vec).await;
        }else{
            ROLE_PERMISSION_CACHI.set(database_data.id.to_string().as_str(), vec![]).await;
        }
        SYS_ROLE_CACHI.set_minute(database_data.id.to_string().as_str(), database_data.clone(),10).await;

        Ok(WebResult::success_none())
    }

    pub async fn add_sys_role_by_id(req: &mut Request,depot: &mut Depot)->Http<String> {
        let current_user = depot.get_user()?;
        let dto = req.parse_json::<SysRoleAddDto>().await?;
        let option = SysRole::select_by_name(RB.deref(),dto.name.clone()).await?;
        if option.is_some() {
            return Err(ResultError::resource_exists("角色".to_string()))
        }
        let permission_ids=  match dto.permission_ids {
            None => {
                return Err(ResultError::param_error("缺少权限id".to_string()))
            }
            Some(permission_ids) => permission_ids
        };

        let mut permission_vec = vec![];
        for x in &permission_ids {
            let role = SysPermissionService::select_by_id(x.clone()).await;
            match role {
                None => {
                    return Err(ResultError::resource_not_found(format!("权限id:{}",x)));
                }
                Some(role) => {permission_vec.push(role);}
            }
        }


        let mut database_data = SysRole::default();
        database_data.id = ID_WORKER.new_id();
        database_data.create_by = current_user.user_name.clone();
        database_data.create_time = DateTime::now();
        database_data.is_del = false;
        database_data.update_by = current_user.user_name.clone();
        database_data.update_time = DateTime::now();
        database_data.name = dto.name;
        let mut tx = DB::get_transaction().await;

        if !permission_vec.is_empty() {
            SysPermission::delete_permission_by_role_id(&mut tx,&database_data.id).await?;
            SysPermission::insert_list(&mut tx, &permission_ids, &database_data.id).await?;
        }
        SysRole::insert(&mut tx, &database_data).await?;
        tx.commit().await?;
        if !permission_vec.is_empty() {
            ROLE_PERMISSION_CACHI.set(database_data.id.to_string().as_str(), permission_vec).await;
        }
        Ok(WebResult::success(database_data.id.to_string()))
    }
    // 查询全部角色
    pub async fn get_all_role()->Http<Vec<SysRoleVo>>{
        let page = SysRole::select_all(RB.deref()).await?;
        let vec = Vec::<SysRoleVo>::from_vo(page);
        Ok(WebResult::success(vec))
    }
}