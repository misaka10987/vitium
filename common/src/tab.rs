use std::{borrow::Cow, collections::HashMap};

use serde::{Deserialize, Serialize};

use crate::{
    delta::{Delta, DeltaList, PackDeltaList},
    game::{Ox, Reg},
    ID, UID,
};

pub struct Tab<'a, T>
where
    T: Clone + AsRef<Option<ID>>,
{
    reg: &'a Reg<T>,
    tab: HashMap<UID<T>, Cow<'a, T>>,
    delta: DeltaList<UID<T>, Ox<T>>,
}

impl<'a, T> Delta for Tab<'a, T>
where
    T: Clone + AsRef<Option<ID>> + Serialize + Deserialize<'static>,
{
    type Pack = PackDeltaList<UID<T>, Ox<T>>;

    fn calc(&mut self) -> &Self::Pack {
        &self.delta.data
    }

    fn diff(&self) -> &Self::Pack {
        &self.delta.data
    }

    fn apply(&mut self, delta: Self::Pack) {
        for (k, v) in delta {
            if let Some(p) = self.reg.inst(v) {
                self.tab.insert(k, p);
            }
        }
    }
}

impl<'a, T> Tab<'a, T>
where
    T: Clone + AsRef<Option<ID>>,
{
    pub fn new(reg: &'a Reg<T>, delta_cap: usize) -> Self {
        Self {
            reg,
            tab: HashMap::new(),
            delta: DeltaList::new(delta_cap),
        }
    }
    pub fn read(&self, index: UID<T>) -> Option<&Cow<'a, T>> {
        self.tab.get(&index)
    }
    pub fn write(&mut self, index: UID<T>) -> Option<&mut Cow<'a, T>> {
        match self.tab.get_mut(&index) {
            Some(r) => {
                self.delta.append(index, self.reg.save(r));
                Some(r)
            }
            None => None,
        }
    }
}
