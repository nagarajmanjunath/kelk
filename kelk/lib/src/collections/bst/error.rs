//! Error types for Storage Binary Tree

use core::fmt::{self, Debug};

// TODO: convert it to enum

/// A view into an occupied entry in a `BTreeMap`.
/// It is part of the [`Entry`] enum.
pub struct InvalidOffset {
    pub(super) offset: u32,
}

impl Debug for InvalidOffset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("InvalidOffset")
            .field("offset", &self.offset)
            .finish()
    }
}
impl fmt::Display for InvalidOffset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid offset: {:?}", self.offset)
    }
}
