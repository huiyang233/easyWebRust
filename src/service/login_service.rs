use std::ops::Deref;

use crypto::digest::Digest;
use crypto::md5::Md5;
use lazy_static::lazy_static;
use rand::distributions::Uniform;
use rand::Rng;
use rbatis::rbdc::Uuid;
use salvo::Request;
use serde::{Deserialize, Serialize};

use crate::model::permission::SysPermissionVo;
use crate::model::result::{Http, ResultError, WebResult};
use crate::model::role::SysRoleVo;
use crate::model::user::{SysUser, UserVo};
use crate::service::log_server::SysLogService;
use crate::service::permission_service::SysPermissionService;
use crate::service::role_service::SysRoleService;
use crate::task::sms_task::SmsMessage;
use crate::utils::captcha::CaptchaBuilder;
use crate::utils::db::Redis;
use crate::utils::vec::FromVo;
use crate::{RB, SMS_SERVER};

lazy_static! {
    pub static ref USER_LOGIN_CACHI: Redis<u64> = Redis::<u64>::new("UserLogin");
    // 短信验证
    pub static ref SMS_VERIFICATION_CODE_CACHI: Redis<String> = Redis::<String>::new("SmsVerificationCode");
    // 验证码
    pub static ref VERIFICATION_CODE_CACHI: Redis<String> = Redis::<String>::new("VerificationCode");
}

#[derive(Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginDto {
    pub verification_code_uuid: Option<String>,
    pub verification_code: Option<String>,
    // 登录方式 1=账号密码登录 2=短信验证码登录
    pub login_type: u8,
    pub username:String,
    pub password:String
}

#[derive(Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResultVo{
    pub(crate) token: String,
    pub(crate) user: UserVo,
    pub(crate) permissions: Vec<SysPermissionVo>,
    pub(crate) roles:Vec<SysRoleVo>
}

#[derive(Serialize,Deserialize)]
pub struct CaptchaVo {
    pub(crate) uuid: String,
    pub(crate) img: String,
}

#[derive(Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSmsVerificationCodeDto{
    pub(crate) phone_number: String,
}

pub struct LoginService;
impl LoginService {

