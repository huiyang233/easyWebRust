use salvo::{handler, Depot, FlowCtrl, Request, Response};

use crate::model::permission::SysPermission;
use crate::model::result::ResultError;
use crate::model::user::SysUser;
use crate::service::login_service::USER_LOGIN_CACHI;
use crate::service::permission_service::SysPermissionService;
use crate::service::user_service::UserService;

///
///## 权限验证中间件
///验证成功后会给用户信息放到 depot 中; `depot.obtain::<SysUser>();` 可以取出来
///
#[handler]
pub async fn auth_check(req: &mut Request, depot: &mut Depot, _res: &mut Response, ctrl: &mut FlowCtrl) ->Result<(),ResultError> {
    let token = req.header::<String>("Authorization");
    // 判断有没有token
    let token = match token {
        None => {
            ctrl.skip_rest();
            return Err(ResultError::token_not_found())
        }
        Some(token) => {token}
    };
    // 判断token格式
    let token = if token.starts_with("Bearer ") {
        token.replace("Bearer ", "")
    } else {
        ctrl.skip_rest();
        return Err(ResultError::token_error())
    };
    let user_id = USER_LOGIN_CACHI.get(token.as_str()).await;
    let x = req.headers_mut();

    let user_id = match user_id {
        None => {
            ctrl.skip_rest();
            return Err(ResultError::token_error())
        }
        Some(data) => {
            x.insert("user_id", data.to_string().parse().unwrap());
            data
        }
    };
    // 查数据库
    let user_info = UserService::get_user_by_id(user_id).await;
    let user_info= match user_info {
        None => {
            ctrl.skip_rest();
            return Err(ResultError::user_not_found())
        }
        Some(user_info) => {
            user_info
        }
    };
    if user_info.enable{
        depot.inject(user_info);
        Ok(())
    }else{
        ctrl.skip_rest();
        Err(ResultError::user_not_enable())
    }
}

pub async fn get_user(token:Option<String>)->Option<SysUser>{
    let token = match token {
        None => {
          return None
        }
        Some(token) => {token}
    };
    // 判断token格式
    let token = if token.starts_with("Bearer ") {
        token.replace("Bearer ", "")
    } else {
       return None
    };
    let user_id = USER_LOGIN_CACHI.get(token.as_str()).await;

    let user_id = match user_id {
        None => {
            return None
        }
        Some(data) => {
            data
        }
    };
    // 查数据库
    let user_info = UserService::get_user_by_id(user_id).await;
    user_info

}

pub trait AuthCheck {
    fn get_user(&self)->Result<&SysUser,ResultError>;
    async fn get_permission(&self) ->Result<Vec<SysPermission>,ResultError>;
    async fn check_permission(&self,permissions: &[&str])->Result<(),ResultError>;
    async fn check_any_permission(&self,permissions: &[&str]) -> Result<(), ResultError>;
}

#[derive(Clone)]
pub struct AuthDetails<T = String>
    where
        T: PartialEq,
{
    pub permissions: Vec<T>,
}

impl<T> AuthDetails<T>
    where
        T: PartialEq + Clone,
{
    pub fn new(permissions: Vec<T>) -> AuthDetails<T> {
        AuthDetails { permissions }
    }
}

pub trait PermissionsCheck<T: PartialEq> {
    fn has_permission(&self, permission: T) -> bool;
    fn has_permissions(&self, permissions: &[T]) -> bool;
    fn has_any_permission(&self, permissions: &[T]) -> bool;
}

impl<T: PartialEq + Clone> PermissionsCheck<&T> for AuthDetails<T> {
    fn has_permission(&self, permission: &T) -> bool {
        self.permissions.iter().any(|auth| auth == permission)
    }

    fn has_permissions(&self, permissions: &[&T]) -> bool {
        permissions.iter().all(|auth| self.has_permission(auth))
    }

    fn has_any_permission(&self, permissions: &[&T]) -> bool {
        permissions.iter().any(|auth| self.has_permission(auth))
    }
}

impl PermissionsCheck<&str> for AuthDetails {
    fn has_permission(&self, permission: &str) -> bool {
        self.permissions
            .iter()
            .any(|auth| auth.as_str() == permission)
    }

    fn has_permissions(&self, permissions: &[&str]) -> bool {
        permissions.iter().all(|auth| self.has_permission(*auth))
    }

    fn has_any_permission(&self, permissions: &[&str]) -> bool {
        permissions.iter().any(|auth| self.has_permission(*auth))
    }
}

/// ## 权限校验
/// 对 Depot 实现 AuthCheck
/// 主要作用是取用户和权限校验
/// 在 handler 中获取到 `depot: &mut Depot` 然后 `depot.check_permission(&["agent_info"]).await?;`
///
impl AuthCheck for Depot {
    fn get_user(&self)->Result<&SysUser,ResultError> {
        let user = self.obtain::<SysUser>();
        user.map(|user| {
            return Ok(user);
        }).unwrap_or_else(|_| {
            return Err(ResultError::token_not_valid());
        })
    }

    async fn get_permission(&self)->Result<Vec<SysPermission>,ResultError>{
        let user = self.get_user()?;
        let permission_list = SysPermissionService::select_by_user_id(user.id).await;
        let permission_list = match permission_list {
            None => {
                return Err(ResultError::param_error("此用户找不到权限".to_string()))
            }
            Some(permission_list) => {
                permission_list
            }
        };

        Ok(permission_list)
    }

    /// 必须得拥有全部权限才行
    async fn check_permission(&self,permissions: &[&str]) -> Result<(), ResultError> {
        let current_user = self.get_user()?;
        if current_user.is_super_admin { return Ok(()) }
        if !current_user.enable { return Err(ResultError::user_not_enable()) }
        let database_permission_list =self.get_permission().await?;
        let mut permission_list:Vec<String> = vec![];
        for sys_permission in database_permission_list {
            permission_list.push(sys_permission.value)
        }
        let details = AuthDetails::new(permission_list);
        if !details.has_permissions(permissions) {
            return Err(ResultError::not_permission())
        }
        Ok(())
    }

    /// 拥有其中一个权限就行
    async fn check_any_permission(&self,permissions: &[&str]) -> Result<(), ResultError> {
        let current_user = self.get_user()?;
        if current_user.is_super_admin { return Ok(()) }
        if !current_user.enable { return Err(ResultError::user_not_enable()) }
        let database_permission_list =self.get_permission().await?;
        let mut permission_list:Vec<String> = vec![];
        for sys_permission in database_permission_list {
            permission_list.push(sys_permission.value)
        }

        let details = AuthDetails::new(permission_list);
        if !details.has_any_permission(permissions) {
            return Err(ResultError::not_permission())
        }
        Ok(())
    }
}