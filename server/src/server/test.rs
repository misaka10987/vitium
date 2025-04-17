use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::{select, time::sleep};
use tracing::error;

use super::{auth::Token, Server};

pub fn router() -> Router<Server> {
    Router::new()
        .route("/auth", get(auth))
        .route("/ws", get(ws))
}

async fn auth(Token(user): Token) -> String {
    user
}

async fn ws(ws: WebSocketUpgrade) -> impl IntoResponse {
    async fn update(sock: &mut WebSocket) -> anyhow::Result<()> {
        loop {
            select! {
                res = sock.recv() => {
                    match res {
                        Some(msg) => sock.send(msg?).await?,
                        None => return Ok(()),
                    }
                }
                _ = sleep(Duration::from_secs(10)) => {
                    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
                    sock.send(Message::Text(
                        format!("Hello, WebSocket! Current UNIX timestamp is {now}.").into(),
                    ))
                    .await?;
                }
            }
        }
    }
    async fn handle(mut sock: WebSocket) {
        let res = update(&mut sock).await;
        if let Err(e) = res {
            error!("{e}")
        }
    }
    ws.on_upgrade(handle)
}
