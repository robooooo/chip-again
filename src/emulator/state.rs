use crate::emulator::{fontset, input::Input, opcodes};
use std::default::Default;

#[derive(Copy, Clone)]
/// The state in memory of the emulator at any given time.
pub struct State {
    /// The system's 4K of RAM.
    pub(crate) mem: [u8; 4096],
    /// The system's 16 general-purpose registers.
    pub(crate) reg_v: [u8; 16],
    /// The system's single 16-bit register.
    pub(crate) reg_i: u16,
    /// The system's sound timer.
    ///
    /// This is decremented every frame. Furthermore, when it reaches zero, a beep is emitted
    /// by the system.
    pub(crate) sound: u8,
    /// The system's delay timer.
    ///
    /// This is decremented every frame.
    pub(crate) delay: u8,
    /// The address of the next instruction.
    pub(crate) pc: u16,
    /// The program's stack.
    ///
    /// This is used to store the return address from jumping functions.
    pub(crate) stack: [u16; 16],
    /// Pointer to the current value in the stack.
    pub(crate) sp: usize,
    /// Current contents of the screen.
    pub(crate) display: [bool; 2048],
}

impl State {
    /// Width of the display.
    pub const WIDTH: usize = 64;

    /// Height of the display.
    pub const HEIGHT: usize = 32;

    /// Create a new emulator with the given program ROM.
    /// In this case, the ROM is loaded into memory at the address 0x200, which is where the
    /// majority of CHIP-8 programs start. The program counter is also initialised to point to
    /// this location.
    pub fn new(mem: &[u8]) -> Self {
        let mut res = Self::default();
        // Load into emulator's memory starting at 0x200
        // Silently truncates extra bytes (maybe worth changing?)
        for (dst, &src) in res.mem[0x200..].iter_mut().zip(mem) {
            *dst = src;
        }
        res
    }

    /// Step forward one instruction in the logical simulation. This is provided keys are currently
    /// being pressed, to assume they are not, see `State::step_forward`.
    pub fn step(&mut self, input: Input) {
        // We want to get the opcode at the program counter and simply match against it to call
        // a function from the `opcodes` module. The opcode consists of two bytes,
        // we're interested in each nibble, so 4 values
        let mut opcode = [0u8; 4];
        opcode[0] = (self.mem[self.pc as usize + 0] & 0xF0) >> 4;
        opcode[1] = self.mem[self.pc as usize + 0] & 0x0F;
        opcode[2] = (self.mem[self.pc as usize + 1] & 0xF0) >> 4;
        opcode[3] = self.mem[self.pc as usize + 1] & 0x0F;

        self.pc += 2;
        self.delay = self.delay.saturating_sub(1);
        self.sound = self.sound.saturating_sub(1);

        match opcode {
            // 00E0 - Clear the display.
            [0x0, 0x0, 0xE, 0x0] => self.display = [false; 2048],
            // 00EE - Return from a subroutine.
            [0x0, 0x0, 0xE, 0xE] => opcodes::r#return(self),
            // 1nnn - Jump to location *nnn*.
            [0x1, n1, n2, n3] => self.pc = addr(n1, n2, n3),
            // 2nnn - Call subroutine at nnn.
            [0x2, n1, n2, n3] => opcodes::call(self, addr(n1, n2, n3)),
            // 3xkk - Skip next instruction if Vx = kk.
            [0x3, x, k1, k2] => opcodes::skip_if_equal(self, x, byte(k1, k2)),
            // 4xkk - Skip next instruction if Vx != kk.
            [0x4, x, k1, k2] => opcodes::skip_if_not_equal(self, x, byte(k1, k2)),
            // 5xy0 - Skip next instruction if Vx = Vy.
            [0x5, x, y, 0x0] => opcodes::skip_reg_equal(self, x, y),
            // 6xkk - Set Vx = kk.
            [0x6, x, k1, k2] => self.reg_v[x as usize] = byte(k1, k2),
            // 7xkk - Set Vx = Vx + kk.
            [0x7, x, k1, k2] => {
                self.reg_v[x as usize] = self.reg_v[x as usize].wrapping_add(byte(k1, k2))
            }
            // 8xy0 - Set Vx = Vy.
            [0x8, x, y, 0x0] => self.reg_v[x as usize] = self.reg_v[y as usize],
            // 8xy1 - Set Vx = Vx OR Vy.
            [0x8, x, y, 0x1] => self.reg_v[x as usize] |= self.reg_v[y as usize],
            // 8xy2 - Set Vx = Vx AND Vy.
            [0x8, x, y, 0x2] => self.reg_v[x as usize] &= self.reg_v[y as usize],
            // 8xy3 - Set Vx = Vx XOR Vy.
            [0x8, x, y, 0x3] => self.reg_v[x as usize] ^= self.reg_v[y as usize],
            // 8xy4 - Set Vx = Vx + Vy, set VF = carry.
            [0x8, x, y, 0x4] => opcodes::add(self, x, y),
            // 8xy5 - Set Vx = Vx - Vy, set VF = NOT borrow.
            [0x8, x, y, 0x5] => opcodes::subtract(self, x, y),
            // 8xy6 - Set Vx = Vx SHR 1.
            [0x8, x, _, 0x6] => opcodes::shift_right(self, x),
            // 8xy7 - Set Vx = Vy - Vx, set VF = NOT borrow.
            [0x8, x, y, 0x7] => opcodes::subtract(self, y, x),
            // 8xyE - Set Vx = Vx SHL 1.
            [0x8, x, _, 0xE] => opcodes::shift_left(self, x),
            // 9xy0 - Skip next instruction if Vx != Vy.
            [0x9, x, y, 0x0] => opcodes::skip_reg_not_equal(self, x, y),
            // Annn - Set I = nnn.
            [0xA, n1, n2, n3] => self.reg_i = addr(n1, n2, n3),
            // Bnnn - Jump to location nnn + V0.
            [0xB, n1, n2, n3] => self.pc = addr(n1, n2, n3) + dbg!(self.reg_v[0x0] as u16),
            // Cxkk - Set Vx = random byte AND kk.
            [0xC, x, k1, k2] => opcodes::random(self, x, byte(k1, k2)),
            // Dxyn - Display n-byte sprite starting at memory location I at (Vx, Vy)
            // Set VF = collision.
            [0xD, x, y, n] => opcodes::draw_sprite(self, x, y, n),
            // Ex9E - Skip next instruction if key with the value of Vx is pressed.
            [0xE, x, 0x9, 0xE] => opcodes::skip_if_pressed(self, input, x),
            // ExA1 - Skip next instruction if key with the value of Vx is not pressed.
            [0xE, x, 0xA, 0x1] => opcodes::skip_if_unpressed(self, input, x),
            // Fx07 - Set Vx = delay timer value.
            [0xF, x, 0x0, 0x7] => self.reg_v[x as usize] = self.delay,
            // Fx0A - Block and wait for a key press, store the value of the key in Vx.
            [0xF, x, 0x0, 0xA] => opcodes::block_input(self, input, x),
            // Fx15 - Set delay timer = Vx.
            [0xF, x, 0x1, 0x5] => self.delay = self.reg_v[x as usize],
            // Fx18 - Set sound timer = Vx.
            [0xF, x, 0x1, 0x8] => self.sound = self.reg_v[x as usize],
            // Fx1E - Set I = I + Vx.
            [0xF, x, 0x1, 0xE] => self.reg_i += self.reg_v[x as usize] as u16,
            // Fx29 - Set I = location of sprite for digit Vx.
            [0xF, x, 0x2, 0x9] => opcodes::sprite_location(self, x),
            // Fx33 - Store BCD representation of Vx in memory locations I, I+1, and I+2.
            [0xF, x, 0x3, 0x3] => opcodes::store_bcd(self, x),
            // Fx55 - Store registers V0 through Vx in memory starting at location I.
            [0xF, x, 0x5, 0x5] => opcodes::copy_registers(self, x),
            // Fx65 - Read registers V0 through Vx from memory starting at location I.
            [0xF, x, 0x6, 0x5] => opcodes::load_registers(self, x),

            _ => panic!("Interpreter encountered an unknown opcode: {:?}", opcode),
        }
    }

