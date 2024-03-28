use crate::chara::chara;
use tokio::fs::read_to_string;
use vitium_common::obj;

/// Loads all game data from the save folder `path`.
pub async fn load(path: &str) {
    chara().await.clone_from(
        &obj(&read_to_string(format!("{}/chara.json", path))
            .await
            .expect("failed to load"))
        .unwrap(),
    )
}
