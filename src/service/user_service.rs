use std::ops::Deref;

use crypto::digest::Digest;
use crypto::md5::Md5;
use lazy_static::lazy_static;
use rbatis::rbdc::DateTime;
use rbatis::{Page, PageRequest};
use salvo::{Depot, Request};
use tracing::{error, info};

use crate::auth::auth_check::AuthCheck;
use crate::model::result::{Http, HttpPage, PageDto, ResultError, WebResult};
use crate::model::role::{SysRole, SysRoleVo};
use crate::model::user::{ChangePasswordDto, ChangeUserInfoDto, SysUser, UserAddDto, UserEditDto, UserVo};
use crate::service::role_service::{SysRoleService, USER_SYS_ROLE_CACHI};
use crate::utils::db::{Redis, DB};
use crate::utils::vec::FromVo;
use crate::{ID_WORKER, RB};

lazy_static! {
    static ref SYS_USER_CACHI: Redis<SysUser> = Redis::<SysUser>::new("SysUser");
}
pub struct UserService;
impl UserService{
    pub async fn get_user_by_id(id: u64) -> Option<SysUser> {
        let option = SYS_USER_CACHI.get(id.to_string().as_str()).await;
        match option {
            None => {
                let sys_user = SysUser::select_by_id(RB.deref(),&id).await;
                match sys_user {
                    Ok(sys_user) => {
                        match sys_user {
                            None => {None}
                            Some(sys_user) => {
                                SYS_USER_CACHI.set_minute(id.to_string().as_str(), sys_user.clone(),3).await.ok();
                                Some(sys_user)
                            }
                        }
                    }
                    Err(_) => { None }
                }
            }
            Some(sys_user) => {
                SYS_USER_CACHI.extend_out_time_minute(id.to_string().as_str(), 3).await.ok();
                Some(sys_user)

            }
        }
    }

    pub async fn get_user_details(req: &mut Request)->Http<UserVo>{
        let id = req.param::<u64>("id");
        match id {
            None => {
                Err(ResultError::param_error("id不能为空".to_string()))
            }
            Some(id) => {
                match UserService::get_user_by_id(id).await {
                    None => {
                        Err(ResultError::user_not_found())
                    }
                    Some(user) => {
                        let mut vo = UserVo::from(user);
                        let option = SysRoleService::select_by_user_id(vo.id).await;
                        vo.roles = Vec::<SysRoleVo>::from_vo(option.unwrap());
                        Ok(WebResult::success(vo))
                    }
                }

            }
        }

    }

    pub async fn get_user_by_page(req: &mut Request)->HttpPage<UserVo>{
        let page_dto = req.parse_queries::<PageDto>().unwrap_or_else(|_| PageDto{ page: 1, page_size: 10});
        let page_request = PageRequest::from(page_dto);
        let user_name = req.query::<String>("userName");
        let phone_number = req.query::<String>("phoneNumber");
        let user_page = SysUser::select_page_by_user_name(RB.deref(), &page_request, user_name,phone_number).await?;
        let mut page_vo = Page::<UserVo>::from(user_page);
        for  x in &mut page_vo.records {
            let option = SysRoleService::select_by_user_id(x.id).await;
            let vec = Vec::<SysRoleVo>::from_vo(option.unwrap());
            x.roles = vec;
        }

        Ok(WebResult::success_page(page_vo))
    }

    pub async fn delete_user_by_id(req: &mut Request)->Http<String> {
        let id = req.param::<u64>("id");

        match id {
            None => {
                Err(ResultError::param_error("id不能为空".to_string()))
            }
            Some(id) => {
                if id==1 {
                    return return Err(ResultError::not_operation_admin())
                }
                SysUser::delete_by_id(RB.deref(), &id).await?;
                SYS_USER_CACHI.remove(id.to_string().as_str()).await.ok();
                Ok(WebResult::success_none())
            }
        }
    }

    pub async fn edit_user_by_id(req: &mut Request,depot: &mut Depot)->Http<String> {
        let current =depot.get_user()?;
        let user = req.parse_json::<UserEditDto>().await?;
        info!("user:{:?}",&user);
        if user.id==1 {
            return Err(ResultError::not_operation_admin())
        }
        let option = UserService::get_user_by_id(user.id).await;
        let mut database_user = match option {
            None => {
                return Err(ResultError::user_not_found())
            }
            Some(user) => {user}
        };
        let role_ids= user.role_ids.unwrap_or_else(|| {
            vec![]
        });

        let mut role_vec = vec![];
        for x in &role_ids {
            let role = SysRoleService::select_by_id(x.clone()).await;
            match role {
                None => {
                    return Err(ResultError::resource_not_found(format!("角色ID:{}",x)));
                }
                Some(role) => {role_vec.push(role);}
            }
        }
        // 有密码然后加密
        user.password.map(|password| {
            let mut md5 = Md5::new();
            md5.input_str(password.as_str());
            database_user.password = Some(md5.result_str());
        });

        user.enable.map(|x| {
            database_user.enable = x;
        });

        user.phone_number.map(|x| {
            database_user.phone_number = Some(x)
        }) ;
        user.gender.map(|x| {
            database_user.gender = Some(x);
        });
        user.name.map(|x| {
            database_user.name = Some(x);
        });

        database_user.update_by = current.user_name.clone();
        database_user.update_time = DateTime::now();
        let mut tx = DB::get_transaction().await;
        if !role_ids.is_empty() {
            SysRole::delete_role_by_user_id(&mut tx,&database_user.id).await?;
            SysRole::insert_list(&mut tx, &role_ids,&database_user.id).await?;

        }
        SysUser::update_by_column(&mut tx, &database_user, "id").await?;
        tx.commit().await?;
        // if !role_ids.is_empty() {
        //     USER_SYS_ROLE_CACHI.set_minute(database_user.id.to_string().as_str(),role_vec,60).await;
        // }
        SYS_USER_CACHI.set_minute(database_user.id.to_string().as_str(), database_user.clone(),60).await.ok();

        Ok(WebResult::success_none())
    }

