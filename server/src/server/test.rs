use axum::{routing::get, Router};
use axum_pass::Token;

use super::Server;

pub fn router() -> Router<Server> {
    Router::new().route("/auth", get(auth))
}

async fn auth(Token(user): Token) -> String {
    user
}
