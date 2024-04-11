use axum::{
    extract::State,
    http::{header::AUTHORIZATION, HeaderMap, StatusCode},
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
use tokio::{net::TcpListener, signal, sync::RwLock};
use tower_http::trace::TraceLayer;
use tracing::error;
use vitium_common::{
    cmd::Echo,
    config::{obj, toml, ServerConfig},
    player::Player,
    req::{self, Chat},
    sync::Sync,
    PC,
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
    banned_player: Lock<HashSet<String>>,
    banned_ip: Lock<HashSet<IpAddr>>,
    pswd: Lock<HashMap<String, String>>,
    pc: Lock<HashMap<String, PC>>,
    op: Lock<HashSet<String>>,
    chat: Lock<VecDeque<(String, Chat)>>,
    game: Lock<Game>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            cfg: ServerConfig::new(),
            player: lock(HashMap::new()),
            banned_player: lock(HashSet::new()),
            banned_ip: lock(HashSet::new()),
            pswd: lock(HashMap::new()),
            pc: lock(HashMap::new()),
            op: lock(HashSet::new()),
            chat: lock(VecDeque::new()),
            game: lock(Game::new()),
        }
    }
    /// Reads from the header and get authentication info.
    pub async fn auth(&self, head: &HeaderMap) -> Option<String> {
        if let Some(token) = head.get(AUTHORIZATION) {
            if let Ok(s) = token.to_str() {
                let v: Vec<_> = s.split(' ').collect();
                if v.len() == 3 && v[0] == "vitium" {
                    return self.pswd.read().await.get(v[1]).map_or(None, |p| {
                        if v[2] == p {
                            Some(v[1].to_string())
                        } else {
                            None
                        }
                    });
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
        if let Some(c) = s.pc.read().await.get(&req.chara) {
            if c.player == name {
                // if !s.game().await.enrolled(&req.chara).await {
                //     return StatusCode::NOT_FOUND;
                // }
                // match s.game().await.proc(req).await.await {
                //     Ok(c) => c,
                //     Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
                // }
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

async fn sync(State(s): State<Server>, head: HeaderMap) -> (StatusCode, Json<Sync>) {
    if let Some(_) = s.auth(&head).await {};
    todo!()
}

async fn cmd() -> (StatusCode, Json<Echo>) {
    todo!()
}

/// Command executors. Note that permission will **NOT** be verified.
pub mod exec {
    use super::Server;
    use vitium_common::cmd::Echo;
    pub fn hello() -> Echo {
        Echo {
            value: 0,
            output: "Hello, world!\n".to_string(),
        }
    }
    pub async fn grant(s: &Server, player: &str) -> Echo {
        let p = s.player.read().await;
        let mut o = s.op.write().await;
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
    pub async fn shutdown(_s: &Server) -> Echo {
        todo!()
        // info!("shutting down internal server");
        // s.game().await.shutdown().await;
        // spawn(async {
        //     process::Command::new(format!("kill -s SIGINT {}", std::process::id()));
        // });
        // Echo {
        //     value: 0,
        //     output: "exit".to_string(),
        // }
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
