use anyhow::bail;
use chrono::{DateTime, Utc};
use clearscreen::clear;
use std::process::{id as pid, Command};
use std::{collections::VecDeque, process::exit};
use vitium_api::net::Chat;

use crate::Server;

#[cfg(unix)]
fn term() {
    Command::new("kill")
        .arg("-INT")
        .arg(pid().to_string())
        .status()
        .unwrap();
}

fn resolve(cmd: &str) -> (&str, Vec<&str>) {
    let mut token: VecDeque<_> = cmd.trim().split(' ').collect();
    (token.pop_front().unwrap(), token.into())
}

pub async fn proc(cmd: &str, server: &Server) -> anyhow::Result<()> {
    let (cmd, arg) = resolve(cmd);
    match cmd {
        #[cfg(unix)]
        "exit" => {
            term();
            Ok(())
        }
        "help" => bail!("  TODO"),
        "clear" => Ok(clear()?),
        "kill" => exit(-1),
        "say" => {
            let t = server.chat.push(Chat::new("".into(), arg.join(" "))).await;
            let t = DateTime::<Utc>::from(t);
            eprintln!("  said at {}", t);
            Ok(())
        }
        _ => bail!("  {} not found", cmd),
    }
}
