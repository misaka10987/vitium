mod clear;
mod echo;
mod help;
mod say;
mod shutdown;

use anyhow::{anyhow, bail};
use basileus::{perm::PermManage, Perm};
use clap::Parser;
use clear::Clear;
use colored::Colorize;
use echo::Echo;
use help::Help;
use say::Say;
use shutdown::Shutdown;
use std::{
    collections::{HashMap, HashSet},
    future::Future,
    pin::Pin,
    sync::Arc,
};
use tokio::{
    io::{stdin, AsyncBufReadExt, BufReader},
    select, spawn,
    sync::broadcast,
    task::JoinHandle,
};
use tracing::warn;
use vitium_api::cmd::CommandLine;

use crate::{should_shutdown, trigger_shutdown, wait_shutdown, Server};

impl Server {
    pub fn handle_input(&self) -> JoinHandle<()> {
        let server = self.clone();
        let stdin = BufReader::new(stdin());
        let mut line = stdin.lines();
        spawn(async move {
            loop {
                select! {
                    line = line.next_line() => {
                        match line {
                            Ok(Some(line)) => {
                                if line.is_empty() || line.chars().all(|c|c.is_whitespace()) {
                                    continue;
                                }
                                server.server_run_cmd(line.clone()).await;
                                if should_shutdown() {
                                    break;
                                }
                            },
                            // EOF
                            Ok(None) => {
                                trigger_shutdown();
                                break;
                            },
                            Err(e) => {
                                trigger_shutdown();
                                panic!("{e:?}")
                            }
                        }
                    },
                    _ = wait_shutdown() => break
                }
            }
        })
    }
}

fn print_info(cmd: &CommandLine, status: &anyhow::Result<String>) {
    if let Some(user) = &cmd.user {
        eprintln!(
            "{} {} {}",
            user.purple().bold(),
            ">".bright_blue().bold(),
            cmd.line
        )
    }
    match status {
        Ok(o) if !o.is_empty() => eprintln!("{} {o}", "=>".green().bold()),
        Err(e) => eprintln!("{} {e}", "=>".red().bold()),
        _ => {}
    }
}

trait CommandExec: Send + Sync {
    fn exec(
        &self,
        line: String,
        server: Server,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<String>> + Send>>;
}

impl<F, Fut> CommandExec for F
where
    F: Fn(String, Server) -> Fut + Send + Sync,
    Fut: Future<Output = anyhow::Result<String>> + Send + 'static,
{
    fn exec(
        &self,
        line: String,
        server: Server,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<String>> + Send>> {
        Box::pin(self(line, server))
    }
}

pub trait Command: Parser {
    fn exec(
        self,
        server: Server,
    ) -> impl std::future::Future<Output = anyhow::Result<String>> + Send;
    fn perm_req() -> Perm;
}

pub struct CommandInst {
    pub clap: clap::Command,
    pub perm: Perm,
    exe: Box<dyn CommandExec>,
}

impl CommandInst {
    pub fn from<T: Command>() -> Self {
        let clap = T::command();
        let perm = T::perm_req();
        let exe = |line: String, server| async move {
            let arg = T::try_parse_from(line.split_whitespace())
                .map_err(|e| anyhow!("{}", e.render().ansi()))?;
            arg.exec(server).await
        };
        Self {
            clap,
            perm,
            exe: Box::new(exe),
        }
    }
    pub async fn run(&self, arg: String, server: Server) -> anyhow::Result<String> {
        self.exe.exec(arg, server).await
    }
}

pub struct CommandModule {
    map: HashMap<String, CommandInst>,
    output: broadcast::Sender<Arc<(CommandLine, anyhow::Result<String>)>>,
}

impl CommandModule {
    pub fn new() -> Self {
        let (output, _) = broadcast::channel(255);
        let mut init = Self {
            map: HashMap::new(),
            output,
        };
        init.register_cmd::<Shutdown>();
        init.register_cmd::<Echo>();
        init.register_cmd::<Clear>();
        init.register_cmd::<Help>();
        init.register_cmd::<Say>();
        init
    }
    pub fn register_cmd<T: Command>(&mut self) {
        let cmd = CommandInst::from::<T>();
        self.map.insert(cmd.clap.get_name().into(), cmd);
    }
    pub fn resolve(&self, name: &str) -> Option<&CommandInst> {
        if let Some(cmd) = self.map.get(name) {
            return Some(cmd);
        }
        for cmd in self.map.values() {
            let alias = cmd.clap.get_visible_aliases();
            let alias = alias.chain(cmd.clap.get_aliases());
            if alias.collect::<HashSet<_>>().contains(name) {
                return Some(cmd);
            }
        }
        None
    }
    pub fn broadcast(&self, cmd: CommandLine, res: anyhow::Result<String>) {
        let res = self.output.send(Arc::new((cmd, res)));
        if res.is_err() {
            warn!("failed send command output")
        }
    }
}

pub trait CommandServer {
    fn print_cmd_output(&self) -> JoinHandle<()>;
    fn server_run_cmd(&self, line: String) -> impl std::future::Future<Output = ()> + Send;
    fn run_cmd(&self, user: &str, line: String) -> impl std::future::Future<Output = ()> + Send;
}

fn resolve_cmd<'a>(server: &'a Server, line: &str) -> anyhow::Result<&'a CommandInst> {
    let name = line
        .split_whitespace()
        .next()
        .ok_or_else(|| anyhow!("unable to parse command"))?;
    let cmd = server
        .cmd
        .resolve(name)
        .ok_or_else(|| anyhow!("unknown command: '{name}'"))?;
    Ok(cmd)
}

async fn run_cmd_checked(server: &Server, user: &str, line: String) -> anyhow::Result<String> {
    let cmd = resolve_cmd(server, &line)?;
    if !server.basileus.check_perm(user, &cmd.perm).await? {
        bail!("permission denied");
    }
    cmd.run(line, server.clone()).await
}

async fn run_cmd_unchecked(server: &Server, line: String) -> anyhow::Result<String> {
    let cmd = resolve_cmd(server, &line)?;
    cmd.run(line, server.clone()).await
}

impl CommandServer for Server {
    async fn run_cmd(&self, user: &str, line: String) {
        let res = run_cmd_checked(self, user, line.clone()).await;
        let cmd = CommandLine {
            user: Some(user.into()),
            line,
        };
        self.cmd.broadcast(cmd, res);
    }
    async fn server_run_cmd(&self, line: String) {
        let res = run_cmd_unchecked(self, line.clone()).await;
        let cmd = CommandLine { user: None, line };
        self.cmd.broadcast(cmd, res);
    }
    fn print_cmd_output(&self) -> JoinHandle<()> {
        let mut output = self.cmd.output.subscribe();
        tokio::spawn(async move {
            loop {
                select! {
                    res = output.recv() => {
                        let res = res.unwrap();
                        let (cmd, res) = &*res;
                        print_info(cmd, res);
                    }
                    _ = wait_shutdown() => break
                }
            }
        })
    }
}
