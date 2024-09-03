pub mod cmd;
pub mod game;
pub mod player;
pub mod prelude;
pub mod reg;
pub mod req;
pub mod res;
pub mod uid;

pub use prelude::*;

pub type Dice = String;

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
