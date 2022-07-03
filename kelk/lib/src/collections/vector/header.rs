use core::mem::size_of;

#[repr(C)]
pub(super) struct Header {
    pub boom: u32,
    pub reserved: u16,
    pub value_len: u16,
    pub size: u32,
    pub capacity: u32,
}

impl Header {
    pub fn new<V: Sized>(capacity: u32) -> Self {
        Self {
            boom: 0xb3000000,
            reserved: 0,
            value_len: size_of::<V>() as u16,
            size: 0,
            capacity,
        }
    }
}
