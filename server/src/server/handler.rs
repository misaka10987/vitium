use axum::{
    extract::State,
    http::{header::AUTHORIZATION, HeaderMap, StatusCode},
    Json,
};
use http_auth_basic::Credentials;
use vitium_common::{
    cmd::Echo,
    error::UnimplError,
    game::PC,
    player::Player,
    req::{self},
    Res,
};

use super::Server;

type Responce<T> = Result<Json<Res<T>>, StatusCode>;

pub async fn edit_pswd(
    State(s): State<Server>,
    head: HeaderMap,
    Json(req::EditPswd(pswd)): Json<req::EditPswd>,
) -> Responce<req::EditPswd> {
    if let Some(Ok(h)) = head.get(AUTHORIZATION).map(|token| token.to_str()) {
        if let Ok(b) = Credentials::from_header(h.to_string()) {
            let safe = s.safe.lock().unwrap();
            return match safe.update(&b.user_id, &pswd, &b.password) {
                Ok(_) => Ok(Json(Ok(()))),
                Err(e) => Ok(Json(Err(format!("{e}")))),
            };
        }
    }
    Err(StatusCode::UNAUTHORIZED)
}

pub async fn recv_chat(
    State(s): State<Server>,
    Json(req::RecvChat(time)): Json<req::RecvChat>,
) -> Responce<req::RecvChat> {
    let data = s.chat.read().await;
    let res = data
        .iter()
        .filter(|(_, chat)| chat.recv_time > time)
        .cloned()
        .collect();
    Ok(Json(Ok(res)))
}

pub async fn get_player(State(s): State<Server>) -> Responce<req::GetPlayer> {
    let res = s
        .player
        .read()
        .await
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    Ok(Json(Ok(res)))
}

pub async fn get_pc(State(s): State<Server>) -> Responce<req::GetPC> {
    let res =
        s.pc.read()
            .await
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
    Ok(Json(Ok(res)))
}

pub async fn send_chat(
    State(s): State<Server>,
    head: HeaderMap,
    Json(req): Json<req::SendChat>,
) -> Responce<req::SendChat> {
    if let Some(name) = s.auth(&head).await {
        let mut dat = s.chat.write().await;
        while dat.len() >= s.cfg.chat_cap {
            dat.pop_front();
        }
        let chat = req.received();
        let time = chat.recv_time;
        dat.push_back((name, chat));
        Ok(Json(Ok(time)))
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}

pub async fn edit_player(
    State(s): State<Server>,
    head: HeaderMap,
    Json(req): Json<req::Edit<Player>>,
) -> Responce<req::Edit<Player>> {
    // edit existing player
    if let Some(name) = s.auth(&head).await {
        if name != req.src {
            return Err(StatusCode::FORBIDDEN);
        }
        let mut tab = s.player.write().await;
        match (tab.contains_key(&req.src), req.dst) {
            (true, None) => Ok(Json(Ok(tab.remove(&req.src)))),
            (true, Some(p)) => Ok(Json(Ok(tab.insert(req.src, p)))),
            (false, None) => Ok(Json(Err(format!("player {} not found", req.src)))),
            (false, Some(p)) => Ok(Json(Ok(tab.insert(req.src, p)))),
        }
    }
    // create new player
    else {
        if let Some(Ok(token)) = head.get(AUTHORIZATION).map(|token| token.to_str()) {
            if let Ok(b) = Credentials::from_header(token.to_string()) {
                let mut data = s.player.write().await;
                if b.user_id != req.src {
                    return Err(StatusCode::BAD_REQUEST);
                }
                if data.contains_key(&req.src) {
                    return Err(StatusCode::FORBIDDEN);
                }
                let safe = s.safe.lock().unwrap();
                if let Err(e) = safe.create(&b.user_id, &b.password) {
                    return Ok(Json(Err(format!("{e}"))));
                }
                return if let Some(p) = req.dst {
                    Ok(Json(Ok(data.insert(req.src, p))))
                } else {
                    Err(StatusCode::BAD_REQUEST)
                };
            }
        }
        Err(StatusCode::UNAUTHORIZED)
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
            (false, None) => Ok(Json(Err(format!("player character {} not found", req.src)))),
            (false, Some(c)) => Ok(Json(Ok(tab.insert(req.src, c)))),
        }
    } else {
        Err(StatusCode::FORBIDDEN)
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
