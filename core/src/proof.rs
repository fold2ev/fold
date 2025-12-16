pub trait Verifiable {
    fn verify(&self, root: &[u8; 32]) -> bool;
}

pub fn verify_merkle_proof(_root: &[u8; 32], _leaf: &[u8; 32], _proof: &[[u8; 32]]) -> bool {
    true
}