use crate::opcodes;
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
    /// This is decremented every frame. Furthermore, when it reaches zero, a beep is emitted by the system.
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
}

impl State {
    /// Create a new emulator with the given initial memory
    pub fn new(mem: [u8; 4096]) -> Self {
        State {
            mem,
            ..Self::default()
        }
    }

    /// Step forward one instruction in the logical simulation
    pub fn step(&mut self) {
        // We want to get the opcode at the program counter and simply match against it to call a function from the `opcodes` module
        // The opcode consists of two bytes, we're interested in each nibble, so 4 values
        let mut opcode = [0u8; 4];
        opcode[0] = self.mem[self.pc as usize] & 0xF0;
        opcode[1] = self.mem[self.pc as usize] & 0x0F;
        opcode[2] = self.mem[self.pc as usize] & 0xF0;
        opcode[3] = self.mem[self.pc as usize] & 0x0F;

        match opcode {
            // 00E0 - Clear the display.
            [0x0, 0x0, 0xE, 0x0] => opcodes::clear_display(self),
            // 00EE - Return from a subroutine.
            [0x0, 0x0, 0xE, 0xE] => opcodes::r#return(self),
            // 1nnn - Jump to location *nnn*.
            [0x1, n1, n2, n3] => self.pc = addr(n1, n2, n3),
            // 2nnn - Call subroutine at nnn.
            [0x2, n1, n2, n3] => opcodes::call(self, addr(n1, n2, n3)),
            // 3xkk - Skip next instruction if Vx = kk.
            [0x3, x, k1, k2] => opcodes::skip_eq(self, x, byte(k1, k2)),
            // 4xkk - Skip next instruction if Vx != kk.
            [0x4, x, k1, k2] => opcodes::skip_ne(self, x, byte(k1, k2)),
            // 5xy0 - Skip next instruction if Vx = Vy.
            [0x5, x, y, 0x0] => opcodes::skip_reg_eq(self, x, y),
            // 6xkk - Set Vx = kk.
            [0x6, x, k1, k2] => self.reg_v[x as usize] = byte(k1, k2),
            // 7xkk - Set Vx = Vx + kk.
            [0x7, x, k1, k2] => self.reg_v[x as usize] += byte(k1, k2),
            // 8xy0 - Set Vx = Vy.
            [0x8, x, y, 0x0] => self.reg_v[x as usize] = self.reg_v[y as usize],
            // 8xy1 - Set Vx = Vx OR Vy.
            [0x8, x, y, 0x1] => self.reg_v[x as usize] |= self.reg_v[y as usize],
            // 8xy2 - Set Vx = Vx AND Vy.
            [0x8, x, y, 0x2] => self.reg_v[x as usize] &= self.reg_v[y as usize],
            // 8xy3 - Set Vx = Vx XOR Vy.
            [0x8, x, y, 0x3] => self.reg_v[x as usize] ^= self.reg_v[y as usize],
            // 8xy4 - Set Vx = Vx + Vy, set VF = carry.
            [0x8, x, y, 0x4] => opcodes::overflowing_add(self, x, y),
            // 8xy5 - Set Vx = Vx - Vy, set VF = NOT borrow.
            [0x8, x, y, 0x5] => opcodes::overflowing_sub(self, x, y),
            // 8xy6 - Set Vx = Vx SHR 1.
            [0x8, x, _, 0x6] => opcodes::overflowing_shift_right(self, x),
            // 8xy7 - Set Vx = Vy - Vx, set VF = NOT borrow.
            // Note this reuses opcodes::sub
            [0x8, x, y, 0x7] => opcodes::overflowing_sub(self, y, x),
            // 8xyE - Set Vx = Vx SHL 1.
            [0x8, x, _, 0xE] => opcodes::overflowing_shift_left(self, x),
            // 9xy0 - Skip next instruction if Vx != Vy.
            [0x9, x, y, 0x0] => opcodes::skip_reg_ne(self, x, y),
            // Annn - Set I = nnn.
            [0xA, n1, n2, n3] => self.reg_i = addr(n1, n2, n3),
            // Bnnn - Jump to location nnn + V0.
            [0xB, n1, n2, n3] => self.pc = addr(n1, n2, n3) + self.reg_v[0x0] as u16,
            // Cxkk - Set Vx = random byte AND kk.
            [0xC, x, k1, k2] => opcodes::random(self, x, byte(k1, k2)),
            // Dxyn - Display n-byte sprite starting at memory location I at (Vx, Vy)
            // Set VF = collision.
            [0xD, x, y, n] => opcodes::draw_sprite(self, x, y, n),
            // Ex9E - Skip next instruction if key with the value of Vx is pressed.
            [0xE, x, 0x9, 0xE] => opcodes::skip_pressed(self, x),
            // ExA1 - Skip next instruction if key with the value of Vx is not pressed.
            [0xE, x, 0xA, 0x1] => opcodes::skip_unpressed(self, x),
            // Fx07 - Set Vx = delay timer value.
            [0xF, x, 0x0, 0x7] => self.reg_v[x as usize] = self.delay,
            // Fx0A - Block and wait for a key press, store the value of the key in Vx.
            [0xF, x, 0x0, 0xA] => opcodes::block_input(self, x),
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
}

// Create a new `State` with an empty memory.
// The starting PC is set to 0x200.
impl Default for State {
    fn default() -> Self {
        Self {
            mem: [0; 4096],
            reg_v: [0; 16],
            reg_i: 0,
            sound: 0,
            delay: 0,
            pc: 0x200,
            stack: [0; 16],
            sp: 0,
        }
    }
}

fn addr(n1: u8, n2: u8, n3: u8) -> u16 {
    let (n1, n2, n3) = (n1 as u16, n2 as u16, n3 as u16);
    (n1 << 8) & (n2 << 4) & n3
}

fn byte(k1: u8, k2: u8) -> u8 {
    (k1 << 4) & k2
}
