use crate::{deser::DeSer, link::Link, utils::is_pow2};
use serde::{Deserialize, Serialize};
use st_file::{traits::IndexedAccess, MemFile};
use std::{cmp::Ordering, marker::PhantomData};

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

    #[inline]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn len_bytes(&self) -> usize {
        self.items.raw_len() + self.items.len() * 4 + self.entries.len() * 4
    }

    /// Gets a link at the given position
    #[inline]
    fn get_item(&self, pos: u32) -> Option<Link<T, V>> {
        let enc = self.items.get(pos as usize)?;
        Link::<T, V>::decode(enc)
    }
}

impl<T, V> SkipMap<T, V>
where
    T: DeSer + Ord,
    V: DeSer,
{
    /// Finds the given order func in the skip map and returns its value and position
    #[inline]
    pub fn find(&self, key: &T) -> Option<(u32, V)> {
        self.find_by(|other| other.cmp(key))
    }

    /// Finds the given order func in the skip map and returns its value and position.
    /// The comparator function should return an order code that indicates whether
    /// its argument is Less, greater or equal to the value its looking for
    pub fn find_by<C>(&self, cmp: C) -> Option<(u32, V)>
    where
        C: Fn(&T) -> Ordering,
    {
        let mut prev_ep: Option<u32> = None;

        for entry_point in self.entries.iter().copied() {
            let item = self.get_item(entry_point)?;

            match (cmp)(&item.item) {
                Ordering::Equal => return Some((entry_point, item.value)),
                Ordering::Greater => break,
                Ordering::Less => (),
            }

            prev_ep = Some(entry_point);
        }

        let mut p = prev_ep?;
        loop {
            let p_item = self.get_item(p)?;
            match (cmp)(&p_item.item) {
                Ordering::Less => (),
                Ordering::Greater => return None,
                Ordering::Equal => return Some((p, p_item.value)),
            }

            if !p_item.has_next() {
                break;
            }

            p = p_item.next;
        }

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
