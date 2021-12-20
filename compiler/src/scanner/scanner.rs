use crate::scanner::token::*;
use std::fmt::{Display, Formatter};

pub struct Scanner {
    source: String,
    start: usize,
    current: usize,
    pos: Pos,
}

#[derive(Debug)]
pub struct ScannerError {
    message: &'static str,
    pos: Pos,
}

impl Display for ScannerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} at line {}:{}",
            self.message, self.pos.row, self.pos.col
        )
    }
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            start: 0,
            current: 0,
            pos: Pos { col: 0, row: 1 },
        }
    }

    pub fn scan_token(&mut self) -> Result<Token, ScannerError> {
        self.skip_whitespace()?;
        self.scan_token_int()
    }

    fn scan_token_int(&mut self) -> Result<Token, ScannerError> {
        self.start = self.current;

        if self.at_end() {
            return Ok(self.make_token(TokenType::EOF));
        }

        let c = self.advance();

        match c {
            '(' => Ok(self.make_token(TokenType::LeftParen)),
            ')' => Ok(self.make_token(TokenType::RightParen)),
            '{' => Ok(self.make_token(TokenType::LeftBrace)),
            '}' => Ok(self.make_token(TokenType::RightBrace)),
            '[' => Ok(self.make_token(TokenType::LeftSquare)),
            ']' => Ok(self.make_token(TokenType::RightSquare)),
            ';' => Ok(self.make_token(TokenType::Semicolon)),
            ',' => Ok(self.make_token(TokenType::Comma)),
            '.' => Ok(self.make_token(TokenType::Dot)),
            '-' => Ok(self.make_token(TokenType::Minus)),
            '+' => Ok(self.make_token(TokenType::Plus)),
            '/' => Ok(self.make_token(TokenType::Slash)),
            '*' => Ok(self.make_token(TokenType::Star)),

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

            _ => Err(self.token_error("Unexpected character")),
        }
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
                                    return Ok(());
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
        self.pos.col += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }

    fn new_line(&mut self) {
        self.pos.row += 1;
        self.pos.col = 0;
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.at_end() {
            return false;
        }
        if self.source.chars().nth(self.current) != Some(expected) {
            return false;
        }

        self.current += 1;
        self.pos.col += 1;

        true
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        Token {
            token_type,
            pos: self.pos.clone(),
            lexme: self.source[self.start..self.current].to_string(),
        }
    }

    fn token_error(&self, message: &'static str) -> ScannerError {
        ScannerError {
            message,
            pos: self.pos.clone(),
        }
    }
}
