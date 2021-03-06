// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::Bound::{Included, Excluded};
use std::collections::btree_map::{BTreeMap, self};

/// An extension trait for a `Map` whose keys have a defined total ordering.
/// This trait provides convenience methods which take advantage of the map's ordering.
pub trait SortedMapExt<K, V>
    where K: Clone + Ord,
          V: Clone 
{
    /// An iterator over immutable references to the key-value pairs in this map whose keys fall
    /// within a given range.
    type RangeIter;

    /// An iterator over mutable references to the key-value pairs in this map whose keys fall
    /// within a given range.
    type RangeIterMut;

    /// A by-value iterator yielding key-value pairs whose keys fall within a given range and
    /// which have just been removed from this map.
    type RangeRemoveIter;

    /// Returns an immutable reference to the first (least) key currently in this map.
    /// Returns `None` if this map is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate "sorted-collections" as sorted_collections;
    ///
    /// use std::collections::BTreeMap;
    /// use sorted_collections::SortedMapExt;
    ///
    /// fn main() {
    ///     let map: BTreeMap<u32, u32> =
    ///         vec![(1u32, 1u32), (2, 2), (3, 3), (4, 4), (5, 5)].into_iter().collect();
    ///     assert_eq!(map.first().unwrap(), &1u32);
    /// }
    /// ```
    fn first(&self) -> Option<&K>;

    /// Removes and returns the first (least) key currently in this map and its associated value.
    /// Returns `None` if this map is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate "sorted-collections" as sorted_collections;
    ///
    /// use std::collections::BTreeMap;
    /// use sorted_collections::SortedMapExt;
    ///
    /// fn main() {
    ///     let mut map: BTreeMap<u32, u32> =
    ///         vec![(1u32, 1u32), (2, 2), (3, 3), (4, 4), (5, 5)].into_iter().collect();
    ///     assert_eq!(map.first_remove().unwrap(), (1u32, 1u32));
    ///     assert_eq!(map.into_iter().collect::<Vec<(u32, u32)>>(),
    ///         vec![(2u32, 2u32), (3, 3), (4, 4), (5, 5)]);
    /// }
    /// ```
    fn first_remove(&mut self) -> Option<(K, V)>;

    /// Returns an immutable reference to the last (greatest) key currently in this map.
    /// Returns `None` if this map is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate "sorted-collections" as sorted_collections;
    ///
    /// use std::collections::BTreeMap;
    /// use sorted_collections::SortedMapExt;
    ///
    /// fn main() {
    ///     let map: BTreeMap<u32, u32> =
    ///         vec![(1u32, 1u32), (2, 2), (3, 3), (4, 4), (5, 5)].into_iter().collect();
    ///     assert_eq!(map.last().unwrap(), &5u32);
    /// }
    /// ```
    fn last(&self) -> Option<&K>;

    /// Removes and returns the last (greatest) key currently in this map and its associated value.
    /// Returns `None` if this map is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate "sorted-collections" as sorted_collections;
    ///
    /// use std::collections::BTreeMap;
    /// use sorted_collections::SortedMapExt;
    ///
    /// fn main() {
    ///     let mut map: BTreeMap<u32, u32> =
    ///         vec![(1u32, 1u32), (2, 2), (3, 3), (4, 4), (5, 5)].into_iter().collect();
    ///     assert_eq!(map.last_remove().unwrap(), (5u32, 5u32));
    ///     assert_eq!(map.into_iter().collect::<Vec<(u32, u32)>>(),
    ///         vec![(1u32, 1u32), (2, 2), (3, 3), (4, 4)]);
    /// }
    /// ```
    fn last_remove(&mut self) -> Option<(K, V)>;

    /// Returns an immutable reference to the least key in this map greater than or equal to `key`.
    /// Returns `None` if there is no such key.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate "sorted-collections" as sorted_collections;
    ///
    /// use std::collections::BTreeMap;
    /// use sorted_collections::SortedMapExt;
    ///
    /// fn main() {
    ///     let map: BTreeMap<u32, u32> =
    ///         vec![(1u32, 1u32), (2, 2), (3, 3), (4, 4), (5, 5)].into_iter().collect();
    ///     assert_eq!(map.ceiling(&3).unwrap(), &3u32);
    /// }
    /// ```
    fn ceiling(&self, key: &K) -> Option<&K>;

    /// Removes and returns the least key in this map greater than or equal to `key` and its
    /// associated value.
    /// Returns `None` if there is no such element.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate "sorted-collections" as sorted_collections;
    ///
    /// use std::collections::BTreeMap;
    /// use sorted_collections::SortedMapExt;
    ///
    /// fn main() {
    ///     let mut map: BTreeMap<u32, u32> =
    ///         vec![(1u32, 1u32), (2, 2), (3, 3), (4, 4), (5, 5)].into_iter().collect();
    ///     assert_eq!(map.ceiling_remove(&3).unwrap(), (3u32, 3u32));
    ///     assert_eq!(map.into_iter().collect::<Vec<(u32, u32)>>(),
    ///         vec![(1u32, 1u32), (2, 2), (4, 4), (5, 5)]);
    /// }
    /// ```
    fn ceiling_remove(&mut self, key: &K) -> Option<(K, V)>;

    /// Returns an immutable reference to the greatest key in this map less than or equal to `key`.
    /// Returns `None` if there is no such key.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate "sorted-collections" as sorted_collections;
    ///
    /// use std::collections::BTreeMap;
    /// use sorted_collections::SortedMapExt;
    ///
    /// fn main() {
    ///     let map: BTreeMap<u32, u32> =
    ///         vec![(1u32, 1u32), (2, 2), (3, 3), (4, 4), (5, 5)].into_iter().collect();
    ///     assert_eq!(map.floor(&3).unwrap(), &3u32);
    /// }
    /// ```
    fn floor(&self, key: &K) -> Option<&K>;

    /// Removes and returns the greatest key in this map less than or equal to `key` and its
    /// associated value.
    /// Returns `None` if there is no such element.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate "sorted-collections" as sorted_collections;
    ///
    /// use std::collections::BTreeMap;
    /// use sorted_collections::SortedMapExt;
    ///
    /// fn main() {
    ///     let mut map: BTreeMap<u32, u32> =
    ///         vec![(1u32, 1u32), (2, 2), (3, 3), (4, 4), (5, 5)].into_iter().collect();
    ///     assert_eq!(map.floor_remove(&3).unwrap(), (3u32, 3u32));
    ///     assert_eq!(map.into_iter().collect::<Vec<(u32, u32)>>(),
    ///         vec![(1u32, 1u32), (2, 2), (4, 4), (5, 5)]);
    /// }
    /// ```
    fn floor_remove(&mut self, key: &K) -> Option<(K, V)>;

    /// Returns an immutable reference to the least key in this map strictly greater than `key`.
    /// Returns `None` if there is no such key.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate "sorted-collections" as sorted_collections;
    ///
    /// use std::collections::BTreeMap;
    /// use sorted_collections::SortedMapExt;
    ///
    /// fn main() {
    ///     let map: BTreeMap<u32, u32> =
    ///         vec![(1u32, 1u32), (2, 2), (3, 3), (4, 4), (5, 5)].into_iter().collect();
    ///     assert_eq!(map.higher(&3).unwrap(), &4u32);
    /// }
    /// ```
    fn higher(&self, key: &K) -> Option<&K>;

    /// Removes and returns the least key in this map strictly greater than `key` and its
    /// associated value.
    /// Returns `None` if there is no such element.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate "sorted-collections" as sorted_collections;
    ///
    /// use std::collections::BTreeMap;
    /// use sorted_collections::SortedMapExt;
    ///
    /// fn main() {
    ///     let mut map: BTreeMap<u32, u32> =
    ///         vec![(1u32, 1u32), (2, 2), (3, 3), (4, 4), (5, 5)].into_iter().collect();
    ///     assert_eq!(map.higher_remove(&3).unwrap(), (4u32, 4u32));
    ///     assert_eq!(map.into_iter().collect::<Vec<(u32, u32)>>(),
    ///         vec![(1u32, 1u32), (2, 2), (3, 3), (5, 5)]);
    /// }
    /// ```
    fn higher_remove(&mut self, key: &K) -> Option<(K, V)>;


    /// Returns an immutable reference to the greatest key in this map strictly less than `key`.
    /// Returns `None` if there is no such key.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate "sorted-collections" as sorted_collections;
    ///
    /// use std::collections::BTreeMap;
    /// use sorted_collections::SortedMapExt;
    ///
    /// fn main() {
    ///     let map: BTreeMap<u32, u32> =
    ///         vec![(1u32, 1u32), (2, 2), (3, 3), (4, 4), (5, 5)].into_iter().collect();
    ///     assert_eq!(map.lower(&3).unwrap(), &2u32);
    /// }
    /// ```
    fn lower(&self, key: &K) -> Option<&K>;

    /// Removes and returns the greatest key in this map strictly less than `key` and its
    /// associated value.
    /// Returns `None` if there is no such element.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate "sorted-collections" as sorted_collections;
    ///
    /// use std::collections::BTreeMap;
    /// use sorted_collections::SortedMapExt;
    ///
    /// fn main() {
    ///     let mut map: BTreeMap<u32, u32> =
    ///         vec![(1u32, 1u32), (2, 2), (3, 3), (4, 4), (5, 5)].into_iter().collect();
    ///     assert_eq!(map.lower_remove(&3).unwrap(), (2u32, 2u32));
    ///     assert_eq!(map.into_iter().collect::<Vec<(u32, u32)>>(),
    ///         vec![(1u32, 1u32), (3, 3), (4, 4), (5, 5)]);
    /// }
    /// ```
    fn lower_remove(&mut self, key: &K) -> Option<(K, V)>;

    /// Returns an iterator over pairs of immutable key-value references into this map,
    /// with the pairs being iterated being those whose keys are in the range [from_key, to_key).
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate "sorted-collections" as sorted_collections;
    ///
    /// use std::collections::BTreeMap;
    /// use sorted_collections::SortedMapExt;
    ///
    /// fn main() {
    ///     let map: BTreeMap<u32, u32> =
    ///         vec![(1u32, 1u32), (2, 2), (3, 3), (4, 4), (5, 5)].into_iter().collect();
    ///     assert_eq!(map.range_iter(&2, &4).map(|(&k, &v)| (k, v)).collect::<Vec<(u32, u32)>>(),
    ///         vec![(2u32, 2u32), (3, 3)]);
    /// }
    /// ```
    fn range_iter(&self, from_key: &K, to_key: &K) -> Self::RangeIter;

    /// Returns an iterator over pairs of immutable-key/mutable-value references into this map,
    /// with the pairs being iterated being those whose keys are in the range [from_key, to_key).
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate "sorted-collections" as sorted_collections;
    ///
    /// use std::collections::BTreeMap;
    /// use sorted_collections::SortedMapExt;
    ///
    /// fn main() {
    ///     let mut map: BTreeMap<u32, u32> =
    ///         vec![(1u32, 1u32), (2, 2), (3, 3), (4, 4), (5, 5)].into_iter().collect();
    ///     for (_, v) in map.range_iter_mut(&2, &4) {
    ///         *v += 1;
    ///     }
    ///     assert_eq!(map.into_iter().collect::<Vec<(u32, u32)>>(),
    ///         vec![(1u32, 1u32), (2, 3), (3, 4), (4, 4), (5, 5)]);
    /// }
    /// ```
    fn range_iter_mut(&mut self, from_key: &K, to_key: &K) -> Self::RangeIterMut;

    /// Removes the key-value pairs of this map whose keys lie in the range [from_key, to_key),
    /// and returns a by-value iterator over the removed pairs.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate "sorted-collections" as sorted_collections;
    ///
    /// use std::collections::BTreeMap;
    /// use sorted_collections::SortedMapExt;
    ///
    /// fn main() {
    ///     let mut map: BTreeMap<u32, u32> =
    ///         vec![(1u32, 1u32), (2, 2), (3, 3), (4, 4), (5, 5)].into_iter().collect();
    ///     assert_eq!(map.range_remove_iter(&2, &4).collect::<Vec<(u32, u32)>>(),
    ///         vec![(2u32, 2u32), (3, 3)]);
    ///     assert_eq!(map.into_iter().collect::<Vec<(u32, u32)>>(),
    ///         vec![(1u32, 1u32), (4, 4), (5, 5)]);
    /// }
    /// ```
    fn range_remove_iter(&mut self, from_key: &K, to_key: &K) -> Self::RangeRemoveIter;
}

