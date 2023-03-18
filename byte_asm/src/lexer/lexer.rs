// TODO: some unit tests would be nice

use crate::lexer::{Token, TokenType};

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

    // TODO: this needs to return an error
    pub fn scan_token(&mut self) -> Result<Option<Token>, ()> {
        if self.current >= self.source.len() {
            return Err(());
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

                // scan assembler directives
                '.' => return Ok(self.make_token(TokenType::Dot)),
                // scan binary number
                '%' => return Ok(self.make_token(TokenType::PercentSign)),
                // scan hex number
                '$' => return Ok(self.make_token(TokenType::DollarSign)),
                // scan a decimal number
                _ if c.is_digit(10) => {}
                // scan an identifier
                _ if c.is_alphabetic() => {}

                // there are a couple of different number representations that we would like to support
                // * #$0000    : hex format
                // * #6500     : decimal format
                // * #%00001000: binary format
                // it's not super clear if scanning the numbers should be done here
                _ => todo!(),
            }
        }

        Ok(None)
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
        if self.current < self.source.len() {
            if self.source[self.current] == next {
                self.current += 1;
                return on_true;
            }
        }

        on_false
    }

    // wip
    // TODO: attach more info to the `Token` struct
    // so that we can report errors with proper context
    pub fn make_token(&self, kind: TokenType) -> Option<Token> {
        Some(Token { kind })
    }

    fn scan_comment(&mut self) {
        // advance until either EOF or new line character
        while let Some(c) = self.advance() {
            if c == '\n' {
                return;
            }
        }
    }
}
