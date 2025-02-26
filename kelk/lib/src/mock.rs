//! Mocking Context for testing contracts

use crate::error::HostError;
use crate::storage::Storage;
use alloc::vec::Vec;
use core::cell::RefCell;

/// `MockStorage` mocks the storage for testing purpose.
pub struct MockStorage {
    storage: RefCell<Vec<u8>>,
}

impl MockStorage {
    /// instantiates a new mock
    pub fn new(size: usize) -> Self {
        let storage = RefCell::new(alloc::vec![0; size].to_vec());
        Self { storage }
    }
}

impl Storage for MockStorage {
    fn swrite(&self, offset: u32, data: &[u8]) -> Result<(), HostError> {
        if offset as usize + data.len() > self.storage.borrow().len() {
            return Err(HostError { code: 1 });
        }
        for (i, d) in data.iter().enumerate() {
            self.storage.borrow_mut()[i + offset as usize] = *d;
        }
        Ok(())
    }

    fn sread(&self, offset: u32, length: u32) -> Result<Vec<u8>, HostError> {
        if (offset + length) as usize > self.storage.borrow().len() {
            return Err(HostError { code: 1 });
        }
        let c = &self.storage.borrow()[offset as usize..(offset + length) as usize];
        Ok(c.into())
    }
}

/// mocks the storage
pub fn mock_storage(storage_size: usize) -> MockStorage {
    MockStorage::new(storage_size)
}
