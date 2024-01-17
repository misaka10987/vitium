use crate::game;
use crate::game::{push_act, turn};
use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet};
use tokio::fs;
use tokio::{
    signal,
    sync::{Mutex, MutexGuard},
};
use tower_http::trace::TraceLayer;
use vitium_common::config::{obj, toml, ServerConfig};
use vitium_common::req::Exit;
use vitium_common::{
    act::Act,
    chara::Chara,
    cmd::Cmd,
    //module::Module,
    player::{Player, Token},
    req::EditPswd,
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

type Map<K, V> = Lazy<Mutex<HashMap<K, V>>>;
macro_rules! map {
    () => {
        Lazy::new(|| Mutex::new(HashMap::new()))
    };
}
static PLAYER: Map<String, Player> = map!();
static BANNED_PLAYER: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));
static PSWD: Map<String, String> = map!();
static CHARA: Map<String, Chara> = map!();
static OP: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));

async fn player() -> MutexGuard<'static, HashMap<String, Player>> {
    PLAYER.lock().await
}
async fn banned_player() -> MutexGuard<'static, HashSet<String>> {
    BANNED_PLAYER.lock().await
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

async fn banned(id: &str) -> bool {
    banned_player().await.contains(id)
}

async fn sync(Json(req): Json<Token>) -> (StatusCode, String) {
    if !verify(&req).await {
        return (
            StatusCode::FORBIDDEN,
            "sync: FORBIDDEN Hello, world!\n".to_string(),
        );
    }
    (
        StatusCode::NOT_IMPLEMENTED,
        "sync: Hello, world!\n".to_string(),
    )
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

async fn exit(Json(req): Json<Exit>) -> StatusCode {
    use crate::chara::chara as game_chara;
    use crate::chara::exit;
    if !game_chara().await.contains_key(&req.chara) {
        return StatusCode::NOT_FOUND;
    }
    match game_chara().await.get(&req.chara) {
        Some(c) => {
            if req.token.id != c.player {
                return StatusCode::UNAUTHORIZED;
            }
            if !verify(&req.token).await {
                return StatusCode::FORBIDDEN;
            }
            exit(req.chara).await;
            StatusCode::OK
        }
        None => StatusCode::NOT_FOUND,
    }
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
pub struct Server {}

impl Server {
    pub fn new() -> Self {
        Server{}
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
            let c = toml(&config().await.clone());
            fs::write(path, c)
                .await
                .expect(&format!("{} io error", path));
            return Server::new();
        }
        config().await.clone_from(&obj::<ServerConfig>(
            &fs::read_to_string(path)
                .await
                .expect(&format!("{} io error", path)),
        ));
        Server::new()
    }
    pub async fn run(self) -> Result<(), std::io::Error> {
        let app = Router::new()
            .route("/sync", get(sync))
            .route("/pswd", post(edit_pswd))
            .route("/act", post(act))
            .route("/cmd", post(cmd))
            .route("/exit", post(exit))
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

pub mod root {
    use super::{banned, banned_player, op, player, pswd};
    use tracing::info;
    pub async fn grant(arg: &str) -> i8 {
        let id = arg.trim();
        if player().await.contains_key(id) && pswd().await.contains_key(id) {
            op().await.insert(id.to_string());
            info!("opped player[id=\"{}\"]", id);
            println!("  Success>> opped player[id=\"{}\"]", id);
            0
        } else {
            println!("  Failure>> player[id=\"{}\"] not found", id);
            -1
        }
    }
    pub async fn ban(arg: &str) -> i8 {
        let id = arg.trim();
        if !player().await.contains_key(id) {
            println!("  Failure>> player[id=\"{}\"] not found", id);
            return -1;
        }
        if banned(id).await {
            println!("  Failure>> player[id=\"{}\"] is already banned", id);
            return -1;
        }
        player().await.remove(id);
        pswd().await.remove(id);
        banned_player().await.insert(id.to_string());
        println!("  Success>> banned player[id=\"{}\"]", id);
        0
    }
}
