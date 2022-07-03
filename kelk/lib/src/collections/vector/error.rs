//! Error types for Storage Binary Tree

use core::fmt::{self, Debug};

/// A general list of Storage Binary Tree error
pub enum Error {
    /// Host error code
    HostError(i32),

    /// Invalid offset
    InvalidOffset(u32),

    /// Capacity is full
    OutOfCapacity,
}

impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::HostError(code) => f.debug_struct("HostError").field("code", code).finish(),

            Error::InvalidOffset(offset) => f
                .debug_struct("InvalidOffset")
                .field("offset", &offset)
                .finish(),

            Error::OutOfCapacity => f.debug_struct("Capacity is full").finish(),
        }
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::HostError(code) => write!(f, "host error code: {:?}", code),
            Error::InvalidOffset(offset) => write!(f, "invalid offset: {:?}", offset),
            Error::OutOfCapacity => write!(f, "Capacity is full"),
        }
    }
}

impl From<crate::error::HostError> for Error {
    fn from(error: crate::error::HostError) -> Self {
        Error::HostError(error.code)
    }
}
