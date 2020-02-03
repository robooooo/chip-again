/// Includes the `Render` trait and several implementors.
pub mod display;
/// Main emulator logic, includes emulated opcodes.
pub mod emulator;
/// Error handling and ErrorKind enum
pub mod error;
/// Main program loop and input handling.
pub mod exec;
/// Utility and helpful functions.
pub mod utils;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "chip-again",
    about = "Yet another chip-8 emulator (because YAC8 was taken)"
)]
pub(crate) struct Opt {
    #[structopt(
        parse(from_os_str),
        name = "rom",
        help = "Path to a chip8 compatible ROM file."
    )]
    rom_path: PathBuf,
    // TODO: Add a display selector?
    fps: u64,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);

    if let Err(e) = exec::main_loop(opt) {
        eprintln!("An error occurred in execution.");
        eprintln!("{:?}", e);
    }
}