// A generic reusable impl of SortedMapExt.
macro_rules! sortedmap_impl {
    ($typ:ty) => (
        fn first(&self) -> Option<&K> {
            self.keys().min()
        }

        fn first_remove(&mut self) -> Option<(K, V)> {
            if let Some(key) = self.first().cloned() {
                let val = self.remove(&key);
                assert!(val.is_some());
                Some((key, val.unwrap()))
            } else {
                None
            }
        }

        fn last(&self) -> Option<&K> {
            self.keys().max()
        }

        fn last_remove(&mut self) -> Option<(K, V)> {
            if let Some(key) = self.last().cloned() {
                let val = self.remove(&key);
                assert!(val.is_some());
                Some((key, val.unwrap()))
            } else {
                None
            }
        }

        fn ceiling(&self, key: &K) -> Option<&K> {
            self.keys().filter(|&k| k >= key).min()
        }

        fn ceiling_remove(&mut self, key: &K) -> Option<(K, V)> {
            if let Some(ceiling) = self.ceiling(key).cloned() {
                let val = self.remove(&ceiling);
                assert!(val.is_some());
                Some((ceiling, val.unwrap()))
            } else {
                None
            }
        }

        fn floor(&self, key: &K) -> Option<&K> {
            self.keys().filter(|&k| k <= key).max()
        }

        fn floor_remove(&mut self, key: &K) -> Option<(K, V)> {
            if let Some(floor) = self.floor(key).cloned() {
                let val = self.remove(&floor);
                assert!(val.is_some());
                Some((floor, val.unwrap()))
            } else {
                None
            }
        }

        fn higher(&self, key: &K) -> Option<&K> {
            self.keys().filter(|&k| k > key).min()
        }

        fn higher_remove(&mut self, key: &K) -> Option<(K, V)> {
            if let Some(higher) = self.higher(key).cloned() {
                let val = self.remove(&higher);
                assert!(val.is_some());
                Some((higher, val.unwrap()))
            } else {
                None
            }
        }

        fn lower(&self, key: &K) -> Option<&K> {
            self.keys().filter(|&k| k < key).max()
        }

        fn lower_remove(&mut self, key: &K) -> Option<(K, V)> {
            if let Some(lower) = self.lower(key).cloned() {
                let val = self.remove(&lower);
                assert!(val.is_some());
                Some((lower, val.unwrap()))
            } else {
                None
            }
        }
    );
}

