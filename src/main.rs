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

use env_logger;
use std::path::PathBuf;
use structopt::StructOpt;
use display::DisplayKind;

// TODO: Add a display selector?
#[derive(StructOpt, Debug)]
#[structopt(
    name = "chip-again",
    about = "Another CHIP-8 emulator, for the terminal, written with Rust."
)]
pub struct Opt {
    #[structopt(name = "rom", help = "Path to a chip8 compatible ROM file.")]
    rom_path: PathBuf,
    #[structopt(short = "f", default_value = "60", help = "Frames-per-second of the emulator.")]
    fps: u64,
    #[structopt(name = "display", default_value = "Terminal", help = "Display mode.")]
    display: DisplayKind,
}

fn main() {
    // For error, warn, info, debug and trace macros
    env_logger::init();

    let opt = Opt::from_args();

    if let Err(e) = exec::main_loop(opt) {
        eprintln!("An error occurred in execution.");
        eprintln!("{:?}", e);
    }
}
