mod age;
mod dice;
mod item;
mod json;
mod player;

pub use std::ffi::c_char;
use std::{ffi::CString, ptr::null};

fn ptr<T>(dat: &T) -> *const T {
    Box::into_raw(Box::new(dat)) as *const T
}

/// Defines types that can be converted to C struct `CType`.
pub trait C<CType> {
    /// Converts self to `CType`, which has to be fully C-compatable.
    fn c(&self) -> CType;
}

impl C<*const c_char> for &str {
    fn c(&self) -> *const c_char {
        self.to_string().c()
    }
}
impl C<*const c_char> for String {
    fn c(&self) -> *const c_char {
        CString::new(self.clone()).unwrap().into_raw()
    }
}

/// rust `std::Vec<T>`'s C-equivalant.
///
/// Note that type infomation `T` will be lost since it is converted to C's pointer
/// therefore it is wise to export `T`'s C-equivalant and documentation as well.
/// Also, it is not guaranteed here that `T` correctly implements trait `C`.
#[repr(C)]
pub struct CVector {
    pub head: *const i8,
    pub len: usize,
}
impl<T> C<CVector> for Vec<T> {
    fn c(&self) -> CVector {
        CVector {
            head: self.as_ptr() as *const i8,
            len: self.len(),
        }
    }
}

#[repr(C)]
pub struct COption {
    pub value: *const i8,
}
impl<T> C<*const T> for Option<T> {
    fn c(&self) -> *const T {
        match self {
            Some(v) => ptr(v),
            None => null(),
        }
    }
}

#[export_name = "vitium_common_cfi_hello"]
pub extern "C" fn hello() -> *const c_char {
    "Hello, world!\n".c()
}

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