// An impl of SortedMapExt for the standard library BTreeMap
impl<'a, K, V> SortedMapExt<K, V> for BTreeMap<K, V>
    where K: Clone + Ord,
          V: Clone
{
    type RangeIter = BTreeMapRangeIter<'a, K, V>;
    type RangeIterMut = BTreeMapRangeIterMut<'a, K, V>;
    type RangeRemoveIter = BTreeMapRangeRemoveIter<K, V>;

    sortedmap_impl!(BTreeMap<K, V>);

    fn range_iter(&self, from_key: &K, to_key: &K) -> BTreeMapRangeIter<K, V> {
        BTreeMapRangeIter { iter: self.range(Included(from_key), Excluded(to_key)) }
    }

    fn range_iter_mut(&mut self, from_key: &K, to_key: &K) -> BTreeMapRangeIterMut<K, V> {
        BTreeMapRangeIterMut { iter: self.range_mut(Included(from_key), Excluded(to_key)) }
    }

    fn range_remove_iter(&mut self, from_key: &K, to_key: &K) -> BTreeMapRangeRemoveIter<K, V> {
        let ret: BTreeMap<K, V> = 
                self.range_iter(from_key, to_key)
                .map(|(ref k, ref v)| ((**k).clone(), (**v).clone()))
                .collect();

        for key in ret.keys() {
            assert!(self.remove(key).is_some());
        }
        BTreeMapRangeRemoveIter { iter: ret.into_iter() }
    }
}

