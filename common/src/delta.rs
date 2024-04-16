use std::collections::{linked_list, BTreeSet, LinkedList};

use serde::{Deserialize, Serialize};

pub trait Delta {
    type Pack: Serialize + Deserialize<'static>;
    fn calc(&mut self) -> &Self::Pack;
    fn diff(&self) -> &Self::Pack;
    fn apply(&mut self, delta: Self::Pack);
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DeltaList<K, V>
where
    K: Clone + Eq + Ord,
{
    pub cap: usize,
    pub data: LinkedList<(K, V)>,
    key: BTreeSet<K>,
}

pub type PackDeltaList<K, V> = LinkedList<(K, V)>;

impl<K, V> Extend<(K, V)> for DeltaList<K, V>
where
    K: Clone + Eq + Ord,
    V: Clone,
{
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        for (k, v) in iter {
            self.append(k, v)
        }
    }
}

impl<K, V> IntoIterator for DeltaList<K, V>
where
    K: Clone + Eq + Ord,
{
    type Item = (K, V);

    type IntoIter = linked_list::IntoIter<(K, V)>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<K, V> DeltaList<K, V>
where
    K: Clone + Eq + Ord,
    V: Clone,
{
    pub fn new(cap: usize) -> Self {
        Self {
            cap,
            data: LinkedList::new(),
            key: BTreeSet::new(),
        }
    }
    pub fn append(&mut self, key: K, value: V) {
        while self.data.len() >= self.cap {
            self.data.pop_front();
        }
        if self.key.contains(&key) {
            for (k, v) in &mut self.data {
                if key == *k {
                    *v = value;
                    return;
                }
            }
        } else {
            self.key.insert(key.clone());
            self.data.push_back((key, value));
        }
    }
    pub fn pack(&self) -> PackDeltaList<K, V> {
        self.data.clone()
    }
}
