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
    ops::{Deref, DerefMut},
    path::PathBuf,
    sync::Arc,
};
use tokio::{net::TcpListener, signal, sync::RwLock};
use tower_http::trace::TraceLayer;
use tracing::trace;
use vitium_api::{game::PC, net::Chat, player::Player};

// use crate::game::{self, Game};

pub struct ServerInst {
    pub cfg: ServerConfig,
    player: RwLock<HashMap<String, Player>>,
    safe: SafeBox,
    pc: RwLock<HashMap<String, PC>>,
    op: RwLock<HashSet<String>>,
    chat: RwLock<VecDeque<(String, Chat)>>,
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

    /// Consumes `self` and start the server.
    pub async fn run(self) -> Result<(), std::io::Error> {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", self.cfg.port))
            .await
            .expect("failed to bind TCP listener");
        let auth = Router::new()
            .route("/login", get(handler::login))
            .route("/pass", post(handler::edit_pass));
        let app = Router::new()
            .nest("/auth", auth)
            .route("/hello", get("Hello, world!"))
            .route("/chat", get(handler::recv_chat))
            .route("/chat", post(handler::send_chat))
            .route("/player", get(handler::get_player))
            .route("/player", post(handler::edit_player))
            .route("/pc", get(handler::get_pc))
            .route("/pc", post(handler::edit_pc))
            .route("/sync", get(handler::sync))
            .route("/cmd", post(handler::cmd));
        // .nest("/act", game::act_handler());
        let app = Router::new()
            .nest("/api", app)
            .route("/", get(Redirect::to(&self.cfg.page_url)))
            .fallback(any(StatusCode::NOT_FOUND))
            .with_state(self)
            .layer(TraceLayer::new_for_http());
        axum::serve(listener, app)
            .with_graceful_shutdown(sig_shut())
            .await
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
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host_dir: PathBuf::from("."),
            port: 10987,
            chat_cap: 255,
            page_url: "https://github.com/misaka10987/vitium".to_string(),
        }
    }
}
