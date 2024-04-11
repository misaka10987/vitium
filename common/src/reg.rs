use std::{borrow::Cow, collections::HashMap};

use crate::ID;

pub struct Reg<T> {
    map: HashMap<ID, T>,
    inv: HashMap<usize, ID>,
}

impl<T: Clone> Reg<T> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            inv: HashMap::new(),
        }
    }
    pub fn inst(&self, ox: Ox<T>) -> Option<Cow<T>> {
        match ox {
            Ox::Reg(id) => self.map.get(&id).map(|r| Cow::Borrowed(r)),
            Ox::Inst(p) => Some(Cow::Owned(*p)),
        }
    }
    pub fn save(&self, cow: &Cow<T>) -> Option<Ox<T>> {
        match cow {
            Cow::Borrowed(r) => {
                let r: &T = r;
                let p = r as *const T as usize;
                self.inv.get(&p).map(|id| Ox::Reg(id.clone()))
            }
            Cow::Owned(t) => Some(Ox::Inst(Box::new(t.clone()))),
        }
    }
}

pub enum Ox<T> {
    Reg(ID),
    Inst(Box<T>),
}
