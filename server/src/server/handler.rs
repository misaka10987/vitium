use std::time::SystemTime;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Form, Json,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use axum_pass::{safe, Password, Token};

use tracing::error;
use vitium_api::net::{self, Req};

use super::Server;

type Responce<T> = Result<Json<<T as Req>::Response>, StatusCode>;

pub async fn login(State(s): State<Server>, Password(user): Password) -> CookieJar {
    let token = s.safe.issue_token(&user);
    let cookie = CookieJar::new().add(Cookie::new("token", token));
    cookie
}

pub async fn signup(State(s): State<Server>, Form(form): Form<net::SignUp>) -> StatusCode {
    let res = s.safe.create(&form.user, &form.pass).await;
    match res {
        Ok(_) => StatusCode::OK,
        Err(safe::Error::UserAlreadyExist(_)) => StatusCode::CONFLICT,
        Err(e) => {
            error!("{e}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

pub async fn edit_pass(
    State(s): State<Server>,
    Password(user): Password,
    Form(net::EditPass(pass)): Form<net::EditPass>,
) -> StatusCode {
    let res = s.safe.update(&user, &pass).await;
    match res {
        Ok(_) => StatusCode::OK,
        Err(safe::Error::UserNotExist(_)) => StatusCode::NOT_FOUND,
        Err(e) => {
            error!("{e}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

pub async fn read_chat(
    State(s): State<Server>,
    Form(after): Form<SystemTime>,
) -> Result<Json<Vec<(SystemTime, net::Chat)>>, StatusCode> {
    Ok(Json({
        s.chat
            .pull(after)
            .await
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    }))
}

pub async fn list_user(State(s): State<Server>) -> Responce<net::ListPlayer> {
    let res = s
        .player
        .read()
        .await
        .iter()
        .map(|(k, _)| k.clone())
        .collect();
    Ok(Json(res))
}

pub async fn read_user(
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

pub async fn update_user(
    State(s): State<Server>,
    Path(name): Path<String>,
    Token(user): Token,
    Json(net::EditPlayer(_, player)): Json<net::EditPlayer>,
) -> Responce<net::EditPlayer> {
    if user == name {
        let mut tab = s.player.write().await;
        tab.insert(user, player);
        return Ok(Json(()));
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

pub async fn create_chat(
    State(s): State<Server>,
    Token(user): Token,
    Json(chat): Json<net::Chat>,
) -> Result<Json<SystemTime>, StatusCode> {
    if user != chat.sender {
        return Err(StatusCode::FORBIDDEN);
    }
    if chat.msg.chars().nth(0) == Some('/') {
        let res = if s.is_op(&user).await {
            s.op_cmd(&chat.msg[1..]).await
        } else {
            s.cmd(&chat.msg[1..]).await
        };
        let res = match res {
            Ok(o) => o,
            Err(e) => e.to_string(),
        };
        s.chat
            .broadcast(format!("{user} {} -- {res}", chat.msg))
            .await;
        return Ok(Json(SystemTime::now()));
    }
    Ok(Json(s.chat.push(chat).await))
}

/// Handler for `POST /chara`.
pub async fn edit_pc(
    State(s): State<Server>,
    Path(name): Path<String>,
    Token(user): Token,
    Json(net::EditPC(_, new)): Json<net::EditPC>,
) -> Responce<net::EditPC> {
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
}

pub async fn sync(State(_): State<Server>, Token(_): Token) -> StatusCode {
    todo!()
}
