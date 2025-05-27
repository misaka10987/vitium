use axum::{Form, Router, extract::State, http::StatusCode, response::Redirect, routing::get};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};

use basileus::{Basileus, pass::PassManage, token::TokenManage, user::UserManage};
use tracing::error;
use vitium_api::net;

use super::{Server, internal_server_error};

use axum::{
    extract::FromRequestParts,
    http::{header::AUTHORIZATION, request::Parts},
};

use http_auth_basic::Credentials;

/// Used to extract a password authorized user.
pub struct Password(pub String);

impl<S> FromRequestParts<S> for Password
where
    S: Sync + AsRef<Basileus>,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let cred = match parts.headers.get(AUTHORIZATION).map(|token| {
            token
                .to_str()
                .map(|b| Credentials::from_header(b.to_owned()))
        }) {
            Some(Ok(Ok(x))) => x,
            Some(_) => return Err(StatusCode::BAD_REQUEST),
            None => return Err(StatusCode::UNAUTHORIZED),
        };
        match state.verify_pass(&cred.user_id, &cred.password).await {
            Ok(true) => Ok(Password(cred.user_id)),
            Ok(false) | Err(basileus::err::VerifyPassError::UserNotExist(_)) => {
                Err(StatusCode::UNAUTHORIZED)
            }
            Err(e) => {
                error!("{e}");
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}

/// Used to extract a token authorized user.
pub struct Token(pub String);

impl<S> FromRequestParts<S> for Token
where
    S: Sync + AsRef<Basileus>,
{
    type Rejection = StatusCode;
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_headers(&parts.headers);
        let token = match jar.get("token") {
            Some(x) => x.value(),
            None => return Err(StatusCode::UNAUTHORIZED),
        };
        let user = match state.verify_token(token) {
            Some(x) => x,
            None => return Err(StatusCode::UNAUTHORIZED),
        };
        Ok(Self(user))
    }
}

/// The REST API method router.
pub fn rest() -> Router<Server> {
    Router::new().route("/", get(read).post(create).put(update).delete(delete))
}

async fn read(State(s): State<Server>, Password(user): Password) -> CookieJar {
    let token = s.issue_token(&user);
    let mut cookie = Cookie::new("token", token);
    cookie.set_http_only(true);
    cookie.set_partitioned(true);
    cookie.set_same_site(Some(SameSite::None));
    cookie.set_secure(true);
    let cookie = CookieJar::new().add(cookie);
    cookie
}

async fn create(
    State(s): State<Server>,
    Form(form): Form<net::SignUp>,
) -> Result<Redirect, StatusCode> {
    s.create_user(&form.user).await.map_err(|e| match e {
        basileus::err::CreateUserError::UserAlreadyExist(_) => StatusCode::CONFLICT,
        e => internal_server_error(e),
    })?;
    s.update_pass(&form.user, &form.pass)
        .await
        .map_err(internal_server_error)?;
    Ok(Redirect::temporary("/login"))
}

async fn update(
    State(s): State<Server>,
    Password(user): Password,
    Form(form): Form<net::EditPass>,
) -> Result<(), StatusCode> {
    s.update_pass(&user, &form.0).await.map_err(|e| match e {
        basileus::err::UpdatePassError::UserNotExist(_) => StatusCode::NOT_FOUND,
        e => internal_server_error(e),
    })
}

async fn delete(State(s): State<Server>, Password(user): Password) -> Result<(), StatusCode> {
    s.delete_user(&user).await.map_err(|e| match e {
        basileus::err::DeleteUserError::UserNotExist(_) => StatusCode::NOT_FOUND,
        e => internal_server_error(e),
    })
}
