use std::collections::{HashMap, HashSet};

/// Refers to the current game status.
pub struct GameStat {
    /// Whether the game is ongoing now.
    pub on: bool,
    /// All player characters in this game.
    pub chara: HashSet<String>,
    /// Active player characters and if they had submitted their action.
    pub active: HashMap<String, bool>,
    /// Whether it has a finished turn now.
    pub done: bool,
    /// Whether the game has ended.
    pub term: bool,
    /// Turn number the game has reached.
    pub turn: i64,
    /// Host player of this game.
    pub host: HashSet<String>,
    /// Current mods loaded.
    pub modlist: HashSet<String>,
}
