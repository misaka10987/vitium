use std::collections::HashMap;

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
