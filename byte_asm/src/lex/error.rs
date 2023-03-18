use thiserror::Error;

#[derive(Error, Debug)]
pub enum LexerError {
    #[error("Unknown assembler directive: {0}")]
    UnknownDirective(String),
    #[error("Unknown character at line {0} index {1}: {2}")]
    UnknownCharacter(usize, usize, char),
    #[error("No number is specified after hex or binary number symbol")]
    NumberExpected,
    #[error("place-holder error value")]
    Unknown,
}