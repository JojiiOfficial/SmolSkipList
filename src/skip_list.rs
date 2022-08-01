use crate::{deser::DeSer, iter::SkListIter, skip_map::SkipMap};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

/// A memory efficient balanced Skip-list implementation
#[derive(Deserialize, Serialize)]
pub struct SkipList<T> {
    pub(crate) map: SkipMap<T, ()>,
}

impl<T> SkipList<T> {
    /// Create a new empty SkipList
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.map.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

impl<T> SkipList<T>
where
    T: DeSer,
{
    /// Returns an iterator over all items in the skip list ordered
    #[inline]
    pub fn iter(&self) -> SkListIter<T> {
        SkListIter::new(self)
    }

    /// Gets an item by its ID
    #[inline]
    pub fn get(&self, pos: usize) -> Option<T> {
        self.map.get(pos).map(|i| i.0)
    }
}

impl<T> SkipList<T>
where
    T: DeSer + Ord,
{
    /// Creates a new SkipList from a list of sorted items. If the items aren't
    /// ordered searching won't work.
    #[inline]
    pub fn from_sorted_iter<I>(list: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let map = SkipMap::from_sorted_iter(list.into_iter().map(|i| (i, ())));
        Self { map }
    }

    /// Finds an item within the skip-list using a key
    #[inline]
    pub fn find(&self, key: &T) -> Option<usize> {
        self.map.find_by(|other| other.cmp(key)).map(|i| i.0)
    }

    /// Finds an item in the skip map and returns its value and position.
    /// The comparator function should return an order code that indicates whether
    /// its argument is Less, greater or equal to the value its looking for
    #[inline]
    pub fn find_by<C>(&self, cmp: C) -> Option<usize>
    where
        C: Fn(&T) -> Ordering,
    {
        self.map.find_by(cmp).map(|i| i.0)
    }
}

impl<T> Default for SkipList<T> {
    #[inline]
    fn default() -> Self {
        Self {
            map: Default::default(),
        }
    }
}
