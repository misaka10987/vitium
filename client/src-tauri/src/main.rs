// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::RwLock;

use once_cell::sync::Lazy;
use reqwest::Client;
use tracing::{trace, Level};

static CLIENT: Lazy<Client> = Lazy::new(|| Client::new());
static SERVER: Lazy<RwLock<String>> = Lazy::new(|| RwLock::new(String::new()));
static USER: Lazy<RwLock<String>> = Lazy::new(|| RwLock::new(String::new()));
static PASS: Lazy<RwLock<String>> = Lazy::new(|| RwLock::new(String::new()));
static TOKEN: Lazy<RwLock<String>> = Lazy::new(|| RwLock::new(String::new()));

#[tauri::command]
fn hello() -> &'static str {
    "Hello, world!"
}

#[tauri::command]
async fn connect(server: &str, user: &str, pass: &str) -> Result<(), String> {
    let url = format!("http://{server}/api/auth/login");
    let req = CLIENT
        .get(url)
        .basic_auth(user, Some(pass))
        .build()
        .unwrap();
    trace!("connecting to '{server}'...");
    let res = CLIENT.execute(req).await;
    if res.is_err() {
        return Err("failed to connect".to_string());
    }
    let res = res.unwrap();
    if !res.status().is_success() {
        let status = res.status();
        let reason = status.canonical_reason().unwrap();
        return Err(format!("HTTP {status} {reason}"));
    }
    let cookie: Vec<_> = res.cookies().filter(|c| c.name() == "token").collect();
    if cookie.len() != 1 {
        return Err(format!("expected one token, found {}", cookie.len()));
    }
    let cookie = &cookie[0];
    *TOKEN.write().unwrap() = cookie.value().to_owned();
    Ok(())
}

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();
    trace!("logger initialized");
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![hello, connect])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
