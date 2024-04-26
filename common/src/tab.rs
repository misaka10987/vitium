use std::{borrow::Cow, collections::HashMap};

use bevy_ecs::storage::SparseSet;
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
    tab: SparseSet<UID<T>, Ox<T>>,
    delta: DeltaList<UID<T>, Ox<T>>,
}

impl<T> Tab<T>
where
    T: Clone + AsRef<Option<ID>>,
{
    pub fn new(reg: &'static Reg<T>, delta_cap: usize) -> Self {
        Self {
            reg,
            tab: SparseSet::new(),
            delta: DeltaList::new(delta_cap),
        }
    }
    // pub fn read(&self, index: UID<T>) -> Option<&Cow<'static, T>> {
    //     if let Some(ox) = self.tab.get(index) {
    //         self.reg.inst(ox)
    //     } else {
    //         None
    //     }
    // }
    // pub fn write(&mut self, index: UID<T>) -> Option<&mut Cow<'static, T>> {
    //     match self.tab.get_mut(index) {
    //         Some(r) => {
    //             println!("not implemented");
    //             Some(r)
    //         }
    //         None => None,
    //     }
    // }
}
