use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
    net::IpAddr,
    path::Path,
    sync::Arc,
};
use tokio::{
    net::TcpListener,
    signal,
    sync::{Mutex, MutexGuard},
};
use tower_http::trace::TraceLayer;
use tracing::{error, info, trace};
use vitium_common::{
    chara::Chara,
    cmd::{Command, Echo},
    config::{obj, toml, ServerConfig},
    player::{Player, Token},
    req::{self, Chat, Cmd},
    sync::Sync,
};

use crate::game::Game;

type Lock<T> = Arc<Mutex<T>>;
fn lock<T>(value: T) -> Lock<T> {
    Arc::new(Mutex::new(value))
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
    _player: Lock<HashMap<String, Player>>,
    _banned_player: Lock<HashSet<String>>,
    _banned_ip: Lock<HashSet<IpAddr>>,
    _pswd: Lock<HashMap<String, String>>,
    _chara: Lock<HashMap<String, Chara>>,
    _op: Lock<HashSet<String>>,
    _chat: Lock<VecDeque<Chat>>,
    _game: Lock<Game>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            cfg: ServerConfig::new(),
            _player: lock(HashMap::new()),
            _banned_player: lock(HashSet::new()),
            _banned_ip: lock(HashSet::new()),
            _pswd: lock(HashMap::new()),
            _chara: lock(HashMap::new()),
            _op: lock(HashSet::new()),
            _chat: lock(VecDeque::new()),
            _game: lock(Game::new()),
        }
    }
    pub async fn player(&self) -> MutexGuard<'_, HashMap<String, Player>> {
        self._player.lock().await
    }
    pub async fn banned_player(&self) -> MutexGuard<'_, HashSet<String>> {
        self._banned_player.lock().await
    }
    pub async fn banned_ip(&self) -> MutexGuard<'_, HashSet<IpAddr>> {
        self._banned_ip.lock().await
    }
    pub async fn pswd(&self) -> MutexGuard<'_, HashMap<String, String>> {
        self._pswd.lock().await
    }
    pub async fn chara(&self) -> MutexGuard<'_, HashMap<String, Chara>> {
        self._chara.lock().await
    }
    pub async fn op(&self) -> MutexGuard<'_, HashSet<String>> {
        self._op.lock().await
    }
    pub async fn chat(&self) -> MutexGuard<'_, VecDeque<Chat>> {
        self._chat.lock().await
    }
    pub async fn game(&self) -> MutexGuard<'_, Game> {
        self._game.lock().await
    }
    /// Verifies if a `Token` is valid.
    pub async fn verify(&self, token: &Token) -> bool {
        if let Some(pswd) = self.pswd().await.get(&token.id) {
            pswd == &token.pswd
        } else {
            false
        }
    }
    /// Verifies if a `Token` is valid **and** has operator access.
    pub async fn sudoer(&self, token: &Token) -> bool {
        self.verify(token).await && self.op().await.contains(&token.id)
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
            .route("/chara", get(get_chara))
            .route("/chat", post(send_chat))
            .route("/pswd", post(edit_pswd))
            .route("/player", post(edit_player))
            .route("/chara", post(edit_chara))
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

async fn recv_chat(State(s): State<Server>) -> (StatusCode, Json<VecDeque<Chat>>) {
    (StatusCode::OK, Json(s.chat().await.clone()))
}

async fn get_player(State(s): State<Server>) -> (StatusCode, Json<HashMap<String, Player>>) {
    (StatusCode::OK, Json(s.player().await.clone()))
}

async fn get_chara(State(s): State<Server>) -> (StatusCode, Json<HashMap<String, Chara>>) {
    (StatusCode::OK, Json(s.chara().await.clone()))
}

async fn send_chat(State(s): State<Server>, Json(req): Json<req::SendChat>) -> StatusCode {
    if !s.verify(&req.token).await {
        StatusCode::FORBIDDEN
    } else if req.token.id != req.chat.player {
        StatusCode::FORBIDDEN
    } else {
        let mut dat = s.chat().await;
        while dat.len() >= s.cfg.chat_cap {
            dat.pop_front();
        }
        let mut content = req.chat;
        dat.push_back(content.renew().clone());
        StatusCode::ACCEPTED
    }
}

async fn edit_pswd(State(s): State<Server>, Json(req): Json<req::EditPswd>) -> StatusCode {
    if !s.verify(&req.token).await {
        StatusCode::FORBIDDEN
    } else if let Some(p) = s.pswd().await.get_mut(&req.token.id) {
        *p = req.pswd;
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}

async fn edit_player(State(s): State<Server>, Json(req): Json<req::EditPlayer>) -> StatusCode {
    let mut dat = s.player().await;
    if let Some(player) = dat.get_mut(&req.player.id) {
        if s.verify(&req.token).await {
            *player = req.player.clone();
            StatusCode::ACCEPTED
        } else {
            StatusCode::FORBIDDEN
        }
    } else {
        dat.insert(req.player.id.clone(), req.player.clone());
        s.pswd().await.insert(req.player.id, req.token.pswd);
        StatusCode::CREATED
    }
}

/// Handler for `POST /chara`.
async fn edit_chara(State(s): State<Server>, Json(req): Json<req::EditChara>) -> StatusCode {
    let mut dat = s.chara().await;
    if !s.verify(&req.token).await {
        StatusCode::FORBIDDEN
    } else if let Some(chara) = dat.get_mut(&req.dest) {
        if chara.player != req.token.id {
            return StatusCode::FORBIDDEN;
        }
        *chara = req.new;
        StatusCode::ACCEPTED
    } else {
        dat.insert(req.dest, req.new);
        StatusCode::CREATED
    }
}

async fn act(State(s): State<Server>, Json(req): Json<req::Act>) -> StatusCode {
    if !s.verify(&req.token).await {
        // token is invalid
        StatusCode::FORBIDDEN
    } else if let Some(c) = s.chara().await.get(&req.token.id) {
        if c.player == req.token.id {
            if !s.game().await.enrolled(req.chara).await {
                return StatusCode::NOT_FOUND;
            }
            match s.game().await.proc(req).await.await {
                Ok(c) => c,
                Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
            }
        } else {
            // the request has a token but not matches the character it operates on
            StatusCode::UNAUTHORIZED
        }
    } else {
        // trying to request act on a non-exist character
        StatusCode::NOT_FOUND
    }
}

async fn sync(State(s): State<Server>, Json(req): Json<req::Token>) -> (StatusCode, Json<Sync>) {
    if !s.verify(&req).await {
        (StatusCode::FORBIDDEN, Json(Sync::new()))
    } else {
        let (code, data) = s.game().await.fetch(req.id);
        (code, Json(data))
    }
}

macro_rules! perm_deny {
    () => {
        Echo {
            value: -1,
            output: "Permission denied.".to_string(),
        }
    };
}

async fn cmd(State(s): State<Server>, Json(req): Json<Cmd>) -> (StatusCode, Json<Echo>) {
    if !s.verify(&req.token).await {
        (StatusCode::FORBIDDEN, Json(perm_deny!()))
    } else if req.cmd.op() && !s.sudoer(&req.token).await {
        (StatusCode::FORBIDDEN, Json(perm_deny!()))
    } else {
        let p = req.token.id;
        info!("executing command issued by player[id={}]", p);
        let code = StatusCode::ACCEPTED;
        let echo = match req.cmd {
            Command::Hello => exec::hello(),
            Command::Grant(p) => exec::grant(&s, &p).await,
            Command::ShutDown => exec::shutdown(&s).await,
        };
        trace!("player[id={}]: {}", p, echo.output);
        info!("player[id={}]'s command returned {}", p, echo.value);
        (code, Json(echo))
    }
}

/// Command executors. Note that permission will **NOT** be verified.
pub mod exec {
    use super::Server;
    use tokio::{process, spawn};
    use tracing::info;
    use vitium_common::cmd::Echo;
    pub fn hello() -> Echo {
        Echo {
            value: 0,
            output: "Hello, world!\n".to_string(),
        }
    }
    pub async fn grant(s: &Server, player: &str) -> Echo {
        let p = s.player().await;
        let mut o = s.op().await;
        if !p.contains_key(player) {
            Echo {
                value: 1,
                output: format!("player[id={}] non exist", player),
            }
        } else if o.contains(player) {
            Echo {
                value: 2,
                output: format!("player[id={}] is already operator", player),
            }
        } else {
            o.insert(player.to_string());
            Echo {
                value: 0,
                output: format!("opped player[id={}]", player),
            }
        }
    }
    pub async fn shutdown(s: &Server) -> Echo {
        info!("shutting down internal server");
        s.game().await.shutdown().await;
        spawn(async {
            process::Command::new(format!("kill -s SIGINT {}", std::process::id()));
        });
        Echo {
            value: 0,
            output: "exit".to_string(),
        }
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
