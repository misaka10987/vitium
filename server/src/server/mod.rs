mod auth;
mod chat;
mod cmd;
mod prelude;
mod profile;
mod proxy;
#[cfg(debug_assertions)]
mod test;

use axum::{
    http::StatusCode,
    routing::{any, get},
    Json, Router,
};
use basileus::Basileus;
use chat::ChatModule;
use cmd::CommandModule;
use proxy::ProxyServer;
use serde::{Deserialize, Serialize};
use sqlx::{query, sqlite::SqliteConnectOptions, SqlitePool};
use std::{
    collections::HashMap,
    error::Error,
    net::{Ipv6Addr, SocketAddr},
    ops::{Deref, DerefMut},
    sync::Arc,
    time::SystemTime,
};
use tokio::{net::TcpListener, sync::RwLock};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{error, info};
use vitium_api::user::UserProfile;

use crate::wait_shutdown;

pub use prelude::*;

// use crate::game::{self, Game};

const DB_INIT_QUERY: &'static str = r#"
CREATE TABLE IF NOT EXISTS chat (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    time INTEGER UNSIGNED,
    sender TEXT,
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
    chat: ChatModule,
    cmd: CommandModule,
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
            chat: ChatModule::new(),
            cmd: CommandModule::new(),
        }));
        Ok(value)
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
        let port = self.cfg.port.unwrap_or(0);
        let ip = if self.cfg.direct_api {
            Ipv6Addr::UNSPECIFIED
        } else {
            Ipv6Addr::LOCALHOST
        };
        let addr = SocketAddr::from((ip, port));
        let listener = TcpListener::bind(addr).await?;
        info!("start API server on {}", listener.local_addr()?);
        let port = listener.local_addr()?.port();
        let proxy = ProxyServer::new(self.cfg.proxy.clone(), port);
        proxy.start()?;
        let app = Router::new();
        #[cfg(debug_assertions)]
        let app = app.nest("/test", test::router());
        let app = app
            .route("/ping", get(|| async { Json(SystemTime::now()) }))
            .route("/hello", get("Hello, world!"))
            .nest("/auth", auth::rest())
            .nest("/chat", chat::rest())
            .nest("/cmd", cmd::rest())
            .nest("/profile", profile::rest())
            .fallback(any(StatusCode::NOT_FOUND))
            .layer(CorsLayer::very_permissive())
            .layer(TraceLayer::new_for_http())
            .with_state(self);
        axum::serve(listener, app)
            .with_graceful_shutdown(wait_shutdown())
            .await?;
        Ok(())
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

impl AsRef<CommandModule> for Server {
    fn as_ref(&self) -> &CommandModule {
        &self.cmd
    }
}

fn f() -> bool {
    false
}

/// Server configuration.
#[derive(Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub port: Option<u16>,
    #[serde(default = "f")]
    pub direct_api: bool,
    pub proxy: proxy::Config,
    #[serde(default)]
    pub motd: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            port: None,
            direct_api: false,
            proxy: Default::default(),
            motd: String::new(),
        }
    }
}

fn internal_server_error(err: impl Error) -> StatusCode {
    error!("{err}");
    StatusCode::INTERNAL_SERVER_ERROR
}
