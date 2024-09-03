use std::collections::HashSet;

/// Instance of scenario.
pub struct Scena {
    /// Displayed name.
    pub name: String,
    /// Description showed when a character enters.
    pub descr: String,
    pub pos: (f64, f64),
    /// Player characters.
    pub pc: HashSet<String>,
}
