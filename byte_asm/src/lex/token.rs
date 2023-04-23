// TODO: lex instructions differently
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Hash,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    DollarSign,
    NumberSign,
    PercentSign,
    Colon,

    Identifier,
    OrgDirective,
    DBDirective,
    DWDirective,
    Equ,
    Include,

    String,
    Number,

    Comment,
    EndOfFile,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Location {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenLiteral {
    String(String),
    Number(u64),
    None,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token<'a> {
    pub kind: TokenType,
    pub text: &'a str,
    pub literal: TokenLiteral,
    pub location: Location,
}

impl TryFrom<&str> for TokenType {
    type Error = ();

    #[rustfmt::skip]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use TokenType::*;

        let kind = match value {
            "org"     => OrgDirective,
            "db"      => DBDirective,
            "dw"      => DWDirective,
            "equ"     => Equ,
            "include" => Include,
            // this is usually the case we end up with
            // when `try_from` is called for a user-defined
            // identifier
            _         => return Err(())
        };

        Ok(kind)
    }
}
