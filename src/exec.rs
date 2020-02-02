use crate::{
    display::{self, Render},
    emulator::{self, State},
    Opt,
};
use crossterm::event::{self, KeyCode, KeyEvent};
use std::{fs::File, io::prelude::*, time::Duration};
// TODO: Replace Box<dyn Error>
/// Main loop, we want to take parsed command line input from main and run the emulator in a loop.
/// It is also our responsibility to handle input, and pass the display state to an instance of
/// `Render`, which we do here.
pub(crate) fn main_loop(options: Opt) -> Result<(), Box<dyn std::error::Error>> {
    // ROM size, 2048 bytes of memory, 0x200 of which reserved for interpreter.
    const ROM_SIZE: usize = 2048 - 0x200;

    let mut buf = Vec::with_capacity(2048 - 0x200);
    let mut handle = File::open(options.rom_path)?;
    handle.read_to_end(&mut buf);

    let mut cpu = State::new(&buf[..ROM_SIZE]);
    // TODO: Allow changing, choosing renderer.
    let disp = Box::new(display::DebugRenderer);

    loop {
        let mut input = Default::default();
        //        if event::poll(Duration::from_millis(1000 / options.fps))? {
        //            //            match event::read()?.code {
        //            match unimplemented!() {
        //                KeyCode::Char(' ') => todo!(),
        //                _ => todo!(),
        //            }
        //        }

        cpu.step(input);
        display::DebugRenderer::render(cpu.display);
    }

    Ok(())
}
