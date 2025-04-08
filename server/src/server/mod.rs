mod auth;
mod chat;
mod cmd;
mod profile;
#[cfg(debug_assertions)]
mod test;

use anyhow::bail;
use axum::{
    http::StatusCode,
    response::Redirect,
    routing::{any, get},
    Json, Router,
};

use axum_pass::safe::Safe;
use chat::ChatServer;
use serde::{Deserialize, Serialize};
use sqlx::{query, sqlite::SqliteConnectOptions, SqlitePool};
use std::{
    collections::{HashMap, HashSet},
    ops::{Deref, DerefMut},
    path::PathBuf,
    sync::Arc,
    time::SystemTime,
};
use tokio::{net::TcpListener, sync::RwLock};
use vitium_api::user::UserProfile;

use crate::recv_shutdown;

// use crate::game::{self, Game};

const DB_INIT_QUERY: &'static str = r#"
CREATE TABLE IF NOT EXISTS chat (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    time INTEGER UNSIGNED,
    sender TEXT NOT NULL,
    content TEXT NOT NULL,
    html BOOLEAN NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_chat_time ON chat(time);
"#;

pub struct ServerInst {
    pub cfg: ServerConfig,
    db: SqlitePool,
    player: RwLock<HashMap<String, UserProfile>>,
    safe: Safe,
    op: RwLock<HashSet<String>>,
    pub chat: ChatServer,
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
    pub async fn new(cfg: ServerConfig) -> anyhow::Result<Self> {
        let conn_opt = SqliteConnectOptions::new()
            .filename("./server.db")
            .create_if_missing(true);
        let pool = SqlitePool::connect_with(conn_opt).await?;
        query(DB_INIT_QUERY).execute(&pool).await?;
        let value = Self(Arc::new(ServerInst {
            cfg,
            db: pool.clone(),
            player: RwLock::const_new(HashMap::new()),
            safe: Safe::new("./password.db").await?,
            op: RwLock::const_new(HashSet::new()),
            chat: ChatServer::new(pool.clone()),
        }));
        Ok(value)
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
        let app = Router::new();
        #[cfg(debug_assertions)]
        let app = app.nest("/test", test::router());
        let app = app
            .route("/", get(Redirect::to(&self.cfg.page_url)))
            .route("/ping", get(|| async { Json(SystemTime::now()) }))
            .route("/hello", get("Hello, world!"))
            .nest("/auth", auth::rest())
            .nest("/chat", chat::rest())
            .nest("/profile", profile::rest())
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
