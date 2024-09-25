use clearscreen::clear;
use std::process::{id as pid, Command};
use std::{collections::VecDeque, process::exit};
use vitium_api::net::Chat;

use crate::Server;

#[cfg(unix)]
fn term() -> ! {
    Command::new("kill")
        .arg("-INT")
        .arg(pid().to_string())
        .status()
        .unwrap();
    panic!("never")
}

fn resolve(cmd: &str) -> (&str, Vec<&str>) {
    let mut token: VecDeque<_> = cmd.trim().split(' ').collect();
    (token.pop_front().unwrap(), token.into())
}

pub async fn proc(cmd: &str, server: &Server) -> Result<(), String> {
    let (cmd, arg) = resolve(cmd);
    match cmd {
        #[cfg(unix)]
        "exit" => term(),
        "help" => Err("TODO".to_string()),
        "clear" => clear().map_err(|e| e.to_string()),
        "kill" => exit(-1),
        "say" => {
            server.push_chat(Chat::new("".into(), arg.join(" "))).await;
            Ok(())
        }
        _ => Err(format!("{} not found", cmd)),
    }
}
