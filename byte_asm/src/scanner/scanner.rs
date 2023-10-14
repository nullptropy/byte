use super::cursor::Cursor;
use super::{Location, Token, TokenKind, TokenValue};
use super::{ScannerError, ScannerResult};

pub struct Scanner<'a> {
    cursor: Cursor<'a>,
    source: &'a str,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            cursor: Cursor::new(source),
            source,
        }
    }

    pub fn scan_token(&mut self) -> ScannerResult<Token> {
        Ok(self.make_token(TokenKind::EOF, None))
    }

    pub fn make_token(&mut self, kind: TokenKind, value: Option<TokenValue>) -> Token {
        let token = Token {
            kind,
            value,
            location: self.cursor.location(),
        };
        self.cursor.sync();

        token
    }
}
