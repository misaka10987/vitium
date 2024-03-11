use crate::chara::chara;
use tokio::fs::write;
use vitium_common::json;

/// Saves all game data to save folder `path`.
pub async fn save(path: &str) {
    write(
        format!("{}/chara.json", path),
        json(&chara().await.clone()).unwrap(),
    )
    .await
    .expect("failed to save");
}
