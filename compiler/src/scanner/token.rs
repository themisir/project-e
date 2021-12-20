pub struct Token {
    token_type: TokenType,
    lexme: String,
    line: u32,
}

pub enum TokenType {
    // Single-character tokens.
    LeftParen = 0,
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

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
}