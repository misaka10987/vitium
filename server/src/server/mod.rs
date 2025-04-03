mod auth;
mod chat;
mod cmd;
mod handler;
mod profile;

use anyhow::bail;
use axum::{
    http::StatusCode,
    response::Redirect,
    routing::{any, get, post},
    Json, Router,
};

use axum_pass::safe::Safe;
use chat::ChatStore;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    ops::{Deref, DerefMut},
    path::PathBuf,
    sync::Arc,
    time::SystemTime,
};
use tokio::{net::TcpListener, sync::RwLock};
use vitium_api::{game::PlayerChar, user::UserProfile};

use crate::recv_shutdown;

// use crate::game::{self, Game};

pub struct ServerInst {
    pub cfg: ServerConfig,
    player: RwLock<HashMap<String, UserProfile>>,
    safe: Safe,
    pc: RwLock<HashMap<String, PlayerChar>>,
    op: RwLock<HashSet<String>>,
    pub chat: ChatStore,
    // pub game: RwLock<Game>,
}

/// Defines the server. This is a more abstract one, see `crate::game` for specific game logics.
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
        let chat = ChatStore::new(cfg.chat_cap);
        Self(Arc::new(ServerInst {
            cfg,
            player: RwLock::new(HashMap::new()),
            safe: Safe::new("./password.db").await.unwrap(),
            pc: RwLock::new(HashMap::new()),
            op: RwLock::new(HashSet::new()),
            chat,
            // game: RwLock::new(Game::new()),
        }))
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

    #[cfg(debug_assertions)]
    async fn dev_hooks(&self) {
        let _ = self.safe.create("dev", "dev").await;
    }

    /// Consumes `self` and start the server.
    pub async fn run(self) -> anyhow::Result<()> {
        #[cfg(debug_assertions)]
        self.dev_hooks().await;
        let listener = TcpListener::bind(format!("localhost:{}", self.cfg.port)).await?;
        // .nest("/act", game::act_handler());
        let app = Router::new()
            .route("/", get(Redirect::to(&self.cfg.page_url)))
            .route("/ping", get(|| async { Json(SystemTime::now()) }))
            .nest("/auth", auth::rest())
            .route("/hello", get("Hello, world!"))
            .nest("/profile", profile::rest())
            .nest("/chat", chat::rest())
            .route("/pc", get(handler::list_pc))
            .route("/pc/{name}", get(handler::get_pc))
            .route("/pc/{name}", post(handler::edit_pc))
            .route("/sync", get(handler::sync))
            .fallback(any(StatusCode::NOT_FOUND))
            .with_state(self);
        let res = axum::serve(listener, app)
            .with_graceful_shutdown(recv_shutdown())
            .await;
        Ok(res?)
    }
}

impl AsRef<Safe> for Server {
    fn as_ref(&self) -> &Safe {
        &self.safe
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
