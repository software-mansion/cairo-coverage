use std::collections::HashMap;
use std::hash::Hash;

trait Merge {
    fn merge(&mut self, other: Self);
}

impl<K> Merge for HashMap<K, usize>
where
    K: Eq + Hash,
{
    fn merge(&mut self, other: Self) {
        for (key, value) in other {
            *self.entry(key).or_default() += value;
        }
    }
}

impl<K, V> Merge for HashMap<K, V>
where
    K: Eq + Hash,
    V: Merge + Clone,
{
    fn merge(&mut self, other: Self) {
        for (key, value) in other {
            self.entry(key)
                .and_modify(|e| e.merge(value.clone()))
                .or_insert(value);
        }
    }
}

pub trait MergeOwned {
    fn merge_owned(self, other: Self) -> Self;
}

impl<T> MergeOwned for T
where
    T: Merge,
{
    fn merge_owned(mut self, other: Self) -> Self {
        self.merge(other);
        self
    }
}
