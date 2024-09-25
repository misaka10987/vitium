mod handler;

use axum::{
    http::StatusCode,
    response::Redirect,
    routing::{any, get, post},
    Router,
};
use axum_extra::extract::CookieJar;
use safe_box::SafeBox;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    io,
    ops::{Deref, DerefMut},
    path::PathBuf,
    sync::Arc,
    time::SystemTime,
};
use tokio::{
    net::TcpListener,
    signal, spawn,
    sync::{oneshot, RwLock},
};
use tower_http::trace::TraceLayer;
use tracing::trace;
use vitium_api::{game::PC, net::Chat, player::Player};

use crate::input::proc;

// use crate::game::{self, Game};

pub struct ServerInst {
    pub cfg: ServerConfig,
    player: RwLock<HashMap<String, Player>>,
    safe: SafeBox,
    pc: RwLock<HashMap<String, PC>>,
    op: RwLock<HashSet<String>>,
    chat: RwLock<VecDeque<Chat>>,
    // pub game: RwLock<Game>,
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

impl Server {
    /// Create a server with specified configuration.
    pub async fn new(cfg: ServerConfig) -> Self {
        Self(Arc::new(ServerInst {
            cfg,
            player: RwLock::new(HashMap::new()),
            safe: SafeBox::new("./password.db").await.unwrap(),
            pc: RwLock::new(HashMap::new()),
            op: RwLock::new(HashSet::new()),
            chat: RwLock::new(VecDeque::new()),
            // game: RwLock::new(Game::new()),
        }))
    }

    /// Reads from the header and get authentication info.
    pub fn auth(&self, jar: &CookieJar) -> Option<String> {
        if let Some(token) = jar.get("token") {
            let token = token.value();
            match self.safe.verify_token(token) {
                Ok(user) => {
                    trace!("authorized {user} by token");
                    return Some(user);
                }
                Err(e) => trace!("failed to authorize with token {token}: {e}"),
            }
        }
        None
    }

    pub async fn push_chat(&self, chat: Chat) -> SystemTime {
        let chat = chat.received();
        let t = chat.recv_time;
        let mut w = self.chat.write().await;
        w.push_front(chat);
        w.truncate(self.cfg.chat_cap);
        t
    }

    /// Consumes `self` and start the server.
    pub async fn run(self) -> Result<(), std::io::Error> {
        let listener = TcpListener::bind(format!("localhost:{}", self.cfg.port))
            .await
            .expect("failed to bind TCP listener");
        let auth = Router::new()
            .route("/login", get(handler::login))
            .route("/signup", post(handler::signup))
            .route("/pass", post(handler::edit_pass));
        let api = Router::new()
            .nest("/auth", auth)
            .route("/hello", get("Hello, world!"))
            .route("/chat", get(handler::recv_chat))
            .route("/chat", post(handler::send_chat))
            .route("/player", get(handler::list_player))
            .route("/player/*name", get(handler::get_player))
            .route("/player/*name", post(handler::edit_player))
            .route("/pc", get(handler::list_pc))
            .route("/pc/*name", get(handler::get_pc))
            .route("/pc/*name", post(handler::edit_pc))
            .route("/sync", get(handler::sync))
            .route("/cmd", post(handler::cmd));
        // .nest("/act", game::act_handler());
        let app = Router::new()
            .route("/", get(Redirect::to(&self.cfg.page_url)))
            .nest("/api", api)
            .fallback(any(StatusCode::NOT_FOUND))
            .with_state(self)
            .layer(TraceLayer::new_for_http());
        axum::serve(listener, app)
            .with_graceful_shutdown(sig_shut())
            .await
    }

    pub fn input(&self) -> oneshot::Sender<()> {
        let (s, mut r) = oneshot::channel();
        let server = self.clone();
        spawn(async move {
            while let Err(_) = r.try_recv() {
                let mut buf = String::new();
                io::stdin().read_line(&mut buf).unwrap();
                if let Err(e) = proc(&buf, &server).await {
                    eprintln!("  !! {}", e)
                }
            }
        });
        s
    }
}

/// Command executors. Note that permission will **NOT** be verified.
pub mod exec {
    pub fn hello() -> String {
        "Hello, world!".to_string()
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
    pub host_dir: PathBuf,
    pub port: u16,
    pub chat_cap: usize,
    pub page_url: String,
    #[serde(default)]
    pub motd: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host_dir: PathBuf::from("."),
            port: 10987,
            chat_cap: 255,
            page_url: "https://github.com/misaka10987/vitium".into(),
            motd: String::new(),
        }
    }
}
