/// A set of flags representing which keys are pressed. The CHIP-8 keypad is a hexadecimal number
/// pad with characters 0123456789ABCDEF. Hence, if the 1 and B keys are pressed, the values of the
/// flags at indexes 0x1 (1) and 0xB (11) should be pressed
pub type Input = [bool; 16];
