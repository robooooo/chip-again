/// A set of flags representing which keys are pressed. The CHIP-8 keypad is a hexadecimal number
/// pad with characters 0123456789ABCDEF. Hence, if the 1 and B keys are pressed, the values of the
/// flags at indexes 0x1 (1) and 0xB (11) should be pressed
pub type Input = [bool; 16];

// Index-based keymap, the character at index k (in hexadecimal) corresponds to the letter k on the CHIP-8 keypad.
// Note that this keypad does not start at 0, so some rearranging of letters is neccesary.
pub const KEYMAP: [char; 16] = [
    'x', '1', '2', '3', 'q', 'w', 'e', 'a', 's', 'd', 
    'z', 'c', '4', 'r', 'f', 'v'
];

