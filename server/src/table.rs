use std::{
    collections::{BTreeMap, VecDeque},
    marker::PhantomData,
};

use tokio::sync::{Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard};
use vitium_common::UID;

pub struct Table<T> {
    alloc: UIDAlloc<T>,
    delta: RwLock<VecDeque<UID<T>>>,
    pub table: BTreeMap<UID<T>, RwLock<T>>,
}

impl<T: Clone> Table<T> {
    pub fn new() -> Self {
        Self {
            alloc: UIDAlloc::new(),
            delta: RwLock::new(VecDeque::with_capacity(128)),
            table: BTreeMap::new(),
        }
    }
    pub async fn insert(&mut self, content: T) -> UID<T> {
        let uid = self.alloc.gen().await;
        self.table.insert(uid, RwLock::new(content));
        uid
    }
    pub fn read(&self, uid: UID<T>) -> Option<RwLockReadGuard<T>> {
        self.table.get(&uid).map(|x| x.blocking_read())
    }
    pub fn write(&self, uid: UID<T>) -> Option<RwLockWriteGuard<T>> {
        let mut d = self.delta.blocking_write();
        while d.len() >= d.capacity() {
            d.pop_front();
        }
        d.push_back(uid);
        self.table.get(&uid).map(|x| x.blocking_write())
    }
    pub async fn delta(&self) -> Vec<(UID<T>, Option<T>)> {
        self.delta
            .read()
            .await
            .iter()
            .copied()
            .map(|x| (x, self.read(x).map(|y| y.clone())))
            .collect()
    }
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
