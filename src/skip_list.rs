use std::cmp::Ordering;

use crate::{deser::DeSer, skip_map::SkipMap};
use serde::{Deserialize, Serialize};

/// A memory efficient balanced Skip-list implementation
#[derive(Deserialize, Serialize)]
pub struct SkipList<T> {
    pub(crate) map: SkipMap<T, ()>,
}

impl<T> SkipList<T>
where
    T: DeSer,
{
    /// Create a new empty SkipList
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

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

    #[inline]
    pub fn len(&self) -> usize {
        self.map.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    #[inline]
    pub fn len_bytes(&self) -> usize {
        self.map.len_bytes()
    }
}

impl<T> SkipList<T>
where
    T: DeSer + Ord,
{
    /// Finds the given order func in the skip map and returns its value and position
    #[inline]
    pub fn find(&self, key: &T) -> Option<u32> {
        self.map.find_by(|other| other.cmp(key)).map(|i| i.0)
    }

    /// Finds the given order func in the skip map and returns its value and position.
    /// The comparator function should return an order code that indicates whether
    /// its argument is Less, greater or equal to the value its looking for
    #[inline]
    pub fn find_by<C>(&self, cmp: C) -> Option<u32>
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
