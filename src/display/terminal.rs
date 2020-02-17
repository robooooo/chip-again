use crate::{display::Render, emulator::State, error::ErrorKind};
use crossterm::{
    cursor,
    style::{self, Color},
    terminal::{self, ClearType},
    QueueableCommand,
};
use itertools::Itertools;
use std::{
    cmp::Ordering,
    io::{stdout, Write},
};

/// The default, most fully-featured renderer.
pub struct TerminalRenderer {
    prev: [bool; 2048],
}

impl TerminalRenderer {
    /// Create a new TerminalRenderer, clearing the screen.
    pub fn new() -> Result<TerminalRenderer, ErrorKind> {
        let mut stdout = stdout();
        terminal::enable_raw_mode()?;
        stdout
            .queue(cursor::Hide)?
            .queue(terminal::Clear(ClearType::All))?;

        // Screen starts as blank, but let's set it to be all-white to render blank screen
        let mut res = TerminalRenderer {
            prev: [true; 2048],
        };

        // Let's render an all-blank screen first
        res.render([false; 2048])?;
        Ok(res)
    }
}

impl Render for TerminalRenderer {
    fn render(&mut self, screen: [bool; 2048]) -> Result<(), ErrorKind> {
        // Skip rendering if nothing has changed
        if self.prev.cmp(&screen) == Ordering::Equal {
            return Ok(());
        }

        let mut stdout = stdout();
        // Do line-by-line rendering
        for ((y, line), old_line) in screen
            .chunks_exact(State::WIDTH)
            .enumerate()
            .zip(self.prev.chunks_exact(State::WIDTH))
        {
            // Skip per-line rendering if nothing has changed
            if line.cmp(&old_line) == Ordering::Equal {
                continue;
            }

            stdout.queue(cursor::MoveTo(0, y as u16))?;
            // RLE to avoid colour switching when possible
            // group_by(id) for booleans will group equal elements
            for (key, group) in line.into_iter().group_by(|x| *x).into_iter() {
                stdout.queue(style::SetBackgroundColor(if *key {
                    Color::White
                } else {
                    Color::Black
                }))?;
                for _ in group {
                    write!(stdout, " ")?;
                }
            }
        }

        stdout.flush()?;
        self.prev = screen;
        Ok(())
    }
}

impl Drop for TerminalRenderer {
    fn drop(&mut self) {
        // Implicitly ignore errors here (can't return them)
        let mut stdout = stdout();
        let _ = terminal::disable_raw_mode();
        let _ = stdout.queue(style::ResetColor);
        let _ = stdout.queue(terminal::Clear(ClearType::All));
        let _ = stdout.queue(cursor::MoveTo(0, 0));
        let _ = stdout.queue(cursor::Show);
        let _ = stdout.flush();
    }
}
