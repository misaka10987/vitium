use std::{
    borrow::Cow,
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use crate::ID;

pub struct Reg<T: AsRef<Option<ID>>>(HashMap<ID, T>);

impl<T> Deref for Reg<T>
where
    T: Clone + AsRef<Option<ID>>,
{
    type Target = HashMap<ID, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Reg<T>
where
    T: Clone + AsRef<Option<ID>>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Reg<T>
where
    T: Clone + AsRef<Option<ID>>,
{
    pub fn new() -> Self {
        Self(HashMap::new())
    }
    pub fn inst(&self, ox: Ox<T>) -> Option<Cow<T>> {
        match ox {
            Ox::Reg(id) => self.get(&id).map(|r| Cow::Borrowed(r)),
            Ox::Inst(p) => Some(Cow::Owned(*p)),
        }
    }
    /// # Panics
    /// Panic if `cow` is `Borrowed` from a registry item that does not specify an `ID`.
    pub fn save(&self, cow: &Cow<T>) -> Ox<T> {
        match cow {
            Cow::Borrowed(r) => Ox::Reg(r.as_ref().clone().unwrap()),
            Cow::Owned(t) => Ox::Inst(Box::new(t.clone())),
        }
    }
}

pub enum Ox<T> {
    Reg(ID),
    Inst(Box<T>),
}
