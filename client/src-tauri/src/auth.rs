use std::time::Duration;

use anyhow::bail;
use reqwest::StatusCode;
use tauri::{async_runtime::spawn, AppHandle, Emitter};
use tokio::{sync::RwLock, time::sleep};
use tracing::warn;

use crate::{CLIENT, SERVER_ADDR, USER};

pub static PASS: RwLock<String> = RwLock::const_new(String::new());
pub static TOKEN: RwLock<String> = RwLock::const_new(String::new());

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
        .timeout(Duration::from_secs(30))
        .send()
        .await?;
    match res.status() {
        StatusCode::FORBIDDEN => bail!("bad password"),
        _ if !res.status().is_success() => bail!("HTTP {}", res.status()),
        _ => (),
    }
    let cookie: Vec<_> = res
        .cookies()
        .filter(|c| c.name() == "token")
        .map(|c| c.value().to_owned())
        .collect();
    let token = match cookie.len() {
        1 => cookie[0].clone(),
        _ => bail!("{} tokens returned, expected 1", cookie.len()),
    };
    *TOKEN.write().await = token;
    Ok(())
}

async fn timed_refresh(hand: AppHandle) {
    loop {
        sleep(Duration::from_secs(240)).await;
        if refresh_token().await.is_err() {
            warn!("failed to refresh token, redirect to login page");
            hand.emit("token-refresh-fail", ()).unwrap();
            break;
        }
    }
}
