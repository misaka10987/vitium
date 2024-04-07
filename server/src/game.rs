use std::marker::PhantomData;

use axum::http::StatusCode;
use tokio::sync::{oneshot::Sender, Mutex};
use vitium_common::{act::Act, Item, UID};

/// Action item waiting the server to process.
pub(self) struct ActProc {
    pub act: Act,
    pub sender: Sender<StatusCode>,
}

/// A UID allocator that thread-safely generates ascending UIDs.
pub struct UIDAlloc<T> {
    now: Mutex<u64>,
    _t: PhantomData<T>,
}

impl<T> UIDAlloc<T> {
    /// Creates a new allocator.
    pub fn new() -> Self {
        Self {
            now: Mutex::new(0),
            _t: PhantomData,
        }
    }
    /// Creates a new allocator with specified starting point.
    pub fn with_start(start: u64) -> Self {
        Self {
            now: Mutex::new(start),
            _t: PhantomData,
        }
    }
    /// Generate a new UID.
    pub async fn gen(&self) -> UID<T> {
        let mut x = self.now.lock().await;
        *x += 1;
        UID::new(*x)
    }
}

/// Internal game server.
pub struct Game {
    pub on: bool,
    pub uid: UIDAlloc<Item>,
}

impl Game {
    /// Creates new instance without config.
    pub fn new() -> Self {
        Self {
            on: false,
            uid: UIDAlloc::new(),
        }
    }
}
