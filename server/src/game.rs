use std::{collections::HashSet, path::PathBuf};

use vitium_common::game::GameStat;

pub mod reg;

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
        }
    }
}
