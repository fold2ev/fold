pub fn to_be_bytes(val: u64) -> [u8; 8] {
    val.to_be_bytes()
}

pub fn pad_buffer(buf: &[u8], len: usize) -> Vec<u8> {
    let mut v = buf.to_vec();
    v.resize(len, 0);
    v
}