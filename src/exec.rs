use crate::{
    display::{self, Render},
    emulator::{input::Input, State},
    error::ErrorKind,
    Opt,
};
use crossterm::event::{self, Event::Key, KeyCode};
use std::{fs::File, io::prelude::*, time::Duration};

const KEYMAP: [char; 16] = [
    '1', '2', '3', '4', 'q', 'w', 'e', 'r', 'a', 's', 'd', 'f', 'z', 'x', 'c', 'v',
];

/// Main loop, we want to take parsed command line input from main and run the emulator in a loop.
/// It is also our responsibility to handle input, and pass the display state to an instance of
/// `Render`, which we do here.
pub(crate) fn main_loop(options: Opt) -> Result<(), ErrorKind> {
    // ROM size, 2048 bytes of memory, 0x200 of which reserved for interpreter.
    // const ROM_SIZE: usize = 2048 - 0x200;

    let mut buf = Vec::with_capacity(2048 - 0x200);
    let mut handle = File::open(options.rom_path)?;
    handle.read_to_end(&mut buf)?;

    let mut cpu = State::new(&buf);
    // TODO: Allow changing, choosing renderer.
    let mut disp = Box::new(display::TerminalRenderer::new()?);
    // let mut disp = Box::new(display::debug::DebugRenderer([true; 2048]));

    loop {
        let mut input: Input = Default::default();
        if event::poll(Duration::from_millis(1000 / options.fps))? {
            if let Key(key_event) = event::read()? {
                for (idx, &key) in KEYMAP.into_iter().enumerate() {
                    if let KeyCode::Char(pressed) = key_event.code {
                        if key == pressed {
                            input[idx] = true;
                        }
                    }
                }
                if let KeyCode::Char('0') = key_event.code {
                    unimplemented!()
                }
            }
        }

        cpu.step(input);
        disp.render(cpu.display)?;
    }
}
