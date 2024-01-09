use crate::game;
use crate::game::{push_act, turn};
use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet, VecDeque};
use tokio::fs;
use tokio::{
    signal,
    sync::{Mutex, MutexGuard},
};
use tower_http::trace::TraceLayer;
use vitium_common::config::{obj, toml, ServerConfig};
use vitium_common::{
    act::Act,
    chara::Chara,
    cmd::Cmd,
    //module::Module,
    player::{Player, Token},
    request::{Chat, EditChara, EditPlayer, EditPswd, SendChat},
};

static CONFIG: Lazy<Mutex<ServerConfig>> = Lazy::new(|| {
    Mutex::new(ServerConfig {
        port: 19198,
        chat_cap: 127,
        module: vec![],
    })
});
async fn config() -> MutexGuard<'static, ServerConfig> {
    CONFIG.lock().await
}

pub static mut ON: bool = false;

static CHAT: Lazy<Mutex<VecDeque<Chat>>> = Lazy::new(|| Mutex::new(VecDeque::new()));

type Map<K, V> = Lazy<Mutex<HashMap<K, V>>>;
macro_rules! map {
    () => {
        Lazy::new(|| Mutex::new(HashMap::new()))
    };
}
static PLAYER: Map<String, Player> = map!();
static PSWD: Map<String, String> = map!();
static CHARA: Map<String, Chara> = map!();
static OP: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));

async fn chat() -> MutexGuard<'static, VecDeque<Chat>> {
    CHAT.lock().await
}
async fn player() -> MutexGuard<'static, HashMap<String, Player>> {
    PLAYER.lock().await
}
async fn pswd() -> MutexGuard<'static, HashMap<String, String>> {
    PSWD.lock().await
}
async fn chara() -> MutexGuard<'static, HashMap<String, Chara>> {
    CHARA.lock().await
}
async fn op() -> MutexGuard<'static, HashSet<String>> {
    OP.lock().await
}

async fn verify(token: &Token) -> bool {
    if let Some(pswd) = pswd().await.get(&token.id) {
        pswd == &token.pswd
    } else {
        false
    }
}

async fn access(token: Token) -> bool {
    verify(&token).await && op().await.contains(&token.id)
}

async fn recv_chat() -> (StatusCode, Json<VecDeque<Chat>>) {
    (StatusCode::OK, Json(chat().await.clone()))
}

async fn get_player() -> (StatusCode, Json<HashMap<String, Player>>) {
    (StatusCode::OK, Json(player().await.clone()))
}

async fn get_chara() -> (StatusCode, Json<HashMap<String, Chara>>) {
    (StatusCode::OK, Json(chara().await.clone()))
}

async fn send_chat(Json(req): Json<SendChat>) -> StatusCode {
    if !verify(&req.token).await {
        StatusCode::FORBIDDEN
    } else if req.token.id != req.chat.player {
        StatusCode::FORBIDDEN
    } else {
        let mut dat = chat().await;
        while dat.len() >= config().await.chat_cap {
            dat.pop_front();
        }
        let mut content = req.chat;
        dat.push_back(content.renew().clone());
        StatusCode::ACCEPTED
    }
}

async fn edit_pswd(Json(req): Json<EditPswd>) -> StatusCode {
    if verify(&req.token).await {
        *pswd()
            .await
            .get_mut(&req.token.id)
            .expect("internal server error when trying to change password") = req.pswd;
        StatusCode::ACCEPTED
    } else {
        StatusCode::FORBIDDEN
    }
}

async fn edit_player(Json(req): Json<EditPlayer>) -> StatusCode {
    let mut dat = player().await;
    if let Some(player) = dat.get_mut(&req.player.id) {
        if let Some(token) = req.token {
            if verify(&token).await {
                *player = req.player.clone();
                StatusCode::ACCEPTED
            } else {
                StatusCode::FORBIDDEN
            }
        } else {
            StatusCode::UNAUTHORIZED
        }
    } else {
        dat.insert(req.player.id.clone(), req.player.clone());
        StatusCode::CREATED
    }
}

async fn edit_chara(Json(req): Json<EditChara>) -> StatusCode {
    let mut dat = chara().await;
    if !verify(&req.token).await {
        return StatusCode::FORBIDDEN;
    }
    if let Some(chara) = dat.get_mut(&req.chara.player) {
        *chara = req.chara;
        StatusCode::ACCEPTED
    } else {
        dat.insert(req.token.id, req.chara);
        StatusCode::CREATED
    }
}

async fn act(Json(req): Json<Act>) -> StatusCode {
    if !verify(&req.token).await {
        // token is invalid
        StatusCode::FORBIDDEN
    } else if req.turn != *turn().await {
        // the current turn has not yet ended but a new request is received
        StatusCode::LOCKED
    } else if let Some(c) = chara().await.get(&req.token.id) {
        if c.player == req.token.id {
            push_act(req).await;
            StatusCode::ACCEPTED
        } else {
            // the request has a token but not matches the character it operates on
            StatusCode::UNAUTHORIZED
        }
    } else {
        // trying to request act on a non-exist character
        StatusCode::NOT_FOUND
    }
}

async fn cmd(Json(req): Json<Cmd>) -> StatusCode {
    if access(req.token).await {
        game::cmd(req.cmd).await;
        StatusCode::ACCEPTED
    } else {
        StatusCode::FORBIDDEN
    }
}

/// A handler always returns `Hello, world!\n`.
async fn hello() -> &'static str {
    "Hello, World!\n"
}

/// Defines the server. This is a more abstract one, see crate::game for specific game logics.
///
/// Note that only one static instance exists for this struct and it should **NEVER** be manually created.
/// # Examples
/// ```
/// use crate:server:Server;
/// Server::start()
///     .set_port(50000)
///     .run()
///     .unwrap();
/// ```
pub struct Server;

impl Server {
    pub fn start() -> Self {
        Server
    }
    pub async fn config(&self, path: &str) -> Self {
        // Generates a default config file if non exist.
        if !fs::try_exists(path)
            .await
            .expect(&format!("{} io error", path))
        {
            fs::File::create(path)
                .await
                .expect(&format!("{} io error", path));
            let c = toml(config().await.clone());
            fs::write(path, c)
                .await
                .expect(&format!("{} io error", path));
            return Server;
        }
        config().await.clone_from(&obj::<ServerConfig>(
            &fs::read_to_string(path)
                .await
                .expect(&format!("{} io error", path)),
        ));
        Server
    }
    /// The function to run the server.
    #[tokio::main]
    pub async fn run(&self) -> Result<(), std::io::Error> {
        // initialize logger
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::TRACE)
            .init();
        self.config("./config.toml").await;
        // build our application with a route
        let app = Router::new()
            .route("/hello", get(hello))
            .route("/chat", get(recv_chat))
            .route("/player", get(get_player))
            .route("/chara", get(get_chara))
            .route("/chat", post(send_chat))
            .route("/pswd", post(edit_pswd))
            .route("/player", post(edit_player))
            .route("/chara", post(edit_chara))
            .route("/act", post(act))
            .route("/cmd", post(cmd))
            .layer(TraceLayer::new_for_http());
        // listening globally on port 3000
        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config().await.port))
            .await
            .expect("failed to bind TCP listener");
        // run our app with hyper
        axum::serve(listener, app)
            .with_graceful_shutdown(sig_shut())
            .await
    }
}

async fn sig_shut() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };
    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
