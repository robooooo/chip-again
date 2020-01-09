pub mod emulator;
pub mod opcodes;

use structopt::StructOpt;
use std::{
    path::PathBuf,
};

#[derive(StructOpt, Debug)]
#[structopt(name = "chip-again", about = "Yet another chip-8 emulator (because YAC8 was taken)")]
struct Opt {
    #[structopt(parse(from_os_str), name = "rom", help = "Path to a chip8 compatible ROM file.")]
    input: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
