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
    type Error = ();

    #[rustfmt::skip]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use TokenType::*;

        let kind = match value {
            "org"     => OrgDirective,
            "db"      => DBDirective,
            "include" => Include,
            // this is usually the case we end up with
            // when `try_from` is called for a user-defined
            // identifier
            _         => return Err(())
        };

        Ok(kind)
    }
}
