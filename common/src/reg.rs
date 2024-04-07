use std::{borrow::Cow, collections::HashMap};

use crate::ID;

pub struct Reg<T> {
    table: HashMap<ID, T>,
}

impl<T: Clone> Reg<T> {
    pub fn new() -> Reg<T> {
        Reg {
            table: HashMap::new(),
        }
    }
    pub fn id(&self, id: &ID) -> Option<Cow<T>> {
        self.table.get(id).map(|x| Cow::Borrowed(x))
    }
}
