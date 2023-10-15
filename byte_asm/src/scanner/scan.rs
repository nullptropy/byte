use byte_common::opcode::Mnemonic;

use super::cursor::Cursor;
use super::{Directive, Token, TokenKind, TokenValue};
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

        let token = match self.cursor.advance() {
            None => self.make_token(TokenKind::EOF, None),
            Some(c) => match c {
                ')' => self.make_token(TokenKind::CloseParen, None),
                ',' => self.make_token(TokenKind::Comma, None),
                ':' => self.make_token(TokenKind::Colon, None),
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

                '%' => {
                    let value = TokenValue::Number(self.scan_number(2, 1)?);
                    self.make_token(TokenKind::Number, Some(value))
                }
                '$' => {
                    let value = TokenValue::Number(self.scan_number(16, 1)?);
                    self.make_token(TokenKind::Number, Some(value))
                }
                _ if c.is_ascii_digit() => {
                    let value = TokenValue::Number(self.scan_number(10, 0)?);
                    self.make_token(TokenKind::Number, Some(value))
                }

                ';' => {
                    self.scan_comment();
                    self.make_token(TokenKind::Comment, None)
                }

                c if c == '\'' || c == '"' => {
                    let string = TokenValue::String(self.scan_string(c)?);
                    self.make_token(TokenKind::String, Some(string))
                }

                '.' => {
                    let identifier = self.scan_identifier()?.to_lowercase();
                    let directive = Directive::try_from(identifier[1..].to_uppercase().as_str())
                        .map_err(|_| ScannerError::UnknownDirective {
                            line: self.cursor.line,
                            column: self.cursor.column,
                            directive: identifier.to_owned(),
                        })?;

                    self.make_token(TokenKind::Directive, Some(TokenValue::Directive(directive)))
                }

                _ if c.is_alphabetic() => {
                    let identifier = self.scan_identifier()?.to_uppercase();

                    match Mnemonic::try_from(identifier.as_str()) {
                        Ok(mnemonic) => self.make_token(
                            TokenKind::Instruction,
                            Some(TokenValue::Instruction(mnemonic)),
                        ),
                        Err(_) => self.make_token(TokenKind::Identifier, None),
                    }
                }

                n => {
                    return Err(ScannerError::UnknownCharacter {
                        line: self.cursor.line,
                        column: self.cursor.column,
                        character: n,
                    })
                }
            },
        };

        Ok(token)
    }

    fn make_token(&mut self, kind: TokenKind, value: Option<TokenValue>) -> Token {
        Token {
            kind,
            value,
            location: self.cursor.location(),
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(' ' | '\r' | '\t') = self.cursor.peek() {
            self.cursor.advance();
        }
    }

    fn scan_comment(&mut self) {
        while let Some(c) = self.cursor.peek() {
            if c == '\n' {
                break;
            }

            self.cursor.advance();
        }
    }

    fn scan_identifier(&mut self) -> ScannerResult<&str> {
        while let Some(c) = self.cursor.peek() {
            if c.is_ascii_alphanumeric() || c == '_' {
                self.cursor.advance();
            } else {
                break;
            }
        }

        Ok(&self.source[self.cursor.start..self.cursor.current])
    }

    fn scan_string(&mut self, quote: char) -> ScannerResult<String> {
        let mut string = String::new();

        while let Some(c) = self.cursor.peek() {
            if c == quote || c == '\n' {
                break;
            }
            self.cursor.advance();

            // if c isn't a `\\` just push it
            // to `string` and continue processing
            if c != '\\' {
                string.push(c);
                continue;
            }

            // if c is a `\\` we potentially have to decode
            // an escape sequence
            match self.cursor.peek() {
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

            self.cursor.advance();
        }

        if let None | Some('\n') = self.cursor.peek() {
            Err(ScannerError::UnterminatedString {
                line: self.cursor.line,
                column: self.cursor.column,
                quote,
            })
        } else {
            // consume the second quote
            self.cursor.advance();
            Ok(string)
        }
    }

    fn scan_number(&mut self, radix: u32, start_offset: usize) -> ScannerResult<u64> {
        while let Some(c) = self.cursor.peek() {
            if c.is_digit(radix) {
                self.cursor.advance();
            } else {
                break;
            }
        }

        // `scan_number` is called at three different places.
        // --
        // if the radix is either `2` or `16`, `cursor.start`
        // points to either `%` or `$`. so if `cursor.start` is only
        // `1` char away from `cursor.current`, the loop above failed to
        // parse any valid digit in base `radix`.
        if self.cursor.current - self.cursor.start == 1 && radix != 10 {
            return Err(ScannerError::NumberExpected {
                line: self.cursor.line,
                column: self.cursor.column,
                symbol: if radix == 16 { '$' } else { '%' },
            });
        }

        // offset the `start` by `1` if the radix is not `10`.
        // essentially skips `%` and `$`.
        u64::from_str_radix(
            &self.source[self.cursor.start + start_offset..self.cursor.current],
            radix,
        )
        // this should be unreachable
        .map_err(|why| ScannerError::Generic(why.to_string()))
    }
}
