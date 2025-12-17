#[cfg(test)]
mod tests {
    use fold_core::hash::Hasher;
    #[test]
    fn test_hash_bytes() {
        let res = Hasher::hash_bytes(b"test");
        assert_ne!(res, [0u8; 32]);
    }
}