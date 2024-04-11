use std::{
    borrow::Cow,
    collections::HashMap,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use tokio::sync::Mutex;
use vitium_common::UID;

pub struct Table<'a, T, S>
where
    T: Clone + 'a,
    S: Query<'a, T>,
{
    alloc: UIDAlloc<T>,
    map: HashMap<UID<T>, &'a S>,
    _a: PhantomData<&'a ()>,
}

pub type Section<'a, T> = HashMap<UID<T>, Cow<'a, T>>;

pub trait Query<'a, T>
where
    T: Clone + 'a,
{
    fn sec(
        &'a self,
    ) -> impl std::future::Future<Output = impl Deref<Target = Section<'a, T>>> + Send;
    fn sec_mut(
        &'a mut self,
    ) -> impl std::future::Future<Output = impl DerefMut<Target = Section<'a, T>>> + Send;
}

/// A UID allocator that thread-safely generates ascending UIDs.
pub struct UIDAlloc<T> {
    now: Mutex<u64>,
    _t: PhantomData<T>,
}

impl<T> UIDAlloc<T> {
    /// Creates a new allocator.
    pub fn new() -> Self {
        Self {
            now: Mutex::new(0),
            _t: PhantomData,
        }
    }
    /// Creates a new allocator with specified starting point.
    pub fn with_start(start: u64) -> Self {
        Self {
            now: Mutex::new(start),
            _t: PhantomData,
        }
    }
    /// Generate a new UID.
    pub async fn gen(&self) -> UID<T> {
        let mut x = self.now.lock().await;
        *x += 1;
        UID::new(*x)
    }
}
