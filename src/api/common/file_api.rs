use crate::auth::auth_check::{auth_check, AuthCheck};
use crate::model::result::{Http, ResultError, WebResult};
use crate::model::sys_file::{SaveType, SysFile, SysFileVo};
use crate::{get_sqlx_db, ID_WORKER, SERVER_CONFIG};
use chrono::Local;
use crypto::digest::Digest;
use crypto::md5::Md5;
use rimage::config::{Codec, EncoderConfig};
use rimage::{Decoder, Encoder};
use salvo::http::Body;
use salvo::{handler, Depot, Request, Response, Router};
use std::fs::File;
use std::path::Path;
use tokio::fs;
use tracing::{error, info};
use uuid::Uuid;

pub fn init_file_router() -> Router{
     Router::new()
        .push(
            Router::with_path("file/image/{**file_path}").get(get_file)
        )
        .push(
            Router::with_path("file/image/upload").hoop(auth_check).post(put_image_file)
        )
}


#[handler]
pub async fn get_file(req: &mut Request, res: &mut Response){
    let url = req.param::<String>("file_path");

    info!("{:?}", req.params());
    // NamedFile::builder("/file/to/path").attached_name("image.png").send(req.headers(), res).await;
    info!("{:?}{:?}",SERVER_CONFIG.file.file_path,url);
    res.send_file(format!("{}{}", SERVER_CONFIG.file.file_path, url.unwrap()),req.headers()).await;
}
#[handler]
pub async fn put_image_file(req: &mut Request,depot: &mut Depot)->Http<SysFileVo>{
    // 检查包大小
    let size_hint = req.body().size_hint().upper();
    if let Some(upper) = size_hint {
        if upper > 1024 * 1024 * 10 {
            return Err(ResultError::new(40000,"上传文件超过10M".to_string()));
        }
    } else {
        return Err(ResultError::new(40000,"未知的文件大小".to_string()));
    }
    let current_user = depot.get_user()?;
    let file = req.file("file").await;

    let file = match file {
        None => {return  Err(ResultError::new(40000,"上传失败".to_string()))}
        Some(file) => file
    };

    // 路径格式为IMAGE_PATH+yyyymmdd+filename
    let file_name = file.name().unwrap_or("file");
    let path = Path::new(&file_name);
    let extension = match path.extension() {
        None => {
            return Err(ResultError::new(40000,"文件没有后缀名".to_string()));
        }
        Some(e) => {
             e.to_str().unwrap()
        }
    };

    if !matches!(extension, "jpg" | "png" | "jpeg" | "gif" | "bmp") {
        return Err(ResultError::new(40000,"不支持的文件格式".to_string()));
    }
    let date = Local::now().format("%Y-%m-%d");
    let file_path = format!("{}/{}.jpg", date,Uuid::new_v4().to_string());
    let dest = format!("{}{}",SERVER_CONFIG.file.file_path ,&file_path);
    let exists = Path::new(format!("{}/{}", SERVER_CONFIG.file.file_path, &date).as_str()).exists();
    if ! exists {
        if let Err(e) = fs::create_dir_all(format!("{}/{}",SERVER_CONFIG.file.file_path,&date)).await {
            error!("无法创建目录: {}", e);
            return Err(ResultError::new(40000, "上传失败".to_string()));
        }
    }

    // 图片压缩
    let decoder = match Decoder::from_path(&file.path()) {
        Ok(decoder) => {decoder}
        Err(e) => {
            error!("读取图片失败: {}", e);
            return Err(ResultError::new(40000, "上传失败".to_string()));
        }
    };
    let image = match decoder.decode() {
        Ok(image) => image,
        Err(e) => {
            error!("读取图片失败: {}", e);
            return Err(ResultError::new(40000, "上传失败".to_string()));
        }
    };

    let config = EncoderConfig::new(Codec::MozJpeg).with_quality(50.0).unwrap();
    let image_file = match File::create(&dest) {
        Ok(file) => {file}
        Err(e) => {
            error!("创建文件失败: {}", e);
            return Err(ResultError::new(40000, "上传失败".to_string()));
        }
    };

    let encoder = Encoder::new(image_file, image).with_config(config);
    if let Err(e) = encoder.encode() {
        error!("error:{}",e);
        return Err(ResultError::new(40000,"上传失败".to_string()))
    }

    let mut md5 = Md5::new();
    // 读取文件进来md5
    let result = match std::fs::read(&file.path()) {
        Ok(result) => result,
        Err(_) => {
            return Err(ResultError::new(40000,"上传失败".to_string()));
        }
    };
    md5.input(&result);
    info!("{:?}",file.path().to_str());
    let file = SysFile {
        id: ID_WORKER.new_id() as i64,
        create_by: current_user.user_name.clone(),
        create_time: Local::now().to_utc(),
        update_by: current_user.user_name.clone(),
        update_time: Local::now().to_utc(),
        file_path: dest,
        name: file_name.to_string(),
        size: file.size().to_string(),
        suffix: extension.to_string(),
        save_type: SaveType::Local,
        url_path: format!("{}file/image/{}", SERVER_CONFIG.file.url, file_path),
        md5: md5.result_str(),
        is_del: false,
    };
    file.insert(&get_sqlx_db()).await?;
    Ok(WebResult::success(SysFileVo::from(file)))


}


