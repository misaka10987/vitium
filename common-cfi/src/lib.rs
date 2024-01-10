mod act;
mod age;
mod dice;
mod item;
mod json;
mod player;

pub use std::ffi::c_char;
pub use std::ptr::null;
use std::{
    ffi::{CStr, CString},
    mem::size_of,
};

fn ptr<T>(dat: &T) -> *const T {
    Box::into_raw(Box::new(dat)) as *const T
}

/// Defines types that can be converted to C struct `CType`.
pub trait C<CType> {
    /// Converts self to `CType`, which has to be fully C-compatable.
    fn c(&self) -> CType;
}

impl C<i64> for i128 {
    fn c(&self) -> i64 {
        *self as i64
    }
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
pub struct CVector<T> {
    pub head: *mut T,
    pub len: usize,
}
// impl<T> C<CVector<T>> for Vec<T> {
//     fn c(&self) -> CVector<T> {
//         CVector {
//             head: self.as_ptr() as *const T,
//             len: self.len(),
//         }
//     }
// }

// impl<T> C<T> for T
// where
//     T: Clone,
// {
//     fn c(&self) -> T {
//         self.clone()
//     }
// }

impl<T, U> C<CVector<U>> for Vec<T>
where
    T: C<U>,
{
    fn c(&self) -> CVector<U> {
        let mut v = Vec::<U>::new();
        for i in self {
            v.push(i.c());
        }
        CVector {
            head: v.as_mut_ptr(),
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

pub trait Rust<T> {
    /// Unsafely convert a C type to rust.
    unsafe fn rs(&self) -> T;
}

impl Rust<String> for *const c_char {
    unsafe fn rs(&self) -> String {
        CStr::from_ptr(*self)
            .to_str()
            .expect("failed to convert C const char* to rust")
            .to_string()
    }
}

impl<T> Rust<Option<T>> for *mut T {
    unsafe fn rs(&self) -> Option<T> {
        if *self as usize == 0 {
            None
        } else {
            Some(*Box::<T>::from_raw(*self))
        }
    }
}

impl<T,U> Rust<Vec<U>> for CVector<T>
where T:Rust<U>+Sized
{
    unsafe fn rs(&self) -> Vec<U> {
        let mut v=vec![];
        for i in Vec::from_raw_parts(self.head, self.len, self.len*size_of::<T>()){
            v.push(i.rs());
        }
        v
    }
}

// impl<T> Rust<Vec<T>> for CVector<T>
// where
//     T: Sized,
// {
//     unsafe fn rs(&self) -> Vec<T> {
//         Vec::from_raw_parts(self.head, self.len, self.len * size_of::<T>())
//     }
// }

#[export_name = "vitium_common_cfi_hello"]
extern "C" fn hello() -> *const c_char {
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
