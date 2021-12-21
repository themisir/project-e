use crate::parser::ast::Expression;
use crate::scanner::token::{Literal, Token};
use crate::TokenType;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

#[derive(Debug)]
pub struct ParserError {
    message: &'static str,
    token: Token,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at {}", self.message, self.token.range())
    }
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Expression, ParserError> {
        self.expression()
    }

    fn match_single(&mut self, token_type: TokenType) -> bool {
        if self.check(token_type) {
            self.advance();
            return true;
        }
        false
    }

    fn match_any(&mut self, token_type: &[TokenType]) -> bool {
        for token_type in token_type {
            if self.check(token_type.clone()) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&mut self, token_type: TokenType) -> bool {
        if self.at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }

    fn advance(&mut self) -> &Token {
        if !self.at_end() {
            self.current += 1;
        }

        self.tokens.iter().nth(self.current - 1).unwrap()
    }

    fn at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn peek(&self) -> &Token {
        self.tokens.iter().nth(self.current).unwrap()
    }

    fn previous(&self) -> &Token {
        self.tokens.iter().nth(self.current - 1).unwrap()
    }

    fn expression(&mut self) -> Result<Expression, ParserError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expression, ParserError> {
        let mut expr = self.comparison()?;

        while self.match_any(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expression::Binary {
                operator,
                left: Rc::new(expr),
                right: Rc::new(right),
            }
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expression, ParserError> {
        let mut expr = self.term()?;

        while self.match_any(&[
            TokenType::Less,
            TokenType::LessEqual,
            TokenType::Greater,
            TokenType::GreaterEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expression::Binary {
                operator,
                left: Rc::new(expr),
                right: Rc::new(right),
            }
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expression, ParserError> {
        let mut expr = self.factor()?;

        while self.match_any(&[
            TokenType::Minus,
            TokenType::MinusEqual,
            TokenType::Plus,
            TokenType::PlusEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expression::Binary {
                operator,
                left: Rc::new(expr),
                right: Rc::new(right),
            }
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expression, ParserError> {
        let mut expr = self.unary()?;

        while self.match_any(&[
            TokenType::Slash,
            TokenType::SlashEqual,
            TokenType::Star,
            TokenType::StarEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expression::Binary {
                operator,
                left: Rc::new(expr),
                right: Rc::new(right),
            }
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expression, ParserError> {
        if self.match_any(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;

            Ok(Expression::Unary {
                operator,
                right: Rc::new(right),
            })
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expression, ParserError> {
        if self.match_single(TokenType::False) {
            return Ok(Expression::Literal {
                value: Literal::Boolean(false),
            });
        }
        if self.match_single(TokenType::True) {
            return Ok(Expression::Literal {
                value: Literal::Boolean(true),
            });
        }
        if self.match_single(TokenType::Null) {
            return Ok(Expression::Literal {
                value: Literal::Null,
            });
        }

        if self.match_any(&[TokenType::Integer, TokenType::Float, TokenType::String]) {
            return Ok(Expression::Literal {
                value: self.previous().literal.as_ref().unwrap().clone(),
            });
        }

        if self.match_single(TokenType::LeftParen) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression")?;

            return Ok(Expression::Grouping {
                expression: Rc::new(expr),
            });
        }

        Err(self.error(self.peek(), "Expect expression"))
    }

    fn consume(
        &mut self,
        token_type: TokenType,
        error_message: &'static str,
    ) -> Result<&Token, ParserError> {
        if self.check(token_type) {
            return Ok(self.advance());
        }

        Err(self.error(self.peek(), error_message))
    }

    fn error(&self, token: &Token, message: &'static str) -> ParserError {
        ParserError {
            message,
            token: token.clone(),
        }
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Return => {
                    return;
                }
                _ => {
                    self.advance();
                }
            }
        }
    }
}
