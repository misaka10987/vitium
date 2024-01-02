pub mod age;
pub mod armor;
pub mod attr;
pub mod bottle;
pub mod character;
pub mod dice;
pub mod envelop;
pub mod item;
pub mod player;
pub mod price;
pub mod prof;
pub mod registry;
pub mod request;
pub mod response;
pub mod skill;
pub mod util;
pub mod vehicle;
pub mod weapon;

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
