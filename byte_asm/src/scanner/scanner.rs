use super::cursor::Cursor;
use super::{Location, Token, TokenKind, TokenValue};

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

    // implement errors
    pub fn scan_token(&mut self) -> Option<Token> {
        Some(self.make_token(TokenKind::EOF, None))
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
