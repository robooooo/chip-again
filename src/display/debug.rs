use crate::{display::Render, emulator::State, error::ErrorKind};
use std::cmp::Ordering;

/// A simple renderer that repeatedly prints the output to stdout. Included for debugging.
pub struct DebugRenderer(pub [bool; 2048]);

impl Render for DebugRenderer {
    fn render(&mut self, display: [bool; 2048]) -> Result<(), ErrorKind> {
        // Do not render repeated frames
        if display.cmp(&self.0) == Ordering::Equal {
            return Ok(());
        } else {
            self.0 = display;
        }

        for line in display.chunks_exact(State::WIDTH) {
            for b in line {
                print!("{}", if *b { '█' } else { '░' });
            }
            println!();
        }

        Ok(())
    }
}
