use std::collections::{linked_list, BTreeSet, LinkedList};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub trait Delta {
    type Item: Serialize + DeserializeOwned;
    fn calc(&mut self) -> impl Iterator<Item = Self::Item>;
    fn diff(&self) -> impl Iterator<Item = Self::Item>;
    fn apply(&mut self, delta: impl Iterator<Item = Self::Item>);
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DeltaList<K, V>
where
    K: Clone + Eq + Ord,
{
    pub cap: usize,
    data: LinkedList<(K, V)>,
    key: BTreeSet<K>,
}

pub type PackDeltaList<K, V> = Vec<(K, V)>;

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
    pub fn pack(&self) -> impl Iterator<Item = (K, V)> + '_ {
        self.data.iter().cloned()
    }
}
