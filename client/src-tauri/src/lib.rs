mod auth;
mod chat;
mod cmd;
mod err;
mod net;
mod respack;
// mod respack;

use std::{fs::File, io::Read, path::PathBuf};

use clap::{Parser, Subcommand};
use dirs::config_dir;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tauri::{generate_context, generate_handler};
use tokio::sync::RwLock;

use once_cell::sync::Lazy;
use tracing::{info, warn, Level};

#[derive(Parser, Debug)]
#[command(about, version)]
#[command(after_help = "Launch the client GUI if none of the subcommands is provided.")]
struct Args {
    /// Override configuration file.
    #[arg(short, long, default_value = config_dir().unwrap().join("vitium").join("client.toml").into_os_string())]
    pub config: PathBuf,
    #[command(subcommand)]
    pub cmd: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Signup a new user on specified server.
    Signup {
        #[arg(short, long)]
        /// Username.
        user: String,
        #[arg(short, long)]
        /// Password.
        pass: String,
        /// Server address.
        #[arg(value_name = "SERVER")]
        server: String,
    },
    /// Change password for specified user on server.
    #[clap(name = "chpass")]
    ChPass {
        #[arg(short, long)]
        /// Username.
        user: String,
        #[arg(long)]
        /// Old assword.
        old: String,
        #[arg(long)]
        /// New assword.
        new: String,
        /// Server address.
        #[arg(value_name = "SERVER")]
        server: String,
    },
}

static ARG: Lazy<Args> = Lazy::new(Args::parse);

#[derive(Clone, Serialize, Deserialize)]
struct Config {
    pub respack: Vec<PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            respack: Default::default(),
        }
    }
}

fn load_cfg() -> anyhow::Result<Config> {
    let path = &ARG.config;
    let mut buf = String::new();
    File::open(path)?.read_to_string(&mut buf)?;
    Ok(toml::from_str(&buf)?)
}

fn try_load_cfg() -> Config {
    match load_cfg() {
        Ok(c) => c,
        Err(e) => {
            warn!(
                "failed to load {}: {e}, using default",
                ARG.config.display()
            );
            Config::default()
        }
    }
}

static CFG: Lazy<Config> = Lazy::new(try_load_cfg);

static SERVER_ADDR: RwLock<String> = RwLock::const_new(String::new());

static USER: RwLock<String> = RwLock::const_new(String::new());

static CLIENT: Lazy<Client> = Lazy::new(|| Client::new());

#[tauri::command]
fn hello() -> &'static str {
    "Hello, world!"
}

#[tauri::command]
async fn user() -> String {
    USER.read().await.to_owned()
}

#[tauri::command]
async fn server_addr() -> String {
    SERVER_ADDR.read().await.to_owned()
}

#[tauri::command]
fn server_stat() {
    todo!()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> anyhow::Result<()> {
    if let Some(c) = &ARG.cmd {
        return match c {
            Commands::Signup { user, pass, server } => cmd::signup(user, pass, server),
            Commands::ChPass {
                user,
                old,
                new,
                server,
            } => cmd::chpass(user, old, new, server),
        };
    }
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();
    info!("running with {:?}", *ARG);
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(generate_handler![
            hello,
            user,
            server_addr,
            server_stat,
            auth::login,
            chat::recv_chat,
            chat::send_chat,
            chat::render_chat,
            chat::chat_modified,
        ])
        .run(generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
