pub mod act;
pub mod age;
pub mod chara;
pub mod cmd;
pub mod config;
pub mod dice;
pub mod feature;
pub mod game;
pub mod item;
pub mod json;
pub mod module;
pub mod player;
mod prelude;
pub mod record;
pub mod registry;
pub mod req;
pub mod res;
pub mod scene;
pub mod skill;
pub mod sync;
pub mod util;
pub mod vehicle;

pub use crate::prelude::*;

pub const DEBUG_DESCR: &str = "If you see this in game, it is a bug.";

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub trait ID {
    fn id(&self) -> Option<&str>;
}

pub trait UID {
    fn uid(&self) -> u64;
    fn set_uid(&mut self, uid: u64) -> &mut Self;
    fn no_uid(&self) -> bool {
        self.uid() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
