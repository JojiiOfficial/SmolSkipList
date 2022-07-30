use crate::{deser::DeSer, iter::SkMapIter, link::ListItem, utils::is_pow2};
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

impl<T, V> SkipMap<T, V> {
    /// Returns the amount of items in the skip-list
    #[inline]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns `true` if there is no item in the skip-list
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
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
        let mut link_items: Vec<ListItem<T, V>> = vec![];

        for (pos, (k, v)) in list.into_iter().enumerate() {
            let pos = pos as u32;
            let item = ListItem::with_next(k, v, pos + 1);

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

    /// Gets the key and value of the element at the given position
    #[inline]
    pub fn get(&self, pos: usize) -> Option<(T, V)> {
        let enc = self.items.get(pos)?;
        let item = ListItem::<T, V>::decode(enc)?;
        Some((item.item, item.value))
    }

    /// Returns an iterator over all items in the skip list
    #[inline]
    pub fn iter(&self) -> SkMapIter<T, V> {
        SkMapIter::new(self)
    }

    /// Gets a list item at the given position
    #[inline]
    fn get_list_item(&self, pos: u32) -> Option<ListItem<T, V>> {
        let enc = self.items.get(pos as usize)?;
        ListItem::<T, V>::decode(enc)
    }
}

impl<T, V> SkipMap<T, V>
where
    T: DeSer + Ord,
    V: DeSer,
{
    /// Finds an item within the skip-list using a key
    #[inline]
    pub fn find(&self, key: &T) -> Option<(u32, V)> {
        self.find_by(|other| other.cmp(key))
    }

    /// Finds an item in the skip map and returns its value and position.
    /// The comparator function should return an order code that indicates whether
    /// its argument is Less, greater or equal to the value its looking for
    pub fn find_by<C>(&self, f: C) -> Option<(u32, V)>
    where
        C: Fn(&T) -> Ordering,
    {
        let mut prev_ep: Option<u32> = None;

        // Find entrypoint which is bigger then the element
        for entry_point in self.entries.iter().copied() {
            let item = self.get_list_item(entry_point)?;

            let cmp = (f)(&item.item);
            if cmp == Ordering::Greater {
                break;
            } else if cmp == Ordering::Equal {
                return Some((entry_point, item.value));
            }

            prev_ep = Some(entry_point);
        }

        // Walk pointer for list item
        let mut p = prev_ep?;

        // Search on the given level for the element
        loop {
            let p_item = self.get_list_item(p)?;
            let cmp = (f)(&p_item.item);

            if cmp == Ordering::Equal {
                return Some((p, p_item.value));
            } else if cmp == Ordering::Greater || !p_item.has_next() {
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

#[cfg(test)]
mod test {
    use super::super::SkipList;

    #[test]
    fn test_find() {
        let inp = ['A', 'B', 'C', 'D', 'E', 'F'];
        let skip_map = SkipList::from_sorted_iter(inp);

        for (pos, i) in inp.iter().enumerate() {
            let found = skip_map.find(&i);
            assert_eq!(found, Some(pos as u32));
        }
    }

    #[test]
    fn test_find2() {
        let inp: Vec<_> = (0..5000).collect();
        let skip_map = SkipList::from_sorted_iter(inp.clone());

        for (pos, i) in inp.iter().enumerate() {
            let found = skip_map.find(&i);
            assert_eq!(found, Some(pos as u32));
        }
    }
}
