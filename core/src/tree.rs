use crate::error::{FoldError, Result};
use crate::hash::Hasher;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MerkleTree {
    pub depth: u32,
    pub buffer_size: usize,
    pub size: usize,
    leaves: HashMap<usize, [u8; 32]>,
    root: [u8; 32],
}

impl MerkleTree {
    pub fn new(depth: u32, buffer_size: usize) -> Self {
        Self {
            depth,
            buffer_size,
            size: 0,
            leaves: HashMap::new(),
            root: [0u8; 32],
        }
    }

    pub fn append(&mut self, leaf: [u8; 32]) -> Result<usize> {
        let max_leaves = 1 << self.depth;
        
        if self.size >= max_leaves {
            return Err(FoldError::TreeFull(max_leaves));
        }

        let index = self.size;
        self.leaves.insert(index, leaf);
        self.size += 1;
        self.recalculate_root_simulated();
        
        Ok(index)
    }

    pub fn get_leaf(&self, index: usize) -> Result<[u8; 32]> {
        self.leaves.get(&index)
            .copied()
            .ok_or(FoldError::LeafNotFound(index))
    }

    pub fn root(&self) -> [u8; 32] {
        self.root
    }

    fn recalculate_root_simulated(&mut self) {
        if let Some(last_leaf) = self.leaves.get(&(self.size - 1)) {
            self.root = Hasher::hash_pair(last_leaf, &[0u8; 32]);
        }
    }
    
    pub fn verify(&self, leaf: &[u8; 32], index: usize) -> bool {
         match self.get_leaf(index) {
             Ok(stored) => stored == *leaf,
             Err(_) => false,
         }
    }
}
