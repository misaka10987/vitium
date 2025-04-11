use axum::{routing::get, Router};

use super::{auth::Token, Server};

pub fn router() -> Router<Server> {
    Router::new().route("/auth", get(auth))
}

async fn auth(Token(user): Token) -> String {
    user
}
