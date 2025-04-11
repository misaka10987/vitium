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

use axum_server::{tls_rustls::RustlsConfig, Handle};
use basileus::Basileus;
use chat::ChatModule;
use serde::{Deserialize, Serialize};
use sqlx::{query, sqlite::SqliteConnectOptions, SqlitePool};
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    net::ToSocketAddrs,
    ops::{Deref, DerefMut},
    path::PathBuf,
    sync::Arc,
    time::{Duration, SystemTime},
};
use tokio::sync::RwLock;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::error;
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
    basileus: Basileus,
    op: RwLock<HashSet<String>>,
    chat: ChatModule,
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
            basileus: Basileus::new(Default::default()).await?,
            op: RwLock::const_new(HashSet::new()),
            chat: ChatModule::new(),
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
    async fn dev_hooks(&self) -> anyhow::Result<()> {
        use basileus::{pass::PassManage, user::UserManage};
        if !self.basileus.exist_user("dev").await? {
            self.basileus.create_user("dev").await?;
        }
        self.basileus.update_pass("dev", "dev").await?;
        Ok(())
    }

    /// Consumes `self` and start the server.
    pub async fn start(self) -> anyhow::Result<()> {
        #[cfg(debug_assertions)]
        self.dev_hooks().await?;
        let addr = format!("[::]:{}", self.cfg.port)
            .to_socket_addrs()
            .unwrap()
            .next()
            .unwrap();
        let tls_cfg = match &self.cfg.ssl {
            Some(SSLConfig { cert, key }) => {
                Some(RustlsConfig::from_pem_file(cert, key).await.unwrap())
            }
            None => None,
        };
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
            .layer(CorsLayer::very_permissive())
            .layer(TraceLayer::new_for_http())
            .with_state(self);
        let handle = Handle::new();
        let shutdown_handle = handle.clone();
        let shutdown = async move {
            recv_shutdown().await;
            shutdown_handle.graceful_shutdown(Some(Duration::from_secs(30)));
        };
        tokio::spawn(shutdown);
        let res = if let Some(cfg) = tls_cfg {
            axum_server::bind_rustls(addr, cfg)
                .handle(handle)
                .serve(app.into_make_service())
                .await
        } else {
            axum_server::bind(addr)
                .handle(handle)
                .serve(app.into_make_service())
                .await
        };
        Ok(res?)
    }
}

impl AsRef<ChatModule> for Server {
    fn as_ref(&self) -> &ChatModule {
        &self.chat
    }
}

impl AsRef<SqlitePool> for Server {
    fn as_ref(&self) -> &SqlitePool {
        &self.db
    }
}

impl AsRef<Basileus> for Server {
    fn as_ref(&self) -> &Basileus {
        &self.basileus
    }
}

/// Command executors. Note that permission will **NOT** be verified.
pub mod exec {
    pub fn hello() -> String {
        "Hello, world!".to_string()
    }
}

/// Server configuration.
#[derive(Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    pub ssl: Option<SSLConfig>,
    pub chat_cap: usize,
    pub page_url: String,
    #[serde(default)]
    pub motd: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            port: 10987,
            ssl: None,
            chat_cap: 255,
            page_url: "https://github.com/misaka10987/vitium".into(),
            motd: String::new(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SSLConfig {
    pub cert: PathBuf,
    pub key: PathBuf,
}

impl Default for SSLConfig {
    fn default() -> Self {
        Self {
            cert: "./cert.pem".into(),
            key: "./key.pem".into(),
        }
    }
}

fn internal_server_error(err: impl Error) -> StatusCode {
    error!("{err}");
    StatusCode::INTERNAL_SERVER_ERROR
}
