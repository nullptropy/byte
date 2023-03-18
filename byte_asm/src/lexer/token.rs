// keywords, instructions, all other shit
#[derive(Debug)]
pub enum TokenType {
    // common symbols
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

    EndOfFile,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenType,
}
