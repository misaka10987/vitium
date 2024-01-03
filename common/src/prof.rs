use crate::{age::Age, skill::Skill};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Prof {
    pub age: Age,
    pub id: String,
    pub name: String,
    pub credit: u16,
    pub skills: Vec<(Skill, u16)>,
}
