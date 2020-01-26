/// Contains the standard font for the CHIP-8 system.
pub mod fontset;
/// Implementation of some of the more complex opcodes. Called mostly from state.
pub mod opcodes;
/// Contains the State type which describes the current state of the interpreter.
pub mod state;

pub use state::State;
