use crate::{deser::DeSer, link::Link, utils::is_pow2};
use serde::{Deserialize, Serialize};
use st_file::{traits::IndexedAccess, MemFile};
use std::marker::PhantomData;

/// A memory efficient balanced Skip-list implementation with values
#[derive(Deserialize, Serialize)]
pub struct SkipMap<T, V> {
    items: MemFile,
    entries: Vec<u32>,
    p: PhantomData<T>,
    p2: PhantomData<V>,
}

impl<T, V> SkipMap<T, V>
where
    T: DeSer,
    V: DeSer,
{
    /// Create a new empty SkipList
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new SkipList from a list of sorted items. If the items aren't
    /// ordered searching won't work.
    pub fn from_sorted_iter<I>(list: I) -> Self
    where
        I: IntoIterator<Item = (T, V)>,
    {
        // Entry points in search
        let mut entry_points = vec![];

        // All items
        let mut link_items: Vec<Link<T, V>> = vec![];

        for (pos, (k, v)) in list.into_iter().enumerate() {
            let pos = pos as u32;
            let item = Link::with_next(k, v, pos + 1);

            if is_pow2(pos + 1) {
                entry_points.push(pos);

                // Remove 'next' pointer of last element since this
                // item is a new list
                if let Some(last) = link_items.last_mut() {
                    last.next = 0;
                }
            }

            link_items.push(item);
        }

        link_items.last_mut().unwrap().next = 0;

        let mut items = MemFile::new();
        for item in link_items {
            items.insert(&item.encode_vec());
        }

        Self {
            items,
            entries: entry_points,
            p: PhantomData,
            p2: PhantomData,
        }
    }
}

impl<T, V> SkipMap<T, V>
where
    T: DeSer + Ord,
    V: DeSer,
{
    pub fn find(&self, other: &T) -> Option<V> {
        None
    }
}

impl<T, V> Default for SkipMap<T, V> {
    #[inline]
    fn default() -> Self {
        Self {
            items: Default::default(),
            entries: Default::default(),
            p: Default::default(),
            p2: Default::default(),
        }
    }
}