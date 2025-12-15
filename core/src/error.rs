use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum FoldError {
    #[error("Merkle Tree is full. Max capacity: {0}")]
    TreeFull(usize),

    #[error("Invalid Proof provided for root: {0:?}")]
    InvalidProof([u8; 32]),

    #[error("Concurrency limit exceeded. Buffer size: {0}")]
    ConcurrencyLimitExceeded(usize),

    #[error("Leaf not found at index: {0}")]
    LeafNotFound(usize),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Invalid Canopy Depth. Must be between 0 and 17.")]
    InvalidCanopy,
    
    #[error("RPC Error: {0}")]
    RpcError(String),
}

pub type Result<T> = std::result::Result<T, FoldError>;
