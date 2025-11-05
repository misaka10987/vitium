use std::time::{SystemTime, UNIX_EPOCH};

use axum::{
    Json, Router,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Sse, sse::Event},
    routing::get,
};
use sqlx::{Row, SqlitePool, query};
use tokio::sync::watch;
use tokio_stream::{Stream, StreamExt};
use tracing::{error, info};
use vitium_api::chat::Message;

use super::{Server, auth::Token};

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
            let value = s.wait_for_new_chat().await?;
            let event = Event::default().json_data(value)?;
            Ok(event)
        }
    };
    let stream = tokio_stream::iter(0..).then(wait);
    Sse::new(stream)
}

async fn fetch_longpoll(s: Server) -> Result<Json<Message>, StatusCode> {
    s.wait_for_new_chat().await.map(|x| Json(x)).map_err(|e| {
        error!("{e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

async fn read(
    State(s): State<Server>,
    Path(setpoint): Path<u64>,
) -> Result<Json<Vec<Message>>, StatusCode> {
    Ok(Json(s.chat_after(setpoint).await.map_err(|e| {
        error!("{e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?))
}

async fn create(
    State(s): State<Server>,
    Token(user): Token,
    Json(chat): Json<Message>,
) -> Result<(), StatusCode> {
    if chat.sender != Some(user) {
        return Err(StatusCode::FORBIDDEN);
    }
    let res = s.send_chat(chat).await;
    if let Err(e) = res {
        error!("{e}")
    }
    Ok(())
}

pub struct ChatModule {
    db: SqlitePool,
    live: watch::Sender<Message>,
}

impl ChatModule {
    pub fn new(db: SqlitePool) -> Self {
        let (live, _) = watch::channel(Message {
            time: mili_timestamp(SystemTime::now()),
            sender: None,
            content: "".into(),
            html: false,
        });
        Self { db, live }
    }

    pub async fn all_msg_after(&self, setpoint: u64) -> anyhow::Result<Vec<Message>> {
        let row = query("SELECT * FROM chat WHERE time > ? ORDER BY time")
            .bind(setpoint as i64)
            .fetch_all(&self.db)
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

    pub async fn send(&self, msg: Message) -> anyhow::Result<()> {
        let query = query("INSERT INTO chat (time, sender, content, html) VALUES (?, ?, ?, ?);")
            .bind(msg.time as i64)
            .bind(&msg.sender)
            .bind(&msg.content)
            .bind(msg.html);
        query.execute(&self.db).await?;
        let sender = match &msg.sender {
            Some(x) => x,
            None => "",
        };
        info!("{} {}", sender, msg.content);
        self.live.send_replace(msg);
        Ok(())
    }

    pub async fn server_send(&self, content: String) {
        let res = self
            .send(Message {
                time: mili_timestamp(SystemTime::now()),
                sender: None,
                content,
                html: false,
            })
            .await;
        if let Err(e) = res {
            error!("{e}");
        }
    }

    pub async fn wait(&self) -> anyhow::Result<Message> {
        let mut recv = self.live.subscribe();
        recv.changed().await?;
        let value = recv.borrow_and_update().clone();
        Ok(value)
    }
}

impl Server {
    pub async fn chat_after(&self, setpoint: u64) -> anyhow::Result<Vec<Message>> {
        self.chat.all_msg_after(setpoint).await
    }

    pub async fn send_chat(&self, msg: Message) -> anyhow::Result<()> {
        self.chat.send(msg).await
    }

    pub async fn send_server_chat(&self, content: String) {
        self.chat.server_send(content).await
    }

    pub async fn wait_for_new_chat(&self) -> anyhow::Result<Message> {
        self.chat.wait().await
    }
}
