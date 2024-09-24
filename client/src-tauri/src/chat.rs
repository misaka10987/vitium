use std::{collections::VecDeque, time::SystemTime};

use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use vitium_api::net::{Chat, RecvChat};

use crate::net::send;

static CHAT: Lazy<Mutex<VecDeque<Chat>>> = Lazy::new(|| Mutex::new(VecDeque::new()));

static LAST_UPDATE: Lazy<Mutex<SystemTime>> = Lazy::new(|| Mutex::new(SystemTime::UNIX_EPOCH));

#[tauri::command]
pub async fn recv_chat() -> Result<VecDeque<Chat>, String> {
    let last = LAST_UPDATE.lock().await;
    let req = RecvChat(last.clone());
    let res = send(req).await.map_err(|e| e.to_string())?;
    let mut w = CHAT.lock().await;
    for i in res {
        w.push_back(i);
    }
    Ok(w.clone())
}
