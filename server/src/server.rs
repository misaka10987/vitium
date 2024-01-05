use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use tokio::sync::{Mutex, MutexGuard};
use vitium_common::{
    chara::Character,
    item::Item,
    player::{Player, Token},
    registry::RegTable,
    request::EditPlayer,
    scene::Scene,
    skill::{Prof, Skill},
    vehicle::Vehicle,
};

type REG<T> = Lazy<Mutex<RegTable<T>>>;
macro_rules! reg {
    () => {
        Lazy::new(|| Mutex::new(RegTable::new()))
    };
}

static REG_ITEM: REG<Item> = reg!();
static REG_SKILL: REG<Skill> = reg!();
static REG_PROF: REG<Prof> = reg!();
static REG_SCENE: REG<Scene> = reg!();
static REG_VEHICLE: REG<Vehicle> = reg!();

type Map<K, V> = Lazy<Mutex<HashMap<K, V>>>;
macro_rules! map {
    () => {
        Lazy::new(|| Mutex::new(HashMap::new()))
    };
}

static PLAYER: Map<String, Player> = map!();
static PSWD: Map<String, String> = map!();
static CHARA: Map<String, Character> = map!();

async fn player() -> MutexGuard<'static, HashMap<String, Player>> {
    PLAYER.lock().await
}

async fn pswd() -> MutexGuard<'static, HashMap<String, String>> {
    PSWD.lock().await
}

async fn chara() -> MutexGuard<'static, HashMap<String, Character>> {
    CHARA.lock().await
}

async fn verify(token: &Token) -> bool {
    if let Some(pswd) = pswd().await.get(&token.id) {
        pswd == &token.pswd
    } else {
        false
    }
}

async fn get_player() -> (StatusCode, Json<HashMap<String, Player>>) {
    (StatusCode::OK, Json(player().await.clone()))
}

async fn edit_player(Json(req): Json<EditPlayer>) -> StatusCode {
    let mut dat = player().await;
    if let Some(player) = dat.get_mut(&req.player.id) {
        if let Some(token) = req.token {
            if verify(&token).await {
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

// basic handler that responds with a static string
async fn hello() -> String {
    "Hello, World!".to_string()
}

pub struct Server {
    port: u16,
}

impl Server {
    pub fn start() -> Self {
        Server { port: 0 }
    }
    pub fn set_port(&mut self, port: u16) -> &mut Self {
        self.port = port;
        self
    }
    #[tokio::main]
    pub async fn run(&self) -> Result<(), std::io::Error> {
        // initialize tracing
        // tracing_subscriber::fmt::init();
        // build our application with a route
        let app = Router::new()
            .route("/hello", get(hello))
            .route("/player", get(get_player))
            .route("/player", post(edit_player));
        // run our app with hyper, listening globally on port 3000
        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", self.port))
            .await
            .unwrap();
        axum::serve(listener, app).await
    }
}
