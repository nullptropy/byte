// TODO: some unit tests would be nice

use crate::lexer::{Token, TokenType};

pub struct Lexer {
    code: Vec<char>,
    start: usize,
    current: usize,
}

impl Lexer {
    pub fn new(code: String) -> Self {
        Self {
            code: code.chars().collect(),
            start: 0,
            current: 0,
        }
    }

    // TODO: this needs to return an error
    pub fn scan_token(&mut self) -> Result<Option<Token>, ()> {
        if self.current >= self.code.len() {
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
                '.' => return Ok(self.make_token(TokenType::Dot)),
                '+' => return Ok(self.make_token(TokenType::Plus)),
                '-' => return Ok(self.make_token(TokenType::Minus)),
                '*' => return Ok(self.make_token(TokenType::Star)),
                '$' => return Ok(self.make_token(TokenType::DollarSign)),

                '=' => {
                    let kind = self.match_next('=', TokenType::EqualEqual, TokenType::Equal);
                    return Ok(self.make_token(kind));
                }
                '!' => {
                    let kind = self.match_next('=', TokenType::BangEqual, TokenType::Bang);
                    return Ok(self.make_token(kind));
                }

                ';' => self.scan_comment(),

                _ => todo!(),
            }
        }

        Ok(None)
    }

    // returns None on EOF
    pub fn advance(&mut self) -> Option<char> {
        if self.current < self.code.len() {
            let c = self.code[self.current];
            self.current += 1;
            Some(c)
        } else {
            None
        }
    }

    pub fn match_next(&mut self, next: char, on_true: TokenType, on_false: TokenType) -> TokenType {
        if self.current < self.code.len() {
            if self.code[self.current] == next {
                self.current += 1;
                return on_true;
            }
        }

        on_false
    }

    // wip
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
