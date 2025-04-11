use std::time::{SystemTime, UNIX_EPOCH};

use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{sse::Event, IntoResponse, Sse},
    routing::get,
    Json, Router,
};
use sqlx::{query, Row, SqlitePool};
use tokio::sync::watch;
use tokio_stream::{Stream, StreamExt};
use tracing::{error, info};
use vitium_api::net::{self, Message};

use super::{auth::Token, Server};

fn mili_timestamp(time: SystemTime) -> u64 {
    time.duration_since(UNIX_EPOCH).unwrap().as_millis() as u64
}

/// The REST API method router.
pub fn rest() -> Router<Server> {
    Router::new()
        .route("/", get(fetch).post(create))
        .route("/{time}", get(read))
}

async fn fetch(State(s): State<Server>, head: HeaderMap) -> impl IntoResponse {
    match head.get("accept") {
        Some(accept) if accept.to_str().unwrap_or("").contains("text/event-stream") => {
            fetch_sse(s).await.into_response()
        }
        _ => fetch_longpoll(s).await.into_response(),
    }
}

async fn fetch_sse(s: Server) -> Sse<impl Stream<Item = anyhow::Result<Event>>> {
    let wait = move |_| {
        let s = s.clone();
        async move {
            let value = s.wait_next_msg().await?;
            let event = Event::default().json_data(value)?;
            Ok(event)
        }
    };
    let stream = tokio_stream::iter(0..).then(wait);
    Sse::new(stream)
}

async fn fetch_longpoll(s: Server) -> Result<Json<Message>, StatusCode> {
    s.wait_next_msg().await.map(|x| Json(x)).map_err(|e| {
        error!("{e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

async fn read(
    State(s): State<Server>,
    Path(setpoint): Path<u64>,
) -> Result<Json<Vec<Message>>, StatusCode> {
    Ok(Json(s.msg_after(setpoint).await.map_err(|e| {
        error!("{e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?))
}

async fn create(
    State(s): State<Server>,
    Token(user): Token,
    Json(chat): Json<net::Message>,
) -> Result<(), StatusCode> {
    if user != chat.sender {
        return Err(StatusCode::FORBIDDEN);
    }
    if chat.content.chars().nth(0) == Some('/') {
        let res = if s.is_op(&user).await {
            s.op_cmd(&chat.content[1..]).await
        } else {
            s.cmd(&chat.content[1..]).await
        };
        let res = match res {
            Ok(o) => o,
            Err(e) => e.to_string(),
        };
        s.send_server_msg(format!("{user} {} -- {res}", chat.content))
            .await;
        return Ok(());
    }
    let res = s.send_msg(chat).await;
    if let Err(e) = res {
        error!("{e}")
    }
    Ok(())
}

pub struct ChatModule {
    send: watch::Sender<Message>,
}

impl ChatModule {
    pub fn new() -> Self {
        let (send, _) = watch::channel(Message {
            time: mili_timestamp(SystemTime::now()),
            sender: "".into(),
            content: "".into(),
            html: false,
        });
        Self { send }
    }
}

pub trait ChatServer {
    async fn msg_after(&self, setpoint: u64) -> anyhow::Result<Vec<Message>>;
    async fn send_msg(&self, msg: Message) -> anyhow::Result<()>;
    async fn send_server_msg(&self, content: String);
    async fn wait_next_msg(&self) -> anyhow::Result<Message>;
}

impl<T> ChatServer for T
where
    T: AsRef<ChatModule> + AsRef<SqlitePool>,
{
    async fn msg_after(&self, setpoint: u64) -> anyhow::Result<Vec<Message>> {
        let db: &SqlitePool = self.as_ref();
        let row = query("SELECT * FROM chat WHERE time > ? ORDER BY time")
            .bind(setpoint as i64)
            .fetch_all(db)
            .await?;
        let mut msg = vec![];
        for row in row {
            msg.push(Message {
                time: row.try_get("time")?,
                sender: row.try_get("sender")?,
                content: row.try_get("content")?,
                html: row.try_get("html")?,
            });
        }
        Ok(msg)
    }

    async fn send_msg(&self, msg: Message) -> anyhow::Result<()> {
        let module: &ChatModule = self.as_ref();
        let db: &SqlitePool = self.as_ref();
        let query = query("INSERT INTO chat (time, sender, content, html) VALUES (?, ?, ?, ?);")
            .bind(msg.time as i64)
            .bind(&msg.sender)
            .bind(&msg.content)
            .bind(msg.html);
        query.execute(db).await?;
        info!("{} {}", msg.sender, msg.content);
        module.send.send_replace(msg);
        Ok(())
    }

    async fn send_server_msg(&self, content: String) {
        let res = self
            .send_msg(Message {
                time: mili_timestamp(SystemTime::now()),
                sender: "".into(),
                content,
                html: false,
            })
            .await;
        if let Err(e) = res {
            error!("{e}");
        }
    }

    async fn wait_next_msg(&self) -> anyhow::Result<Message> {
        let module: &ChatModule = self.as_ref();
        let mut recv = module.send.subscribe();
        recv.changed().await?;
        let value = recv.borrow_and_update().clone();
        Ok(value)
    }
}
