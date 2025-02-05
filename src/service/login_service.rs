use crate::model::permission::SysPermissionVo;
use crate::model::result::{Http, ResultError, WebResult};
use crate::model::role::SysRoleVo;
use crate::model::user::{SysUser, SysUserVo, WxUserVo};
use crate::service::log_server::SysLogService;
use crate::service::permission_service::SysPermissionService;
use crate::service::role_service::SysRoleService;
use crate::task::sms_task::SmsMessage;
use crate::utils::captcha::CaptchaBuilder;
use crate::utils::db::Redis;
use crate::utils::vec::FromVo;
use crate::{get_sqlx_db, ID_WORKER, SERVER_CONFIG, SMS_SERVER};
use chrono::Local;
use crypto::digest::Digest;
use crypto::md5::Md5;
use lazy_static::lazy_static;
use rand::distributions::{Alphanumeric, Uniform};
use rand::{thread_rng, Rng};
use salvo::Request;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::{error, info};
use uuid::Uuid;

lazy_static! {
    pub static ref USER_LOGIN_CACHI: Redis<i64> = Redis::<i64>::new("UserLogin");
    // 短信验证
    pub static ref SMS_VERIFICATION_CODE_CACHI: Redis<String> = Redis::<String>::new("SmsVerificationCode");
    // 验证码
    pub static ref VERIFICATION_CODE_CACHI: Redis<String> = Redis::<String>::new("VerificationCode");
    // 登录失败次数
    pub static ref LOGIN_FAILURES_COUNT: Redis<u8> = Redis::<u8>::new("LoginFailuresCount");
}

// 登录失败次数后禁用账号
const LOGIN_FAILURES_COUNT_LIMIT: u8 = 5;

