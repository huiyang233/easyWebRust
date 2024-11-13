use salvo::{handler, Request, Response, Router};
use tracing::info;

use crate::auth::auth_check::auth_check;

pub fn init_file_router() -> Router{
    Router::new()
        .push(
            Router::with_path("file")
                .push(
                    Router::new().path("image").push(
                        Router::new().path("<**file_path>").get(get_file)
                    )
                )

        )
}

static IMAGE_PATH: &str = "/Users/yanghui/Desktop/disk/image/";
static URL: &str = "http://localhost:8083/";
#[handler]
pub async fn get_file(req: &mut Request, res: &mut Response){
    let url = req.param::<String>("**file_path");

    info!("{:?}", req.params());
    // NamedFile::builder("/file/to/path").attached_name("image.png").send(req.headers(), res).await;
    info!("{:?}{:?}",IMAGE_PATH,url);
    res.send_file(format!("{}{}", IMAGE_PATH, url.unwrap()),req.headers()).await;
}

