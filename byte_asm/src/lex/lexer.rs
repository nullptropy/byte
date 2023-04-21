use super::{LexerError, LexerResult};
use super::{Token, TokenLiteral, TokenType};

pub struct Lexer {
    column: usize,
    current: usize,
    line: usize,
    start: usize,
    source: Vec<char>,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            column: 0,
            current: 0,
            line: 1,
            start: 0,
            source: source.chars().collect(),
        }
    }

    pub fn scan_token(&mut self) -> LexerResult<Token> {
        self.skip_whitespace();
        self.start = self.current;

        let token = match self.advance() {
            None => self.make_token(TokenType::EndOfFile, None),
            Some(c) => match c {
                '(' => self.make_token(TokenType::LeftParen, None),
                ')' => self.make_token(TokenType::RightParen, None),
                '{' => self.make_token(TokenType::LeftBrace, None),
                '}' => self.make_token(TokenType::RightBrace, None),
                ',' => self.make_token(TokenType::Comma, None),
                '+' => self.make_token(TokenType::Plus, None),
                '-' => self.make_token(TokenType::Minus, None),
                '*' => self.make_token(TokenType::Star, None),
                ':' => self.make_token(TokenType::Colon, None),

                '=' => {
                    let kind = self.match_next('=', TokenType::EqualEqual, TokenType::Equal);
                    self.make_token(kind, None)
                }
                '!' => {
                    let kind = self.match_next('=', TokenType::BangEqual, TokenType::Bang);
                    self.make_token(kind, None)
                }

                '%' => {
                    let literal = Some(TokenLiteral::Number(self.scan_number(2)?));
                    self.make_token(TokenType::Number, literal)
                }
                '$' => {
                    let literal = Some(TokenLiteral::Number(self.scan_number(16)?));
                    self.make_token(TokenType::Number, literal)
                }
                _ if c.is_ascii_digit() => {
                    let literal = Some(TokenLiteral::Number(self.scan_number(10)?));
                    self.make_token(TokenType::Number, literal)
                }

                c if c == '\'' || c == '"' => {
                    let string = self.scan_string(c)?;
                    self.make_token(TokenType::String, Some(TokenLiteral::String(string)))
                }

                '.' => {
                    let identifier = self.scan_identifier()?;
                    let kind =
                        TokenType::try_from(&identifier.to_lowercase()[1..]).map_err(|_| {
                            LexerError::UnknownDirective {
                                line: self.line,
                                column: self.column,
                                directive: identifier,
                            }
                        })?;

                    self.make_token(kind, None)
                }
                _ if c.is_alphabetic() => {
                    let identifier = self.scan_identifier()?;
                    let kind = TokenType::try_from(identifier.to_lowercase().as_str())
                        .unwrap_or(TokenType::Identifier);

                    self.make_token(kind, None)
                }

                ';' => {
                    self.scan_comment();
                    self.make_token(TokenType::Comment, None)
                }

                n => {
                    return Err(LexerError::UnknownCharacter {
                        line: self.line,
                        column: self.column,
                        character: n,
                    })
                }
            },
        };

        Ok(token)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            None
        } else {
            Some(self.source[self.current])
        }
    }

    fn advance(&mut self) -> Option<char> {
        if self.is_at_end() {
            None
        } else {
            self.current += 1;
            self.column += 1;
            Some(self.source[self.current - 1])
        }
    }

    fn match_next(&mut self, next: char, on_true: TokenType, on_false: TokenType) -> TokenType {
        match self.peek() {
            Some(c) if c == next => {
                self.advance();
                on_true
            }
            _ => on_false,
        }
    }

    fn make_token(&self, kind: TokenType, literal: Option<TokenLiteral>) -> Token {
        Token {
            kind,
            literal,
            line: self.line,
            column: self.column,
            text: self.source[self.start..self.current].iter().collect(),
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.peek().unwrap_or('\0') {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.advance();
                    self.line += 1;
                    self.column = 0;
                }
                _ => break,
            }
        }
    }

    fn scan_comment(&mut self) {
        while let Some(c) = self.peek() {
            if c == '\n' {
                break;
            }

            self.advance();
        }
    }

    fn scan_string(&mut self, quote: char) -> LexerResult<String> {
        let mut string = String::new();

        while let Some(c) = self.peek() {
            if c == quote || c == '\n' {
                break;
            }
            self.advance();

            // if c isn't a `\\` just push it
            // to `string` and continue processing
            if c != '\\' {
                string.push(c);
                continue;
            }

            // if c is a `\\` we potentially have to decode
            // an escape sequence
            match self.peek() {
                Some('n') => string.push('\n'),
                Some('r') => string.push('\r'),
                Some('t') => string.push('\t'),
                Some('"') => string.push('"'),
                Some('\'') => string.push('\''),
                Some('\\') => string.push('\\'),
                // if the char after `\\` isn't recognized,
                // just push `e` into the string
                Some(e) => string.push(e),
                // don't call `advance` in this case
                None => continue,
            }

            self.advance();
        }

        if let None | Some('\n') = self.peek() {
            Err(LexerError::UnterminatedString {
                line: self.line,
                column: self.column,
                quote,
            })
        } else {
            // consume the second quote
            self.advance();
            Ok(string)
        }
    }

    fn scan_identifier(&mut self) -> LexerResult<String> {
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }

        Ok(self.source[self.start..self.current].iter().collect())
    }

    fn scan_number(&mut self, radix: u32) -> LexerResult<u64> {
        while let Some(c) = self.peek() {
            if c.is_digit(radix) {
                self.advance();
            } else {
                break;
            }
        }

        // `scan_number` gets called at three different places
        // if the radix is either `2` or `16`, `self.start`
        // points to either `%` or `$`. so if `self.start` is only
        // `1` char away from `self.current`, the loop above failed to
        // parse any valid digit in base `radix`.
        if self.start + 1 == self.current && radix != 10 {
            return Err(LexerError::NumberExpected {
                line: self.line,
                column: self.column,
                symbol: if radix == 16 { '$' } else { '%' },
            });
        }

        // offset the `start` by `1` if the radix is not `10`.
        // essentially skips `%` and `$`.
        match u64::from_str_radix(
            self.source[self.start + if radix == 10 { 0 } else { 1 }..self.current]
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
