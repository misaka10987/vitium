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

/// Tests WASM bigint functionality.
///
/// This function prints "Hello, WASM!" to the console and returns 42 as a 64-bit integer.
#[cfg_attr(target_family = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub fn test() -> u64 {
    println!("Hello, WASM!");
    42
}
