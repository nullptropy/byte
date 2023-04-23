pub mod error;
pub mod lexer;
pub mod token;

pub use error::LexerError;
pub use lexer::Lexer;
pub use token::{Location, Token, TokenLiteral, TokenType};

pub type LexerResult<T> = Result<T, error::LexerError>;
