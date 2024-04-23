use axum::{
    extract::State,
    http::{header::AUTHORIZATION, HeaderMap, StatusCode},
    routing::{get, post},
    Json, Router,
};
use http_auth_basic::Credentials;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    error::Error,
    ops::{Deref, DerefMut},
    path::Path,
    sync::Arc,
};
use tokio::{
    fs,
    io::{self, AsyncWriteExt},
    net::TcpListener,
    signal,
    sync::RwLock,
};
use tower_http::trace::TraceLayer;
use tracing::{error, warn};
use vitium_common::{
    // cmd::Echo,
    cmd::Echo,
    error::UnimplError,
    game::PC,
    player::Player,
    req::{self, Chat},
};

use crate::game::Game;

pub struct ServerInst {
    pub cfg: ServerConfig,
    player: RwLock<HashMap<String, Player>>,
    pswd: RwLock<HashMap<String, String>>,
    pc: RwLock<HashMap<String, PC>>,
    op: RwLock<HashSet<String>>,
    chat: RwLock<VecDeque<(String, Chat)>>,
    game: HashMap<String, Game>,
}

/// Defines the server. This is a more abstract one, see `crate::game` for specific game logics.
/// ```
/// use crate::server::Server;
/// Server::default()
///     .run()
///     .await
///     .expect("internal server error");
/// ```
#[derive(Clone)]
pub struct Server(Arc<ServerInst>);

impl Deref for Server {
    type Target = Arc<ServerInst>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Server {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for Server {
    fn default() -> Self {
        Self(Arc::new(ServerInst {
            cfg: ServerConfig::default(),
            player: RwLock::new(HashMap::new()),
            pswd: RwLock::new(HashMap::new()),
            pc: RwLock::new(HashMap::new()),
            op: RwLock::new(HashSet::new()),
            chat: RwLock::new(VecDeque::new()),
            game: HashMap::new(),
        }))
    }
}

impl Server {
    pub fn with_cfg(cfg: ServerConfig) -> Self {
        Self(Arc::new(ServerInst {
            cfg,
            player: RwLock::new(HashMap::new()),
            pswd: RwLock::new(HashMap::new()),
            pc: RwLock::new(HashMap::new()),
            op: RwLock::new(HashSet::new()),
            chat: RwLock::new(VecDeque::new()),
            game: HashMap::new(),
        }))
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
    /// Consumes `self` and start the server.
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

async fn act(
    State(s): State<Server>,
    head: HeaderMap,
    Json(req): Json<req::Act<'_>>,
) -> StatusCode {
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

async fn cmd(State(_s): State<Server>) -> (StatusCode, Json<Echo>) {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(Err(UnimplError("/api/cmd".to_string()).to_string())),
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

/// Server configuration.
#[derive(Serialize, Deserialize, Clone)]
pub struct ServerConfig {
    pub port: u16,
    pub chat_cap: usize,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            port: 10987,
            chat_cap: 255,
        }
    }
}

impl ServerConfig {
    pub async fn load(path: &Path) -> Result<Self, Box<dyn Error>> {
        if !path.exists() {
            let err = io::Error::new(
                io::ErrorKind::NotFound,
                format!("{} not found", path.display()),
            );
            let f = fs::File::create(path);
            let cfg = Self::default();
            f.await?
                .write(toml::to_string(&cfg).unwrap().as_bytes())
                .await?;
            Err(Box::new(err))
        } else {
            let s = fs::read_to_string(path).await?;
            Ok(toml::from_str::<ServerConfig>(&s)?)
        }
    }
    pub async fn try_load(path: &Path) -> Self {
        match Self::load(path).await {
            Ok(cfg) => cfg,
            Err(e) => {
                error!(
                    "failed to load server config at \"{}\": \"{}\"",
                    path.display(),
                    e
                );
                warn!("using default config instead");
                Self::default()
            }
        }
    }
}
