use crate::emulator::State;

// http://devernay.free.fr/hacks/chip8/C8TECH10.HTM

pub fn clear_display(mem: &mut State) {
    todo!()
}

pub fn r#return(mem: &mut State) {
    mem.pc = mem.stack[mem.sp as usize];
    mem.sp -= 1;
}

pub fn call(mem: &mut State, addr: u16) {
    mem.sp += 1;
    mem.stack[mem.sp as usize] = addr;
    mem.pc = addr;
}

pub fn skip_eq(mem: &mut State, x: u8, byte: u8) {
    if mem.reg_v[x as usize] == byte {
        mem.sp += 2;
    }
}

pub fn skip_ne(mem: &mut State, x: u8, byte: u8) {
    if mem.reg_v[x as usize] != byte {
        mem.sp += 2;
    }
}

pub fn skip_reg_eq(mem: &mut State, x: u8, y: u8) {
    if mem.reg_v[x as usize] == mem.reg_v[y as usize] {
        mem.sp += 2;
    }
}

pub fn add(mem: &mut State, x: u8, y: u8) {
    let (res, flag) = mem.reg_v[x as usize].overflowing_add(mem.reg_v[y as usize]);
    mem.reg_v[x as usize] = res;
    mem.reg_v[0xF] = flag as u8;
}

pub fn sub(mem: &mut State, x: u8, y: u8) {
    let (res, flag) = mem.reg_v[x as usize].overflowing_sub(mem.reg_v[y as usize]);
    mem.reg_v[x as usize] = res;
    mem.reg_v[0xF] = !flag as u8;
}

pub fn shr(mem: &mut State, x: u8) {
    mem.reg_v[0xF] = mem.reg_v[x as usize] & 0x01;
    mem.reg_v[x as usize] <<= 1;
}

pub fn shl(mem: &mut State, x: u8) {
    mem.reg_v[0xF] = ((mem.reg_v[x as usize] & 0x80) != 0) as u8;
    mem.reg_v[x as usize] >>= 1;
}

pub fn skip_reg_ne(mem: &mut State, x: u8, y: u8) {
    if mem.reg_v[x as usize] != mem.reg_v[y as usize] {
        mem.sp += 2;
    }
}

pub fn random(mem: &mut State, x: u8, kk: u8) {
    let rng: u8 = rand::random();
    mem.reg_v[x as usize] = rng & kk;
}

pub fn draw_sprite(mem: &mut State, x: u8, y: u8, n: u8) {
    todo!()
}

pub fn skip_pressed(mem: &mut State, x: u8) {
    todo!()
}

pub fn skip_unpressed(mem: &mut State, x: u8) {
    todo!()
}

pub fn block_input(mem: &mut State, x: u8) {
    todo!()
}

pub fn sprite_location(mem: &mut State, x: u8) {
    todo!()
}

pub fn store_bcd(mem: &mut State, x: u8) {
    let I = mem.reg_i as usize;
    let Vx = mem.reg_v[x as usize];
    mem.mem[I + 0] = (Vx % 1000) / 100;
    mem.mem[I + 1] = (Vx % 100) / 10;
    mem.mem[I + 2] = Vx % 10;
}

pub fn copy_registers(mem: &mut State, x: u8) {
    for i in 0..x {
        mem.mem[mem.reg_i as usize + i as usize] = mem.reg_v[i as usize];
    }
}

pub fn load_registers(mem: &mut State, x: u8) {
    for i in 0..x {
        mem.reg_v[i as usize] = mem.mem[mem.reg_i as usize + i as usize];
    }
}
