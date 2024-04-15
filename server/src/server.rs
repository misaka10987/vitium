use axum::{
    extract::State,
    http::{header::AUTHORIZATION, HeaderMap, StatusCode},
    routing::{get, post},
    Json, Router,
};
use http_auth_basic::Credentials;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
    path::Path,
    sync::Arc,
};
use tokio::{net::TcpListener, signal, sync::RwLock};
use tower_http::trace::TraceLayer;
use tracing::error;
use vitium_common::{
    // cmd::Echo,
    config::{obj, toml, ServerConfig},
    error::UnimplError,
    game::PC,
    player::Player,
    req::{self, Chat},
};

use crate::game::Game;

type Lock<T> = Arc<RwLock<T>>;
fn lock<T>(value: T) -> Lock<T> {
    Arc::new(RwLock::new(value))
}

/// Defines the server. This is a more abstract one, see `crate::game` for specific game logics.
/// ```
/// use crate:server:Server;
/// Server::new()
///     .config("./config.toml")
///     .run()
///     .await
///     .expect("internal server error");
/// ```
#[derive(Clone)]
pub struct Server {
    pub cfg: ServerConfig,
    player: Lock<HashMap<String, Player>>,
    pswd: Lock<HashMap<String, String>>,
    pc: Lock<HashMap<String, PC>>,
    op: Lock<HashSet<String>>,
    chat: Lock<VecDeque<(String, Chat)>>,
    game: Arc<Game>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            cfg: ServerConfig::new(),
            player: lock(HashMap::new()),
            pswd: lock(HashMap::new()),
            pc: lock(HashMap::new()),
            op: lock(HashSet::new()),
            chat: lock(VecDeque::new()),
            game: Arc::new(Game::new()),
        }
    }
    /// Reads from the header and get authentication info.
    pub async fn auth(&self, head: &HeaderMap) -> Option<String> {
        if let Some(token) = head.get(AUTHORIZATION) {
            if let Ok(s) = token.to_str() {
                if let Ok(b) = Credentials::from_header(s.to_string()) {
                    if let Some(p) = self.pswd.read().await.get(&b.user_id) {
                        if b.password == *p {
                            return Some(b.user_id);
                        }
                    }
                }
            }
        }
        None
    }
    /// Load configuration from `path`.
    pub fn config(mut self, path: &str) -> Self {
        if !Path::new(path).exists() {
            match fs::File::create(path) {
                Ok(_) => {
                    if let Err(e) = fs::write(path, toml(&self.cfg)) {
                        error!("failed to generate configuration: {}", e);
                    }
                }
                Err(_) => {
                    error!("file {} non exist", path);
                }
            }
            self.cfg.clone_from(&ServerConfig::new());
        } else {
            self.cfg.clone_from(&obj::<ServerConfig>(
                &fs::read_to_string(path).expect(&format!("{} io error", path)),
            ));
        }
        self
    }
    /// Consumes `self` and start the server.
    #[tokio::main]
    pub async fn run(self) -> Result<(), std::io::Error> {
        // listening globally on the port specified
        let listener = TcpListener::bind(format!("0.0.0.0:{}", self.cfg.port))
            .await
            .expect("failed to bind TCP listener");
        // initialize router
        let app = Router::new()
            .route("/hello", get(hello))
            .route("/chat", get(recv_chat))
            .route("/player", get(get_player))
            .route("/chara", get(get_pc))
            .route("/chat", post(send_chat))
            .route("/pswd", post(edit_pswd))
            .route("/player", post(edit_player))
            .route("/chara", post(edit_pc))
            .route("/act", post(act))
            .route("/sync", get(sync))
            .route("/cmd", post(cmd))
            .with_state(self)
            .layer(TraceLayer::new_for_http());
        // run our app with hyper
        axum::serve(listener, app)
            .with_graceful_shutdown(sig_shut())
            .await
    }
}

/// A handler always returns `Hello, world!\n`.
async fn hello() -> &'static str {
    "Hello, World!\n"
}

async fn recv_chat(State(s): State<Server>) -> (StatusCode, Json<VecDeque<(String, Chat)>>) {
    (StatusCode::OK, Json(s.chat.read().await.clone()))
}

async fn get_player(State(s): State<Server>) -> (StatusCode, Json<HashMap<String, Player>>) {
    (StatusCode::OK, Json(s.player.read().await.clone()))
}

async fn get_pc(State(s): State<Server>) -> (StatusCode, Json<HashMap<String, PC>>) {
    (StatusCode::OK, Json(s.pc.read().await.clone()))
}

async fn send_chat(
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

async fn edit_pswd(
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

async fn edit_player(
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
async fn edit_pc(
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

async fn act(State(s): State<Server>, head: HeaderMap, Json(req): Json<req::Act>) -> StatusCode {
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

async fn sync(State(s): State<Server>, head: HeaderMap) -> StatusCode {
    if let Some(_) = s.auth(&head).await {};
    todo!()
}

async fn cmd(State(_s): State<Server>) -> (StatusCode, Json<Result<String, String>>) {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(Err(
            vitium_common::json(&UnimplError("cmd".to_string())).unwrap()
        )),
    )
}

/// Command executors. Note that permission will **NOT** be verified.
pub mod exec {
    use std::error::Error;

    use super::Server;
    use serde::{Deserialize, Serialize};
    use vitium_common::{error::UnimplError, player::NoPlayerError};
    pub fn hello() -> String {
        "Hello, world!".to_string()
    }
    pub async fn grant(s: &Server, player: &str) -> Result<String, String> {
        let p = s.player.read().await;
        let mut o = s.op.write().await;
        if !p.contains_key(player) {
            Err(NoPlayerError(player.to_string()).to_string())
        } else if o.contains(player) {
            Ok(format!(
                "{} is already operator, no modification is made",
                player
            ))
        } else {
            o.insert(player.to_string());
            Ok(format!("opped {}", player))
        }
    }
    pub async fn shutdown(
        _s: &Server,
    ) -> Result<String, impl Error + Serialize + Deserialize<'static>> {
        Err(UnimplError("shutdown".to_string()))
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
