use deadpool_redis::PoolError;
use rbatis::{Page, PageRequest};
use redis::RedisError;
use salvo::http::{ParseError, StatusCode};
use salvo::prelude::Json;
use salvo::{async_trait, Depot, Request, Response, Writer};
use serde::{Deserialize, Serialize};
use std::string::ToString;
use tracing::error;

#[derive(Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageDto {
    pub page: u64,
    pub page_size: u64,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct WebResult<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
    pub flag: bool,
}

#[derive(Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebResultPage<T: Send + Sync>{
    page_data:Vec<T>,
    page:u64,
    total:u64,
    page_size:u64
}

impl <T: Send + Sync> WebResult<Page<T>> {
    pub fn success_page(data:Page<T>) -> WebResult<WebResultPage<T>>{

        let page = Some(WebResultPage {
            page_data: data.records,
            page: data.page_no,
            total: data.total,
            page_size: data.page_size,
        });
        WebResult{
            code:20000,
            flag:true,
            data:page,
            message:String::from("成功")
        }
    }
}

#[async_trait]
impl<T:Serialize+Send> Writer for WebResult<T> {
    async fn write(self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        res.status_code = Some(StatusCode::OK);
        res.render(Json(self))
    }
}

pub type HttpPage<T> =  Result<WebResult<WebResultPage<T>>, ResultError>;

pub type Http<T> =  Result<WebResult<T>, ResultError>;
impl WebResult<String>{
    pub fn error_str(code:i32,message:String) -> WebResult<String>{
        WebResult{
            code,
            flag:false,
            data:None,
            message
        }
    }

    pub fn error(code:i32,message:&str) -> WebResult<String>{
        WebResult{
            code,
            flag:false,
            data:None,
            message:message.to_string()
        }
    }
}

impl <T> WebResult<T> {

    pub fn success_none() -> WebResult<T>{
        WebResult{
            code:20000,
            flag:true,
            data:None,
            message:String::from("成功")
        }
    }
    pub fn success(data:T) -> WebResult<T>{
        WebResult{
            code:20000,
            flag:true,
            data:Some(data),
            message:String::from("成功")
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ResultError {
    pub code: i32,
    pub message: String
}
impl ResultError{
    pub fn new(code:i32,message:String) -> ResultError{
        ResultError{
            code,
            message
        }
    }

    pub fn system_error() -> ResultError{
        ResultError::new(50000,"系统异常".to_string())
    }

    pub fn not_found() -> ResultError{
        ResultError::new(40004,"找不到页面".to_string())
    }

    pub fn token_expire() -> ResultError{
        ResultError::new(30001,"token过期".to_string())
    }

    pub fn token_not_found() -> ResultError{
        ResultError::new(30002,"用户未登录".to_string())
    }

    pub fn token_error() -> ResultError{
        ResultError::new(30003,"token错误".to_string())
    }

    pub fn user_not_found() -> ResultError{
        ResultError::new(30004,"找不到此用户".to_string())
    }

    pub fn not_operation_admin() -> ResultError{
        ResultError::new(30007,"不能操作管理员".to_string())
    }

    pub fn token_not_valid() -> ResultError{
        ResultError::new(30005,"token未验证".to_string())
    }

    pub fn user_not_enable() -> ResultError{
        ResultError::new(30006,"用户未启用".to_string())
    }
    //参数解析错误
    pub fn param_error(message:String) -> ResultError{
        ResultError::new(40000,message)
    }

    pub fn resource_not_found(msg:String) -> ResultError{
        ResultError::new(40000,msg+"找不到")
    }

    pub fn resource_exists(msg:String) -> ResultError{
        ResultError::new(40000,msg+"已经存在")
    }
    //没有权限
    pub fn not_permission() -> ResultError{
        ResultError::new(40000,"没有权限".to_string())
    }

}

impl From<rbatis::rbdc::Error> for ResultError {
    fn from(err: rbatis::rbdc::Error) -> Self {
        error!("rbatis error:{}",err);
        ResultError::system_error()
    }
}

impl From<ParseError> for ResultError{
    fn from(value: ParseError) -> Self {
        ResultError::param_error(value.to_string())
    }
}

impl From<PoolError> for ResultError{
    fn from(value: PoolError) -> Self {
        ResultError::param_error(value.to_string())
    }
}

impl From<RedisError> for ResultError{
    fn from(value: RedisError) -> Self {
        ResultError::param_error(value.to_string())
    }
}



impl From<PageDto> for PageRequest{
    fn from(value: PageDto) -> Self {
        Self::new(value.page,value.page_size)
    }
}



#[async_trait]
impl Writer for ResultError {
    async fn write(self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        res.status_code = Some(StatusCode::OK);
        res.render(Json(WebResult::error(self.code,self.message.as_str())))
    }
}