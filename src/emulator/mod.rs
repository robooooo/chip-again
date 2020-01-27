/// Contains the standard font for the CHIP-8 system.
pub mod fontset;
/// Contains the Input type which provides values for each of the 16 keys as well as input handlers.
pub mod input;
/// Implementation of some of the more complex opcodes. Called mostly from state.
pub mod opcodes;
/// Contains the State type which describes the current state of the interpreter.
pub mod state;

pub use state::State;
