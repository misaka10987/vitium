use clearscreen::clear;
use once_cell::sync::Lazy;
use std::{io::stdin, process::exit};
use tokio::sync::{Mutex, MutexGuard};

//use crate::server_::root::{ban, grant};

static RUNNING: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
async fn running() -> MutexGuard<'static, bool> {
    RUNNING.lock().await
}

pub async fn input() {
    *running().await = true;
    while *running().await {
        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();
        proc(buf.trim()).await;
    }
}

pub async fn stop() {
    *running().await = false;
}

fn resolve(cmd: &str) -> (&str, &str) {
    for (i, &item) in cmd.as_bytes().iter().enumerate() {
        if item == b' ' {
            return (&cmd[0..i], &cmd[i..]);
        }
    }
    (&cmd[..], "")
}

async fn proc(cmd: &str) -> i8 {
    match resolve(cmd) {
        ("help", _) => {
            println!("  TODO");
            -1
        }
        ("clear", _) => {
            clear().unwrap();
            0
        }
        ("kill", _) => exit(-1),
        // ("grant", arg) => grant(arg).await,
        // ("ban", arg) => ban(arg).await,
        _ => {
            println!("  Failure>> \"{}\" not found", cmd);
            -1
        }
    }
}
