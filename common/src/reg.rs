use std::{borrow::Cow, collections::HashMap};

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
    pub fn inst<'a>(&'a self, ox: &'a Ox<T>) -> Option<Cow<'a, T>> {
        match ox {
            Ox::Reg(id) => self.id(&id).map(|x| Cow::Borrowed(x)),
            Ox::Inst(inst) => Some(Cow::Borrowed(inst)),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Ox<T> {
    Reg(ID),
    Inst(Box<T>),
}
