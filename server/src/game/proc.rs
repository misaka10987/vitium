use vitium_common::{
    game::{
        act::{self, atk::Atk},
        Act, Action,
    },
    req, Res,
};

use super::Game;

use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    Json,
};

use crate::Server;

pub trait Proc<T: Act> {
    fn proc(&self, act: Action<T>) -> act::Result<T>;
}

impl Proc<Atk> for Game {
    fn proc(&self, _: Action<Atk>) -> act::Result<Atk> {
        Err(act::Error::Unimplemented)
    }
}

type Response<T> = Result<Json<Res<act::Action<T>>>, StatusCode>;

pub async fn atk(
    State(s): State<Server>,
    head: HeaderMap,
    Json(act): Json<req::Action<Atk>>,
) -> Response<Atk> {
    if let Some(name) = s.auth(&head).await {
        if name != act.pc {
            return Err(StatusCode::FORBIDDEN);
        }
        Ok(Json(Ok(s.game.read().await.proc(act))))
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}
