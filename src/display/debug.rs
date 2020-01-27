use crate::{
    display::{Infallible, Render},
    emulator::State,
};

/// A simple renderer that repeatedly prints the output to stdout. Included for debugging.
struct DebugRenderer;

impl Render for DebugRenderer {
    type Err = Infallible;

    fn render(display: [bool; 2048]) -> Result<(), Infallible> {
        for line in display.chunks_exact(State::WIDTH) {
            for b in line {
                print!("{}", if *b { '█' } else { '░' });
            }
            println!();
        }

        Ok(())
    }
}
