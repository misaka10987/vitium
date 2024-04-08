use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::ID;

pub type Item = ();

pub struct RegTable<T>(HashMap<ID, T>);

impl<T> RegTable<T> {
    fn new() -> Self {
        Self(HashMap::new())
    }
}

pub struct Reg {
    pub item: RegTable<Item>,
}

impl Reg {
    fn new() -> Self {
        Self {
            item: RegTable::new(),
        }
    }
}

static mut REG: Lazy<Reg> = Lazy::new(|| Reg::new());

pub fn reg() -> &'static Lazy<Reg> {
    unsafe { &REG }
}

pub fn reg_mut() -> &'static mut Lazy<Reg> {
    unsafe { &mut REG }
}
