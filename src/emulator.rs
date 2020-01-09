use std::default::Default;

#[derive(Copy, Clone)]
/// The state in memory of the emulator at any given time.
struct State {
    /// The system's 4K of RAM.
    mem: [u8; 4096],
    /// The system's 16 general-purpose registers.
    reg_v: [u8; 16],
    /// The system's single 16-bit register.
    reg_i: u16,
    /// The system's sound timer.
    /// 
    /// This is decremented every frame. Furthermore, when it reaches zero, a beep is emitted by the system.
    sound: u8,
    /// The system's delay timer.
    /// 
    /// This is decremented every frame.
    delay: u8,
    /// The address of the next instruction.
    pc: u16,
    /// The program's stack.
    /// 
    /// This is used to store the return address from jumping functions.
    stack: [u16; 16],
    /// Pointer to the current value in the stack.
    sp: usize,
}

impl State {
    /// Create a new emulator with the given initial memory
    fn new(mem: [u8; 4096]) -> Self {
        State {
            mem: mem,
            .. Self::default()
        }
    }

    fn step(&mut self) {
        
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