#[derive(Deserialize)]
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
pub struct WxLoginDto {
    pub code:String,
    pub p_code:Option<String>
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResultVo{
    pub token: String,
    pub user: SysUserVo,
    pub permissions: Vec<SysPermissionVo>,
    pub roles:Vec<SysRoleVo>
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WxLoginResultVo{
    pub token: String,
    pub user: WxUserVo,
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

#[derive(Deserialize)]
pub struct WxLoginResponse {
    pub openid: String,
    pub session_key: String,
    pub unionid: Option<String>,
    pub errcode: Option<i32>,
    pub errmsg: Option<String>
}

#[derive(Deserialize)]
pub struct WxGetAccessTokenResponse {
    pub access_token: String,
    pub expires_in: i32,
}
impl LoginService {

    fn generate_random_string(length: usize) -> String {
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(length)
            .map(char::from)
            .collect()
    }


    pub async fn code_to_open_id(code:&str)->Result<WxLoginResponse,ResultError>{
        let url = format!("https://api.weixin.qq.com/sns/jscode2session?appid={}&secret={}&js_code={}&grant_type=authorization_code",
                             SERVER_CONFIG.wx_config.app_id, SERVER_CONFIG.wx_config.secret, code);
        let resp = reqwest::get(url).await;
        info!("resp:{:?}",resp);
        let resp = match resp {
            Ok(resp) => {resp}
            Err(e) => {
                error!("resp:{:?}",e);
                return Err(ResultError::new(40000, "微信服务请求失败".to_string()));
            }
        };

        let result = resp.json::<WxLoginResponse>().await;
        let result = match result {
            Ok(result) => {result}
            Err(e) => {
                error!("resp:{:?}",e);
                return Err(ResultError::new(40000, "微信服务请求失败".to_string()));
            }
        };
        if result.errcode.is_some() {
            return Err(ResultError::new(40000, "微信服务请求失败".to_string()));
        }

        Ok(result)
    }


    pub async fn p_code_to_phone_number(p_code:&str)->Result<String,ResultError>{
        let get_token_url = format!("https://api.weixin.qq.com/cgi-bin/token?grant_type=client_credential&appid={}&secret={}",
                             SERVER_CONFIG.wx_config.app_id, SERVER_CONFIG.wx_config.secret);
        let resp = reqwest::get(get_token_url).await;
        let resp = match resp {
            Ok(resp) => {resp}
            Err(e) => {
                error!("resp:{:?}",e);
                return Err(ResultError::new(40000, "微信服务请求失败".to_string()));
            }
        };
        let result = resp.json::<WxGetAccessTokenResponse>().await;
        let result = match result {
            Ok(result) => {result}
            Err(e) => {
                error!("resp:{:?}",e);
                return Err(ResultError::new(40000, "微信服务请求失败".to_string()));
            }
        };
        let get_phone_number_url=format!("https://api.weixin.qq.com/wxa/business/getuserphonenumber?access_token={}",result.access_token);
        let resp = reqwest::Client::new().post(get_phone_number_url).json(&serde_json::json!({"code": p_code})).send().await;
        let resp = match resp {
            Ok(resp) => {resp}
            Err(e) => {
                error!("resp:{:?}",e);
                return Err(ResultError::new(40000, "微信服务请求失败".to_string()));
            }
        };

        let result = resp.json::<Value>().await;
        info!("p_code_to_phone_number wx_response:{:?}",result);
        let result = match result {
            Ok(result) => {result}
            Err(e) => {
                error!("resp:{:?}",e);
                return Err(ResultError::new(40000, "微信服务请求失败".to_string()));
            }
        };
        let i = result.get("errcode").unwrap().as_i64().unwrap();
        if i!=0 {
            return Err(ResultError::new(40000, "微信服务请求失败".to_string()));
        }
        let phone_number = result.get("phone_info").unwrap().get("phoneNumber").unwrap().as_str().unwrap().to_string();
        Ok(phone_number)
    }

    pub async fn wx_login(req: &mut Request) -> Http<WxLoginResultVo> {
        let dto = req.parse_json::<WxLoginDto>().await?;
        let response = LoginService::code_to_open_id(&dto.code).await?;
        let phone_number = if let Some(p_code) =  dto.p_code{
            let phone_number = LoginService::p_code_to_phone_number(&p_code).await?;
            Some(phone_number)
        }else{
            None
        };

        let user = SysUser::select_by_wx_open_id(&response.openid).await?;
        let random_id = ID_WORKER.new_id().to_string();
        let user = match user {
            None => {
                let name = format!("{}{}", "微信用户", random_id);
                let user = SysUser{
                    id: ID_WORKER.new_id() as i64,
                    name:name.clone(),
                    user_name: random_id,
                    phone_number,
                    password: "".to_string(),
                    enable: true,
                    gender: None,
                    is_del: false,
                    is_super_admin: false,
                    create_time:Local::now().to_utc(),
                    update_time:Local::now().to_utc(),
                    create_by: "sys".to_string(),
                    update_by: "sys".to_string(),
                    avatar: None,
                    wx_open_id: Some(response.openid),
                };

                user.update(&get_sqlx_db()).await?;

                user
            }
            Some(user) => {user}
        };
        if !user.enable {
            return Err(ResultError::param_error("用户被禁用,请联系管理员".to_string()));
        };
        let token = Uuid::new_v4().to_string();
        USER_LOGIN_CACHI.set_minute( token.as_str(), &user.id, 60*24).await?;
        SysLogService::add_login_log(user.user_name.clone(), req.remote_addr().to_string(),"小程序登录".to_string()).await;
        Ok(WebResult::success(WxLoginResultVo{
            token,
            user: WxUserVo {
                id: user.id,
                name:user.name,
                user_name: user.user_name,
                phone_number: user.phone_number,
                gender: user.gender,
                avatar_url: user.avatar
            }
        }))
    }

    // 后台支持两种方式登录 type =1 账号密码 ; type=2手机验证码
    pub async fn login(req: &mut Request) -> Http<LoginResultVo> {
        let dto = req.parse_json::<LoginDto>().await?;
        let user =if dto.login_type==1 {
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

            let user = SysUser::select_by_user_name(&dto.username).await?;

            let mut user = match user {
                None => { return Err(ResultError::new(40000, "用户名或密码错误".to_string()));}
                Some(user) => user
            };

            if !user.enable {
                return Err(ResultError::param_error("用户被禁用,请联系管理员".to_string()));
            };

            let mut md5 = Md5::new();
            md5.input_str(dto.password.as_str());
            if user.password.clone() != md5.result_str() {
                match LOGIN_FAILURES_COUNT.get(&user.user_name).await {
                    None => {
                        LOGIN_FAILURES_COUNT.set_minute(&user.user_name, &1,60 * 2).await?;
                    }
                    Some(count) => {
                        LOGIN_FAILURES_COUNT.set(&user.user_name, &(count+1)).await?;
                        if count+1 >= LOGIN_FAILURES_COUNT_LIMIT {
                            user.enable = false;
                            user.update(&get_sqlx_db()).await?;
                            return Err(ResultError::new(40000, "用户被禁用,请联系管理员".to_string()));
                        }

                    }
                }

                return Err(ResultError::new(40000, "用户名或密码错误".to_string()));
            }
            user
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
            let user = SysUser::select_by_user_name(&dto.username).await?;
            let user = match user {
                None => {
                    return Err(ResultError::new(40000, "账号未注册".to_string()))
                }
                Some(user) =>  user,
            };
            if !user.enable {
                return Err(ResultError::param_error("用户被禁用,请联系管理员".to_string()));
            };
            user
        }else {
            return Err(ResultError::param_error("登录方式错误".to_string()));
        };


        let user = SysUserVo::from(user.clone());
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
            let token = Uuid::new_v4().to_string();
            USER_LOGIN_CACHI.set_minute( token.as_str(), &user.id, 60*24).await?;
            SysLogService::add_login_log(user.user_name.clone(), req.remote_addr().to_string(),"登录后台".to_string()).await;
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
            let token = Uuid::new_v4().to_string();
            USER_LOGIN_CACHI.set_minute( token.as_str(), &user.id, 60*24).await?;
            SysLogService::add_login_log(
                user.user_name.clone(),
                req.remote_addr().to_string(),
                "登录后台".to_string()
            ).await;
            LOGIN_FAILURES_COUNT.remove(&user.user_name).await?;
            Ok(WebResult::success(LoginResultVo{
                token,
                user,
                permissions,
                roles,
            }))
        }
    }

    pub async fn logout(req: &mut Request) -> Http<String> {
        let token = req.header::<String>("Authorization");
        match token {
            None => {}
            Some(token) => {
                USER_LOGIN_CACHI.remove(&*token).await.ok();
            }
        }

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
        SMS_VERIFICATION_CODE_CACHI.set_second(dto.phone_number.as_str(), &code,120).await.ok();
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

        let uuid = Uuid::new_v4().to_string();
        VERIFICATION_CODE_CACHI.set_minute(uuid.as_str(), &captcha.text, 2).await?;
        Ok(WebResult::success(CaptchaVo{ uuid, img: captcha.to_base64() }))
    }
}