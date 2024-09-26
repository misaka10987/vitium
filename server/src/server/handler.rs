use axum::{
    extract::{Path, State},
    http::{header::AUTHORIZATION, HeaderMap, StatusCode},
    Json,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use http_auth_basic::Credentials;

use safe_box::err::SafeBoxError;
use tracing::{error, trace};
use vitium_api::{
    cmd::Echo,
    net::{self, Req, SendChat},
};

use super::Server;

type Responce<T> = Result<Json<<T as Req>::Response>, StatusCode>;

pub async fn login(State(s): State<Server>, head: HeaderMap) -> Result<CookieJar, StatusCode> {
    if let Some(Ok(Ok(cred))) = head.get(AUTHORIZATION).map(|token| {
        token
            .to_str()
            .map(|b| Credentials::from_header(b.to_owned()))
    }) {
        let res = s.safe.verify(&cred.user_id, &cred.password).await;
        match res {
            Ok(token) => Ok(CookieJar::new().add(Cookie::new("token", token))),
            Err(SafeBoxError::BadPass { user, .. }) | Err(SafeBoxError::UserNotExist(user)) => {
                trace!("failed attempt to login '{user}'");
                Err(StatusCode::FORBIDDEN)
            }
            _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}

pub async fn signup(
    State(s): State<Server>,
    Json(req): Json<net::SignUp>,
) -> Responce<net::SignUp> {
    let res = s.safe.create(&req.user, &req.pass).await;
    if let Err(e) = res {
        match e {
            SafeBoxError::UserAlreadyExist(_) => Err(StatusCode::CONFLICT),
            e => {
                error!("{e}");
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    } else {
        Ok(Json(()))
    }
}

pub async fn edit_pass(
    State(s): State<Server>,
    head: HeaderMap,
    Json(net::EditPass(pass)): Json<net::EditPass>,
) -> Responce<net::EditPass> {
    if let Some(Ok(Ok(cred))) = head.get(AUTHORIZATION).map(|token| {
        token
            .to_str()
            .map(|b| Credentials::from_header(b.to_owned()))
    }) {
        let res = s.safe.update(&cred.user_id, &cred.password, &pass).await;
        match res {
            Ok(_) => Ok(Json(())),
            Err(SafeBoxError::BadPass { user, .. }) => {
                trace!("failed attempt to change password for '{user}'");
                Err(StatusCode::FORBIDDEN)
            }
            Err(SafeBoxError::UserNotExist(_)) => Err(StatusCode::NOT_FOUND),
            _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

pub async fn recv_chat(
    State(s): State<Server>,
    Json(net::RecvChat(time)): Json<net::RecvChat>,
) -> Responce<net::RecvChat> {
    Ok(Json({
        s.chat
            .pull(time)
            .await
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    }))
}

pub async fn list_player(State(s): State<Server>) -> Responce<net::ListPlayer> {
    let res = s
        .player
        .read()
        .await
        .iter()
        .map(|(k, _)| k.clone())
        .collect();
    Ok(Json(res))
}

pub async fn get_player(
    State(s): State<Server>,
    Path(name): Path<String>,
) -> Responce<net::GetPlayer> {
    let res = s.player.read().await.get(&name).cloned();
    if let Some(p) = res {
        Ok(Json(p))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn edit_player(
    State(s): State<Server>,
    Path(name): Path<String>,
    jar: CookieJar,
    Json(net::EditPlayer(_, player)): Json<net::EditPlayer>,
) -> Responce<net::EditPlayer> {
    if let Some(user) = s.auth(&jar) {
        if user == name {
            let mut tab = s.player.write().await;
            tab.insert(user, player);
            return Ok(Json(()));
        }
    }
    Err(StatusCode::FORBIDDEN)
}

pub async fn list_pc(State(s): State<Server>) -> Responce<net::ListPC> {
    let res =
        s.pc.read()
            .await
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
    Ok(Json(res))
}

pub async fn get_pc(State(s): State<Server>, Path(name): Path<String>) -> Responce<net::GetPC> {
    let res = s.pc.read().await.get(&name).cloned();
    if let Some(p) = res {
        Ok(Json(p))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn send_chat(
    State(s): State<Server>,
    jar: CookieJar,
    Json(SendChat(chat)): Json<net::SendChat>,
) -> Responce<net::SendChat> {
    match s.auth(&jar) {
        Some(user) if user == chat.sender => Ok(Json(s.chat.push(chat).await)),
        _ => Err(StatusCode::FORBIDDEN),
    }
}

/// Handler for `POST /chara`.
pub async fn edit_pc(
    State(s): State<Server>,
    Path(name): Path<String>,
    jar: CookieJar,
    Json(net::EditPC(_, new)): Json<net::EditPC>,
) -> Responce<net::EditPC> {
    if let Some(user) = s.auth(&jar) {
        let mut tab = s.pc.write().await;
        if let Some(c) = tab.get(&name) {
            if user != c.player {
                return Err(StatusCode::FORBIDDEN);
            }
        }
        match (tab.contains_key(&name), new) {
            (true, None) => {
                tab.remove(&name);
                Ok(Json(()))
            }
            (_, Some(c)) => {
                tab.insert(name, c);
                Ok(Json(()))
            }
            (false, None) => Err(StatusCode::NOT_FOUND),
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
