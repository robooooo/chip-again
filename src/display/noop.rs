use crate::display::Render;

/// This no-variant enumeration is used to tell the compiler that this type of renderer can't fail.
pub enum Infallible;

/// A simple no-operation renderer. This is useful for testing game logic, without needing to draw
/// output to the screen.
struct DummyRenderer;

impl Render for DummyRenderer {
    type Err = Infallible;

    fn render(_display: [bool; 2048]) -> Result<(), Self::Err> {
        Ok(())
    }
}
