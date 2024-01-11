use once_cell::sync::Lazy;
use std::io::stdin;
use tokio::sync::{Mutex, MutexGuard};

static RUNNING: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
async fn running() -> MutexGuard<'static, bool> {
    RUNNING.lock().await
}

pub async fn input() {
    *running().await = true;
    while *running().await {
        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();
        println!("Input: {}", buf.trim());
    }
}

pub async fn stop() {
    *running().await = false;
}
