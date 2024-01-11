use once_cell::sync::Lazy;
use std::collections::HashMap;
use tokio::sync::{Mutex, MutexGuard};
use vitium_common::chara::Chara;

static CHARA: Lazy<Mutex<HashMap<i128, Chara>>> =
    Lazy::new(|| Mutex::new(HashMap::<i128, Chara>::new()));

pub async fn chara() -> MutexGuard<'static, HashMap<i128, Chara>> {
    CHARA.lock().await
}

/// Whether a character is enrolled in the game.
pub async fn enrolled(uid: i128) -> bool {
    chara().await.contains_key(&uid)
}

/// Make a character temporarily exit the game, returns true if success.
pub async fn exit(uid: i128) -> bool {
    match chara().await.remove(&uid) {
        Some(_) => true,
        None => false,
    }
}
