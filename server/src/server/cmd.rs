use anyhow::bail;
use clearscreen::clear;
use colored::Colorize;
use std::process::exit;
use tokio::{
    io::{stdin, AsyncBufReadExt, BufReader},
    select, spawn,
    task::JoinHandle,
};

use crate::{dice::roll, recv_shutdown, shutdown, Server};

use super::chat::ChatServer;

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
            loop {
                select! {
                    line = line.next_line() => {
                        match line {
                            Ok(Some(line)) => {
                                match line.as_str() {
                                    "exit" | "stop" | "shutdown" => {
                                        shutdown();
                                        break;
                                    }
                                    other => {
                                        let res = server.exec(other).await;
                                        if let Err(e) = res {
                                            eprintln!("{} {e}", "=>".red().bold());
                                        }
                                    }
                                }
                            },
                            // EOF
                            Ok(None) => {
                                shutdown();
                                break;
                            },
                            Err(e) => {
                                shutdown();
                                panic!("{e:?}")
                            }
                        }
                    },
                    _ = recv_shutdown() => break
                }
            }
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
            "clear" => Ok(clear()?),
            "kill" => exit(-1),
            "broadcast" => Ok(self.send_server_msg(arg.into()).await),
            _ => Ok(self.send_server_msg(self.op_cmd(cmd).await?).await),
        }
    }
}
