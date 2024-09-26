pub mod err;
pub mod game;
pub mod player;
pub mod prelude;
pub mod net;
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
