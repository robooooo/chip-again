use crossterm::ErrorKind as TermErrorKind;
use std::io::Error as IoError;
use thiserror::*;
/// This enumeration represents all the errors that can occur throughout the program.
/// They should be bubbled up and then handled in main.rs
#[derive(Debug, Error)]
pub enum ErrorKind {
    #[error("Error in terminal: {0:?}")]
    TermErr(#[from] TermErrorKind),
    #[error("Error with files: {0:?}")]
    IoErr(#[from] IoError),
}

#[derive(Debug, Error)]
#[error("{:?}", _0)]
pub struct DisplayKindError(pub String);