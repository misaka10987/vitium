use axum::{
    extract::State,
    http::{header::AUTHORIZATION, HeaderMap, StatusCode},
    Json,
};
use http_auth_basic::Credentials;
use std::collections::{HashMap, VecDeque};
use vitium_common::{
    cmd::Echo,
    error::UnimplError,
    game::PC,
    player::Player,
    req::{self, Action, Chat},
    Res,
};

use super::Server;

type Responce<T> = Result<Json<Res<T>>, StatusCode>;

pub async fn recv_chat(State(s): State<Server>) -> (StatusCode, Json<VecDeque<(String, Chat)>>) {
    (StatusCode::OK, Json(s.chat.read().await.clone()))
}

pub async fn get_player(State(s): State<Server>) -> (StatusCode, Json<HashMap<String, Player>>) {
    (StatusCode::OK, Json(s.player.read().await.clone()))
}
pub async fn get_pc(State(s): State<Server>) -> (StatusCode, Json<HashMap<String, PC>>) {
    (StatusCode::OK, Json(s.pc.read().await.clone()))
}

pub async fn send_chat(
    State(s): State<Server>,
    head: HeaderMap,
    Json(req): Json<req::Chat>,
) -> StatusCode {
    if let Some(name) = s.auth(&head).await {
        let mut dat = s.chat.write().await;
        while dat.len() >= s.cfg.chat_cap {
            dat.pop_front();
        }
        dat.push_back((name, req.received()));
        StatusCode::ACCEPTED
    } else {
        StatusCode::FORBIDDEN
    }
}

pub async fn edit_pswd(
    State(s): State<Server>,
    head: HeaderMap,
    Json(req::EditPswd(pswd)): Json<req::EditPswd>,
) -> StatusCode {
    if let Some(Ok(h)) = head.get(AUTHORIZATION).map(|token| token.to_str()) {
        if let Ok(b) = Credentials::from_header(h.to_string()) {
            let safe = s.safe.lock().unwrap();
            if safe.update(&b.user_id, &pswd, &b.password).is_ok() {
                return StatusCode::OK;
            }
        }
    }
    StatusCode::FORBIDDEN
}

pub async fn edit_player(
    State(s): State<Server>,
    head: HeaderMap,
    Json(req): Json<req::Edit<Player>>,
) -> StatusCode {
    if let Some(name) = s.auth(&head).await {
        if name != req.src {
            return StatusCode::FORBIDDEN;
        }
        let mut tab = s.player.write().await;
        match (tab.contains_key(&req.src), req.dst) {
            (true, None) => {
                tab.remove(&req.src);
                StatusCode::OK
            }
            (true, Some(p)) => {
                tab.insert(req.src, p);
                StatusCode::OK
            }
            (false, None) => StatusCode::NOT_FOUND,
            (false, Some(p)) => {
                tab.insert(req.src, p);
                StatusCode::CREATED
            }
        }
    } else {
        StatusCode::FORBIDDEN
    }
}

/// Handler for `POST /chara`.
pub async fn edit_pc(
    State(s): State<Server>,
    head: HeaderMap,
    Json(req): Json<req::Edit<PC>>,
) -> Responce<req::Edit<PC>> {
    if let Some(name) = s.auth(&head).await {
        let mut tab = s.pc.write().await;
        if let Some(c) = tab.get(&req.src) {
            if name != c.player {
                return Err(StatusCode::FORBIDDEN);
            }
        }
        match (tab.contains_key(&req.src), req.dst) {
            (true, None) => Ok(Json(Ok(tab.remove(&req.src)))),
            (true, Some(c)) => Ok(Json(Ok(tab.insert(req.src, c)))),
            (false, None) => Ok(Json(Err(format!("no player character: {}", req.src)))),
            (false, Some(c)) => Ok(Json(Ok(tab.insert(req.src, c)))),
        }
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}

pub async fn act(
    State(s): State<Server>,
    head: HeaderMap,
    Json(req): Json<(String, Action)>,
) -> StatusCode {
    if let Some(name) = s.auth(&head).await {
        let (pc, _) = req;
        if let Some(c) = s.pc.read().await.get(&pc) {
            if c.player == name {
                let _ = s.game;
                todo!()
            } else {
                // the request has a token but not matches the character it operates on
                StatusCode::FORBIDDEN
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
        let g = s.game.read().await;
        let _ = req;
        if g.stat.host == name {
            let err = UnimplError("command".to_owned());
            return (
                StatusCode::NOT_IMPLEMENTED,
                Json(Some(Err(err.to_string()))),
            );
        }
    }
    (StatusCode::FORBIDDEN, Json(None))
}
