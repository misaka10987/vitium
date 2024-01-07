use crate::C;
use std::ffi::c_char;
use vitium_common::age::Age;

#[repr(transparent)]
pub struct CAge(pub *const c_char);
impl C<CAge> for Age {
    fn c(&self) -> CAge {
        CAge(self.c())
    }
}
