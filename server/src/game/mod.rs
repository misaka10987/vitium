use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use vitium_api::game::GameStat;

pub mod prelude;
// pub mod proc;
pub mod reg;
pub mod slave;


pub use self::prelude::*;

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
    pub pc_stat: HashMap<String, (usize, u64)>,
    // slave: HashMap<usize, Mutex<Slave>>,
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
            pc_stat: HashMap::new(),
            // slave: HashMap::new(),
        }
    }
    // /// Process an act submitted.
    // pub async fn proc(act: Act, pc: String) -> Receiver<StatusCode> {
    //     let _ = (act, pc);
    //     let (s, r) = oneshot::channel();
    //     s.send(StatusCode::NOT_IMPLEMENTED).unwrap();
    //     r
    // }
}

// pub fn act_handler() -> Router<Server> {
//     Router::new().route("/attack", post(proc::atk))
// }
