use crate::auth::auth_check::{auth_check, AuthCheck};
use crate::utils::sse::{SSEMessage, SseEvent, SseKeepAlive};
use reqwest::StatusCode;
use salvo::{handler, Depot, Request, Response, Router};
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::LazyLock;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::{mpsc, RwLock};
use tracing::{error, info};
///
/// 发给某个用户
/// SSE::send_message_to_user(user_id,"hello")
///
/// 发给所有在线的用户
/// SSE::send_message_to_all("广播")
///

pub fn init_u_sse_router() -> Router{
    Router::with_path("sse").post(send).hoop(auth_check).get(user_connected)
}




type Users = RwLock<HashMap<usize, UnboundedSender<SSEMessage>>>;

// 存在在线用户
pub static ONLINE_USERS: LazyLock<Users> = LazyLock::new(Users::default);

// 存userid和next_id的映射
pub static USER_ID_TO_CONN_ID: LazyLock<RwLock<HashMap<i64, Vec<usize>>>> = LazyLock::new(RwLock::default);

pub struct SSE;

impl SSE{
    pub async fn get_online_user_count() -> usize {
        USER_ID_TO_CONN_ID.read().await.len()
    }

    pub async fn get_online_client_count() -> usize {
        ONLINE_USERS.read().await.len()
    }

    pub async fn send_message_to_user(user_id: i64, msg: &str) -> bool {
        let guard = USER_ID_TO_CONN_ID.read().await;
        if  !guard.contains_key(&user_id) {
            info!("用户{}不在线", user_id);
            return false;
        }

        let users = ONLINE_USERS.read().await;
        if let Some(conn_ids)= guard.get(&user_id) {
            for conn_id in conn_ids {
                if !users.contains_key(conn_id) {
                    info!("用户{}的连接{}不在线", user_id, conn_id);
                    continue;
                }

                if let Some(tx) = users.get(conn_id) {
                    let result = tx.send(
                        SSEMessage::Reply(SseEvent::default().text(msg.to_string()))
                    );
                    if result.is_ok() {
                        continue
                    }
                }
                // 如果发送失败，获取写锁并移除用户
                let mut users = ONLINE_USERS.write().await;
                if let Some(tx) = users.remove(conn_id) {
                    tx.closed().await;
                }

            }
        }else {
            return false
        };
        true
    }

    pub async fn send_message_to_all(msg: &str) {
        ONLINE_USERS.write().await.retain(|_, tx| {
            tx.send(SSEMessage::Reply(SseEvent::default().text(msg.to_string()))).is_ok()
        })
    }
}


#[handler]
async fn send(req: &mut Request,res: &mut Response) {
    let msg = req.query::<String>("msg");
    if let Some(msg)= msg {
        SSE::send_message_to_user(1,&msg).await;
    }
    res.status_code = Some(StatusCode::OK)
}

static NEXT_ID: AtomicUsize = AtomicUsize::new(1);
#[handler]
async fn user_connected(depot: &mut Depot,res: &mut Response) {
    let user = match depot.get_user() {
        Ok(user) => {user}
        Err(e) => {
            error!("error:{:?}",e);
            return
        }
    };
    let user_id = user.id.clone();
    let my_id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    info!("有用户进入user_id:{} id:{}",&user_id,my_id);

    let mut guard = USER_ID_TO_CONN_ID.write().await;
    if !guard.contains_key(&user_id){
        guard.insert(user_id, vec![my_id]);
    }else{
        guard.get_mut(&user_id).unwrap().push(my_id);
    }

    let (tx, rx) = mpsc::unbounded_channel();
    tx.send(SSEMessage::Reply(SseEvent::default().name("user").text("hello"))).unwrap();
    let my_id_c = my_id.clone();
    let user_id_2 = user.id.clone();
    ONLINE_USERS.write().await.insert(my_id, tx);
    SseKeepAlive::new().stream(res,rx,move || async move{
        info!("有用户离开 user_id:{} id:{}",&user_id_2,my_id_c);
        ONLINE_USERS.write().await.remove(&my_id_c);
        USER_ID_TO_CONN_ID.write().await.get_mut(&user_id_2).unwrap().retain(|&x| x != my_id_c);
        if USER_ID_TO_CONN_ID.read().await.get(&user_id_2).unwrap().len() == 0 {
            USER_ID_TO_CONN_ID.write().await.remove(&user_id_2);
        }
        info!("当前客户端连接数:{} 当前用户在线数:{}",SSE::get_online_client_count().await,SSE::get_online_user_count().await)
    }).await;
}


