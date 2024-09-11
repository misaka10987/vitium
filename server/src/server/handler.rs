use axum::{
    extract::State,
    http::{header::AUTHORIZATION, HeaderMap, StatusCode},
    Json,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use http_auth_basic::Credentials;

use safe_box::err::SafeBoxError;
use tracing::trace;
use vitium_api::{
    cmd::Echo,
    game::PC,
    net::{self, Res},
};

use super::Server;

type Responce<T> = Result<Json<Res<T>>, StatusCode>;

pub async fn login(State(s): State<Server>, head: HeaderMap) -> Result<CookieJar, StatusCode> {
    if let Some(Ok(h)) = head.get(AUTHORIZATION).map(|token| token.to_str()) {
        if let Ok(b) = Credentials::from_header(h.to_string()) {
            let res = s.safe.verify(&b.user_id, &b.password).await;
            return match res {
                Ok(token) => Ok(CookieJar::new().add(Cookie::new("token", token))),
                Err(e) => {
                    if let SafeBoxError::BadPass { user, pass } = e {
                        trace!("bad password {pass} for user {user}");
                        Err(StatusCode::FORBIDDEN)
                    } else {
                        Err(StatusCode::INTERNAL_SERVER_ERROR)
                    }
                }
            };
        }
    }
    Err(StatusCode::BAD_REQUEST)
}

pub async fn edit_pass(
    State(s): State<Server>,
    head: HeaderMap,
    Json(net::EditPass(pswd)): Json<net::EditPass>,
) -> Responce<net::EditPass> {
    if let Some(Ok(h)) = head.get(AUTHORIZATION).map(|token| token.to_str()) {
        if let Ok(b) = Credentials::from_header(h.to_string()) {
            let res = s.safe.update(&b.user_id, &b.password, &pswd).await;
            if let Err(e) = res {
                if let SafeBoxError::BadPass { user, pass } = e {
                    trace!("bad password {pass} for user {user}");
                    return Err(StatusCode::FORBIDDEN);
                }
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
            return Ok(Json(Ok(())));
        }
    }
    Err(StatusCode::UNAUTHORIZED)
}

pub async fn edit_player(
    State(s): State<Server>,
    jar: CookieJar,
    Json(net::EditPlayer(player)): Json<net::EditPlayer>,
) -> Responce<net::EditPlayer> {
    if let Some(user) = s.auth(&jar) {
        let mut tab = s.player.write().await;
        tab.insert(user, player);
        Ok(Json(Ok(())))
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}

pub async fn recv_chat(
    State(s): State<Server>,
    Json(net::RecvChat(time)): Json<net::RecvChat>,
) -> Responce<net::RecvChat> {
    let data = s.chat.read().await;
    let res = data
        .iter()
        .filter(|(_, chat)| chat.recv_time > time)
        .cloned()
        .collect();
    Ok(Json(Ok(res)))
}

pub async fn get_player(State(s): State<Server>) -> Responce<net::GetPlayer> {
    let res = s
        .player
        .read()
        .await
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    Ok(Json(Ok(res)))
}

pub async fn get_pc(State(s): State<Server>) -> Responce<net::GetPC> {
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
    jar: CookieJar,
    Json(req): Json<net::SendChat>,
) -> Responce<net::SendChat> {
    if let Some(user) = s.auth(&jar) {
        let mut list = s.chat.write().await;
        while list.len() >= s.cfg.chat_cap {
            list.pop_front();
        }
        let chat = req.received();
        let t = chat.recv_time;
        list.push_back((user, chat));
        Ok(Json(Ok(t)))
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}

/// Handler for `POST /chara`.
pub async fn edit_pc(
    State(s): State<Server>,
    jar: CookieJar,
    Json(req): Json<net::Edit<PC>>,
) -> Responce<net::Edit<PC>> {
    if let Some(user) = s.auth(&jar) {
        let mut tab = s.pc.write().await;
        if let Some(c) = tab.get(&req.src) {
            if user != c.player {
                return Err(StatusCode::FORBIDDEN);
            }
        }
        match (tab.contains_key(&req.src), req.dst) {
            (true, None) => {
                tab.remove(&req.src);
                Ok(Json(Ok(())))
            }
            (_, Some(c)) => {
                tab.insert(req.src, c);
                Ok(Json(Ok(())))
            }
            (false, None) => Ok(Json(Err(format!("player character {} not found", req.src)))),
        }
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}

pub async fn sync(State(s): State<Server>, jar: CookieJar) -> StatusCode {
    if let Some(_) = s.auth(&jar) {};
    todo!()
}

pub async fn cmd(
    State(s): State<Server>,
    jar: CookieJar,
    Json(_): Json<net::Cmd>,
) -> (StatusCode, Json<Option<Echo>>) {
    if let Some(_) = s.auth(&jar) {
        // let g = s.game.read().await;
        // let _ = req;
        // if g.stat.host == name {
        //     let err = UnimplError("command".to_owned());
        //     return (
        //         StatusCode::NOT_IMPLEMENTED,
        //         Json(Some(Err(err.to_string()))),
        //     );
        // };
        return (
            StatusCode::NOT_IMPLEMENTED,
            Json(Some(Err("not implemented".to_string()))),
        );
    }
    (StatusCode::FORBIDDEN, Json(None))
}
