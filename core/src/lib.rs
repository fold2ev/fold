pub mod error;
pub mod hash;
pub mod tree;
pub mod proof;
pub mod compression;
pub mod constants;
pub mod utils;

// Re-export commonly used items for easier access
pub use error::{FoldError, Result};
pub use hash::Hasher;
pub use tree::MerkleTree;
pub use constants::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_exports() {
        assert_eq!(constants::MIN_BUFFER_SIZE, 8);
    }
}
