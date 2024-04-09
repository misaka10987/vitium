use axum::http::StatusCode;
use tokio::sync::oneshot::Sender;
use vitium_common::{act::Act, PC};

use crate::table::Table;

/// Action item waiting the server to process.
pub(self) struct ActProc {
    pub act: Act,
    pub sender: Sender<StatusCode>,
}

/// Internal game server.
pub struct Game {
    pub on: bool,
    pub pc: Table<PC>,
}

impl Game {
    /// Creates new instance without config.
    pub fn new() -> Self {
        Self {
            on: false,
            pc: Table::new(),
        }
    }
}
