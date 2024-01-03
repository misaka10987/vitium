pub mod action;
pub mod age;
pub mod chara;
pub mod dice;
pub mod game;
pub mod item;
pub mod load;
pub mod module;
pub mod player;
pub mod registry;
pub mod request;
pub mod response;
pub mod scene;
pub mod script;
pub mod skill;
pub mod util;
pub mod vehicle;

pub fn add(left: usize, right: usize) -> usize {
    left + right
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
