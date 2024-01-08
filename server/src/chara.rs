use once_cell::sync::Lazy;
use std::collections::HashMap;
use tokio::sync::{Mutex, MutexGuard};
use vitium_common::{chara::Chara, UID};

static CHARA: Lazy<Mutex<HashMap<i128, Chara>>> =
    Lazy::new(|| Mutex::new(HashMap::<i128, Chara>::new()));

pub async fn chara() -> MutexGuard<'static, HashMap<i128, Chara>> {
    CHARA.lock().await
}

pub async fn exist(character: Chara) -> bool {
    chara().await.contains_key(&character.uid())
}
