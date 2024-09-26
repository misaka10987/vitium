use anyhow::bail;
use clearscreen::clear;
use std::process::exit;
use tokio::{
    io::{stdin, AsyncBufReadExt, BufReader},
    spawn,
    task::JoinHandle,
};
use vitium_api::net::Chat;

use crate::{shutdown, Server};

fn resolve(cmd: &str) -> (&str, Vec<&str>) {
    let mut token = cmd.trim().split(' ');
    (token.next().unwrap(), token.collect())
}

impl Server {
    pub fn input(&self) -> JoinHandle<()> {
        let server = self.clone();
        let stdin = BufReader::new(stdin());
        let mut line = stdin.lines();
        spawn(async move {
            while let Ok(Some(line)) = line.next_line().await {
                if let Err(e) = server.proc(&line).await {
                    eprintln!("{e}")
                }
            }
            shutdown();
        })
    }
    pub async fn proc(&self, cmd: &str) -> anyhow::Result<()> {
        let (cmd, arg) = resolve(cmd);
        match cmd {
            "exit" | "stop" | "shutdown" => Ok(shutdown()),
            "help" => bail!("  TODO"),
            "clear" => Ok(clear()?),
            "kill" => exit(-1),
            "broadcast" => {
                self.chat.push(Chat::new("".into(), arg.join(" "))).await;
                Ok(())
            }
            _ => bail!("  {} not found", cmd),
        }
    }
}
