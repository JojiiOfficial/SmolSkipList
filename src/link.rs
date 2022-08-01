use crate::deser::DeSer;
use serde::{Deserialize, Serialize};

/// A linked list item
#[derive(Serialize, Deserialize)]
pub(crate) struct ListItem<T, V> {
    pub(crate) item: T,
    pub(crate) next: u32,
    pub(crate) value: V,
}

impl<T, V> ListItem<T, V>
where
    T: DeSer,
    V: DeSer,
{
    /// Create a new ListItem with key and value
    #[inline]
    pub fn new(item: T, value: V) -> Self {
        Self {
            item,
            next: 0,
            value,
        }
    }

    /// Creates a new ListItem with a value and next-pointer
    #[inline]
    pub fn with_next(item: T, value: V, next: u32) -> Self {
        Self { item, next, value }
    }

    /// Returns `true` if the itemt has a next item
    #[inline]
    pub fn has_next(&self) -> bool {
        self.next != 0
    }
}
