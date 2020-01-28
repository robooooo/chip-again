/// A simple renderer that repeatedly prints the output to stdout. Included for debugging.
pub mod debug;
/// A simple no-operation renderer.
pub mod noop;

/// A simple renderer that repeatedly prints the output to stdout. Included for debugging.
pub use debug::DebugRenderer;
/// A simple no-operation renderer.
pub use noop::DummyRenderer;

/// The `Render` trait describes types which implement some kind of rendering protocol. It
/// exposes the method `Render::render`, which takes the current state of the games display and
/// attempts to render it to the given output. It also has an associated type `Render::Err`,
/// and a value of `Result::<Render::Err>::Err` will be returned when the renderer has failed
/// to render.
pub(crate) trait Render {
    /// Type that represents all errors that can occur in the `Render::render` function.
    type Err;

    fn render(display: [bool; 2048]) -> Result<(), Self::Err>;
}

/// This no-variant enumeration is used to tell the compiler that this type of renderer can't fail.
pub enum Infallible {}
