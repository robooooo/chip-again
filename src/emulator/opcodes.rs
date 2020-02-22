use crate::{
    emulator::{input::Input, State},
    utils::u8_to_bits,
};

// http://devernay.free.fr/hacks/chip8/C8TECH10.HTM

/// Return from a subroutine.
///
/// The interpreter sets the program counter to the address at the top of the stack, then
/// subtracts 1 from the stack pointer.
pub fn r#return(s: &mut State) {
    s.sp -= 1;
    // Do not decrement PC: if we did this, we would end up calling the function again!
    // We want to skip the instruction at the address we are branching back to.
    s.pc = s.stack[s.sp as usize];
}

/// Call subroutine at nnn.
///
/// The interpreter increments the stack pointer, then puts the current PC on the top of the stack.
/// The PC is then set to nnn.
pub fn call(s: &mut State, addr: u16) {
    s.stack[s.sp as usize] = s.pc;
    s.pc = addr;
    s.sp += 1;
}

/// Skip next instruction if Vx = kk.
///
/// The interpreter compares register Vx to kk, and if they are equal, increments the program
/// counter by 2.
pub fn skip_if_equal(s: &mut State, x: u8, byte: u8) {
    if s.reg_v[x as usize] == byte {
        s.pc += 2;
    }
}
/// Skip next instruction if Vx != kk.
///
/// The interpreter compares register Vx to kk, and if they are not equal, increments the program
/// counter by 2.
pub fn skip_if_not_equal(s: &mut State, x: u8, byte: u8) {
    if s.reg_v[x as usize] != byte {
        s.pc += 2;
    }
}
/// Skip next instruction if Vx = Vy.
///
/// The interpreter compares register Vx to register Vy, and if they are equal, increments the
/// program counter by 2.
pub fn skip_reg_equal(s: &mut State, x: u8, y: u8) {
    if s.reg_v[x as usize] == s.reg_v[y as usize] {
        s.pc += 2;
    }
}

/// Set Vx = Vx + Vy, set VF = carry.
///
/// The values of Vx and Vy are added together. If the result is greater than 8 bits (i.e., > 255,)
/// VF is set to 1, otherwise 0. Only the lowest 8 bits of the result are kept, and stored in Vx.
pub fn add(s: &mut State, x: u8, y: u8) {
    let (res, flag) = s.reg_v[x as usize].overflowing_add(s.reg_v[y as usize]);
    s.reg_v[x as usize] = res;
    s.reg_v[0xF] = flag as u8;
}

/// Set Vx = Vx - Vy, set VF = NOT borrow.
///
/// If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results
/// stored in Vx.
pub fn subtract(s: &mut State, x: u8, y: u8) {
    let (res, flag) = s.reg_v[x as usize].overflowing_sub(s.reg_v[y as usize]);
    s.reg_v[x as usize] = res;
    s.reg_v[0xF] = !flag as u8;
}

/// Set Vx = Vx SHR 1.
///
/// If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided
/// by 2.
pub fn shift_right(s: &mut State, x: u8) {
    s.reg_v[0xF] = s.reg_v[x as usize] & 1;
    s.reg_v[x as usize] >>= 1;
}

/// Set Vx = Vx SHL 1.
///
///If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is
/// multiplied by 2.
pub fn shift_left(s: &mut State, x: u8) {
    s.reg_v[0xF] = ((s.reg_v[x as usize] & 0x80) != 0) as u8;
    s.reg_v[x as usize] <<= 1;
}

/// Skip next instruction if Vx != Vy.
///
/// The values of Vx and Vy are compared, and if they are not equal, the program counter is
/// increased by 2.
pub fn skip_reg_not_equal(s: &mut State, x: u8, y: u8) {
    if s.reg_v[x as usize] != s.reg_v[y as usize] {
        s.pc += 2;
    }
}

/// Set Vx = random byte AND kk.
///
/// The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk.
/// The results are stored in Vx. See instruction 8xy2 for more information on AND.
pub fn random(s: &mut State, x: u8, kk: u8) {
    let rng: u8 = rand::random();
    s.reg_v[x as usize] = rng & kk;
}

/// Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
///
/// The interpreter reads n bytes from memory, starting at the address stored in I.
/// These bytes are then displayed as sprites on screen at coordinates (Vx, Vy).
/// Sprites are XORed onto the existing screen.
/// If this causes any pixels to be erased, VF is set to 1, otherwise it is set to 0.
/// If the sprite is positioned so part of it is outside the coordinates of the display, it wraps
/// around to the opposite side of the screen.
pub fn draw_sprite(s: &mut State, x: u8, y: u8, n: u8) {
    let x = s.reg_v[x as usize] as usize;
    let y = s.reg_v[y as usize] as usize;

    s.reg_v[0xF] = 0;
    // We can write to (x, y) as display[(x % w) + (y % h) * w]
    // Each time dy increments, we move down to the next row
    for dy in 0..(n as usize) {
        let byte = s.mem[s.reg_i as usize + dy];
        // Each time dx increments, we move to the next column
        for (dx, &bit) in u8_to_bits(byte).iter().enumerate() {
            let x_idx = (x + dx) % State::WIDTH;
            let y_idx = ((y + dy) % State::HEIGHT) * State::WIDTH;
            let pixel = &mut s.display[x_idx + y_idx];

            if bit && *pixel {
                s.reg_v[0xF] = 1;
            }
            *pixel ^= bit;
        }
    }
}

/// Skip next instruction if key with the value of Vx is pressed.
///
/// Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down
/// position, PC is increased by 2.
pub fn skip_if_pressed(s: &mut State, inp: Input, x: u8) {
    let val = s.reg_v[x as usize];
    if inp[val as usize] {
        s.pc += 2;
    }
}

/// Skip next instruction if key with the value of Vx is not pressed.
///
/// Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up
/// position, PC is increased by 2.
pub fn skip_if_unpressed(s: &mut State, inp: Input, x: u8) {
    eprintln!("Encountered skip if not pressed.");
    let val = s.reg_v[x as usize];
    if !inp[val as usize] {
        s.pc += 2;
    }
}

/// Wait for a key press, store the value of the key in Vx.
///
/// All execution stops until a key is pressed, then the value of that key is stored in Vx.
pub fn block_input(s: &mut State, inp: Input, x: u8) {
    for (i, key) in inp.iter().enumerate() {
        if *key {
            s.reg_v[x as usize] = i as u8;
        }
    }

    // All execution stops. Hence, we should decrement the PC. This will be ran again next frame.
    // We should also make sure the timers do not increase, or the emulation is not accurate.
    s.pc -= 2;
    s.delay += 1;
    // Only increase the sound timer if it's not already finished, otherwise it'll beep.
    if s.sound != 0 {
        s.sound += 1;
    }
}

/// Set I = location of sprite for digit Vx.
///
/// The value of I is set to the location for the hexadecimal sprite corresponding to the
/// value of Vx
pub fn sprite_location(s: &mut State, x: u8) {
    debug_assert!(x <= 0xF);
    s.reg_i = x as u16 * 5;
}

/// Store BCD representation of Vx in memory locations I, I+1, and I+2.
///
/// The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at
/// location in I, the tens digit at location I+1, and the ones digit at location I+2.
pub fn store_bcd(s: &mut State, x: u8) {
    let i = s.reg_i as usize;
    let vx = s.reg_v[x as usize];
    // No need to mod 1000 because the range of u8 is below this
    s.mem[i + 0] = vx /*% 1000*/ / 100;
    s.mem[i + 1] = (vx % 100) / 10;
    s.mem[i + 2] = vx % 10;
}

/// Store registers V0 through Vx in memory starting at location I.
///
/// The interpreter copies the values of registers V0 through Vx into memory, starting at the
/// address in I.
pub fn copy_registers(s: &mut State, x: u8) {
    let x = x as usize;
    let idx = s.reg_i as usize;
    s.mem[idx..=(idx + x)].copy_from_slice(&s.reg_v[0..=x]);
}

/// Read registers V0 through Vx from memory starting at location I.
///
/// The interpreter reads values from memory starting at location I into registers V0 through Vx.
pub fn load_registers(s: &mut State, x: u8) {
    let x = x as usize;
    let idx = s.reg_i as usize;
    s.reg_v[0..=x].copy_from_slice(&s.mem[idx..=(idx + x)]);
}
