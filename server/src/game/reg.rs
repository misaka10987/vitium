use vitium_common::game::{Item, Reg, Spell};

pub struct GameReg {
    pub item: &'static Reg<Item>,
    pub spell: &'static Reg<Spell>,
}

impl Drop for GameReg {
    fn drop(&mut self) {
        unsafe {
            Reg::drop(self.item);
        }
    }
}

impl GameReg {
    pub fn new(reg: RegLoader) -> Self {
        Self {
            item: reg.item.leak(),
            spell: reg.spell.leak(),
        }
    }
}

pub struct RegLoader {
    pub item: Reg<Item>,
    pub spell: Reg<Spell>,
}
