use crate::chara::chara;
use tokio::fs::write;
use vitium_common::json::json;

pub async fn save(path: &str) {
    write(format!("{}/chara.json", path), json(chara().await.clone()))
        .await
        .expect("failed to save");
}
