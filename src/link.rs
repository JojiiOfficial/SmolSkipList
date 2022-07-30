use crate::deser::DeSer;
use serde::{Deserialize, Serialize};

/// A linked list item
#[derive(Serialize, Deserialize)]
pub struct Link<T, V> {
    pub(crate) item: T,
    pub(crate) next: u32,
    pub(crate) value: V,
}

impl<T, V> Link<T, V>
where
    T: DeSer,
    V: DeSer,
{
    #[inline]
    pub fn new(item: T, value: V) -> Self {
        Self {
            item,
            next: 0,
            value,
        }
    }

    #[inline]
    pub fn with_next(item: T, value: V, next: u32) -> Self {
        Self { item, next, value }
    }

    #[inline]
    pub fn next(&self) -> u32 {
        self.next
    }

    #[inline]
    pub fn has_next(&self) -> bool {
        self.next != 0
    }
}
