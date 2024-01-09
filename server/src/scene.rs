use std::collections::HashMap;

use once_cell::sync::Lazy;
use tokio::sync::{Mutex, MutexGuard};
use vitium_common::scene::Scene;

static SCENE: Lazy<Mutex<HashMap<i128, Scene>>> =
    Lazy::new(|| Mutex::new(HashMap::<i128, Scene>::new()));

pub async fn scene() -> MutexGuard<'static, HashMap<i128, Scene>> {
    SCENE.lock().await
}
