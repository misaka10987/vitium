use std::{borrow::Cow, collections::HashMap};

use serde::{Deserialize, Serialize};

use crate::{
    delta::{Delta, DeltaList},
    game::{Ox, Reg},
    ID, UID,
};

pub struct Tab<T>
where
    T: Clone + AsRef<Option<ID>> + 'static,
{
    reg: &'static Reg<T>,
    tab: HashMap<UID<T>, Cow<'static, T>>,
    delta: DeltaList<UID<T>, Ox<T>>,
}

impl<T> Delta for Tab<T>
where
    T: Clone + AsRef<Option<ID>> + Serialize + Deserialize<'static>,
{
    type Pack = (UID<T>, Ox<T>);

    fn calc(&mut self) -> impl Iterator<Item = &Self::Pack> {
        self.delta.data.iter()
    }

    fn diff(&self) -> impl Iterator<Item = &Self::Pack> {
        self.delta.data.iter()
    }

    fn apply(&mut self, delta: impl Iterator<Item = Self::Pack>) {
        for (k, v) in delta {
            if let Some(p) = self.reg.inst(v) {
                self.tab.insert(k, p);
            }
        }
    }
}

impl<T> Tab<T>
where
    T: Clone + AsRef<Option<ID>>,
{
    pub fn new(reg: &'static Reg<T>, delta_cap: usize) -> Self {
        Self {
            reg,
            tab: HashMap::new(),
            delta: DeltaList::new(delta_cap),
        }
    }
    pub fn read(&self, index: UID<T>) -> Option<&Cow<T>> {
        self.tab.get(&index)
    }
    pub fn write(&mut self, index: UID<T>) -> Option<&mut Cow<'static, T>> {
        match self.tab.get_mut(&index) {
            Some(r) => {
                println!("not implemented");
                Some(r)
            }
            None => None,
        }
    }
}
