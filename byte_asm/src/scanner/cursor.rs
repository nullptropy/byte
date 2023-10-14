use super::Location;
use std::{iter::Peekable, str::Chars};

pub struct Cursor<'a> {
    pub chars: Peekable<Chars<'a>>,
    pub column: usize,
    pub current: usize,
    pub line: usize,
    pub start: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            chars: source.chars().peekable(),
            column: 0,
            current: 0,
            line: 1,
            start: 0,
        }
    }

    pub fn is_at_end(&mut self) -> bool {
        self.chars.peek().is_none()
    }

    pub fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    pub fn sync(&mut self) {
        self.start = self.current;
    }

    pub fn advance(&mut self) -> Option<char> {
        if self.is_at_end() {
            None
        } else {
            self.column += 1;
            self.current += 1;
            self.chars.next()
        }
    }

    pub fn advance_line(&mut self) {
        self.line += 1;
        self.column = 0;
    }

    pub fn location(&self) -> Location {
        Location {
            column: self.column,
            length: self.current - self.start,
            line: self.line,
            start: self.start,
        }
    }
}
