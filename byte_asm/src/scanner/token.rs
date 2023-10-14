use byte_common::opcode::Mnemonic;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Location {
    pub column: usize,
    pub length: usize,
    pub line: usize,
    pub start: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenValue {
    String(String),
    Number(u64),
    Directive(Directive),
    Instruction(Mnemonic),
}

#[derive(Debug, Clone, Copy, PartialEq, strum::EnumString)]
pub enum Directive {
    DB,
    DW,
    EQU,
    INCLUDE,
    ORG,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    CloseParen,
    Colon,
    Comma,
    Comment,
    Directive,
    EOF,
    Hash,
    Identifier,
    Instruction,
    Minus,
    NewLine,
    Number,
    OpenParen,
    Plus,
    Semicolon,
    Slash,
    Star,
    String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub value: Option<TokenValue>,
    pub location: Location,
}

impl Token {
    pub fn eof(&self) -> bool {
        self.kind == TokenKind::EOF
    }

    pub fn text<'a>(&self, source: &'a str) -> &'a str {
        &source[self.location.start..self.location.start + self.location.length]
    }
}
