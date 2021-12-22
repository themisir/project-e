use crate::scanner::pos::*;
use phf::phf_map;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Literal {
    Null,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexme: String,
    pub start_pos: Pos,
    pub end_pos: Pos,
    pub literal: Option<Literal>,
}

impl Token {
    pub fn range(&self) -> PosRange {
        PosRange(self.start_pos, self.end_pos)
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
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
    Colon,
    Semicolon,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Minus,
    MinusEqual,
    MinusMinus,
    Plus,
    PlusEqual,
    PlusPlus,
    Slash,
    SlashEqual,
    Star,
    StarEqual,

    // Literals.
    Identifier,
    String,
    Integer,
    Float,

    // Keywords.
    And,
    Break,
    Class,
    Continue,
    Else,
    False,
    Fun,
    For,
    If,
    Null,
    Or,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
}

pub(crate) static KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
    "and" => TokenType::And,
    "break" => TokenType::Break,
    "class" => TokenType::Class,
    "continue" => TokenType::Continue,
    "else" => TokenType::Else,
    "false" => TokenType::False,
    "fun" => TokenType::Fun,
    "for" => TokenType::For,
    "if" => TokenType::If,
    "null" => TokenType::Null,
    "or" => TokenType::Or,
    "return" => TokenType::Return,
    "super" => TokenType::Super,
    "this" => TokenType::This,
    "true" => TokenType::True,
    "var" => TokenType::Var,
    "while" => TokenType::While,
};
