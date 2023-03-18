pub mod error;
pub mod token;
pub mod lexer;

pub use error::LexerError;
pub use lexer::Lexer;
pub use token::{Token, TokenType, TokenLiteral};

pub type LexerResult<T> = Result<T, error::LexerError>;