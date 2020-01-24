use crate::emulator::State;

// http://devernay.free.fr/hacks/chip8/C8TECH10.HTM

/// Clear the display
pub fn clear_display(mem: &mut State) {
    todo!()
}

/// Return from a subroutine.
///
/// The interpreter sets the program counter to the address at the top of the stack, then
/// subtracts 1 from the stack pointer.
pub fn r#return(mem: &mut State) {
    mem.pc = mem.stack[mem.sp as usize];
    mem.sp -= 1;
}

/// Call subroutine at nnn.
///
/// The interpreter increments the stack pointer, then puts the current PC on the top of the stack.
/// The PC is then set to nnn.
pub fn call(mem: &mut State, addr: u16) {
    mem.sp += 1;
    mem.stack[mem.sp as usize] = addr;
    mem.pc = addr;
}

/// Skip next instruction if Vx = kk.
///
/// The interpreter compares register Vx to kk, and if they are equal, increments the program
/// counter by 2.
pub fn skip_eq(mem: &mut State, x: u8, byte: u8) {
    if mem.reg_v[x as usize] == byte {
        mem.sp += 2;
    }
}
/// Skip next instruction if Vx != kk.
///
/// The interpreter compares register Vx to kk, and if they are not equal, increments the program
/// counter by 2.
pub fn skip_ne(mem: &mut State, x: u8, byte: u8) {
    if mem.reg_v[x as usize] != byte {
        mem.sp += 2;
    }
}
/// Skip next instruction if Vx = Vy.
///
/// The interpreter compares register Vx to register Vy, and if they are equal, increments the
/// program counter by 2.
pub fn skip_reg_eq(mem: &mut State, x: u8, y: u8) {
    if mem.reg_v[x as usize] == mem.reg_v[y as usize] {
        mem.sp += 2;
    }
}

/// Set Vx = Vx + Vy, set VF = carry.
///
/// The values of Vx and Vy are added together. If the result is greater than 8 bits (i.e., > 255,)
/// VF is set to 1, otherwise 0. Only the lowest 8 bits of the result are kept, and stored in Vx.
pub fn overflowing_add(mem: &mut State, x: u8, y: u8) {
    let (res, flag) = mem.reg_v[x as usize].overflowing_add(mem.reg_v[y as usize]);
    mem.reg_v[x as usize] = res;
    mem.reg_v[0xF] = flag as u8;
}

/// Set Vx = Vx - Vy, set VF = NOT borrow.
///
/// If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results
/// stored in Vx.
pub fn overflowing_sub(mem: &mut State, x: u8, y: u8) {
    let (res, flag) = mem.reg_v[x as usize].overflowing_sub(mem.reg_v[y as usize]);
    mem.reg_v[x as usize] = res;
    mem.reg_v[0xF] = !flag as u8;
}

/// Set Vx = Vx SHR 1.
///
/// If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided
/// by 2.
pub fn overflowing_shift_right(mem: &mut State, x: u8) {
    mem.reg_v[0xF] = mem.reg_v[x as usize] & 0x01;
    mem.reg_v[x as usize] <<= 1;
}

/// Set Vx = Vx SHL 1.
///
///If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is
/// multiplied by 2.
pub fn overflowing_shift_left(mem: &mut State, x: u8) {
    mem.reg_v[0xF] = ((mem.reg_v[x as usize] & 0x80) != 0) as u8;
    mem.reg_v[x as usize] >>= 1;
}

/// Skip next instruction if Vx != Vy.
///
/// The values of Vx and Vy are compared, and if they are not equal, the program counter is
/// increased by 2.
pub fn skip_reg_ne(mem: &mut State, x: u8, y: u8) {
    if mem.reg_v[x as usize] != mem.reg_v[y as usize] {
        mem.sp += 2;
    }
}

/// Set Vx = random byte AND kk.
///
/// The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk.
/// The results are stored in Vx. See instruction 8xy2 for more information on AND.
pub fn random(mem: &mut State, x: u8, kk: u8) {
    let rng: u8 = rand::random();
    mem.reg_v[x as usize] = rng & kk;
}

/// Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
///
/// The interpreter reads n bytes from memory, starting at the address stored in I.
/// These bytes are then displayed as sprites on screen at coordinates (Vx, Vy).
/// Sprites are XORed onto the existing screen.
/// If this causes any pixels to be erased, VF is set to 1, otherwise it is set to 0.
/// If the sprite is positioned so part of it is outside the coordinates of the display, it wraps
/// around to the opposite side of the screen.
pub fn draw_sprite(mem: &mut State, x: u8, y: u8, n: u8) {
    todo!()
}

/// Skip next instruction if key with the value of Vx is pressed.
///
/// Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down position, PC is increased by 2.
pub fn skip_pressed(mem: &mut State, x: u8) {
    todo!()
}

/// Skip next instruction if key with the value of Vx is not pressed.
///
/// Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position, PC is increased by 2.
pub fn skip_unpressed(mem: &mut State, x: u8) {
    todo!()
}

/// Wait for a key press, store the value of the key in Vx.
///
/// All execution stops until a key is pressed, then the value of that key is stored in Vx.
pub fn block_input(mem: &mut State, x: u8) {
    todo!()
}

/// Set I = location of sprite for digit Vx.
///
/// The value of I is set to the location for the hexadecimal sprite corresponding to the
/// value of Vx
pub fn sprite_location(mem: &mut State, x: u8) {
    todo!()
}

/// Store BCD representation of Vx in memory locations I, I+1, and I+2.
///
/// The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at
/// location in I, the tens digit at location I+1, and the ones digit at location I+2.
pub fn store_bcd(mem: &mut State, x: u8) {
    let I = mem.reg_i as usize;
    let Vx = mem.reg_v[x as usize];
    // No need to mod 1000 because the range of u8 is below this
    mem.mem[I + 0] = Vx /*% 1000*/ / 100;
    mem.mem[I + 1] = (Vx % 100) / 10;
    mem.mem[I + 2] = Vx % 10;
}

/// Store registers V0 through Vx in memory starting at location I.
///
/// The interpreter copies the values of registers V0 through Vx into memory, starting at the
/// address in I.
pub fn copy_registers(mem: &mut State, x: u8) {
    for i in 0..x {
        mem.mem[mem.reg_i as usize + i as usize] = mem.reg_v[i as usize];
    }
}

/// Read registers V0 through Vx from memory starting at location I.
///
/// The interpreter reads values from memory starting at location I into registers V0 through Vx.
pub fn load_registers(mem: &mut State, x: u8) {
    for i in 0..x {
        mem.reg_v[i as usize] = mem.mem[mem.reg_i as usize + i as usize];
    }
}
