use std::iter::Peekable;
use std::str::Chars;

use super::{LexerError, LexerResult};
use super::{Location, Token, TokenLiteral, TokenType};

pub struct Lexer<'a> {
    column: usize,
    current: usize,
    line: usize,
    start: usize,
    source: &'a str,
    chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            column: 0,
            current: 0,
            line: 1,
            start: 0,
            source,
            chars: source.chars().peekable(),
        }
    }

    pub fn scan_token(&mut self) -> LexerResult<Token> {
        self.skip_whitespace();
        self.start = self.current;

        let token = match self.advance() {
            None => self.make_token(TokenType::EndOfFile, TokenLiteral::None),
            Some(c) => match c {
                '(' => self.make_token(TokenType::LeftParen, TokenLiteral::None),
                ')' => self.make_token(TokenType::RightParen, TokenLiteral::None),
                '{' => self.make_token(TokenType::LeftBrace, TokenLiteral::None),
                '}' => self.make_token(TokenType::RightBrace, TokenLiteral::None),
                ',' => self.make_token(TokenType::Comma, TokenLiteral::None),
                '+' => self.make_token(TokenType::Plus, TokenLiteral::None),
                '-' => self.make_token(TokenType::Minus, TokenLiteral::None),
                '*' => self.make_token(TokenType::Star, TokenLiteral::None),
                '#' => self.make_token(TokenType::Hash, TokenLiteral::None),
                ':' => self.make_token(TokenType::Colon, TokenLiteral::None),

                '=' => {
                    let kind = self.match_next('=', TokenType::EqualEqual, TokenType::Equal);
                    self.make_token(kind, TokenLiteral::None)
                }
                '!' => {
                    let kind = self.match_next('=', TokenType::BangEqual, TokenType::Bang);
                    self.make_token(kind, TokenLiteral::None)
                }

                '%' => {
                    let literal = TokenLiteral::Number(self.scan_number(2, 1)?);
                    self.make_token(TokenType::Number, literal)
                }
                '$' => {
                    let literal = TokenLiteral::Number(self.scan_number(16, 1)?);
                    self.make_token(TokenType::Number, literal)
                }
                _ if c.is_ascii_digit() => {
                    let literal = TokenLiteral::Number(self.scan_number(10, 0)?);
                    self.make_token(TokenType::Number, literal)
                }

                c if c == '\'' || c == '"' => {
                    let string = self.scan_string(c)?;
                    self.make_token(TokenType::String, TokenLiteral::String(string))
                }

                '.' => {
                    let identifier = self.scan_identifier()?.to_lowercase();
                    let kind = TokenType::try_from(&identifier[1..]).map_err(|_| {
                        LexerError::UnknownDirective {
                            line: self.line,
                            column: self.column,
                            directive: identifier.to_owned(),
                        }
                    })?;

                    self.make_token(kind, TokenLiteral::None)
                }
                _ if c.is_alphabetic() => {
                    // if the next char is a `:` then we return a Label token
                    let identifier = self.scan_identifier()?.to_lowercase();
                    let kind =
                        TokenType::try_from(identifier.as_str()).unwrap_or(TokenType::Identifier);

                    // i might rever this change
                    if kind == TokenType::Identifier && self.peek() == Some(':') {
                        self.advance();
                        self.make_token(TokenType::Label, TokenLiteral::None)
                    } else {
                        self.make_token(kind, TokenLiteral::None)
                    }
                }

                ';' => {
                    self.scan_comment();
                    self.make_token(TokenType::Comment, TokenLiteral::None)
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

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    fn advance(&mut self) -> Option<char> {
        if self.is_at_end() {
            None
        } else {
            self.current += 1;
            self.column += 1;
            self.chars.next()
        }
    }

    fn match_next(&mut self, next: char, on_true: TokenType, on_false: TokenType) -> TokenType {
        (self.peek() == Some(next))
            .then(|| {
                self.advance();
                on_true
            })
            .unwrap_or(on_false)
    }

    fn make_token(&self, kind: TokenType, literal: TokenLiteral) -> Token {
        let location = Location {
            start: self.start,
            end: self.current,
            line: self.line,
            column: self.column,
        };

        Token {
            kind,
            literal,
            location,
            text: &self.source[self.start..self.current],
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.peek() {
                Some(' ' | '\r' | '\t') => {
                    self.advance();
                }
                Some('\n') => {
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

    fn scan_identifier(&mut self) -> LexerResult<&str> {
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }

        Ok(&self.source[self.start..self.current])
    }

    fn scan_number(&mut self, radix: u32, start_offset: usize) -> LexerResult<u64> {
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
        if self.current - self.start == 1 && radix != 10 {
            return Err(LexerError::NumberExpected {
                line: self.line,
                column: self.column,
                symbol: if radix == 16 { '$' } else { '%' },
            });
        }

        // offset the `start` by `1` if the radix is not `10`.
        // essentially skips `%` and `$`.
        u64::from_str_radix(&self.source[self.start + start_offset..self.current], radix)
            // this should be unreachable
            .map_err(|why| LexerError::Generic(why.to_string()))
    }
}
