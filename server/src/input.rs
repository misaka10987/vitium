use clearscreen::clear;
use once_cell::sync::Lazy;
use std::process::{id as pid, Command};
use std::{collections::VecDeque, io::stdin, process::exit};
use tokio::sync::{Mutex, MutexGuard};

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

fn term() -> i8 {
    let _ = Command::new("kill")
        .arg("-INT")
        .arg(pid().to_string())
        .status();
    0
}

pub async fn stop() {
    *running().await = false;
}

fn resolve(cmd: &str) -> (&str, Vec<&str>) {
    let mut token: VecDeque<_> = cmd.split(' ').collect();
    (token.pop_front().unwrap(), token.into())
}

async fn proc(cmd: &str) -> i8 {
    match resolve(cmd) {
        ("exit", _) => term(),
        ("help", _) => {
            println!("  => TODO");
            -1
        }
        ("clear", _) => {
            clear().unwrap();
            0
        }
        ("kill", _) => exit(-1),
        _ => {
            println!("  => failure: \"{}\" not found", cmd);
            -1
        }
    }
}
