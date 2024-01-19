pub use crate::load::load;
pub use crate::save::save;
use axum::http::StatusCode;
use once_cell::sync::Lazy;
use std::collections::VecDeque;
use tokio::sync::{
    oneshot::{channel, Receiver, Sender},
    Mutex, MutexGuard,
};
use tracing::info;
use vitium_common::{act::Act, sync::Sync, UID};

static TURN: Lazy<Mutex<i128>> = Lazy::new(|| Mutex::new(255));
/// Starts from `0`, defines how many turns has passed after the game starts.
pub async fn turn() -> MutexGuard<'static, i128> {
    TURN.lock().await
}

pub(self) struct ActProc {
    pub act: Act,
    pub sender: Sender<StatusCode>,
}

/// Internal game server.
pub struct Game {
    pub on: bool,
    _turn: Mutex<i128>,
    _act: Mutex<VecDeque<ActProc>>,
    _uid_alloc: Mutex<i128>,
}

impl Game {
    /// Creates new instance without config.
    pub fn new() -> Self {
        Self {
            on: false,
            _turn: Mutex::new(0),
            _act: Mutex::new(VecDeque::new()),
            _uid_alloc: Mutex::new(255),
        }
    }
    /// Lock getter.
    pub(self) async fn act(&self) -> MutexGuard<'_, VecDeque<ActProc>> {
        self._act.lock().await
    }
    /// Current game turn.
    pub async fn turn(&self) -> MutexGuard<'_, i128> {
        self._turn.lock().await
    }
    /// Generate new uid.
    pub(self) async fn gen_uid(&self) -> i128 {
        let mut curr = self._uid_alloc.lock().await;
        *curr += 1;
        *curr
    }
    /// Calculates all waiting acts.
    pub(self) async fn update(&self) {
        let mut proc = self.act().await;
        while let Some(a) = proc.pop_front() {
            info!(
                "processing act[uid={}] submitted by player[id={}]",
                a.act.uid(),
                a.act.token.id
            );
            a.sender
                .send(StatusCode::NOT_IMPLEMENTED)
                .expect("game server failed to connect with http server");
        }
    }
    /// Get all current game data.
    pub fn fetch(&self, player: String) -> (StatusCode, Sync) {
        info!("player[id={}] synchronizes game state", player);
        (StatusCode::NOT_IMPLEMENTED, Sync::new())
    }
    /// Process an `Act` to the act queue, returns `Receiver` for status of future execution.
    pub async fn proc(&self, mut act: Act) -> Receiver<StatusCode> {
        act.set_uid(self.gen_uid().await);
        let (s, r) = channel::<StatusCode>();
        info!(
            "{} submitted an act: {}",
            act.token.id,
            format!("act[uid={}]", act.uid)
        );
        self.act().await.push_back(ActProc { act, sender: s });
        self.update().await;
        r
    }
}
