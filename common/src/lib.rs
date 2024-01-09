pub mod act;
pub mod age;
pub mod chara;
pub mod cmd;
pub mod config;
pub mod dice;
pub mod game;
pub mod item;
pub mod json;
pub mod module;
pub mod player;
pub mod registry;
pub mod request;
pub mod response;
pub mod scene;
pub mod skill;
pub mod story;
pub mod util;
pub mod vehicle;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub trait ID {
    fn id(&self) -> String;
}

pub trait UID {
    fn uid(&self) -> i128;
    fn set_uid(&mut self, uid: i128) -> &mut Self;
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
