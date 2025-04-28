mod clear;
mod echo;
mod help;
mod kill;
mod loglevel;
mod say;
mod shutdown;

use anyhow::{anyhow, bail};
use axum::{
    Json, Router,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Sse, sse::Event},
    routing::get,
};
use basileus::{Perm, perm::PermManage};
use clap::Parser;
use clear::Clear;
use colored::Colorize;
use echo::Echo;
use help::Help;
use kill::Kill;
use loglevel::LogLevel;
use say::Say;
use shutdown::Shutdown;
use shutup::ShutUp;
use std::{
    collections::{HashMap, HashSet},
    future::Future,
    pin::Pin,
    sync::Arc,
};
use tokio::{select, sync::broadcast, task::JoinHandle};
use tokio_stream::{Stream, StreamExt};
use tracing::warn;
use vitium_api::cmd::{CommandLine, CommandStatus};

use crate::Server;

use super::{auth::Token, internal_server_error};

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
        let perm = T::perm_req();
        let exe = |line: String, server| async move {
            let it = shell_words::split(&line)?;
            let mut clap = T::command();

            if let Err(e) = clap.clone().try_get_matches_from(&it) {
                if e.kind() == clap::error::ErrorKind::DisplayHelp {
                    return Ok(format!("{}", clap.render_long_help().ansi()));
                }
            }

            let arg = T::try_parse_from(it).map_err(|e| anyhow!("{}", e.render().ansi()))?;
            arg.exec(server).await
        };
        Self {
            clap: T::command(),
            perm,
            exe: Box::new(exe),
        }
    }
    pub async fn run(&self, arg: String, server: Server) -> anyhow::Result<String> {
        self.exe.exec(arg, server).await
    }
    pub fn help_page(&self) -> String {
        let help = self.clap.clone().render_long_help();
        format!("{help}")
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
        init.register_cmd::<Kill>();
        init.register_cmd::<LogLevel>();
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
    fn print_cmd_output(&self, shutdown: ShutUp) -> JoinHandle<()>;
    fn server_run_cmd(&self, line: String) -> impl std::future::Future<Output = ()> + Send;
    fn run_cmd(&self, user: &str, line: String) -> impl std::future::Future<Output = ()> + Send;
    fn cmd_output(&self) -> broadcast::Receiver<Arc<(CommandLine, anyhow::Result<String>)>>;
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
    if !server.check_perm(user, &cmd.perm).await? {
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
    fn cmd_output(&self) -> broadcast::Receiver<Arc<(CommandLine, anyhow::Result<String>)>> {
        self.cmd.output.subscribe()
    }
    fn print_cmd_output(&self, shutdown: ShutUp) -> JoinHandle<()> {
        let mut output = self.cmd.output.subscribe();
        tokio::spawn(async move {
            loop {
                select! {
                    res = output.recv() => {
                        if let Ok(res) = res {
                            let (cmd, res) = &*res;
                            print_info(cmd, res);
                        }
                    }
                    _ = shutdown.wait() => break
                }
            }
        })
    }
}

/// The REST API method router.
pub fn rest() -> Router<Server> {
    Router::new().route("/", get(fetch).post(create))
}

async fn fetch(State(s): State<Server>, head: HeaderMap) -> impl IntoResponse {
    match head.get("accept") {
        Some(accept) if accept.to_str().unwrap_or("").contains("text/event-stream") => {
            fetch_sse(s).await.into_response()
        }
        _ => fetch_longpoll(s).await.into_response(),
    }
}

async fn fetch_sse(s: Server) -> Sse<impl Stream<Item = anyhow::Result<Event>>> {
    let wait = move |_| {
        let s = s.clone();
        async move {
            let mut output = s.cmd_output();
            let (cmd, res) = &*output.recv().await?;
            let res = match res {
                Ok(x) => Ok(x),
                Err(e) => Err(format!("{e}")),
            };
            let event = Event::default().json_data((cmd, res))?;
            Ok(event)
        }
    };
    let stream = tokio_stream::iter(0..).then(wait);
    Sse::new(stream)
}

async fn fetch_longpoll(s: Server) -> Result<Json<(CommandLine, CommandStatus)>, StatusCode> {
    let output = s.cmd_output().recv().await.map_err(internal_server_error)?;
    let (cmd, res) = &*output;
    let res = match res {
        Ok(x) => Ok(x.clone()),
        Err(e) => Err(format!("{e}")),
    };
    Ok(Json((cmd.clone(), res)))
}

async fn create(
    State(s): State<Server>,
    Token(user): Token,
    Json(cmd): Json<CommandLine>,
) -> Result<(), StatusCode> {
    if cmd.user.is_some_and(|x| x == user) {
        return Err(StatusCode::FORBIDDEN);
    }
    s.run_cmd(&user, cmd.line).await;
    Ok(())
}
