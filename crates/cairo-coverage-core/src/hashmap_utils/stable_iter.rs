use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;

/// Extension trait for [`HashMap`] to enable deterministic ownership iteration.
pub trait IntoStableIter<K, V> {
    fn into_stable_iter(self) -> impl Iterator<Item = (K, V)>;
}

impl<K: Ord, V> IntoStableIter<K, V> for HashMap<K, V> {
    fn into_stable_iter(self) -> impl Iterator<Item = (K, V)> {
        self.into_iter().sorted_by(keys)
    }
}

/// Extension trait for [`HashMap`] to enable deterministic borrowing iteration.
pub trait StableIter<K, V> {
    fn stable_iter<'a>(&'a self) -> impl Iterator<Item = (&'a K, &'a V)>
    where
        K: 'a,
        V: 'a;
}

impl<K: Ord, V> StableIter<K, V> for HashMap<K, V> {
    fn stable_iter<'a>(&'a self) -> impl Iterator<Item = (&'a K, &'a V)>
    where
        K: 'a,
        V: 'a,
    {
        self.iter().sorted_by(keys)
    }
}

/// Sorts keys in ascending order.
fn keys<K: Ord, V>((k1, _): &(K, V), (k2, _): &(K, V)) -> Ordering {
    k1.cmp(k2)
}
