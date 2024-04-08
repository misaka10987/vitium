use std::collections::HashMap;

use crate::{util::Ox, ID};

pub struct Reg<T>(HashMap<ID, T>);

impl<T> Reg<T> {
    pub fn id(&self, id: &ID) -> Option<&T> {
        self.0.get(id)
    }
    pub fn inst<'a>(&'a self, ox: Ox<'a, T>) -> Option<&'a T> {
        match ox {
            Ox::Reg(id) => self.id(&id),
            Ox::Inst(inst) => Some(&inst),
        }
    }
}
