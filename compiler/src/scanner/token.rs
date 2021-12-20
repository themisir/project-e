#[derive(Debug, Clone, Copy)]
pub struct Pos {
    pub row: u16,
    pub col: u16,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexme: String,
    pub pos: Pos,
}

#[derive(Copy, Clone, Debug)]
#[allow(dead_code)]
#[repr(u8)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen = 1,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftSquare,
    RightSquare,
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
    Integer,
    Float,

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
