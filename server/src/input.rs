use clearscreen::clear;
use std::io;
use std::process::{id as pid, Command, ExitStatus};
use std::thread::spawn;
use std::{collections::VecDeque, process::exit};
use tokio::sync::oneshot;

pub fn input() -> oneshot::Sender<()> {
    let (s, mut r) = oneshot::channel();
    spawn(move || {
        while let Err(_) = r.try_recv() {
            let mut buf = String::new();
            io::stdin().read_line(&mut buf).unwrap();
            if let Err(e) = proc(&buf) {
                eprintln!("  !! {}", e)
            }
        }
    });
    s
}

fn term() -> io::Result<ExitStatus> {
    Command::new("kill")
        .arg("-INT")
        .arg(pid().to_string())
        .status()
}

fn resolve(cmd: &str) -> (&str, Vec<&str>) {
    let mut token: VecDeque<_> = cmd.trim().split(' ').collect();
    (token.pop_front().unwrap(), token.into())
}

fn proc(cmd: &str) -> Result<(), String> {
    let (cmd, _) = resolve(cmd);
    match cmd {
        "exit" => term().map_err(|e| e.to_string()).map(|_| ()),
        "help" => Err("TODO".to_string()),
        "clear" => clear().map_err(|e| e.to_string()),
        "kill" => exit(-1),
        _ => Err(format!("{} not found", cmd)),
    }
}
