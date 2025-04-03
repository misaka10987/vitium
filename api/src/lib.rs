pub mod err;
pub mod game;
pub mod net;
pub mod prelude;
pub mod uid;
pub mod user;

pub use prelude::*;

pub type Dice = String;

#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

/// This is some documentation.
#[cfg_attr(target_family = "wasm", wasm_bindgen)]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg_attr(target_family = "wasm", wasm_bindgen(typescript_custom_section))]
const _TS_APPEND_CONTENT: &'static str = r#"export type Id<T> = string;"#;

// #[cfg_attr(target_family = "wasm", wasm_bindgen)]
// pub fn _fool_wasm_pack() -> Id<Item> {
//     panic!()
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
