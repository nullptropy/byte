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
        // // Instruction,
        // // Semicolon,
        // // Comment,
        // // Directive,
        let token = match self.cursor.advance() {
            None => self.make_token(TokenKind::EOF, None),
            Some(c) => match c {
                ')' => self.make_token(TokenKind::CloseParen, None),
                ',' => self.make_token(TokenKind::Comma, None),
                ':' => self.make_token(TokenKind::Colon, None),
                '.' => self.make_token(TokenKind::Dot, None),
                '#' => self.make_token(TokenKind::Hash, None),
                '-' => self.make_token(TokenKind::Minus, None),
                '(' => self.make_token(TokenKind::OpenParen, None),
                '+' => self.make_token(TokenKind::Plus, None),
                '/' => self.make_token(TokenKind::Slash, None),
                '*' => self.make_token(TokenKind::Star, None),

                // hmmm, should this be considered as whitespace??
                '\n' => self.make_token(TokenKind::NewLine, None),

                _ => todo!("not yet implemented :0"),
            },
        };

        Ok(token)
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
