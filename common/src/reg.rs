use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::ID;

pub struct Reg<T>(HashMap<ID, T>);

impl<T: Clone> Reg<T> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
    pub fn id(&self, id: &ID) -> Option<&T> {
        self.0.get(id)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Ox<T> {
    Reg(ID),
    Inst(Box<T>),
}
