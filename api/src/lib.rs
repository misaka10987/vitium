pub mod chat;
pub mod cmd;
pub mod game;
pub mod net;
pub mod prelude;
pub mod uid;
pub mod user;

pub use prelude::*;

#[cfg_attr(target_family = "wasm", tsify_next::declare)]
pub type Dice = String;

/// This is some documentation.
#[cfg_attr(target_family = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}
