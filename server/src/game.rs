use axum::http::StatusCode;
use tokio::sync::{oneshot::Sender, Mutex};
use vitium_common::act::Act;

/// Action item waiting the server to process.
pub(self) struct ActProc {
    pub act: Act,
    pub sender: Sender<StatusCode>,
}

pub struct UIDAlloc {
    now: Mutex<u64>,
}

impl UIDAlloc {
    pub fn new(start: u64) -> Self {
        Self {
            now: Mutex::new(start),
        }
    }
    pub async fn curr(&self) -> u64 {
        *self.now.lock().await
    }
    pub async fn gen(&self) -> u64 {
        let mut x = self.now.lock().await;
        *x += 1;
        *x
    }
}

/// Internal game server.
pub struct Game {
    pub on: bool,
    pub uid: UIDAlloc,
}

impl Game {
    /// Creates new instance without config.
    pub fn new() -> Self {
        Self {
            on: false,
            uid: UIDAlloc::new(65535),
        }
    }
}
