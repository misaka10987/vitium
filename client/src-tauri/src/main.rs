// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use once_cell::sync::Lazy;
use reqwest::{Client, Request};
use tauri::{http::status::StatusCode, App, Manager, Window};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

static CLIENT: Lazy<Client> = Lazy::new(|| Client::new());
static SERVER: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));
static USER: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));
static PASS: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));

async fn connect(server: &str, user: &str, pass: &str) -> Result<(), StatusCode> {
    let url = format!("http://{server}/auth/login");
    let req = CLIENT
        .get(url)
        .basic_auth(user, Some(pass))
        .build()
        .unwrap();
    let res = CLIENT.execute(req).await.unwrap();
    // res.headers().
    todo!()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
