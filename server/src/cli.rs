use std::thread;

use rustyline::{error::ReadlineError, DefaultEditor};
use tokio::runtime;

use crate::{server::CommandServer, Server};

pub fn start(server: Server) -> anyhow::Result<()> {
    let mut rl = DefaultEditor::new()?;
    thread::spawn(move || {
        let server = server;
        loop {
            if server.shutdown.off() {
                break;
            }
            let readline = rl.readline("");
            match readline {
                Ok(line) => {
                    if line.is_empty() || line.chars().all(|x| x.is_whitespace()) {
                        continue;
                    }
                    rl.add_history_entry(line.as_str()).unwrap();
                    let server = server.clone();
                    runtime::Builder::new_current_thread()
                        .build()
                        .unwrap()
                        .block_on(async move {
                            server.server_run_cmd(line).await;
                        });
                }
                Err(ReadlineError::Interrupted | ReadlineError::Eof) => {
                    shutup::ROOT.shut();
                    break;
                }
                Err(e) => {
                    shutup::ROOT.shut();
                    panic!("{e:?}")
                }
            }
        }
    });
    Ok(())
}
