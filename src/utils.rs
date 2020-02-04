/// Converts a u8 to a byte array representing each of its binary digits.
///
/// The LSB will be stored in the last position in the array. This means that a literal like
/// 0b00110010 will map to [false, false, true, true, false, false, true, false].
pub fn u8_to_bits(mut x: u8) -> [bool; 8] {
    let mut res = [false; 8];
    for bit in res.iter_mut().rev() {
        *bit = x & 1 != 0;
        x >>= 1;
    }
    res
}

/// Converts a u8 to its binary coded decimal representation.
///
/// The hundreds digits of the number will come first, then the tens, and finally the digits
///
/// # Examples
///
/// bcd(123) -> [1, 2, 3]
/// bcd(020) -> [0, 2, 0]
pub fn bcd(x: u8) -> [u8; 3] {
    let hundreds = x /*% 1000*/ / 100;
    let tens = (x % 100) / 10;
    let ones = x % 10;

    [hundreds, tens, ones]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_bits() {
        assert_eq!([false; 8], u8_to_bits(0b00000000));
        assert_eq!(
            [false, false, true, true, false, true, false, true],
            u8_to_bits(0b00110101)
        );
    }

    #[test]
    fn test_bcd() {
        assert_eq!([0, 0, 0], bcd(000));
        assert_eq!([1, 2, 3], bcd(123));
        assert_eq!([0, 2, 1], bcd(021));
    }
}
