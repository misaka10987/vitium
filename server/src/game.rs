pub use crate::load::load;
pub use crate::save::save;
use axum::http::StatusCode;
use std::collections::{HashMap, VecDeque};
use tokio::sync::{
    oneshot::{channel, Receiver, Sender},
    Mutex, MutexGuard,
};
use tracing::{info, warn};
use vitium_common::{act::Act, chara::Chara, sync::Sync, UID};

/// Action item waiting the server to process.
pub(self) struct ActProc {
    pub act: Act,
    pub sender: Sender<StatusCode>,
}

/// Internal game server.
pub struct Game {
    pub on: bool,
    _turn: Mutex<i128>,
    _chara_status: Mutex<HashMap<i128, bool>>,
    _act: Mutex<VecDeque<ActProc>>,
    _chara: Mutex<HashMap<i128, Chara>>,
    _uid_alloc: Mutex<i128>,
}

impl Game {
    /// Creates new instance without config.
    pub fn new() -> Self {
        Self {
            on: false,
            _turn: Mutex::new(0),
            _chara_status: Mutex::new(HashMap::new()),
            _act: Mutex::new(VecDeque::new()),
            _chara: Mutex::new(HashMap::new()),
            _uid_alloc: Mutex::new(255),
        }
    }
    /// Lock getter.
    pub(self) async fn chara_status(&self) -> MutexGuard<'_, HashMap<i128, bool>> {
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
    pub(self) async fn act(&self) -> MutexGuard<'_, VecDeque<ActProc>> {
        self._act.lock().await
    }
    /// Lock getter.
    pub(self) async fn chara(&self) -> MutexGuard<'_, HashMap<i128, Chara>> {
        self._chara.lock().await
    }
    /// Current game turn.
    pub async fn turn(&self) -> MutexGuard<'_, i128> {
        self._turn.lock().await
    }
    /// Whether a character is enrolled in the game.
    pub async fn enrolled(&self, uid: i128) -> bool {
        self.chara().await.contains_key(&uid)
    }
    /// Generate new uid.
    pub(self) async fn gen_uid(&self) -> i128 {
        let mut curr = self._uid_alloc.lock().await;
        *curr += 1;
        *curr
    }
    /// Shutdown the internal game server.
    pub async fn shutdown(&self) {
        let mut proc = self.act().await;
        while let Some(a) = proc.pop_front() {
            warn!(
                "giving up act[uid={}] submitted by chara[uid={}]",
                a.act.uid, a.act.chara
            );
            a.sender
                .send(StatusCode::SERVICE_UNAVAILABLE)
                .expect("channel error");
            info!("act queue clear");
        }
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
        match self.chara_status().await.get(&act.chara) {
            Some(p) => {
                if *p {
                    s.send(StatusCode::CONFLICT).expect("channel error");
                } else {
                    self.act().await.push_back(ActProc { act, sender: s });
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
