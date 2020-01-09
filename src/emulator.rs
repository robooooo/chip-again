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
            mem: mem,
            ..Self::default()
        }
    }

    /// Step forward one frame in the logical simulation
    pub fn step(&mut self) {
        // We want to get the opcode at the program counter and simply match against it to call a function from the `opcodes` module
        // The opcode consists of two bytes, we're interested in each nibble, so 4 values
        let mut opcode = [0u8; 4];
        opcode[0] = self.mem[self.pc as usize] & 0xF0;
        opcode[1] = self.mem[self.pc as usize] & 0x0F;
        opcode[2] = self.mem[self.pc as usize] & 0xF0;
        opcode[3] = self.mem[self.pc as usize] & 0x0F;

        match opcode {
            [0x0, 0x0, 0xE, 0x0] => unimplemented!(),
            [0x0, 0x0, 0xE, 0xE] => opcodes::ret(self),
            [0x1, n1, n2, n3] => opcodes::jp_addr(self, addr(n1, n2, n3)),
            [0x2, n1, n2, n3] => opcodes::call_addr(self, addr(n1, n2, n3)),
            [0x3, x, k1, k2] => opcodes::se_vx_byte(self, x, byte(k1, k2)),
            [0x4, x, k1, k2] => opcodes::sne_vx_byte(self, x, byte(k1, k2)),
            [0x5, x, y, 0x0] => opcodes::se_vx_vy(self, x, y),
            [0x6, x, k1, k2] => opcodes::ld_vx_byte(self, x, byte(k1, k2)),
            [0x7, x, k1, k2] => opcodes::add_vx_byte(self, x, byte(k1, k2)),
            [0x8, x, y, 0x0] => opcodes::ld_vx_vy(self, x, y),
            [0x8, x, y, 0x1] => opcodes::or_vx_vy(self, x, y),
            [0x8, x, y, 0x2] => opcodes::and_vx_vy(self, x, y),
            [0x8, x, y, 0x3] => opcodes::xor_vx_vy(self, x, y),
            [0x8, x, y, 0x4] => opcodes::add_vx_vy(self, x, y),
            [0x8, x, y, 0x5] => opcodes::sub_vx_vy(self, x, y),
            [0x8, x, _y, 0x6] => opcodes::shr_vx_vy(self, x),
            [0x8, x, y, 0x7] => opcodes::subn_vx_vy(self, x, y),
            [0x8, x, _y, 0xE] => opcodes::shl_vx_vy(self, x),
            [0x9, x, y, 0x0] => opcodes::sne_vx_vy(self, x, y),
            [0xA, n1, n2, n3] => opcodes::ld_i_addr(self, addr(n1, n2, n3)),
            [0xB, n1, n2, n3] => opcodes::jp_v_addr(self, addr(n1, n2, n3)),
            
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