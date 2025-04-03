use std::{
    collections::BTreeMap,
    time::{SystemTime, UNIX_EPOCH},
};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use axum_pass::Token;
use tokio::sync::{oneshot, Mutex, RwLock};
use tracing::info;
use vitium_api::net::{self, Chat};

use super::Server;

/// Storage for [`Chat`] messages.
pub struct ChatStore {
    /// Message capacity.
    cap: usize,
    /// List of messages, with the latest at the first.
    list: RwLock<BTreeMap<SystemTime, Chat>>,
    /// Subscribers to update.
    watch: Mutex<Vec<oneshot::Sender<Vec<(SystemTime, Chat)>>>>,
}

impl ChatStore {
    /// Create a chat storage with specified capacity.
    pub const fn new(cap: usize) -> Self {
        Self {
            cap,
            list: RwLock::const_new(BTreeMap::new()),
            watch: Mutex::const_new(Vec::new()),
        }
    }

    pub async fn pull(&self, after: SystemTime) -> oneshot::Receiver<Vec<(SystemTime, Chat)>> {
        let (s, r) = oneshot::channel();
        let list = self.list.read().await;
        let res = list
            .range(after..)
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect::<Vec<_>>();
        if res.len() > 0 {
            let _ = s.send(res);
        } else {
            let mut watch = self.watch.lock().await;
            watch.push(s);
        }
        r
    }

    pub async fn push(&self, chat: Chat) -> SystemTime {
        info!("<{}> {}", chat.sender, chat.msg);
        let time = SystemTime::now();
        for i in self.watch.lock().await.drain(..) {
            let _ = i.send(vec![(time, chat.clone())]);
        }
        let mut list = self.list.write().await;
        list.insert(time, chat);
        // list.push_front(chat);
        // list.truncate(self.cap);
        time
    }

    pub async fn broadcast(&self, msg: String) {
        let chat = Chat {
            sender: "".into(),
            msg,
        };
        self.push(chat).await;
    }
}

/// The REST API method router.
pub fn rest() -> Router<Server> {
    Router::new()
        .route("/", get(list).post(create))
        .route("/{time}", get(read))
}

async fn list(State(s): State<Server>) -> Result<Json<Vec<(SystemTime, net::Chat)>>, StatusCode> {
    Ok(Json(
        s.chat
            .pull(UNIX_EPOCH)
            .await
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    ))
}

async fn read(
    State(s): State<Server>,
    Path(after): Path<SystemTime>,
) -> Result<Json<Vec<(SystemTime, net::Chat)>>, StatusCode> {
    Ok(Json(
        s.chat
            .pull(after)
            .await
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    ))
}

async fn create(
    State(s): State<Server>,
    Token(user): Token,
    Json(chat): Json<net::Chat>,
) -> Result<Json<SystemTime>, StatusCode> {
    if user != chat.sender {
        return Err(StatusCode::FORBIDDEN);
    }
    if chat.msg.chars().nth(0) == Some('/') {
        let res = if s.is_op(&user).await {
            s.op_cmd(&chat.msg[1..]).await
        } else {
            s.cmd(&chat.msg[1..]).await
        };
        let res = match res {
            Ok(o) => o,
            Err(e) => e.to_string(),
        };
        s.chat
            .broadcast(format!("{user} {} -- {res}", chat.msg))
            .await;
        return Ok(Json(SystemTime::now()));
    }
    Ok(Json(s.chat.push(chat).await))
}
