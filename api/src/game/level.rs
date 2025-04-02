use std::ops::{Add, Sub};

use serde::{Deserialize, Serialize};

#[cfg(target_family = "wasm")]
use {tsify_next::Tsify, wasm_bindgen::prelude::wasm_bindgen};

#[derive(Serialize, Deserialize, Clone, Copy)]
#[cfg_attr(target_family = "wasm", derive(Tsify))]
#[cfg_attr(target_family = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
pub struct Level {
    /// Level when the character is created.
    pub born: i16,
    /// Level grown during the game.
    pub growth: i16,
    /// Bonus currently have.
    pub bonus: i16,
}

impl Add<i16> for Level {
    type Output = i16;

    fn add(self, rhs: i16) -> Self::Output {
        self.curr() + rhs
    }
}

impl Sub<i16> for Level {
    type Output = i16;

    fn sub(self, rhs: i16) -> Self::Output {
        self.curr() - rhs
    }
}

impl Add for Level {
    type Output = i16;

    fn add(self, rhs: Self) -> Self::Output {
        self.curr() + rhs.curr()
    }
}

impl Sub for Level {
    type Output = i16;

    fn sub(self, rhs: Self) -> Self::Output {
        self.curr() - rhs.curr()
    }
}

impl Level {
    pub fn new(born: i16) -> Self {
        Self {
            born,
            growth: 0,
            bonus: 0,
        }
    }
    pub fn base(&self) -> i16 {
        self.born + self.growth
    }
    pub fn curr(&self) -> i16 {
        self.base() + self.bonus
    }
}