pub struct BTreeMapRangeIter<'a, K: 'a, V: 'a> {
    iter: btree_map::Range<'a, K, V>
}

impl<'a, K, V> Iterator for BTreeMapRangeIter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<(&'a K, &'a V)> { self.iter.next() }
    fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }
}
impl<'a, K, V> DoubleEndedIterator for BTreeMapRangeIter<'a, K, V> {
    fn next_back(&mut self) -> Option<(&'a K, &'a V)> { self.iter.next_back() }
}

pub struct BTreeMapRangeIterMut<'a, K: 'a, V: 'a> {
    iter: btree_map::RangeMut<'a, K, V>
}

impl<'a, K, V> Iterator for BTreeMapRangeIterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    fn next(&mut self) -> Option<(&'a K, &'a mut V)> { self.iter.next() }
    fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }
}
impl<'a, K, V> DoubleEndedIterator for BTreeMapRangeIterMut<'a, K, V> {
    fn next_back(&mut self) -> Option<(&'a K, &'a mut V)> { self.iter.next_back() }
}

pub struct BTreeMapRangeRemoveIter<K, V> {
    iter: btree_map::IntoIter<K, V>
}

impl<K, V> Iterator for BTreeMapRangeRemoveIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<(K, V)> { self.iter.next() }
    fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }
}
impl<K, V> DoubleEndedIterator for BTreeMapRangeRemoveIter<K, V> {
    fn next_back(&mut self) -> Option<(K, V)> { self.iter.next_back() }
}
impl<K, V> ExactSizeIterator for BTreeMapRangeRemoveIter<K, V> {
    fn len(&self) -> usize { self.iter.len() }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::SortedMapExt;

