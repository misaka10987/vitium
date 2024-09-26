use anyhow::bail;
use clearscreen::clear;
use std::process::exit;
use tokio::{
    io::{stdin, AsyncBufReadExt, BufReader},
    spawn,
    task::JoinHandle,
};

use crate::{dice::roll, shutdown, Server};

fn resolve(cmd: &str) -> (&str, &str) {
    let mut token = cmd.trim().splitn(2, " ");
    (token.next().unwrap(), token.next().unwrap_or(""))
}

impl Server {
    pub fn input(&self) -> JoinHandle<()> {
        let server = self.clone();
        let stdin = BufReader::new(stdin());
        let mut line = stdin.lines();
        spawn(async move {
            while let Ok(Some(line)) = line.next_line().await {
                if let Err(e) = server.exec(&line).await {
                    eprintln!("  {e}")
                }
            }
            shutdown();
        })
    }

    pub async fn cmd(&self, cmd: &str) -> anyhow::Result<String> {
        let (cmd, arg) = resolve(cmd);
        match cmd {
            "roll" => Ok(roll(&arg)?.to_string()),
            _ => bail!("command not found: {cmd}"),
        }
    }

    pub async fn op_cmd(&self, cmd: &str) -> anyhow::Result<String> {
        let (exe, arg) = resolve(cmd);
        match exe {
            "op" => {
                self.op(arg).await?;
                Ok(format!("opped {arg}"))
            }
            "deop" => {
                self.deop(arg).await?;
                Ok(format!("deopped {arg}"))
            }
            _ => self.cmd(cmd).await,
        }
    }

    pub async fn exec(&self, cmd: &str) -> anyhow::Result<()> {
        let (exe, arg) = resolve(cmd);
        match exe {
            "exit" | "stop" | "shutdown" => Ok(shutdown()),
            "clear" => Ok(clear()?),
            "kill" => exit(-1),
            "broadcast" => Ok(self.chat.broadcast(arg.into()).await),
            _ => Ok(self.chat.broadcast(self.op_cmd(cmd).await?).await),
        }
    }
}
