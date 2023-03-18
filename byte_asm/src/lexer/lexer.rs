// TODO: some unit tests would be nice

use crate::error::ScannerError;
use crate::lexer::{Token, TokenType};
use crate::ScannerResult;

use super::token::TokenLiteral;

pub struct Lexer {
    source: Vec<char>,
    line: usize,
    start: usize,
    current: usize,
}

impl Lexer {
    pub fn new(code: String) -> Self {
        Self {
            source: code.chars().collect(),
            line: 1,
            start: 0,
            current: 0,
        }
    }

    pub fn scan_token(&mut self) -> ScannerResult<Option<Token>> {
        if self.current >= self.source.len() {
            return Err(ScannerError::Unknown);
        }
        self.start = self.current;

        if let Some(c) = self.advance() {
            match c {
                '(' => return Ok(self.make_token(TokenType::LeftParen)),
                ')' => return Ok(self.make_token(TokenType::RightParen)),
                '{' => return Ok(self.make_token(TokenType::LeftBrace)),
                '}' => return Ok(self.make_token(TokenType::RightBrace)),
                ',' => return Ok(self.make_token(TokenType::Comma)),
                '+' => return Ok(self.make_token(TokenType::Plus)),
                '-' => return Ok(self.make_token(TokenType::Minus)),
                '*' => return Ok(self.make_token(TokenType::Star)),
                ':' => return Ok(self.make_token(TokenType::Colon)),
                ';' => self.scan_comment(),

                '=' => {
                    let kind = self.match_next('=', TokenType::EqualEqual, TokenType::Equal);
                    return Ok(self.make_token(kind));
                }
                '!' => {
                    let kind = self.match_next('=', TokenType::BangEqual, TokenType::Bang);
                    return Ok(self.make_token(kind));
                }

                '\n' => self.line += 1,
                ' ' | '\t' | '\r' => (),

                '%' => {
                    let literal = TokenLiteral::Number(self.scan_number(2)?);
                    return Ok(self.make_token_literal(TokenType::Number, literal));
                }
                '$' => {
                    let literal = TokenLiteral::Number(self.scan_number(16)?);
                    return Ok(self.make_token_literal(TokenType::Number, literal));
                }
                _ if c.is_ascii_digit() => {
                    let literal = TokenLiteral::Number(self.scan_number(10)?);
                    return Ok(self.make_token_literal(TokenType::Number, literal));
                }

                '.' => {
                    let identifier = self.scan_identifier()?;
                    let kind: ScannerResult<TokenType> =
                        identifier.to_lowercase().as_str().try_into();

                    return Ok(self.make_token(
                        kind.map_err(|_err| ScannerError::UnknownDirective(identifier))?,
                    ));
                }
                _ if c.is_alphabetic() => {
                    let kind: TokenType =
                        // check if the identifier is actually a reserved keyword
                        match self.scan_identifier()?.to_lowercase().as_str().try_into() {
                            Ok(kind) => kind,
                            Err(_) => TokenType::Identifier,
                        };
                    return Ok(self.make_token(kind));
                }

                n => return Err(ScannerError::UnknownCharacter(self.line, self.current, n)),
            }
        }

        Ok(None)
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    // returns None on EOF
    pub fn advance(&mut self) -> Option<char> {
        if self.current < self.source.len() {
            let c = self.source[self.current];
            self.current += 1;
            Some(c)
        } else {
            None
        }
    }

    pub fn match_next(&mut self, next: char, on_true: TokenType, on_false: TokenType) -> TokenType {
        if self.current < self.source.len() && self.source[self.current] == next {
            self.current += 1;
            return on_true;
        }

        on_false
    }

    pub fn make_token(&self, kind: TokenType) -> Option<Token> {
        Some(Token {
            kind,
            text: self.source[self.start..self.current].iter().collect(),
            literal: None,
        })
    }

    pub fn make_token_literal(&self, kind: TokenType, literal: TokenLiteral) -> Option<Token> {
        Some(Token {
            kind,
            text: self.source[self.start..self.current].iter().collect(),
            literal: Some(literal),
        })
    }

    fn scan_comment(&mut self) {
        // advance until either EOF or new line character
        while let Some(c) = self.advance() {
            if c == '\n' {
                return;
            }
        }
    }

    fn scan_identifier(&mut self) -> ScannerResult<String> {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        Ok(self.source[self.start..self.current].iter().collect())
    }

    fn scan_number(&mut self, radix: u32) -> ScannerResult<u64> {
        while self.peek().is_digit(radix) {
            self.advance();
        }

        if self.start + 1 == self.current && radix != 10 {
            return Err(ScannerError::NumberExpected);
        }

        let start = if radix == 10 {
            self.start
        } else {
            self.start + 1
        };

        match u64::from_str_radix(
            self.source[start..self.current]
                .iter()
                .collect::<String>()
                .as_str(),
            radix,
        ) {
            Ok(data) => Ok(data),
            Err(why) => unreachable!(),
        }
    }
}
