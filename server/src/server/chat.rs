use std::time::{SystemTime, UNIX_EPOCH};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{sse::Event, Sse},
    routing::get,
    Json, Router,
};
use axum_pass::Token;
use sqlx::{query, Row, SqlitePool};
use tokio::sync::watch;
use tokio_stream::{Stream, StreamExt};
use tracing::error;
use vitium_api::net::{self, Message};

use super::Server;

fn mili_timestamp(time: SystemTime) -> u64 {
    time.duration_since(UNIX_EPOCH).unwrap().as_millis() as u64
}

pub struct ChatServer {
    db: SqlitePool,
    send: watch::Sender<Message>,
}

impl ChatServer {
    pub fn new(db: SqlitePool) -> Self {
        let (send, _) = watch::channel(Message {
            time: mili_timestamp(SystemTime::now()),
            sender: "".into(),
            content: "".into(),
            html: false,
        });
        Self { send, db }
    }

    pub async fn msg_after(&self, setpoint: u64) -> anyhow::Result<Vec<Message>> {
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

    pub fn subscribe(&self) -> watch::Receiver<Message> {
        self.send.subscribe()
    }

    pub async fn handle(&self, msg: Message) -> anyhow::Result<()> {
        let query = query("INSERT INTO chat (time, sender, content, html) VALUES (?, ?, ?, ?);")
            .bind(msg.time as i64)
            .bind(&msg.sender)
            .bind(&msg.content)
            .bind(msg.html);
        query.execute(&self.db).await?;
        self.send.send_replace(msg);
        Ok(())
    }

    pub async fn server_msg(&self, content: String) {
        let res = self
            .handle(Message {
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

    pub async fn wait_new(&self) -> anyhow::Result<Message> {
        let mut recv = self.send.subscribe();
        recv.changed().await?;
        let value = recv.borrow_and_update().clone();
        Ok(value)
    }
}

/// The REST API method router.
pub fn rest() -> Router<Server> {
    Router::new()
        .route("/", get(list).post(create))
        .route("/{time}", get(read))
}

async fn list(State(s): State<Server>) -> Sse<impl Stream<Item = anyhow::Result<Event>>> {
    let wait = move |_| {
        let s = s.clone();
        async move {
            let value = s.chat.wait_new().await?;
            let event = Event::default().json_data(value)?;
            Ok(event)
        }
    };
    let stream = tokio_stream::iter(0..).then(wait);
    Sse::new(stream)
}

async fn read(
    State(s): State<Server>,
    Path(setpoint): Path<u64>,
) -> Result<Json<Vec<Message>>, StatusCode> {
    Ok(Json(s.chat.msg_after(setpoint).await.map_err(|e| {
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
        s.chat
            .server_msg(format!("{user} {} -- {res}", chat.content))
            .await;
        return Ok(());
    }
    let res = s.chat.handle(chat).await;
    if let Err(e) = res {
        error!("{e}")
    }
    Ok(())
}
