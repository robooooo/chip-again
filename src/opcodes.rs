use crate::emulator::State;

// http://devernay.free.fr/hacks/chip8/C8TECH10.HTM

pub fn ret(mem: &mut State) {
    mem.pc = mem.stack[mem.sp];
    mem.sp -= 1;
}

pub fn jp_addr(mem: &mut State, addr: u16) {
    mem.pc = addr;
}

pub fn call_addr(mem: &mut State, addr: u16) {
    mem.sp += 1;
    mem.stack[mem.sp as usize] = addr;   
    mem.pc = addr;
}

pub fn se_vx_byte(mem: &mut State, x: u8, byte: u8) {
    if mem.reg_v[x as usize] == byte {
        mem.sp += 2;
    }
}

pub fn sne_vx_byte(mem: &mut State, x: u8, byte: u8) {
    if mem.reg_v[x as usize] != byte {
        mem.sp += 2;
    }
}

pub fn se_vx_vy(mem: &mut State, x: u8, y: u8) {
    if mem.reg_v[x as usize] == mem.reg_v[y as usize] {
        mem.sp += 2;
    }
}

pub fn ld_vx_byte(mem: &mut State, x: u8, byte: u8) {
    mem.reg_v[x as usize] = byte;
}

pub fn add_vx_byte(mem: &mut State, x: u8, byte: u8) {
    mem.reg_v[x as usize] += byte;
}

pub fn ld_vx_vy(mem: &mut State, x: u8, y: u8) {
    mem.reg_v[x as usize] = mem.reg_v[y as usize];
}

pub fn or_vx_vy(mem: &mut State, x: u8, y: u8) {
    mem.reg_v[x as usize] |= mem.reg_v[y as usize];
}

pub fn and_vx_vy(mem: &mut State, x: u8, y: u8) {
    mem.reg_v[x as usize] &= mem.reg_v[y as usize];
}

pub fn xor_vx_vy(mem: &mut State, x: u8, y: u8) {
    mem.reg_v[x as usize] ^= mem.reg_v[y as usize];
}

pub fn add_vx_vy(mem: &mut State, x: u8, y: u8) {
    let (res, flag) = mem.reg_v[x as usize].overflowing_add(mem.reg_v[y as usize]);
    mem.reg_v[x as usize] = res;
    mem.reg_v[0xF] = flag as u8;
}

pub fn sub_vx_vy(mem: &mut State, x: u8, y: u8) {
    let (res, flag) = mem.reg_v[x as usize].overflowing_sub(mem.reg_v[y as usize]);
    mem.reg_v[x as usize] = res;
    mem.reg_v[0xF] = !flag as u8;
}

pub fn shr_vx_vy(mem: &mut State, x: u8) {
    mem.reg_v[0xF] = mem.reg_v[x as usize] & 0x01;
    mem.reg_v[x as usize] <<= 1;
}

pub fn subn_vx_vy(mem: &mut State, x: u8, y: u8) {
    let (res, flag) = mem.reg_v[y as usize].overflowing_sub(mem.reg_v[x as usize]);
    mem.reg_v[x as usize] = res;
    mem.reg_v[0xF] = !flag as u8;
}

pub fn shl_vx_vy(mem: &mut State, x: u8) {
    mem.reg_v[0xF] = ((mem.reg_v[x as usize] & 0x80) != 0) as u8;
    mem.reg_v[x as usize] >>= 1;
}

pub fn sne_vx_vy(mem: &mut State, x: u8, y: u8) {
    if mem.reg_v[x as usize] != mem.reg_v[y as usize] {
        mem.sp += 2;
    }
}

pub fn ld_i_addr(mem: &mut State, addr: u16) {
    mem.reg_i = addr;
}

pub fn jp_v_addr(mem: &mut State, addr: u16) {
    mem.pc = addr + mem.reg_v[0x0] as u16;
}