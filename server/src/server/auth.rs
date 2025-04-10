use axum::{extract::State, http::StatusCode, routing::get, Form, Router};
use axum_extra::extract::{
    cookie::{Cookie, SameSite},
    CookieJar,
};
use axum_pass::{safe, Password};

use tracing::error;
use vitium_api::net;

use super::Server;

/// The REST API method router.
pub fn rest() -> Router<Server> {
    Router::new().route("/", get(read).post(create).put(update).delete(delete))
}

async fn read(State(s): State<Server>, Password(user): Password) -> CookieJar {
    let token = s.safe.issue_token(&user);
    let mut cookie = Cookie::new("token", token);
    cookie.set_http_only(true);
    cookie.set_partitioned(true);
    cookie.set_same_site(Some(SameSite::None));
    cookie.set_secure(true);
    let cookie = CookieJar::new().add(cookie);
    cookie
}

async fn create(State(s): State<Server>, Form(form): Form<net::SignUp>) -> StatusCode {
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

async fn update(
    State(s): State<Server>,
    Password(user): Password,
    Form(form): Form<net::EditPass>,
) -> StatusCode {
    let res = s.safe.update(&user, &form.0).await;
    match res {
        Ok(_) => StatusCode::OK,
        Err(safe::Error::UserNotExist(_)) => StatusCode::NOT_FOUND,
        Err(e) => {
            error!("{e}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

async fn delete(State(s): State<Server>, Password(user): Password) -> StatusCode {
    let res = s.safe.delete(&user).await;
    match res {
        Ok(_) => StatusCode::OK,
        Err(safe::Error::UserNotExist(_)) => StatusCode::NOT_FOUND,
        Err(e) => {
            error!("{e}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