    #[test]
    fn test_first() {
        let map: BTreeMap<u32, u32> = vec![(1u32, 1u32), (2, 2), (3, 3), (4, 4), (5, 5)].into_iter().collect();
        assert_eq!(map.first().unwrap(), &1u32);
    }

    #[test]
    fn test_first_remove() {
        let mut map: BTreeMap<u32, u32> = vec![(1u32, 1u32), (2, 2), (3, 3), (4, 4), (5, 5)].into_iter().collect();
        assert_eq!(map.first_remove().unwrap(), (1u32, 1u32));
        assert_eq!(map.into_iter().collect::<Vec<(u32, u32)>>(), vec![(2u32, 2u32), (3, 3), (4, 4), (5, 5)]);
    }

    #[test]
    fn test_last() {
        let map: BTreeMap<u32, u32> = vec![(1u32, 1u32), (2, 2), (3, 3), (4, 4), (5, 5)].into_iter().collect();
        assert_eq!(map.last().unwrap(), &5u32);
    }

    #[test]
    fn test_last_remove() {
        let mut map: BTreeMap<u32, u32> = vec![(1u32, 1u32), (2, 2), (3, 3), (4, 4), (5, 5)].into_iter().collect();
        assert_eq!(map.last_remove().unwrap(), (5u32, 5u32));
        assert_eq!(map.into_iter().collect::<Vec<(u32, u32)>>(), vec![(1u32, 1u32), (2, 2), (3, 3), (4, 4)]);
    }

    #[test]
    fn test_ceiling() {
        let map: BTreeMap<u32, u32> = vec![(1u32, 1u32), (2, 2), (3, 3), (4, 4), (5, 5)].into_iter().collect();
        assert_eq!(map.ceiling(&3).unwrap(), &3u32);
        assert_eq!(map.into_iter().collect::<Vec<(u32, u32)>>(), vec![(1u32, 1u32), (2, 2), (3, 3), (4, 4), (5, 5)]);
    }

    #[test]
    fn test_ceiling_remove() {
        let mut map: BTreeMap<u32, u32> = vec![(1u32, 1u32), (2, 2), (3, 3), (4, 4), (5, 5)].into_iter().collect();
        assert_eq!(map.ceiling_remove(&3).unwrap(), (3u32, 3u32));
        assert_eq!(map.into_iter().collect::<Vec<(u32, u32)>>(), vec![(1u32, 1u32), (2, 2), (4, 4), (5, 5)]);
    }

