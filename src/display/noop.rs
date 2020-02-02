use crate::display::{Infallible, Render};

/// A simple no-operation renderer. This is useful for testing game logic, without needing to draw
/// output to the screen.
pub struct DummyRenderer;

impl Render for DummyRenderer {
    type Err = Infallible;

    fn render(_display: [bool; 2048]) -> Result<(), Self::Err> {
        Ok(())
    }
}
