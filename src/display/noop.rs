use crate::{display::Render, error::ErrorKind};

/// A simple no-operation renderer. This is useful for testing game logic, without needing to draw
/// output to the screen.
pub struct DummyRenderer;

impl Render for DummyRenderer {
    fn render(&mut self, _display: [bool; 2048]) -> Result<(), ErrorKind> {
        Ok(())
    }
}
