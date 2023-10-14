mod cursor;
pub mod error;
pub mod scanner;
pub mod token;

pub use error::ScannerError;
pub use scanner::Scanner;
pub use token::{Directive, Location, Token, TokenKind, TokenValue};

pub type ScannerResult<T> = std::result::Result<T, ScannerError>;
