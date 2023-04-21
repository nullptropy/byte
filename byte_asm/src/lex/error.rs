use thiserror::Error;

#[derive(Error, Debug)]
pub enum LexerError {
    #[error("[{line}:{column}] unknown assembler directive: {directive}")]
    UnknownDirective {
        line: usize,
        column: usize,
        directive: String,
    },
    #[error("[{line}:{column}] unknown character: {character}")]
    UnknownCharacter {
        line: usize,
        column: usize,
        character: char,
    },
    #[error("[{line}:{column}] no number is specified after number symbol: {symbol}")]
    NumberExpected {
        line: usize,
        column: usize,
        symbol: char,
    },
    #[error("[{line}:{column}] unterminated string quote")]
    UnterminatedString {
        line: usize,
        column: usize,
        quote: char,
    },
    #[error("{0}")]
    Generic(String),
}
