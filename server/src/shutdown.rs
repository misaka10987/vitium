use std::sync::{atomic::AtomicBool, LazyLock};

use tokio::sync::broadcast;

static SHUTDOWN: LazyLock<broadcast::Sender<()>> = LazyLock::new(|| broadcast::channel(1).0);

static SHOULD_SHUTDOWN: AtomicBool = AtomicBool::new(false);

pub fn should_shutdown() -> bool {
    SHOULD_SHUTDOWN.load(std::sync::atomic::Ordering::SeqCst)
}

pub fn trigger_shutdown() {
    let _ = SHUTDOWN.send(());
    SHOULD_SHUTDOWN.store(true, std::sync::atomic::Ordering::SeqCst);
}

pub async fn wait_shutdown() {
    SHUTDOWN.subscribe().recv().await.unwrap()
}
