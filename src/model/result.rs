use backtrace::Backtrace;
use deadpool_redis::PoolError;
use redis::RedisError;
use salvo::http::{ParseError, StatusCode};
use salvo::prelude::Json;
use salvo::{async_trait, Depot, Request, Response, Writer};
use serde::de::StdError;
use serde::{Deserialize, Serialize};
use std::string::ToString;
use tracing::error;
use validator::{ValidationErrors, ValidationErrorsKind};

#[derive(Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageDto {
    pub page: i64,
    pub page_size: i64,
}

impl PageDto {
    pub fn get_limit(&self) -> i64{
        let page = self.page;
        let page_size = self.page_size;
        let limit = (page - 1) * page_size;
        // let offset = page * page_size;
        limit
    }
}

#[derive(Debug,Serialize)]
pub struct WebResult<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
    pub flag: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WebResultPage<T: Send + Sync>{
    pub page_data:Vec<T>,
    pub page:i64,
    pub total:i64,
    pub page_size:i64
}
impl<V: Send + Sync> WebResultPage<V> {
    pub fn from<T: Send + Sync>(arg: WebResultPage<T>) -> Self
    where
        V: From<T>,
    {
        let mut page = WebResultPage::<V>{
            page_data: Vec::with_capacity(arg.page_data.len()),
            page: arg.page,
            total: arg.total,
            page_size: arg.page_size,
        };
        for x in arg.page_data {
            page.page_data.push(V::from(x));
        }
        page
    }
}


impl <T: Send + Sync> WebResult<WebResultPage<T>> {

    pub fn success_page(data:WebResultPage<T>) -> WebResult<WebResultPage<T>>{

        WebResult{
            code:20000,
            flag:true,
            data:Some(data),
            message:String::from("成功"),
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
    pub fn parameter_error(str:String) -> ResultError{
        ResultError::new(40001,str)
    }

}




impl From<Box<dyn StdError + Send + Sync>> for ResultError {
    fn from(err: Box<dyn StdError + Send + Sync>) -> Self {
        error!("sqlx error:{}",err);
        let backtrace = Backtrace::new();
        error!("error:{:?}",backtrace);
        ResultError::system_error()
    }
}
impl From<sqlx::Error> for ResultError {
    fn from(err: sqlx::Error) -> Self {
        error!("sqlx error:{:?}",err);
        let backtrace = Backtrace::new();
        error!("error:{:?}",backtrace);
        ResultError::system_error()
    }
}

impl From<ParseError> for ResultError{
    fn from(err: ParseError) -> Self {
        error!("ParseError error:{}",err);
        ResultError::param_error("解析参数失败".to_string())
    }
}

impl From<ValidationErrors> for ResultError{
    fn from(err: ValidationErrors) -> Self {
        error!("ValidationErrors error:{}",err);
         for (_, err) in err.errors() {
             let msg = match err {
                 ValidationErrorsKind::Struct(_) => {"".to_string()}
                 ValidationErrorsKind::List(_) => { "".to_string() }
                 ValidationErrorsKind::Field(msg) => {
                     msg[0].to_string()
                 }
             };
            return ResultError::param_error(msg);
        }
        // 如果没有找到任何错误，可以返回一个默认的 ResultError 或者 panic
        ResultError::param_error(err.to_string())
    }
}



impl From<PoolError> for ResultError{
    fn from(err: PoolError) -> Self {
        error!("PoolError error:{}",err);
        ResultError::system_error()
    }
}

impl From<RedisError> for ResultError{
    fn from(err: RedisError) -> Self {
        error!("RedisError error:{}",err);
        ResultError::system_error()
    }
}



#[async_trait]
impl Writer for ResultError {
    async fn write(self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        res.status_code = Some(StatusCode::OK);
        res.render(Json(WebResult::error(self.code,self.message.as_str())))
    }
}