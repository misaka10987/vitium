use serde_derive::{Deserialize, Serialize};

/// Defines a game module, eg `coc7`.
#[derive(Serialize, Deserialize, Clone)]
pub struct Module {
    /// Module string id, must be unique.
    pub id: String,
    /// Module name.
    pub name: String,
    /// Optional download source when the mod is not found.
    pub url: Option<String>,
    /// Dependencies.
    pub dep: Vec<String>,
}
