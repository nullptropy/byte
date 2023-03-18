use thiserror::Error;

#[derive(Error, Debug)]
pub enum ScannerError {
    #[error("Unknown assembler directive: {0}")]
    UnknownDirective(String),
    #[error("Unknown character at line {0} index {1}: {2}")]
    UnknownCharacter(usize, usize, char),
    #[error("place-holder error value")]
    Unknown,
}