    pub async fn add_user_by_id(req: &mut Request,depot: &mut Depot)->Http<String> {
        let current = depot.get_user()?;
        let user = req.parse_json::<UserAddDto>().await?;
        let option = SysUser::select_by_user_name(RB.deref(),user.user_name.clone().as_str()).await?;
        if option.is_some() {
            return Err(ResultError::resource_exists("用户".to_string()))
        }
        let role_ids=  match user.role_ids {
            None => {
                return Err(ResultError::param_error("缺少角色id".to_string()))
            }
            Some(role_ids) => role_ids
        };

        let mut role_vec = vec![];
        for x in &role_ids {
            let role = SysRoleService::select_by_id(x.clone()).await;
            match role {
                None => {
                    return Err(ResultError::resource_not_found(format!("角色ID:{}",x)));
                }
                Some(role) => {role_vec.push(role);}
            }
        }

        let mut database_user = SysUser::default();

        database_user.user_name = user.user_name;
        database_user.phone_number = user.phone_number;
        database_user.id = ID_WORKER.new_id();
        // 有密码然后加密
        let mut md5 = Md5::new();
        md5.input_str(user.password.as_str());
        database_user.password = Some(md5.result_str());
        database_user.enable = user.enable;
        database_user.gender = user.gender;
        database_user.is_super_admin = user.is_super_admin;
        database_user.update_by = current.user_name.clone();
        database_user.update_time = DateTime::now();
        database_user.create_by = current.user_name.clone();
        database_user.create_time = DateTime::now();
        database_user.is_del = false;
        database_user.avatar = user.avatar;
        let mut tx = DB::get_transaction().await;
        SysRole::insert_list(&mut tx, &role_ids,&database_user.id).await?;
        SysUser::insert(RB.deref(), &database_user).await?;
        tx.commit().await?;
        // if !role_ids.is_empty() {
        //     USER_SYS_ROLE_CACHI.set_minute(database_user.id.to_string().as_str(), role_vec,60).await;
        // }
        Ok(WebResult::success(database_user.id.to_string()))
    }
    // 用户修改密码的逻辑
    pub async fn change_password(req: &mut Request,depot: &mut Depot)->Http<String> {
        let mut current = depot.get_user()?.clone();
        let user = req.parse_json::<ChangePasswordDto>().await?;

        if user.old_password.is_some() {
            let mut md5 = Md5::new();
            md5.input_str(user.old_password.unwrap().as_str());
            if md5.result_str() != current.password.unwrap() {
                return Err(ResultError::param_error("旧密码错误".to_string()))
            }
        }else{
            return Err(ResultError::param_error("旧密码不能为空".to_string()))
        }

        if user.new_password.is_some() {
            let mut md5 = Md5::new();
            md5.input_str(user.new_password.unwrap().as_str());
            current.password = Some(md5.result_str());
        }else{
            return Err(ResultError::param_error("新密码不能为空".to_string()))
        }
        // 更新user进数据库
        SysUser::update_by_column(RB.deref(), &current, "id").await?;
        SYS_USER_CACHI.set_minute(current.id.to_string().as_str(), current,60).await.ok();
        Ok(WebResult::success_none())

    }

    // 用户修改自己信息的逻辑，可以修改名字，头像，性别，手机号信息
    pub async fn change_user_info(req: &mut Request,depot: &mut Depot)->Http<String> {
        let mut current = depot.get_user()?.clone();
        let user = req.parse_json::<ChangeUserInfoDto>().await?;
        if user.name.is_some() {
            current.name = user.name;
        }
        if user.gender.is_some() {
            current.gender = user.gender;
        }
        if user.phone_number.is_some() {
            current.phone_number = user.phone_number;
        }
        if user.avatar.is_some() {
            current.avatar = user.avatar;
        }
        SysUser::update_by_column(RB.deref(), &current, "id").await?;
        SYS_USER_CACHI.set_minute(current.id.to_string().as_str(), current,60).await.ok();
        Ok(WebResult::success_none())
    }

}

