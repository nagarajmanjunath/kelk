use crate::storage::codec::Codec;
use crate::storage::Offset;
use crate::Codec;

#[derive(Debug, Clone, Codec)]
pub(super) struct Node<K: Codec + Clone + Ord, V: Clone + Codec> {
    pub left: Offset,
    pub right: Offset,
    pub key: K,
    pub value: V,
}

impl<K: Codec + Clone + Ord, V: Clone + Codec> Node<K, V> {
    pub fn new(key: K, value: V) -> Self {
        Self {
            key,
            value,
            left: 0,
            right: 0,
        }
    }
}
