mod auth;
mod chat;
mod cmd;
mod log;
mod network;
mod prelude;
mod profile;
#[cfg(debug_assertions)]
mod test;

use ::http::HeaderValue;
use askama::Template;
use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::{Html, Redirect},
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
    iter::once,
    ops::Deref,
    path::PathBuf,
    sync::{Arc, atomic::AtomicBool},
    time::{Duration, SystemTime},
};
use tokio::sync::RwLock;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{error, info};
use url::Url;
use urlencoding::Encoded;
use vitium_api::user::UserProfile;

use crate::server::network::{NetworkConfig, NetworkModule};

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
    network: NetworkModule,
}

#[derive(Template)]
#[template(path = "client.html")]
struct Client {
    pub url: Url,
    pub comment: String,
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
        let network = NetworkModule::new(cfg.network.clone());
        let val = Self(Arc::new(ServerInst {
            cfg,
            shutdown,
            started: AtomicBool::new(false),
            chat: ChatModule::new(db.clone()),
            db,
            player: RwLock::const_new(HashMap::new()),
            basileus: Basileus::new(Default::default()).await?,
            cmd: CommandModule::new(),
            log,
            network,
        }));
        Ok(val)
    }

    fn embed_client(
        &self,
        path: &str,
        query: Option<HashMap<String, String>>,
    ) -> anyhow::Result<String> {
        let mut url = self.cfg.client.join(path).unwrap();
        let query = query
            .into_iter()
            .flatten()
            .chain(once(("server".into(), self.network.url()?.as_str().into())))
            .map(|(k, v)| format!("{}={}", k, Encoded(v)))
            .collect::<Vec<_>>()
            .join("&");
        url.set_query(Some(&query));
        let client = Client {
            url,
            comment: format!("{}", self.cfg.client.join(path)?),
        };
        let html = client.render()?;
        Ok(html)
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

    fn cors(&self) -> CorsLayer {
        let origin = self.cfg.client.origin();
        CorsLayer::new()
            .allow_origin(origin.ascii_serialization().parse::<HeaderValue>().unwrap())
            .allow_credentials(true)
            .max_age(Duration::from_secs(3600))
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

        let app = Router::new();

        #[cfg(debug_assertions)]
        let app = app.nest("/test", test::router());

        let app = app
            .route("/", get(client_root))
            .route("/ping", get(|| async { Json(SystemTime::now()) }))
            .route("/hello", get("Hello, world!"))
            .route("/contact", get(Redirect::temporary(&self.cfg.contact)))
            .nest("/auth", auth::rest())
            .nest("/chat", chat::rest())
            .nest("/cmd", cmd::rest())
            .nest("/profile", profile::rest())
            .fallback(any(StatusCode::NOT_FOUND))
            .layer(self.cors())
            .layer(TraceLayer::new_for_http());

        let http = self
            .network
            .start(app.with_state(self.clone()).into_make_service())
            .await?;
        self.shutdown.adopt(&http);

        info!("server started at {}", self.network.url()?);

        Ok(self.shutdown.clone())
    }
}

impl AsRef<Basileus> for Server {
    fn as_ref(&self) -> &Basileus {
        &self.basileus
    }
}

/// Server configuration.
#[serde_inline_default]
#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    /// Path to the server database.
    #[serde_inline_default("./server.db".into())]
    pub db: PathBuf,
    #[serde(default)]
    pub network: NetworkConfig,
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
    #[serde_inline_default(Url::parse("https://example.com/").unwrap())]
    pub client: Url,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            db: "./server.db".into(),
            network: Default::default(),
            direct_api: false,
            log: Default::default(),
            motd: String::new(),
            contact: Default::default(),
            client: Url::parse("https://example.com/").unwrap(),
        }
    }
}

fn internal_server_error(err: impl Error) -> StatusCode {
    error!("{err}");
    StatusCode::INTERNAL_SERVER_ERROR
}

async fn client_root(State(s): State<Server>) -> Result<Html<String>, StatusCode> {
    let client = match s.embed_client("/", None) {
        Ok(html) => html,
        Err(e) => {
            error!("{e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    Ok(Html(client))
}
