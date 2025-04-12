use anyhow::{anyhow, bail};
use basileus::Perm;
use clap::Parser;
use clearscreen::clear;
use colored::Colorize;
use std::{
    collections::{HashMap, HashSet},
    process::exit,
    sync::Mutex,
};
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

#[derive(Parser)]
#[command(name = "shutdown", visible_alias = "stop", visible_alias = "exit")]
#[command(about = "shutdown the server")]
struct Shutdown;

impl Command for Shutdown {
    async fn exec(self, _: Server) {
        shutdown();
    }

    fn perm_req() -> Perm {
        Perm::Root
    }
}

pub trait Command: Parser {
    fn exec(self, server: Server) -> impl std::future::Future<Output = ()> + Send;
    fn perm_req() -> Perm;
}

pub struct CommandInst {
    pub clap: clap::Command,
    pub perm: Perm,
    exe: Mutex<Box<dyn Fn(Server, String) + Send>>,
}

impl CommandInst {
    pub fn from<T: Command>() -> Self {
        let clap = T::command();
        let perm = T::perm_req();
        let exe = move |server, line: String| {
            tokio::spawn(async move {
                let arg = T::parse_from(line.split_whitespace());
                arg.exec(server).await;
            });
        };
        Self {
            clap,
            perm,
            exe: Mutex::new(Box::new(exe)),
        }
    }
    pub fn run(&self, server: Server, line: String) {
        (self.exe.lock().unwrap())(server, line)
    }
}

pub struct CommandModule {
    map: Mutex<HashMap<String, CommandInst>>,
}

impl CommandModule {
    pub fn new() -> Self {
        Self {
            map: Mutex::new(HashMap::new()),
        }
    }
    pub fn register_cmd<T: Command>(&self) {
        let cmd = CommandInst::from::<T>();
        self.map
            .lock()
            .unwrap()
            .insert(cmd.clap.get_name().into(), cmd);
    }
}

pub trait CommandServer {
    fn register_cmd<T: Command>(&self);
    fn server_run_cmd(&self, line: String) -> anyhow::Result<String>;
}

impl CommandServer for Server {
    fn register_cmd<T: Command>(&self) {
        self.cmd.register_cmd::<T>();
    }
    fn server_run_cmd(&self, line: String) -> anyhow::Result<String> {
        let name = line
            .split_whitespace()
            .next()
            .ok_or_else(|| anyhow!("unable to parse command"))?;
        let map = self.cmd.map.lock().unwrap();
        if let Some(cmd) = map.get(name) {
            cmd.run(self.clone(), line.clone());
        }
        for cmd in map.values() {
            if cmd
                .clap
                .get_visible_aliases()
                .collect::<HashSet<_>>()
                .contains(name)
            {
                cmd.run(self.clone(), line.clone());
                break;
            }
        }
        bail!("unknown command: {name}")
    }
}
