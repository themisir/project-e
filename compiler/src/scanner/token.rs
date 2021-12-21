use phf::phf_map;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Pos {
    pub row: u16,
    pub col: u16,
    pub index: u16,
}

impl Pos {
    pub(crate) fn initial() -> Pos {
        Pos {
            row: 1,
            col: 1,
            index: 0,
        }
    }

    pub(crate) fn inc_col(&mut self) {
        self.index += 1;
        self.col += 1;
    }

    pub(crate) fn inc_row(&mut self) {
        self.index += 1;
        self.col = 0;
        self.row += 1;
    }
}

pub struct Range(pub(crate) Pos, pub(crate) Pos);

impl Display for Pos {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "line {}, column {}", self.row, self.col)
    }
}

impl Display for Range {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.0 == self.1 {
            write!(f, "{}", self.0)
        } else if self.0.row == self.1.row {
            write!(
                f,
                "line {} column {}..{}",
                self.0.row, self.0.col, self.1.col
            )
        } else {
            write!(f, "{} .. {}", self.0, self.1)
        }
    }
}

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
    pub fn range(&self) -> Range {
        Range(self.start_pos, self.end_pos)
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[allow(dead_code)]
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
    Class,
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
    "class" => TokenType::Class,
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
