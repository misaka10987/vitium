// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod auth;
mod chat;
mod err;
mod net;


use reqwest::Client;
use tokio::sync::RwLock;

use once_cell::sync::Lazy;
use tracing::{trace, Level};

static SERVER_ADDR: Lazy<RwLock<String>> = Lazy::new(|| RwLock::new(String::new()));

pub(crate) static USER: Lazy<RwLock<String>> = Lazy::new(|| RwLock::new(String::new()));

static CLIENT: Lazy<Client> = Lazy::new(|| Client::new());

#[tauri::command]
fn hello() -> &'static str {
    "Hello, world!"
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

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();
    trace!("logger initialized");
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            hello,
            user,
            server_addr,
            server_stat,
            auth::login,
            chat::recv_chat,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
