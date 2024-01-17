use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
    path::Path,
    sync::Arc,
};
use tokio::{
    net::TcpListener,
    signal,
    sync::{Mutex, MutexGuard},
};
use tower_http::trace::TraceLayer;
use tracing::error;
use vitium_common::{
    chara::Chara,
    config::{obj, toml, ServerConfig},
    player::{Player, Token},
    req::{self, Chat},
};

type Lock<T> = Arc<Mutex<T>>;
fn lock<T>(value: T) -> Lock<T> {
    Arc::new(Mutex::new(value))
}

#[derive(Clone)]
pub struct Server {
    pub cfg: ServerConfig,
    _player: Lock<HashMap<String, Player>>,
    _pswd: Lock<HashMap<String, String>>,
    _chara: Lock<HashMap<String, Chara>>,
    _op: Lock<HashSet<String>>,
    _chat: Lock<VecDeque<Chat>>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            cfg: ServerConfig::new(),
            _player: lock(HashMap::new()),
            _pswd: lock(HashMap::new()),
            _chara: lock(HashMap::new()),
            _op: lock(HashSet::new()),
            _chat: lock(VecDeque::new()),
        }
    }
    pub async fn player(&self) -> MutexGuard<'_, HashMap<String, Player>> {
        self._player.lock().await
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
    pub async fn verify(&self, token: &Token) -> bool {
        if let Some(pswd) = self.pswd().await.get(&token.id) {
            pswd == &token.pswd
        } else {
            false
        }
    }
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
            .route("/player", post(edit_player))
            .route("/chara", post(edit_chara))
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

async fn edit_player(State(s): State<Server>, Json(req): Json<req::EditPlayer>) -> StatusCode {
    let mut dat = s.player().await;
    if let Some(player) = dat.get_mut(&req.player.id) {
        if let Some(token) = req.token {
            if s.verify(&token).await {
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

async fn edit_chara(State(s): State<Server>, Json(req): Json<req::EditChara>) -> StatusCode {
    let mut dat = s.chara().await;
    if !s.verify(&req.token).await {
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
