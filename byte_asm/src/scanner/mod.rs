mod cursor;
pub mod error;
pub mod scan;
pub mod token;

pub use error::ScannerError;
pub use scan::Scanner;
pub use token::{Directive, Location, Token, TokenKind, TokenValue};

pub type ScannerResult<T> = std::result::Result<T, ScannerError>;
