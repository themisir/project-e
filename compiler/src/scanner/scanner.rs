use crate::scanner::token::*;
use std::borrow::Borrow;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub struct Scanner {
    source: String,
    start: usize,
    current: usize,
    start_pos: Pos,
    current_pos: Pos,
}

#[derive(Debug)]
pub struct ScannerError {
    message: &'static str,
    start_pos: Pos,
    end_pos: Pos,
}

impl Display for ScannerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} at {}",
            self.message,
            Range(self.start_pos, self.end_pos),
        )
    }
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            start: 0,
            current: 0,
            start_pos: Pos { col: 1, row: 1 },
            current_pos: Pos { col: 1, row: 1 },
        }
    }

    pub fn scan_all(&mut self) -> Result<Vec<Token>, ScannerError> {
        let mut tokens = Vec::new();
        loop {
            let token = self.scan_token()?;
            tokens.push(token.clone());
            if token.token_type == TokenType::EOF {
                break;
            }
        }
        Ok(tokens)
    }

    pub fn scan_token(&mut self) -> Result<Token, ScannerError> {
        self.skip_whitespace()?;
        self.start = self.current;
        self.start_pos = self.current_pos;

        if self.at_end() {
            return Ok(self.make_token(TokenType::EOF));
        }

        let c = self.advance();

        if c.is_ascii_digit() {
            return self.number();
        }
        if c.is_ascii_alphabetic() || c == '_' {
            return self.identifier();
        }

        match c {
            '(' => Ok(self.make_token(TokenType::LeftParen)),
            ')' => Ok(self.make_token(TokenType::RightParen)),
            '{' => Ok(self.make_token(TokenType::LeftBrace)),
            '}' => Ok(self.make_token(TokenType::RightBrace)),
            '[' => Ok(self.make_token(TokenType::LeftSquare)),
            ']' => Ok(self.make_token(TokenType::RightSquare)),
            ':' => Ok(self.make_token(TokenType::Colon)),
            ';' => Ok(self.make_token(TokenType::Semicolon)),
            ',' => Ok(self.make_token(TokenType::Comma)),
            '.' => Ok(self.make_token(TokenType::Dot)),

            '!' => {
                if self.matches('=') {
                    Ok(self.make_token(TokenType::BangEqual))
                } else {
                    Ok(self.make_token(TokenType::Bang))
                }
            }
            '=' => {
                if self.matches('=') {
                    Ok(self.make_token(TokenType::EqualEqual))
                } else {
                    Ok(self.make_token(TokenType::Equal))
                }
            }
            '<' => {
                if self.matches('=') {
                    Ok(self.make_token(TokenType::LessEqual))
                } else {
                    Ok(self.make_token(TokenType::Less))
                }
            }
            '>' => {
                if self.matches('=') {
                    Ok(self.make_token(TokenType::GreaterEqual))
                } else {
                    Ok(self.make_token(TokenType::Greater))
                }
            }

            '-' => {
                if self.matches('-') {
                    Ok(self.make_token(TokenType::MinusMinus))
                } else if self.matches('=') {
                    Ok(self.make_token(TokenType::MinusEqual))
                } else {
                    Ok(self.make_token(TokenType::Minus))
                }
            }
            '+' => {
                if self.matches('+') {
                    Ok(self.make_token(TokenType::PlusPlus))
                } else if self.matches('=') {
                    Ok(self.make_token(TokenType::PlusEqual))
                } else {
                    Ok(self.make_token(TokenType::Plus))
                }
            }
            '/' => {
                if self.matches('/') {
                    Ok(self.make_token(TokenType::SlashEqual))
                } else {
                    Ok(self.make_token(TokenType::Slash))
                }
            }
            '*' => {
                if self.matches('/') {
                    Ok(self.make_token(TokenType::StarEqual))
                } else {
                    Ok(self.make_token(TokenType::Star))
                }
            }

            '"' => self.string(),

            _ => Err(self.token_error("Unexpected character")),
        }
    }

    fn identifier(&mut self) -> Result<Token, ScannerError> {
        loop {
            let c = self.peek();
            if c.is_ascii_alphanumeric() || c == '_' {
                self.advance();
                continue;
            }

            let token_type = self.identifier_type();
            return Ok(self.make_token(token_type));
        }
    }

    fn identifier_type(&mut self) -> TokenType {
        let lexme = self.current_lexme();
        KEYWORDS
            .get(lexme)
            .unwrap_or(&TokenType::Identifier)
            .clone()
    }

    fn number(&mut self) -> Result<Token, ScannerError> {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }

            let value = f64::from_str(self.current_lexme()).unwrap();
            Ok(self.make_literal_token(TokenType::Float, Literal::Float(value)))
        } else {
            let value = i64::from_str(self.current_lexme()).unwrap();
            Ok(self.make_literal_token(TokenType::Integer, Literal::Integer(value)))
        }
    }

    fn string(&mut self) -> Result<Token, ScannerError> {
        while self.peek() != '"' && !self.at_end() {
            if self.peek() == '\n' {
                self.new_line();
            }
            self.advance();
        }

        if self.at_end() {
            return Err(self.token_error("Unterminated string"));
        }

        self.advance();

        let value = self.source[self.start + 1..self.current - 1].to_string();
        Ok(self.make_literal_token(TokenType::String, Literal::String(value)))
    }

    fn skip_whitespace(&mut self) -> Result<(), ScannerError> {
        loop {
            match self.peek() {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.new_line();
                    self.advance();
                }
                '/' => match self.peek_next() {
                    '/' => {
                        while self.peek() != '\n' && !self.at_end() {
                            self.advance();
                        }
                    }
                    '*' => loop {
                        if self.at_end() {
                            return Err(self.token_error("Unterminated comment block"));
                        }

                        match self.advance() {
                            '\n' => {
                                self.new_line();
                            }
                            '*' => {
                                if self.peek() == '/' {
                                    self.advance();
                                    break;
                                }
                            }
                            _ => {}
                        }
                    },
                    _ => return Ok(()),
                },
                _ => return Ok(()),
            };
        }
    }

    fn at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn peek(&self) -> char {
        self.source.chars().nth(self.current).unwrap_or('\0')
    }

    fn peek_next(&self) -> char {
        if self.at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current + 1).unwrap_or('\0')
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.current_pos.col += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }

    fn new_line(&mut self) {
        self.current_pos.row += 1;
        self.current_pos.col = 1;
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.at_end() {
            return false;
        }
        if self.source.chars().nth(self.current) != Some(expected) {
            return false;
        }

        self.current += 1;
        self.current_pos.col += 1;

        true
    }

    fn current_lexme(&self) -> &str {
        self.source[self.start..self.current].borrow()
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        self.make_token_int(token_type, None)
    }

    fn make_literal_token(&self, token_type: TokenType, value: Literal) -> Token {
        self.make_token_int(token_type, Some(value))
    }

    fn make_token_int(&self, token_type: TokenType, literal: Option<Literal>) -> Token {
        Token {
            token_type,
            lexme: String::from(self.current_lexme()),
            start_pos: self.start_pos.clone(),
            end_pos: self.current_pos.clone(),
            literal,
        }
    }

    fn token_error(&self, message: &'static str) -> ScannerError {
        ScannerError {
            message,
            start_pos: self.start_pos.clone(),
            end_pos: self.current_pos.clone(),
        }
    }
}
