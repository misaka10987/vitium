use std::time::{Duration, SystemTime};

use chrono::{DateTime, Local};
use tokio::sync::Mutex;
use tracing::trace;
use vitium_api::net::{Chat, RecvChat, SendChat};

use crate::{net::send, USER};

static CHAT: Mutex<Vec<Chat>> = Mutex::const_new(Vec::new());

static LAST_UPDATE: Mutex<SystemTime> = Mutex::const_new(SystemTime::UNIX_EPOCH);

static PENDING: Mutex<bool> = Mutex::const_new(false);

static MODIFIED: Mutex<bool> = Mutex::const_new(false);

#[tauri::command]
pub async fn chat_modified() -> bool {
    *MODIFIED.lock().await
}

#[tauri::command]
pub async fn recv_chat() -> Result<(), String> {
    let mut pending = PENDING.lock().await;
    if *pending {
        trace!("recv_chat() exited as nothing to do");
        return Ok(());
    }
    let mut last = LAST_UPDATE.lock().await;
    let req = RecvChat(*last);
    *pending = true;
    let res = send(req).await;
    *pending = false;
    *last = SystemTime::now();
    let res = res.map_err(|e| e.to_string())?;
    *MODIFIED.lock().await = true;
    let mut w = CHAT.lock().await;
    for i in res {
        w.push(i);
    }
    Ok(())
}

#[tauri::command]
pub async fn send_chat(msg: String) -> Result<SystemTime, String> {
    let chat = Chat::new(USER.read().await.clone(), msg);
    let req = SendChat(chat);
    send(req).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn render_chat() -> String {
    *MODIFIED.lock().await = false;
    let user = USER.read().await;
    CHAT.lock()
        .await
        .iter()
        .rev()
        .map(|c| render(c.clone(), &user))
        .collect::<Vec<_>>()
        .join("")
}

fn render(chat: Chat, user: &str) -> String {
    let Chat {
        sender,
        msg,
        send_time,
        recv_time,
    } = chat;
    let latency = recv_time
        .duration_since(send_time)
        .unwrap_or(Duration::from_secs(0))
        .as_secs();
    let send_time = DateTime::<Local>::from(send_time).format("%H:%M:%S %m/%d");
    let esc = html_escape::encode_safe(&msg);
    let s = if sender != user {
        format!(
            r###"
        <p>
        <div class="transform overflow-hidden rounded-lg shadow-xl transition-all w-full mb-2 select-text">
          <h3 class="bg-emerald-600 py-1 px-2">
            <span class="font-semibold font-mono">{sender}</span>
            <span class="text-sm">{send_time}</span>
            <span class="text-sm">({latency}s)</span>
          </h3>
          <div class="bg-stone-600 p-1.5 pt-1">
            {esc}
          </div>
        </div>
        </p>
        "###
        )
    } else {
        format!(
            r###"
        <p>
        <div class="transform overflow-hidden rounded-lg shadow-xl transition-all w-full mb-2 select-text">
          <h3 class="bg-purple-600 py-1 px-2">
            <span class="font-semibold font-mono">{sender}</span>
            <span class="text-sm">{send_time}</span>
            <span class="text-sm">({latency}s)</span>
          </h3>
          <div class="bg-stone-600 p-1.5 pt-1">
            {esc}
          </div>
        </div>
        </p>
        "###
        )
    };
    s
}
