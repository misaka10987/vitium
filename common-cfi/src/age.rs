use crate::{Rust, C};
use std::ffi::c_char;
use vitium_common::age::Age;

#[repr(transparent)]
pub struct CAge(pub *const c_char);
impl C<CAge> for Age {
    fn c(&self) -> CAge {
        CAge(self.c())
    }
}
impl Rust<Age> for CAge {
    unsafe fn rs(&self) -> Age {
        self.0.rs()
    }
}
