use axum::http::StatusCode;
use tokio::sync::oneshot::Sender;
use vitium_common::{act::Act};

/// Action item waiting the server to process.
pub(self) struct ActProc {
    pub act: Act,
    pub sender: Sender<StatusCode>,
}

/// Internal game server.
pub struct Game {
    pub on: bool,
}

impl Game {
    /// Creates new instance without config.
    pub fn new() -> Self {
        Self {
            on: false,
        }
    }
}
