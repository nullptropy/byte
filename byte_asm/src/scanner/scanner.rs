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
        self.skip_whitespace();
        self.cursor.sync();

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

                '\n' => {
                    let token = self.make_token(TokenKind::NewLine, None);
                    self.cursor.advance_line();
                    token
                }

                _ => todo!("not yet implemented :0"),
            },
        };

        Ok(token)
    }

    fn skip_whitespace(&mut self) {
        loop {
            if let Some(' ' | '\r' | '\t') = self.cursor.peek() {
                self.cursor.advance();
            } else {
                break;
            }
        }
    }

    fn make_token(&mut self, kind: TokenKind, value: Option<TokenValue>) -> Token {
        Token {
            kind,
            value,
            location: self.cursor.location(),
        }
    }
}
