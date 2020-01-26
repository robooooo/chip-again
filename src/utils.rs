/// Converts a u8 to a byte array representing each of its binary digits.
///
/// The LSB will be stored in the last position in the array. This means that a literal like
/// 0b00110010 will map to [false, false, true, true, false, false, true, false].
pub(crate) fn u8_to_bits(mut x: u8) -> [bool; 8] {
    const BITS: usize = 8;

    let mut res = [false; 8];
    for i in 1..=BITS {
        res[BITS - i] = (i & 1) != 0;
        x >>= 1;
    }
    res
}
