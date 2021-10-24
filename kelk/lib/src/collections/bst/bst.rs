//! Storage Binary Search Tree, is a binary search tree or BST that instead of using Random Access Memory,
//! Read and writes from contract's storage. Therefore it's permanently store inside contract's storage.

use super::error::InvalidOffset;
use crate::storage::Storage;
use core::marker::PhantomData;
use core::mem::size_of;

/// The instance of Storage Binary Search Tree
pub struct StorageBST<'a, S, K, V>
where
    S: Storage,
    K: Eq,
    V: Sized,
{
    storage: &'a S,
    offset: u32,
    _phantom: PhantomData<(K, V)>,
}

#[repr(C)]
struct Header {
    boom: u32,
    el_size: u16,
    reserved_1: u16,
    count: i32,
    reserved_2: u64,
}

impl<'a, S, K, V> StorageBST<'a, S, K, V>
where
    S: Storage,
    K: Eq,
    V: Sized,
{
    /// Creating the new instance of Storage Binary Search Tree
    pub fn new(storage: &'a S, offset: u32) -> Result<Self, InvalidOffset> {
        let mut header = Header {
            boom: 0,
            el_size: 0,
            count: 0,
            reserved_1: 0,
            reserved_2: 0,
        };
        storage.sread(offset, &mut header);

        // TODO:
        // Check boom and reserved field to be correct

        let size = size_of::<V>() ;
        if header.el_size != size as u16 {
            return Err(InvalidOffset{offset: offset});
        }

        Ok(StorageBST {
            storage,
            offset,
            _phantom: PhantomData,
        })
    }

    pub fn set(&self, key:K, value: V) {

    }

    pub fn get(&self, key:K) -> V {

    }
}
