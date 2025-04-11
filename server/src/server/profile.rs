use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};

use vitium_api::UserProfile;

use super::{auth::Token, Server};

/// The REST API method router.
pub fn rest() -> Router<Server> {
    Router::new()
        .route("/", get(list).post(create))
        .route("/{name}", get(read).put(update).delete(delete))
}

async fn list(State(s): State<Server>) -> Json<Vec<String>> {
    let res = s
        .player
        .read()
        .await
        .iter()
        .map(|(k, _)| k.clone())
        .collect();
    Json(res)
}

async fn read(
    State(s): State<Server>,
    Path(name): Path<String>,
) -> Result<Json<UserProfile>, StatusCode> {
    let res = s.player.read().await.get(&name).cloned();
    if let Some(p) = res {
        Ok(Json(p))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn create(
    State(s): State<Server>,
    Token(user): Token,
    Json(new): Json<UserProfile>,
) -> StatusCode {
    let mut tab = s.player.write().await;
    if tab.contains_key(&user) {
        return StatusCode::CONFLICT;
    }
    tab.insert(user, new);
    StatusCode::CREATED
}

async fn update(
    State(s): State<Server>,
    Path(name): Path<String>,
    Token(user): Token,
    Json(new): Json<UserProfile>,
) -> StatusCode {
    if user != name {
        return StatusCode::FORBIDDEN;
    }
    let mut tab = s.player.write().await;
    tab.insert(user, new);
    StatusCode::OK
}

async fn delete(
    State(s): State<Server>,
    Path(name): Path<String>,
    Token(user): Token,
) -> StatusCode {
    if user != name {
        return StatusCode::FORBIDDEN;
    }
    s.player.write().await.remove(&user);
    StatusCode::OK
}
