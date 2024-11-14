use std::collections::HashSet;
use std::ops::Deref;

use crypto::digest::Digest;
use lazy_static::lazy_static;
use rbatis::rbdc::DateTime;
use rbatis::{Page, PageRequest};
use salvo::{Depot, Request};

use crate::auth::auth_check::AuthCheck;
use crate::model::permission::{SysPermission, SysPermissionAddDto, SysPermissionEditDto, SysPermissionVo, Tree};
use crate::model::result::{Http, HttpPage, PageDto, ResultError, WebResult};
use crate::service::role_service::SysRoleService;
use crate::utils::db::Redis;
use crate::{ID_WORKER, RB};

lazy_static! {
    static ref SYS_PERMISSION_CACHI: Redis<SysPermission> = Redis::<SysPermission>::new("SysPermission");
    pub static ref ROLE_PERMISSION_CACHI: Redis<Vec<SysPermission>> = Redis::<Vec<SysPermission>>::new("RoleSysPermission");
}
pub struct SysPermissionService;
impl SysPermissionService{
    pub async fn select_by_id(id: u64) -> Option<SysPermission> {
        let option = SYS_PERMISSION_CACHI.get(id.to_string().as_str()).await;
        match option {
            None => {
                let sys_permission = SysPermission::select_by_id(RB.deref(),&id).await;
                match sys_permission {
                    Ok(sys_permission) => {
                        match sys_permission {
                            None => {None}
                            Some(sys_permission) => {
                                // 权限可以永久放里面。
                                SYS_PERMISSION_CACHI.set(id.to_string().as_str(), sys_permission.clone()).await.ok();
                                Some(sys_permission)
                            }
                        }
                    }
                    Err(_) => { None }
                }
            }
            Some(sys_permission) => Some(sys_permission)
        }
    }

    pub async fn select_by_user_id(user_id: u64) -> Option<Vec<SysPermission>> {
        let roles = SysRoleService::select_by_user_id(user_id).await;
        let roles = match roles {
            None => {return None;}
            Some(roles) => {roles}
        };
        let mut set = HashSet::new();
        for x in roles {
            let result = SysPermissionService::select_by_role_id(x.id ).await;
            let result = match result {
                Ok(result) => {result}
                Err(_) => {return None;}
            };
            ROLE_PERMISSION_CACHI.set(x.id.to_string().as_str(), result.clone()).await.ok();
            result.iter().for_each(|x| {set.insert(x.clone());});
        }
        // HashSet转Vec
        let mut vec = set.into_iter().collect::<Vec<_>>();
        // 按照权限id排序
        vec.sort_by(|a, b| a.id.cmp(&b.id));
        Some(vec)
    }

    pub async fn select_by_role_id(role_id: u64) -> Result<Vec<SysPermission>,ResultError> {
        let option = ROLE_PERMISSION_CACHI.get(role_id.to_string().as_str()).await;
        match option {
            None => {
                let sys_permission = SysPermission::select_by_role_id(RB.deref(),role_id).await;
                match sys_permission {
                    Ok(sys_permission) => {
                        ROLE_PERMISSION_CACHI.set(role_id.to_string().as_str(), sys_permission.clone()).await.ok();
                        Ok(sys_permission)
                    }
                    Err(_) => { Err(ResultError::param_error("数据库查询出错".to_string())) }
                }
            }
            Some(sys_permission) => {Ok(sys_permission)}
        }
    }

    pub async fn tree()->Http<Vec<Tree>>{
        let result = SysPermission::select_all(RB.deref()).await.unwrap();
        let vec = Tree::build_tree_nodes(&*result, 0);
        Ok(WebResult::success(vec))
    }

    pub async fn select_all()-> Option<Vec<SysPermission>>{
        let result = match SysPermission::select_all(RB.deref()).await {
            Ok(res) => {Some(res)}
            Err(_) => {None}
        };
        result
    }

    pub async fn get_self_permission(depot: &mut Depot)->Http<Vec<SysPermissionVo>>{
        let x = depot.get_permission().await?;
        let mut permission_vo_list = vec!();
        for x in x {
            permission_vo_list.push(SysPermissionVo::from(x))
        }
        Ok(WebResult::success(permission_vo_list))
    }

    pub async fn get_sys_permission_details(req: &mut Request)->Http<SysPermissionVo>{
        let id = req.param::<u64>("id");
        match id {
            None => {
                Err(ResultError::param_error("id不能为空".to_string()))
            }
            Some(id) => {
                match SysPermissionService::select_by_id(id).await {
                    None => {
                        return Err(ResultError::resource_not_found("权限".to_string()));
                    }
                    Some(user) => {
                        Ok(WebResult::success(SysPermissionVo::from(user)))
                    }
                }

            }
        }

    }

    pub async fn get_sys_permission_by_page(req: &mut Request)->HttpPage<SysPermissionVo>{
        let page_dto = req.parse_queries::<PageDto>().unwrap_or_else(|_| PageDto{ page: 0, page_size: 10 });;
        let page_request = PageRequest::from(page_dto);
        let name = req.query::<String>("name");
        let page = SysPermission::select_page(RB.deref(), &page_request, name).await?;
        let page_vo = Page::<SysPermissionVo>::from(page);
        Ok(WebResult::success_page(page_vo))
    }

    pub async fn delete_sys_permission_by_id(req: &mut Request)->Http<String> {
        Ok(WebResult::success_none())
    }

    pub async fn edit_sys_permission_by_id(req: &mut Request,depot: &mut Depot)->Http<String> {
        let id = req.param::<u64>("id");
        let id =match id {
            None => {
                return Err(ResultError::param_error("id不能为空".to_string()));
            }
            Some(id) => { id }
        };
        let dto = req.parse_json::<SysPermissionEditDto>().await?;
        let current_user =depot.get_user()?;
        let option = SysPermissionService::select_by_id(id).await;
        let mut database_data = match option {
            None => {
                return Err(ResultError::resource_not_found("权限".to_string()));
            }
            Some(data) => {data}
        };
        database_data.update_by = current_user.user_name.clone();
        database_data.update_time = DateTime::now();
        database_data.name = dto.name;
        database_data.value = dto.value;
        database_data.p_id = dto.p_id;
        SYS_PERMISSION_CACHI.set(database_data.id.to_string().as_str(), database_data.clone()).await.ok();
        SysPermission::update_by_column(RB.deref(), &database_data, "id").await?;
        Ok(WebResult::success_none())
    }

    pub async fn add_sys_permission_by_id(req: &mut Request,depot: &mut Depot)->Http<String> {
        let current_user = depot.get_user()?;
        let dto = req.parse_json::<SysPermissionAddDto>().await?;
        let option = SysPermission::select_by_name(RB.deref(),dto.name.clone()).await?;
        if option.is_some() {
            return Err(ResultError::resource_exists("权限".to_string()))
        }
        let mut database_data = SysPermission::default();
        database_data.id = ID_WORKER.new_id();
        database_data.create_by = current_user.user_name.clone();
        database_data.create_time = DateTime::now();
        database_data.update_by = current_user.user_name.clone();
        database_data.update_time = DateTime::now();
        database_data.name = dto.name;
        database_data.value = dto.value;
        database_data.p_id = dto.p_id;
        SysPermission::insert(RB.deref(), &database_data).await?;
        Ok(WebResult::success(database_data.id.to_string()))
    }
}