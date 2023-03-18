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

    pub fn scan_token(&mut self) -> Option<Token> {
        self.start = self.current;
        if let Some(c) = self.advance() {
            match c {
                '=' => self.make_token(TokenType::Equal),
                '!' => self.make_token(TokenType::Bang),
                _ => todo!(),
            }
        } else {
            None
        }
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

    pub fn match_next(&mut self, next: char) -> bool {
        if self.current < self.code.len() {
            if self.code[self.current] == next {
                self.current += 1;
                return true;
            }
        }

        false
    }

    // wip
    pub fn make_token(&self, kind: TokenType) -> Option<Token> {
        Some(Token { kind })
    }
}
