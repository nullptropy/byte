use super::LexerError;

// keywords, instructions, all other shit
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
    Include,

    String,
    Number,

    Comment,
    EndOfFile,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenLiteral {
    String(String),
    Number(u64),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenType,
    pub text: String,
    pub literal: Option<TokenLiteral>,
    pub line: usize,
    pub column: usize,
}

impl TryFrom<&str> for TokenType {
    type Error = LexerError;

    #[rustfmt::skip]
    fn try_from(value: &str) -> super::LexerResult<Self> {
        use TokenType::*;

        match value {
            ".org"    => Ok(OrgDirective),
            ".db"     => Ok(DBDirective),
            "include" => Ok(Include),
            _         => Err(LexerError::Unknown)
        }
    }
}