    pub fn step_forward(&mut self) {
        self.step([false; 16]);
    }
}

/// Create a new `State` with an empty memory.
/// The starting PC is set to 0x200.
impl Default for State {
    fn default() -> Self {
        let mut res = Self {
            mem: [0; 4096],
            reg_v: [0; 16],
            reg_i: 0,
            sound: 0,
            delay: 0,
            pc: 0x200,
            stack: [0; 16],
            sp: 0,
            display: [false; 2048],
        };
        // Copy fontset into memory
        res.mem[00..05].copy_from_slice(&fontset::NUM_1);
        res.mem[05..10].copy_from_slice(&fontset::NUM_2);
        res.mem[10..15].copy_from_slice(&fontset::NUM_3);
        res.mem[15..20].copy_from_slice(&fontset::NUM_4);
        res.mem[20..25].copy_from_slice(&fontset::NUM_5);
        res.mem[25..30].copy_from_slice(&fontset::NUM_6);
        res.mem[30..35].copy_from_slice(&fontset::NUM_7);
        res.mem[35..40].copy_from_slice(&fontset::NUM_8);
        res.mem[40..45].copy_from_slice(&fontset::NUM_9);
        res.mem[45..50].copy_from_slice(&fontset::DIG_A);
        res.mem[50..55].copy_from_slice(&fontset::DIG_B);
        res.mem[55..60].copy_from_slice(&fontset::DIG_B);
        res.mem[60..65].copy_from_slice(&fontset::DIG_C);
        res.mem[65..70].copy_from_slice(&fontset::DIG_D);
        res.mem[70..75].copy_from_slice(&fontset::DIG_E);
        res
    }
}

/// Combine three nibbles into an address.
fn addr(n1: u8, n2: u8, n3: u8) -> u16 {
    let (n1, n2, n3) = (n1 as u16, n2 as u16, n3 as u16);
    (n1 << 8) | (n2 << 4) | n3
}

/// Combine two nibbles into a byte.
fn byte(k1: u8, k2: u8) -> u8 {
    (k1 << 4) | k2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_addr() {
        assert_eq!(0x0000, addr(0x0, 0x0, 0x0));
        assert_eq!(0x0123, addr(0x1, 0x2, 0x3));
        assert_eq!(0x0FFF, addr(0xF, 0xF, 0xF));
    }

    #[test]
    fn test_byte() {
        assert_eq!(0x00, byte(0x0, 0x0));
        assert_eq!(0x12, byte(0x1, 0x2));
        assert_eq!(0xFF, byte(0xF, 0xF));
    }
}
