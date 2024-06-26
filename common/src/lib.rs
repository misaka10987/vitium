pub mod cmd;
pub mod delta;
pub mod dice;
pub mod error;
pub mod game;
pub mod module;
pub mod player;
pub mod prelude;
pub mod record;
pub mod req;
pub mod res;
pub mod t_recs;
#[cfg(test)]
mod test;
pub mod typename;
pub mod util;

pub use crate::prelude::*;

pub const DEBUG_DESCR: &str = "If you see this in game, it is a bug.";

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let id: Id = obj("\"homo:sapiens\"").unwrap();
        assert_eq!(id, Id::new("homo", "sapiens"));
    }
}
