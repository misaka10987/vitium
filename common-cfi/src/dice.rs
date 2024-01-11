use crate::{Rust, C};
use std::ffi::c_char;
use vitium_common::dice::Dice;

#[repr(transparent)]
pub struct CDice(*const c_char);
impl C<CDice> for Dice {
    fn c(&self) -> CDice {
        CDice(self.c())
    }
}
impl Rust<Dice> for CDice {
    unsafe fn rs(&self) -> Dice {
        self.0.rs()
    }
}