    pub async fn wx_login(req: &mut Request) -> Http<LoginResultVo> {
        let dto = req.parse_json::<LoginDto>().await?;
        Err(ResultError::param_error("用户被禁用,请联系管理员".to_string()))
    }
    pub async fn login(req: &mut Request) -> Http<LoginResultVo> {

        let dto = req.parse_json::<LoginDto>().await?;
        let mut user = None;
        if dto.login_type==1 {
            if dto.verification_code.is_none() || dto.verification_code_uuid.is_none() {
                return Err(ResultError::new(40000, "缺少验证码的信息".to_string()));
            }
            let verification_code = VERIFICATION_CODE_CACHI.get(&dto.verification_code_uuid.clone().unwrap()).await;
            if verification_code.is_none() {
                return Err(ResultError::param_error("验证码已失效,请重新获取".to_string()));
            }
            if verification_code.unwrap().to_lowercase() != dto.verification_code.unwrap().to_lowercase() {
                return Err(ResultError::param_error("验证码错误".to_string()));
            }
            VERIFICATION_CODE_CACHI.remove(&dto.verification_code_uuid.clone().unwrap()).await.ok();

            user = SysUser::select_by_user_name(RB.deref(), &dto.username).await?;
            let user = match user {
                None => { return Err(ResultError::new(40000, "用户名或密码错误".to_string()));}
                Some(ref user) => user
            };
            let mut md5 = Md5::new();
            md5.input_str(dto.password.as_str());
            if user.password.clone().unwrap() != md5.result_str() {
                return Err(ResultError::new(40000, "用户名或密码错误".to_string()));
            }
        }else if dto.login_type==2 {
            let sms_verification_code = SMS_VERIFICATION_CODE_CACHI.get(dto.username.as_str()).await;
            let sms_verification_code = match sms_verification_code {
                None => {
                    return Err(ResultError::param_error("未找到验证码,请重新获取".to_string()));
                }
                Some(sms_verification_code) => sms_verification_code
            };
            if dto.password.clone().to_lowercase() != sms_verification_code {
                return Err(ResultError::param_error("验证码错误".to_string()));
            }
            user = SysUser::select_by_user_name(RB.deref(), &dto.username).await?;

        }else {
            return Err(ResultError::param_error("登录方式错误".to_string()));
        }

        let user = user.unwrap();
        if !user.enable {
            return Err(ResultError::param_error("用户被禁用,请联系管理员".to_string()));
        };

        let user = UserVo::from(user.clone());
        if user.is_super_admin {
            // 超级管理员有所有权限
            let permissions = SysPermissionService::select_all().await;
            let permissions = match permissions {
                None => {
                    return Err(ResultError::param_error("找不到此用户的权限".to_string()));
                }
                Some(permission) => {permission}
            };
            let permissions = Vec::<SysPermissionVo>::from_vo(permissions);
            let token = Uuid::new().to_string();
            USER_LOGIN_CACHI.set_minute( token.as_str(), user.id, 60*24).await.ok();
            SysLogService::add_login_log(user.user_name.clone(), req.remote_addr().to_string()).await;
            Ok(WebResult::success(LoginResultVo{
                token,
                user,
                permissions,
                roles: vec![SysRoleVo{
                    id: 0,
                    create_by: "".to_string(),
                    create_time: Default::default(),
                    name: "超级管理员".to_string(),
                    permissions:vec![],
                }],
            }))
        }else{
            let role = SysRoleService::select_by_user_id(user.id).await;
            let role = match role {
                None => {
                    return Err(ResultError::param_error("找不到此用户的角色".to_string()));
                }
                Some(role) => {role}
            };
            let roles = Vec::<SysRoleVo>::from_vo(role);

            let permissions = SysPermissionService::select_by_user_id( user.id).await;
            let permissions = match permissions {
                None => {
                    return Err(ResultError::param_error("找不到此用户的权限".to_string()));
                }
                Some(permission) => {permission}
            };
            let permissions = Vec::<SysPermissionVo>::from_vo(permissions);
            let token = Uuid::new().to_string();
            USER_LOGIN_CACHI.set_minute( token.as_str(), user.id, 60*24).await.ok();
            SysLogService::add_login_log(
                user.user_name.clone(),
                req.remote_addr().to_string()
            ).await;
            Ok(WebResult::success(LoginResultVo{
                token,
                user,
                permissions,
                roles,
            }))
        }
    }

    pub async fn logout(req: &mut Request) -> Http<String> {
        let token = req.header::<String>("Authorization").unwrap();
        USER_LOGIN_CACHI.remove(&*token).await.ok();
        Ok(WebResult::success("退出成功".to_string()))
    }

    pub async fn send_sms_verification_code(req: &mut Request) -> Http<String> {
        let dto = req.parse_json::<GetSmsVerificationCodeDto>().await?;
        // 生成6位数字的验证码
        let die_range = Uniform::new_inclusive(0, 9);
        let code = rand::thread_rng()
            .sample_iter(&die_range)
            .take(6)
            .map(|n| (b'0' + n as u8) as char)
            .collect::<String>();
        SMS_VERIFICATION_CODE_CACHI.set_second(dto.phone_number.as_str(), code.clone(),120).await.ok();
        // 改成redis后记得加判断，不然小心短信接口欠费；
        SMS_SERVER.send_sms(SmsMessage{ phone_number: dto.phone_number, code }).await;
        Ok(WebResult::success_none())
    }

    pub async fn get_verification_code() -> Http<CaptchaVo> {
        let captcha = CaptchaBuilder::new()
            .length(4)
            .width(120)
            .height(50)
            .dark_mode(false)
            .complexity(1) // min: 1, max: 10
            .compression(40) // min: 1, max: 99
            .build();

        let uuid = Uuid::new().to_string();
        VERIFICATION_CODE_CACHI.set_minute(uuid.as_str(), captcha.text.clone(), 2).await.ok();
        Ok(WebResult::success(CaptchaVo{ uuid, img: captcha.to_base64() }))
    }
}