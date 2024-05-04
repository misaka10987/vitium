// use vitium_common::{
//     game::{Item, Spell},
//     t_recs::reg::RegTab,
// };

// pub struct GameReg {
//     pub item: &'static RegTab<Item>,
//     pub spell: &'static RegTab<(Item)>,
// }

// impl Drop for GameReg {
//     fn drop(&mut self) {
//         unsafe {
//             RegTab::drop(self.item);
//         }
//     }
// }

// impl GameReg {
//     pub fn new(reg: RegLoader) -> Self {
//         Self {
//             item: reg.item.leak(),
//             spell: reg.spell.leak(),
//         }
//     }
// }

// pub struct RegLoader {
//     pub item: RegTab<Item>,
//     pub spell: RegTab<Spell>,
// }
