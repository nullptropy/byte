use crate::error::ScannerError;

// keywords, instructions, all other shit
#[derive(Debug)]
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
    EndOfFile,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenType,
    pub text: String,
}

impl TryFrom<&str> for TokenType {
    type Error = crate::error::ScannerError;

    #[rustfmt::skip]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use TokenType::*;

        match value {
            ".org"    => Ok(OrgDirective),
            ".db"     => Ok(DBDirective),
            "include" => Ok(Include),
            _         => Err(ScannerError::Unknown)
        }
    }
}
