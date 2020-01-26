pub mod display;
pub mod emulator;
pub mod utils;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "chip-again",
    about = "Yet another chip-8 emulator (because YAC8 was taken)"
)]
struct Opt {
    #[structopt(
        parse(from_os_str),
        name = "rom",
        help = "Path to a chip8 compatible ROM file."
    )]
    input: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
