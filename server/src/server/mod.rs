mod auth;
mod chat;
mod cmd;
mod log;
mod prelude;
mod profile;
#[cfg(debug_assertions)]
mod test;

use axum::{
    Json, Router,
    http::StatusCode,
    response::Redirect,
    routing::{any, get},
};
use basileus::Basileus;
use chat::ChatModule;
use cmd::CommandModule;
use log::LogModule;
use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;
use shutup::ShutUp;
use sqlx::{SqlitePool, query, sqlite::SqliteConnectOptions};
use std::{
    collections::HashMap,
    error::Error,
    net::{Ipv6Addr, SocketAddr},
    ops::Deref,
    path::PathBuf,
    sync::{Arc, atomic::AtomicBool},
    time::SystemTime,
};
use tokio::{net::TcpListener, sync::RwLock};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{error, info};
use vitium_api::user::UserProfile;

pub use prelude::*;

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
    pub cfg: Config,
    pub shutdown: ShutUp,
    started: AtomicBool,
    db: SqlitePool,
    player: RwLock<HashMap<String, UserProfile>>,
    basileus: Basileus,
    chat: ChatModule,
    cmd: CommandModule,
    log: LogModule,
}

/// Interface to the entire server.
#[derive(Clone)]
pub struct Server(Arc<ServerInst>);

impl Deref for Server {
    type Target = Arc<ServerInst>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Server {
    /// Create a server with specified configuration.
    pub async fn new(cfg: Config) -> anyhow::Result<Self> {
        let conn_opt = SqliteConnectOptions::new()
            .filename(&cfg.db)
            .create_if_missing(true);
        let db = SqlitePool::connect_with(conn_opt).await?;
        query(DB_INIT_QUERY).execute(&db).await?;
        let shutdown = ShutUp::new();
        let log = LogModule::new(cfg.log.clone())?;
        let val = Self(Arc::new(ServerInst {
            cfg,
            shutdown,
            started: AtomicBool::new(false),
            db,
            player: RwLock::const_new(HashMap::new()),
            basileus: Basileus::new(Default::default()).await?,
            chat: ChatModule::new(),
            cmd: CommandModule::new(),
            log,
        }));
        Ok(val)
    }

    pub fn started(&self) -> bool {
        self.started.load(std::sync::atomic::Ordering::Relaxed)
    }

    #[cfg(debug_assertions)]
    async fn dev_hooks(&self) -> anyhow::Result<()> {
        use basileus::{pass::PassManage, user::UserManage};
        if !self.exist_user("dev").await? {
            self.create_user("dev").await?;
        }
        self.update_pass("dev", "dev").await?;
        Ok(())
    }

    /// Consumes `self` and start the server.
    pub async fn start(self) -> anyhow::Result<ShutUp> {
        if self.started() {
            panic!("can not start server for multiple times")
        }
        self.started
            .store(true, std::sync::atomic::Ordering::Relaxed);

        #[cfg(debug_assertions)]
        self.dev_hooks().await?;

        self.print_cmd_output(self.shutdown.child());

        let port = self.cfg.port.unwrap_or(0);
        let ip = if self.cfg.direct_api {
            Ipv6Addr::UNSPECIFIED
        } else {
            Ipv6Addr::LOCALHOST
        };
        let addr = SocketAddr::from((ip, port));

        let listener = TcpListener::bind(addr).await?;
        info!("start server on {}", listener.local_addr()?);

        let app = Router::new();
        #[cfg(debug_assertions)]
        let app = app.nest("/test", test::router());
        let app = app
            .route("/ping", get(|| async { Json(SystemTime::now()) }))
            .route("/hello", get("Hello, world!"))
            .route("/contact", get(Redirect::temporary(&self.cfg.contact)))
            .nest("/auth", auth::rest())
            .nest("/chat", chat::rest())
            .nest("/cmd", cmd::rest())
            .nest("/profile", profile::rest())
            .fallback(any(StatusCode::NOT_FOUND))
            .layer(CorsLayer::new())
            .layer(TraceLayer::new_for_http());

        let fut = self.shutdown.wait();
        let shutdown = self.shutdown.clone();

        tokio::spawn(async move {
            axum::serve(listener, app.with_state(self))
                .with_graceful_shutdown(fut)
                .await
                .unwrap()
        });
        Ok(shutdown)
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

impl AsRef<LogModule> for Server {
    fn as_ref(&self) -> &LogModule {
        &self.log
    }
}

/// Server configuration.
#[serde_inline_default]
#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    /// Port to start the API server on.
    pub port: Option<u16>,
    /// Path to the server database.
    #[serde_inline_default("./server.db".into())]
    pub db: PathBuf,
    /// Whether to allow direct access to the API server via HTTP from remote.
    #[serde_inline_default(false)]
    pub direct_api: bool,
    /// Configurations for logging.
    #[serde(default)]
    pub log: log::Config,
    /// Message of the day.
    #[serde(default)]
    pub motd: String,
    /// URL to contact the game server administrator, e.g. `mailto:example@example.org`.
    #[serde(default)]
    pub contact: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            port: None,
            db: "./server.db".into(),
            direct_api: false,
            log: Default::default(),
            motd: String::new(),
            contact: Default::default(),
        }
    }
}

fn internal_server_error(err: impl Error) -> StatusCode {
    error!("{err}");
    StatusCode::INTERNAL_SERVER_ERROR
}
