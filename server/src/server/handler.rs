use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::Redirect,
    Json,
};
use std::collections::{HashMap, VecDeque};
use vitium_common::{
    cmd::Echo,
    error::UnimplError,
    game::{GameStat, PC},
    player::Player,
    req::{self, Chat},
};

use super::Server;
pub async fn not_found() -> &'static str {
    "not found"
}

pub async fn get_root(State(s): State<Server>) -> Redirect {
    Redirect::to(&s.cfg.page_url)
}

/// A handler always returns `Hello, world!\n`.
pub async fn hello() -> &'static str {
    "Hello, World!\n"
}

pub async fn recv_chat(State(s): State<Server>) -> (StatusCode, Json<VecDeque<(String, Chat)>>) {
    (StatusCode::OK, Json(s.chat.read().await.clone()))
}

pub async fn get_player(State(s): State<Server>) -> (StatusCode, Json<HashMap<String, Player>>) {
    (StatusCode::OK, Json(s.player.read().await.clone()))
}
pub async fn get_pc(State(s): State<Server>) -> (StatusCode, Json<HashMap<String, PC>>) {
    (StatusCode::OK, Json(s.pc.read().await.clone()))
}

pub async fn list_game(State(s): State<Server>) -> Json<Vec<String>> {
    Json(s.game.read().await.keys().cloned().collect())
}

pub async fn get_game(State(s): State<Server>, Path(name): Path<String>) -> Json<Option<GameStat>> {
    Json(s.game.read().await.get(&name).map(|g| g.stat.clone()))
}

pub async fn send_chat(
    State(s): State<Server>,
    head: HeaderMap,
    Json(req): Json<req::SendChat>,
) -> StatusCode {
    if let Some(name) = s.auth(&head).await {
        let mut dat = s.chat.write().await;
        while dat.len() >= s.cfg.chat_cap {
            dat.pop_front();
        }
        let content = req.chat;
        dat.push_back((name, content));
        StatusCode::ACCEPTED
    } else {
        StatusCode::FORBIDDEN
    }
}

pub async fn edit_pswd(
    State(s): State<Server>,
    head: HeaderMap,
    Json(req): Json<req::EditPswd>,
) -> StatusCode {
    if let Some(name) = s.auth(&head).await {
        if let Some(p) = s.pswd.write().await.get_mut(&name) {
            *p = req.pswd;
            StatusCode::OK
        } else {
            StatusCode::NOT_FOUND
        }
    } else {
        StatusCode::FORBIDDEN
    }
}

pub async fn edit_player(
    State(s): State<Server>,
    head: HeaderMap,
    Json(req): Json<req::EditPlayer>,
) -> StatusCode {
    if let Some(name) = s.auth(&head).await {
        if name != req.name {
            return StatusCode::FORBIDDEN;
        }
        let mut dat = s.player.write().await;
        if let Some(player) = dat.get_mut(&req.name) {
            *player = req;
            StatusCode::OK
        } else {
            StatusCode::NOT_FOUND
        }
    } else {
        StatusCode::FORBIDDEN
    }
}

/// Handler for `POST /chara`.
pub async fn edit_pc(
    State(s): State<Server>,
    head: HeaderMap,
    Json(req): Json<req::EditChara>,
) -> StatusCode {
    if let Some(name) = s.auth(&head).await {
        let mut dat = s.pc.write().await;
        if let Some(chara) = dat.get_mut(&req.dest) {
            if chara.player != name {
                return StatusCode::FORBIDDEN;
            }
            *chara = req.new;
            StatusCode::ACCEPTED
        } else {
            dat.insert(req.dest, req.new);
            StatusCode::CREATED
        }
    } else {
        StatusCode::FORBIDDEN
    }
}

pub async fn act(
    State(s): State<Server>,
    head: HeaderMap,
    Json(req): Json<req::Act>,
) -> StatusCode {
    if let Some(name) = s.auth(&head).await {
        if let Some(c) = s.pc.read().await.get(&req.cha) {
            if c.player == name {
                let _ = s.game;
                todo!()
            } else {
                // the request has a token but not matches the character it operates on
                StatusCode::UNAUTHORIZED
            }
        } else {
            // trying to request act on a non-exist character
            StatusCode::NOT_FOUND
        }
    } else {
        StatusCode::FORBIDDEN
    }
}

pub async fn sync(State(s): State<Server>, head: HeaderMap) -> StatusCode {
    if let Some(_) = s.auth(&head).await {};
    todo!()
}

pub async fn cmd(
    State(s): State<Server>,
    head: HeaderMap,
    Json(req): Json<req::Cmd>,
) -> (StatusCode, Json<Option<Echo>>) {
    if let Some(name) = s.auth(&head).await {
        if let Some(g) = s.game.read().await.get(&req.game) {
            if g.stat.host == name {
                let err = UnimplError("command".to_owned());
                return (
                    StatusCode::NOT_IMPLEMENTED,
                    Json(Some(Err(err.to_string()))),
                );
            }
        }
    }
    (StatusCode::FORBIDDEN, Json(None))
}
