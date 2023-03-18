use super::{LexerError, LexerResult};
use super::{Token, TokenLiteral, TokenType};

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

    pub fn scan_tokens(&mut self) -> LexerResult<Vec<Token>> {
        let mut tokens = Vec::new();

        loop {
            if self.current >= self.source.len() {
                tokens.push(Token {
                    kind: TokenType::EndOfFile,
                    text: "".to_string(),
                    literal: None,
                });
                break;
            }
            if let Some(token) = self.scan_token()? {
                tokens.push(token);
            }
        }

        Ok(tokens)
    }

    pub fn scan_token(&mut self) -> LexerResult<Option<Token>> {
        self.start = self.current;

        if let Some(c) = self.advance() {
            match c {
                '(' => return self.make_token(TokenType::LeftParen),
                ')' => return self.make_token(TokenType::RightParen),
                '{' => return self.make_token(TokenType::LeftBrace),
                '}' => return self.make_token(TokenType::RightBrace),
                ',' => return self.make_token(TokenType::Comma),
                '+' => return self.make_token(TokenType::Plus),
                '-' => return self.make_token(TokenType::Minus),
                '*' => return self.make_token(TokenType::Star),
                ':' => return self.make_token(TokenType::Colon),
                '=' => {
                    let kind = self.match_next('=', TokenType::EqualEqual, TokenType::Equal);
                    return self.make_token(kind);
                }
                '!' => {
                    let kind = self.match_next('=', TokenType::BangEqual, TokenType::Bang);
                    return self.make_token(kind);
                }

                '%' => {
                    let literal = TokenLiteral::Number(self.scan_number(2)?);
                    return self.make_token_literal(TokenType::Number, literal);
                }
                '$' => {
                    let literal = TokenLiteral::Number(self.scan_number(16)?);
                    return self.make_token_literal(TokenType::Number, literal);
                }
                _ if c.is_ascii_digit() => {
                    let literal = TokenLiteral::Number(self.scan_number(10)?);
                    return self.make_token_literal(TokenType::Number, literal);
                }

                '.' => {
                    let identifier = self.scan_identifier()?;
                    let kind = TokenType::try_from(identifier.to_lowercase().as_str())
                        .map_err(|_err| LexerError::UnknownDirective(identifier))?;

                    return self.make_token(kind);
                }
                _ if c.is_alphabetic() => {
                    let kind: TokenType =
                        // check if the identifier is actually a reserved keyword
                        match self.scan_identifier()?.to_lowercase().as_str().try_into() {
                            Ok(kind) => kind,
                            Err(_) => TokenType::Identifier,
                        };
                    return self.make_token(kind);
                }

                ' ' | '\t' | '\r' => (),
                ';' => self.scan_comment(),
                '\n' => self.line += 1,

                n => return Err(LexerError::UnknownCharacter(self.line, self.current, n)),
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

    pub fn advance(&mut self) -> Option<char> {
        if self.current < self.source.len() {
            let next_char = self.source[self.current];
            self.current += 1;
            Some(next_char)
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

    pub fn make_token(&self, kind: TokenType) -> LexerResult<Option<Token>> {
        Ok(Some(Token {
            kind,
            text: self.source[self.start..self.current].iter().collect(),
            literal: None,
        }))
    }

    pub fn make_token_literal(
        &self,
        kind: TokenType,
        literal: TokenLiteral,
    ) -> LexerResult<Option<Token>> {
        Ok(Some(Token {
            kind,
            text: self.source[self.start..self.current].iter().collect(),
            literal: Some(literal),
        }))
    }

    fn scan_comment(&mut self) {
        while let Some(c) = self.advance() {
            if c == '\n' {
                return;
            }
        }
    }

    fn scan_identifier(&mut self) -> LexerResult<String> {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        Ok(self.source[self.start..self.current].iter().collect())
    }

    fn scan_number(&mut self, radix: u32) -> LexerResult<u64> {
        while self.peek().is_digit(radix) {
            self.advance();
        }

        // `scan_number` gets called at thee different places
        // if the radix is either `2` or `16`, `self.start`
        // points to either `%` or `$`. so if `self.start` is only
        // `1` char away from `self.current`, the loop above failed to
        // parse any valid digit in base `radix`.
        if self.start + 1 == self.current && radix != 10 {
            return Err(LexerError::NumberExpected);
        }

        // offset the `start` by `1` if the radix is not `10`.
        // essentially skips `%` and `$`.
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
            Ok(value) => Ok(value),
            Err(_err) => unreachable!(),
        }
    }
}