    #[test]
    fn test_floor() {
        let map: BTreeMap<u32, u32> = vec![(1u32, 1u32), (2, 2), (3, 3), (4, 4), (5, 5)].into_iter().collect();
        assert_eq!(map.floor(&3).unwrap(), &3u32);
        assert_eq!(map.into_iter().collect::<Vec<(u32, u32)>>(), vec![(1u32, 1u32), (2, 2), (3, 3), (4, 4), (5, 5)]);
    }

    #[test]
    fn test_floor_remove() {
        let mut map: BTreeMap<u32, u32> = vec![(1u32, 1u32), (2, 2), (3, 3), (4, 4), (5, 5)].into_iter().collect();
        assert_eq!(map.floor_remove(&3).unwrap(), (3u32, 3u32));
        assert_eq!(map.into_iter().collect::<Vec<(u32, u32)>>(), vec![(1u32, 1u32), (2, 2), (4, 4), (5, 5)]);
    }

    #[test]
    fn test_higher() {
        let map: BTreeMap<u32, u32> = vec![(1u32, 1u32), (2, 2), (3, 3), (4, 4), (5, 5)].into_iter().collect();
        assert_eq!(map.higher(&3).unwrap(), &4u32);
        assert_eq!(map.into_iter().collect::<Vec<(u32, u32)>>(), vec![(1u32, 1u32), (2, 2), (3, 3), (4, 4), (5, 5)]);
    }

    #[test]
    fn test_higher_remove() {
        let mut map: BTreeMap<u32, u32> = vec![(1u32, 1u32), (2, 2), (3, 3), (4, 4), (5, 5)].into_iter().collect();
        assert_eq!(map.higher_remove(&3).unwrap(), (4u32, 4u32));
        assert_eq!(map.into_iter().collect::<Vec<(u32, u32)>>(), vec![(1u32, 1u32), (2, 2), (3, 3), (5, 5)]);
    }

    #[test]
    fn test_lower() {
        let map: BTreeMap<u32, u32> = vec![(1u32, 1u32), (2, 2), (3, 3), (4, 4), (5, 5)].into_iter().collect();
        assert_eq!(map.lower(&3).unwrap(), &2u32);
        assert_eq!(map.into_iter().collect::<Vec<(u32, u32)>>(), vec![(1u32, 1u32), (2, 2), (3, 3), (4, 4), (5, 5)]);
    }

    #[test]
    fn test_lower_remove() {
        let mut map: BTreeMap<u32, u32> = vec![(1u32, 1u32), (2, 2), (3, 3), (4, 4), (5, 5)].into_iter().collect();
        assert_eq!(map.lower_remove(&3).unwrap(), (2u32, 2u32));
        assert_eq!(map.into_iter().collect::<Vec<(u32, u32)>>(), vec![(1u32, 1u32), (3, 3), (4, 4), (5, 5)]);
    }

    #[test]
    fn test_range_iter() {
        let map: BTreeMap<u32, u32> = vec![(1u32, 1u32), (2, 2), (3, 3), (4, 4), (5, 5)].into_iter().collect();
        assert_eq!(map.range_iter(&2, &4).map(|(&k, &v)| (k, v)).collect::<Vec<(u32, u32)>>(),
            vec![(2u32, 2u32), (3, 3)]);
    }

    #[test]
    fn test_range_iter_mut() {
        let mut map: BTreeMap<u32, u32> = vec![(1u32, 1u32), (2, 2), (3, 3), (4, 4), (5, 5)].into_iter().collect();
        for (_, v) in map.range_iter_mut(&2, &4) {
            *v += 1;
        }
        assert_eq!(map.into_iter().collect::<Vec<(u32, u32)>>(),
            vec![(1u32, 1u32), (2, 3), (3, 4), (4, 4), (5, 5)]);
    }

    #[test]
    fn test_range_remove_iter() {
        let mut map: BTreeMap<u32, u32> = vec![(1u32, 1u32), (2, 2), (3, 3), (4, 4), (5, 5)].into_iter().collect();
        assert_eq!(map.range_remove_iter(&2, &4).collect::<Vec<(u32, u32)>>(), vec![(2u32, 2u32), (3, 3)]);
        assert_eq!(map.into_iter().collect::<Vec<(u32, u32)>>(),
            vec![(1u32, 1u32), (4, 4), (5, 5)]);
    }
}
