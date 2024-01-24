mod act;
mod dice;
mod item;
mod json;
mod player;

pub use std::ffi::c_char;
pub use std::ptr::null;
use std::{
    collections::{HashMap, HashSet},
    ffi::{CStr, CString},
    hash::Hash,
    mem::size_of,
    ptr,
};

/// Defines types that can be converted to C struct `CType`.
pub trait C<CType> {
    /// Converts self to `CType`, which has to be fully C-compatable.
    fn c(&self) -> CType;
}

impl C<()> for () {
    fn c(&self) -> () {
        *self
    }
}

impl<T> C<*mut T> for T
where
    T: Clone,
{
    fn c(&self) -> *mut T {
        Box::into_raw(Box::new(self.clone()))
    }
}

impl C<u64> for u64 {
    fn c(&self) -> u64 {
        *self
    }
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

impl C<*const i8> for String {
    fn c(&self) -> *const i8 {
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

impl<T> C<*mut T> for Option<T>
where
    T: Clone,
{
    fn c(&self) -> *mut T {
        match self {
            Some(v) => v.c(),
            None => null::<T>() as *mut T,
        }
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct CKVPair<K, V> {
    pub key: *mut K,
    pub value: *mut V,
}
impl<K, V> C<CKVPair<K, V>> for CKVPair<K, V>
where
    CKVPair<K, V>: Clone,
{
    fn c(&self) -> CKVPair<K, V> {
        self.clone()
    }
}
impl<K, V, Rk, Rv> C<CKVPair<K, V>> for (&Rk, &Rv)
where
    Rk: C<K> + Clone,
    Rv: C<V> + Clone,
    K: Clone,
    V: Clone,
{
    fn c(&self) -> CKVPair<K, V> {
        // `Rk` to `K` to `*mut K`
        CKVPair {
            key: self.0.c().c(),
            value: self.1.c().c(),
        }
    }
}

pub type CHashMap<K, V> = CVector<CKVPair<K, V>>;
impl<K, V, Rk, Rv> C<CHashMap<K, V>> for HashMap<Rk, Rv>
where
    Rk: C<K> + Clone,
    Rv: C<V> + Clone,
    K: Clone,
    V: Clone,
{
    fn c(&self) -> CHashMap<K, V> {
        let mut v = vec![];
        for i in self {
            v.push(i);
        }
        v.c()
    }
}

pub type CHashSet<T> = CVector<T>;

impl<T, U> C<CHashSet<T>> for HashSet<U>
where
    U: C<T> + Clone,
    T: Clone,
{
    fn c(&self) -> CHashSet<T> {
        let mut v = vec![];
        for i in self {
            v.push(i.clone());
        }
        v.clone().c()
    }
}

pub trait Rust<T> {
    /// Unsafely convert a C type to rust.
    unsafe fn rs(&self) -> T;
}

impl Rust<String> for *const i8 {
    unsafe fn rs(&self) -> String {
        CStr::from_ptr(*self)
            .to_str()
            .expect("failed to convert C const char* to rust")
            .to_string()
    }
}

impl Rust<u64> for u64 {
    unsafe fn rs(&self) -> u64 {
        *self
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

impl<T, U> Rust<Vec<U>> for CVector<T>
where
    T: Rust<U> + Sized,
{
    unsafe fn rs(&self) -> Vec<U> {
        let mut v = vec![];
        for i in Vec::from_raw_parts(self.head, self.len, self.len * size_of::<T>()) {
            v.push(i.rs());
        }
        v
    }
}

impl<K, V, Rk, Rv> Rust<(Rk, Rv)> for CKVPair<K, V>
where
    K: Rust<Rk>,
    V: Rust<Rv>,
{
    unsafe fn rs(&self) -> (Rk, Rv) {
        let (k, v) = (self.key, self.value);
        let (k, v) = (ptr::read(k), ptr::read(v));
        (k.rs(), v.rs())
    }
}

fn hashset<T>(v: Vec<T>) -> HashSet<T>
where
    T: Hash + Eq,
{
    let mut c = HashSet::new();
    for i in v {
        c.insert(i);
    }
    c
}

fn hashmap<K, V>(v: Vec<(K, V)>) -> HashMap<K, V>
where
    K: Hash + Eq,
{
    let mut m = HashMap::new();
    for (i, j) in v {
        m.insert(i, j);
    }
    m
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
