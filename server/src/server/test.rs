use axum::{
    extract::{ws::WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::{any, get},
    Router,
};
use std::time::Duration;
use tokio::time::sleep;
use tracing::error;

use super::{auth::Token, Server};

pub fn router() -> Router<Server> {
    Router::new()
        .route("/auth", get(auth))
        .route("/ws", any(ws))
}

async fn auth(Token(user): Token) -> String {
    user
}

async fn ws(ws: WebSocketUpgrade) -> impl IntoResponse {
    async fn handle(mut sock: WebSocket) {
        for i in 0..5 {
            if let Err(e) = sock
                .send(axum::extract::ws::Message::Text(
                    format!("Hello, WebSocket! ({i})").into(),
                ))
                .await
            {
                error!("{e}")
            }
            sleep(Duration::from_secs(2)).await;
        }
    }
    ws.on_upgrade(handle)
}
