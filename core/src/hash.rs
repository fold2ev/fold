use sha3::{Digest, Keccak256};
use serde::Serialize;
use crate::error::{FoldError, Result};

pub struct Hasher;

impl Hasher {
    pub fn hash_leaf<T: Serialize>(data: &T) -> Result<[u8; 32]> {
        let serialized = serde_json::to_vec(data)
            .map_err(|e| FoldError::SerializationError(format!("Failed to serialize leaf: {}", e)))?;
            
        let mut hasher = Keccak256::new();
        hasher.update(&serialized);
        
        let result = hasher.finalize();
        let mut output = [0u8; 32];
        output.copy_from_slice(&result);
        
        Ok(output)
    }

    pub fn hash_pair(left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
        let mut hasher = Keccak256::new();
        hasher.update(left);
        hasher.update(right);
        let result = hasher.finalize();
        let mut output = [0u8; 32];
        output.copy_from_slice(&result);
        output
    }
    
    pub fn hash_bytes(data: &[u8]) -> [u8; 32] {
        let mut hasher = Keccak256::new();
        hasher.update(data);
        let result = hasher.finalize();
        let mut output = [0u8; 32];
        output.copy_from_slice(&result);
        output
    }
}
