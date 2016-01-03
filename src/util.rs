pub fn count_ones(x: u32) -> usize {
    ((x >> 0 & 0xff) + (x >> 8 & 0xff) + (x >> 16 & 0xff) + (x >> 24 & 0xff)) as usize
}

pub fn count_bit_errors(x: u32, y: u32) -> usize {
    count_ones(x ^ y)
}
