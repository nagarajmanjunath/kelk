//! Storage Vector, is a Vector or Array that instead of using Random Access Memory,
//! Read and writes from contract's storage. Therefore it's permanently store inside contract's storage.

use super::error::Error;
use super::header::Header;
use crate::storage::{sread_struct, swrite_struct, Storage};
use core::marker::PhantomData;
use core::mem::size_of;
use core::result::Result;

/// The instance of Storage Vector
pub struct StorageVec<'a, V>
where
    V: Sized,
{
    storage: &'a dyn Storage,
    offset: u32,
    header: Header,
    _phantom: PhantomData<V>,
}

impl<'a, V> StorageVec<'a, V>
where
    V: Sized,
{
    /// creates and store a new instance of Storage Vector at the given offset
    pub fn create(storage: &'a dyn Storage, offset: u32, capacity: u32) -> Result<Self, Error> {
        let header = Header::new::<V>(capacity);
        swrite_struct(storage, offset, &header)?;

        Ok(StorageVec {
            storage,
            offset,
            header,
            _phantom: PhantomData,
        })
    }

    /// load the Storage Vector
    pub fn lazy_load(storage: &'a dyn Storage, offset: u32) -> Result<Self, Error> {
        let header = sread_struct::<Header>(storage, offset)?;

        // TODO:
        // Check boom and reserved field to be correct

        if header.value_len != size_of::<V>() as u16 {
            return Err(Error::InvalidOffset(offset));
        }

        Ok(StorageVec {
            storage,
            offset,
            header,
            _phantom: PhantomData,
        })
    }

    /// Returns the number of elements in the vector, also referred to as its ‘length’.
    pub fn len(&self) -> u32 {
        self.header.size
    }

    /// Returns true if the vector contains no elements.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Appends an element to the back of a vector.
    pub fn push(&mut self, value: V) -> Result<(), Error> {
        if self.header.size >= self.header.capacity {
            return Err(Error::OutOfCapacity);
        }

        let offset = self.offset
            + size_of::<Header>() as u32
            + (self.header.size * self.header.value_len as u32);

        self.header.size += 1;
        swrite_struct(self.storage, self.offset, &self.header)?;
        swrite_struct(self.storage, offset, &value)?;
        Ok(())
    }

    /// Returns an element at the given index or None if out of bounds..
    pub fn get(&self, index: u32) -> Result<Option<V>, Error> {
        if index >= self.header.size {
            return Ok(None);
        }

        let offset =
            self.offset + size_of::<Header>() as u32 + (index * self.header.value_len as u32);
        let val: V = sread_struct(self.storage, offset)?;
        Ok(Some(val))
    }
}

#[cfg(test)]
mod tests {
    use crate::mock::mock_storage;

    use super::*;
    use core::mem::size_of;

    #[test]
    fn test_size() {
        assert_eq!(16, size_of::<Header>());
    }

    #[test]
    fn test_header() {
        let storage = mock_storage(1024);
        StorageVec::<i32>::create(&storage, 512, 16).unwrap();
        let header = sread_struct::<Header>(&storage, 512).unwrap();
        assert_eq!(header.boom, 0xb3000000);
        assert_eq!(header.reserved, 0);
        assert_eq!(header.value_len, 4);
        assert_eq!(header.size, 0);
        assert_eq!(header.capacity, 16);
    }

    #[test]
    fn test_vector() {
        let storage = mock_storage(1024);
        let mut vec = StorageVec::<i32>::create(&storage, 512, 16).unwrap();
        assert_eq!(None, vec.get(0).unwrap());
        assert!(vec.is_empty());

        vec.push(10).unwrap();
        vec.push(11).unwrap();
        vec.push(12).unwrap();

        assert_eq!(3, vec.len());
        assert_eq!(Some(10), vec.get(0).unwrap());
        assert_eq!(Some(11), vec.get(1).unwrap());
        assert_eq!(Some(12), vec.get(2).unwrap());
        assert_eq!(None, vec.get(3).unwrap());
    }

    #[test]
    fn test_load() {
        let storage = mock_storage(1024);
        let mut vec = StorageVec::<i32>::create(&storage, 512, 128).unwrap();
        vec.push(1).unwrap();

        let vec = StorageVec::<i32>::lazy_load(&storage, 512).unwrap();
        let header = sread_struct::<Header>(&storage, 512).unwrap();
        assert_eq!(header.boom, 0xb3000000);
        assert_eq!(header.reserved, 0);
        assert_eq!(header.value_len, 4);
        assert_eq!(header.size, 1);
        assert_eq!(header.capacity, 128);
        assert_eq!(Some(1), vec.get(0).unwrap());
    }

    #[test]
    fn test_capacity() {
        let storage = mock_storage(1024);
        let mut vec = StorageVec::<i32>::create(&storage, 0, 4).unwrap();

        vec.push(1).unwrap();
        vec.push(2).unwrap();
        vec.push(3).unwrap();
        vec.push(4).unwrap();
        assert!(vec.push(5).is_err());
    }
}
