use crate::error::{DisplayKindError, ErrorKind};
use std::str::FromStr;

/// A simple renderer that repeatedly prints the output to stdout. Included for debugging.
pub mod debug;
/// A simple no-operation renderer.
pub mod noop;
/// The default, most fully-featured renderer.
pub mod terminal;

/// A simple renderer that repeatedly prints the output to stdout. Included for debugging.
pub use debug::DebugRenderer;
/// A simple no-operation renderer.
pub use noop::DummyRenderer;
/// The default, most fully-featured renderer.
pub use terminal::TerminalRenderer;

/// The `Render` trait describes types which implement some kind of rendering protocol. It
/// exposes the method `Render::render`, which takes the current state of the games display and
/// attempts to render it to the given output. It also has an associated type `Render::Err`,
/// and a value of `Result::<Render::Err>::Err` will be returned when the renderer has failed
/// to render.
pub trait Render {
    fn render(&mut self, display: [bool; 2048]) -> Result<(), crate::error::ErrorKind>;
}

/// Represents the avaliable choices of display mode.
#[derive(Copy, Clone, Debug)]
pub enum DisplayKind {
    Noop,
    Debug,
    Terminal,
}

impl DisplayKind {
    pub fn to_renderer(&self) -> Result<Box<dyn Render>, ErrorKind> {
        Ok(match *self {
            Self::Noop => Box::new(DummyRenderer),
            Self::Debug => Box::new(DebugRenderer::new()),
            Self::Terminal => Box::new(TerminalRenderer::new()?),
        })
    }
}

impl FromStr for DisplayKind {
    type Err = DisplayKindError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use DisplayKind::*;
        Ok(match s.trim().to_lowercase().as_str() {
            "noop" => Noop,
            "debug" => Debug,
            "terminal" => Terminal,
            _ => return Err(DisplayKindError(s.to_owned())),
        })
    }
}
