use crate::{deser::DeSer, SkipList, SkipMap};

/// Iterator over a skiplist
pub struct SkListIter<'a, T> {
    list: &'a SkipList<T>,
    pos: usize,
}

impl<'a, T> SkListIter<'a, T>
where
    T: DeSer,
{
    #[inline]
    pub(crate) fn new(data: &'a SkipList<T>) -> Self {
        Self { list: data, pos: 0 }
    }
}

impl<'a, T> Iterator for SkListIter<'a, T>
where
    T: DeSer,
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.list.len() {
            return None;
        }

        let item = self.list.get(self.pos)?;
        self.pos += 1;
        Some(item)
    }
}

/// Iterator over a skiplist/map
pub struct SkMapIter<'a, T, V> {
    map: &'a SkipMap<T, V>,
    pos: usize,
}

impl<'a, T, V> SkMapIter<'a, T, V>
where
    T: DeSer,
{
    #[inline]
    pub(crate) fn new(data: &'a SkipMap<T, V>) -> Self {
        Self { map: data, pos: 0 }
    }
}

impl<'a, T, V> Iterator for SkMapIter<'a, T, V>
where
    T: DeSer,
    V: DeSer,
{
    type Item = (T, V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.map.len() {
            return None;
        }

        let item = self.map.get(self.pos)?;
        self.pos += 1;
        Some(item)
    }
}
