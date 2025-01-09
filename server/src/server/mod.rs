mod chat;
mod cmd;
mod handler;

use anyhow::bail;
use axum::{
    http::StatusCode,
    response::Redirect,
    routing::{any, get, post},
    Json, Router,
};
use axum_extra::extract::CookieJar;
use chat::ChatSto;
use safe_box::SafeBox;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    ops::{Deref, DerefMut},
    path::PathBuf,
    sync::Arc,
    time::SystemTime,
};
use tokio::{net::TcpListener, sync::RwLock};
use tracing::trace;
use vitium_api::{game::PC, player::Player};

use crate::recv_shutdown;

// use crate::game::{self, Game};

pub struct ServerInst {
    pub cfg: ServerConfig,
    player: RwLock<HashMap<String, Player>>,
    safe: SafeBox,
    pc: RwLock<HashMap<String, PC>>,
    op: RwLock<HashSet<String>>,
    pub chat: ChatSto,
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
        let chat = ChatSto::new(cfg.chat_cap);
        Self(Arc::new(ServerInst {
            cfg,
            player: RwLock::new(HashMap::from([(
                "foo".into(),
                Player {
                    display_name: "Foo".into(),
                    profile: None,
                },
            )])),
            safe: SafeBox::new("./password.db").await.unwrap(),
            pc: RwLock::new(HashMap::new()),
            op: RwLock::new(HashSet::new()),
            chat,
            // game: RwLock::new(Game::new()),
        }))
    }

    /// Reads from the header and get authentication info.
    pub fn auth(&self, jar: &CookieJar) -> Option<String> {
        if let Some(token) = jar.get("token") {
            let token = token.value();
            match self.safe.verify_token(token) {
                Ok(user) => return Some(user),
                Err(e) => trace!("failed to authorize with token {token}: {e}"),
            }
        }
        None
    }

    pub async fn is_op(&self, user: &str) -> bool {
        self.op.read().await.contains(user)
    }

    pub async fn op(&self, user: &str) -> anyhow::Result<()> {
        if !self.player.read().await.contains_key(user) {
            bail!("user '{user}' does not exist")
        }
        self.op.write().await.insert(user.into());
        Ok(())
    }

    pub async fn deop(&self, user: &str) -> anyhow::Result<()> {
        if !self.op.write().await.remove(user) {
            bail!("user '{user}' is not operator yet")
        }
        Ok(())
    }

    /// Consumes `self` and start the server.
    pub async fn run(self) -> anyhow::Result<()> {
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
            .route("/sync", get(handler::sync));
        // .nest("/act", game::act_handler());
        let app = Router::new()
            .route("/", get(Redirect::to(&self.cfg.page_url)))
            .route("/ping", get(|| async { Json(SystemTime::now()) }))
            .nest("/api", api)
            .fallback(any(StatusCode::NOT_FOUND))
            .with_state(self);
        let res = axum::serve(listener, app)
            .with_graceful_shutdown(recv_shutdown())
            .await;
        Ok(res?)
    }
}

/// Command executors. Note that permission will **NOT** be verified.
pub mod exec {
    pub fn hello() -> String {
        "Hello, world!".to_string()
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
