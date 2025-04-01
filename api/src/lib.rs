pub mod err;
pub mod game;
pub mod net;
pub mod player;
pub mod prelude;
pub mod uid;

pub use prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;

pub type Dice = String;

/// This is some documentation.
#[wasm_bindgen]
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
