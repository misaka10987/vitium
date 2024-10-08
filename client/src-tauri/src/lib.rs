mod auth;
mod chat;
mod err;
mod net;

use reqwest::Client;
use tauri::{generate_handler, Window};
use tokio::sync::RwLock;

use once_cell::sync::Lazy;
use tracing::{trace, Level};

static SERVER_ADDR: RwLock<String> = RwLock::const_new(String::new());

pub(crate) static USER: RwLock<String> = RwLock::const_new(String::new());

static CLIENT: Lazy<Client> = Lazy::new(|| Client::new());

#[tauri::command]
fn hello() -> &'static str {
    "Hello, world!"
}

#[tauri::command]
fn set_window_title(win: Window, title: &str) -> tauri::Result<()> {
    win.set_title(title)
}

#[tauri::command]
async fn user() -> String {
    USER.read().await.to_owned()
}

#[tauri::command]
async fn server_addr() -> String {
    SERVER_ADDR.read().await.to_owned()
}

#[tauri::command]
fn server_stat() {
    todo!()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();
    trace!("logger initialized");
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(generate_handler![
            hello,
            user,
            server_addr,
            server_stat,
            set_window_title,
            auth::login,
            chat::recv_chat,
            chat::send_chat,
            chat::render_chat,
            chat::chat_modified,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
