use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::Hash;

/// Trait for merging two values
pub trait Merge {
    fn merge(&mut self, other: Self);
}

/// Corner case implementation which merges two [`HashMap`]'s
/// with `usize` values by summing the values for each key.
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

/// General implementation which merges two [`HashMap`]'s with values that implement [`Merge`].
impl<K, V> Merge for HashMap<K, V>
where
    K: Eq + Hash,
    V: Merge,
{
    fn merge(&mut self, other: Self) {
        for (key, value) in other {
            match self.entry(key) {
                Entry::Occupied(mut entry) => {
                    entry.get_mut().merge(value);
                }
                Entry::Vacant(entry) => {
                    entry.insert(value);
                }
            }
        }
    }
}

/// Merge two values that implement [`Merge`] and return the result.
pub fn merge<T: Merge>(mut this: T, that: T) -> T {
    this.merge(that);
    this
}
