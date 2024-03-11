pub use crate::load::load;
pub use crate::save::save;
use axum::http::StatusCode;
use std::collections::HashMap;
use tokio::sync::{
    oneshot::{channel, Receiver, Sender},
    Mutex, MutexGuard,
};
use tracing::{info, warn};
use vitium_common::{act::Act, chara::Chara, sync::Sync};

/// Action item waiting the server to process.
pub(self) struct ActProc {
    pub act: Act,
    pub sender: Sender<StatusCode>,
}

/// Internal game server.
pub struct Game {
    pub on: bool,
    _turn: Mutex<u64>,
    _chara_status: Mutex<HashMap<String, bool>>,
    _act: Mutex<HashMap<u64, ActProc>>,
    _chara: Mutex<HashMap<String, Chara>>,
    _uid_alloc: Mutex<u64>,
}

impl Game {
    /// Creates new instance without config.
    pub fn new() -> Self {
        Self {
            on: false,
            _turn: Mutex::new(0),
            _chara_status: Mutex::new(HashMap::new()),
            _act: Mutex::new(HashMap::new()),
            _chara: Mutex::new(HashMap::new()),
            _uid_alloc: Mutex::new(255),
        }
    }
    /// Lock getter.
    pub(self) async fn chara_status(&self) -> MutexGuard<'_, HashMap<String, bool>> {
        self._chara_status.lock().await
    }
    /// Whether all characters have submitted their action.
    pub(self) async fn all_ready(&self) -> bool {
        let mut ans = true;
        for i in self.chara_status().await.values() {
            ans = ans && *i;
        }
        ans
    }
    /// Lock getter.
    pub(self) async fn act(&self) -> MutexGuard<'_, HashMap<u64, ActProc>> {
        self._act.lock().await
    }
    /// Lock getter.
    pub(self) async fn chara(&self) -> MutexGuard<'_, HashMap<String, Chara>> {
        self._chara.lock().await
    }
    /// Current game turn.
    pub async fn turn(&self) -> MutexGuard<'_, u64> {
        self._turn.lock().await
    }
    /// Whether a character is enrolled in the game.
    pub async fn enrolled(&self, id: &str) -> bool {
        self.chara().await.contains_key(id)
    }
    /// Generate new uid.
    pub(self) async fn gen_uid(&self) -> u64 {
        let mut curr = self._uid_alloc.lock().await;
        *curr += 1;
        *curr
    }
    /// Shutdown the internal game server.
    pub async fn shutdown(&self) {
        let mut proc = self.act().await;
        for (k, v) in proc.drain() {
            warn!(
                "giving up act[uid={}] submitted by chara[uid={}]",
                k, v.act.chara
            );
            v.sender
                .send(StatusCode::SERVICE_UNAVAILABLE)
                .expect("channel error");
            info!("act queue clear");
        }
    }
    /// Calculates all waiting acts.
    pub(self) async fn update(&self) {
        let mut proc = self.act().await;
        for (k, v) in proc.drain() {
            info!(
                "processing act[uid={}] submitted by player[id={}]",
                k, v.act.token.id
            );
            warn!("act unimplemented");
            v.sender
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
    pub async fn proc(&self, act: Act) -> Receiver<StatusCode> {
        let uid = self.gen_uid().await;
        let (s, r) = channel::<StatusCode>();
        info!(
            "{} submitted an act: {}",
            act.token.id,
            format!("act[uid={}]", uid)
        );
        match self.chara_status().await.get(&act.chara) {
            Some(p) => {
                if *p {
                    s.send(StatusCode::CONFLICT).expect("channel error");
                } else {
                    self.act().await.insert(uid, ActProc { act, sender: s });
                    if self.all_ready().await {
                        self.update().await;
                    }
                }
            }
            None => {
                s.send(StatusCode::NOT_FOUND).expect("channel error");
            }
        };
        r
    }
}
