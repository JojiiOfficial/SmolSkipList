use crate::{deser::DeSer, skip_map::SkipMap};
use serde::{Deserialize, Serialize};

/// A memory efficient balanced Skip-list implementation
#[derive(Deserialize, Serialize)]
pub struct SkipList<T> {
    map: SkipMap<T, ()>,
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
}

impl<T> Default for SkipList<T> {
    #[inline]
    fn default() -> Self {
        Self {
            map: Default::default(),
        }
    }
}
