use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    path::PathBuf,
    sync::Arc,
};

use axum::http::header::CONNECTION;
use vitium_common::game::GameStat;

use self::{reg::GameReg, slave::Slave};

pub mod reg;
pub mod slave;

// use axum::http::StatusCode;
// use tokio::sync::oneshot::Sender;
// use vitium_common::act::Act;

// /// Action item waiting the server to process.
// pub(self) struct ActProc {
//     pub act: Act,
//     pub sender: Sender<StatusCode>,
// }

/// Internal game server.
pub struct Game {
    /// Directory that hosts the game.
    pub dir: PathBuf,
    /// Current game status.
    pub stat: GameStat,
    reg: GameReg,
    slave: HashMap<usize, Slave>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            dir: PathBuf::new(),
            stat: GameStat {
                on: false,
                chara: HashSet::new(),
                done: false,
                term: false,
                turn: 0,
                host: String::new(),
                modlist: HashSet::new(),
            },
            reg: todo!(),
            slave: HashMap::new(),
        }
    }
}

// fn f(x: &'static String) {}

// fn g(){
//     let y=Arc::new(String::new());
//     f(&y);
// }
