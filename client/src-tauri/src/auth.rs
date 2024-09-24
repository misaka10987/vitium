use std::time::Duration;

use anyhow::anyhow;
use once_cell::sync::Lazy;
use reqwest::StatusCode;
use tauri::{async_runtime::spawn, App, AppHandle, Manager};
use tokio::{sync::RwLock, time::sleep};
use tracing::{trace, warn};

use crate::{CLIENT, SERVER_ADDR, USER};

pub static PASS: Lazy<RwLock<String>> = Lazy::new(|| RwLock::new(String::new()));
pub static TOKEN: Lazy<RwLock<String>> = Lazy::new(|| RwLock::new(String::new()));

#[tauri::command]
pub async fn login(app: AppHandle, server: &str, user: &str, pass: &str) -> Result<(), String> {
    *SERVER_ADDR.write().await = server.into();
    *USER.write().await = user.into();
    *PASS.write().await = pass.into();
    spawn(timed_refresh(app));
    refresh_token().await.map_err(|e| e.to_string())
}

async fn refresh_token() -> anyhow::Result<()> {
    let res = CLIENT
        .get(format!(
            "http://{}/api/auth/login",
            SERVER_ADDR.read().await
        ))
        .basic_auth(USER.read().await, Some(PASS.read().await))
        .send()
        .await?;
    match res.status() {
        StatusCode::FORBIDDEN => return Err(anyhow!("bad password")),
        _ if !res.status().is_success() => return Err(anyhow!("HTTP {}", res.status())),
        _ => (),
    }
    let cookie: Vec<_> = res
        .cookies()
        .filter(|c| c.name() == "token")
        .map(|c| c.value().to_owned())
        .collect();
    let token = match cookie.len() {
        1 => cookie[0].clone(),
        _ => return Err(anyhow!("{} tokens returned, expected 1", cookie.len())),
    };
    trace!("refreshed token '{}*'", &token[..6]);
    *TOKEN.write().await = token;
    Ok(())
}

async fn timed_refresh(hand: AppHandle) {
    loop {
        trace!("waiting for timed refresh");
        sleep(Duration::from_secs(240)).await;
        if refresh_token().await.is_err() {
            warn!("failed to refresh token, redirect to login page");
            for (_, win) in hand.windows() {
                win.eval("window.location.href='/'").unwrap();
            }
        }
    }
}
