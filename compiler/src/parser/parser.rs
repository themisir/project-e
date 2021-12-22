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
    pub message: &'static str,
    pub token: Token,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} at {} token '{}'",
            self.message,
            self.token.range(),
            self.token.lexme
        )
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
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expression, ParserError> {
        let expr = self.or()?;

        if self.match_any(&[
            TokenType::Equal,
            TokenType::MinusEqual,
            TokenType::PlusEqual,
            TokenType::SlashEqual,
            TokenType::StarEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.assignment()?;

            match expr {
                Expression::Member { .. }
                | Expression::Index { .. }
                | Expression::Identifier { .. } => Ok(Expression::Assignment {
                    operator,
                    left: Rc::new(expr),
                    right: Rc::new(right),
                }),
                _ => Err(self.error(&operator, "Invalid assignment target")),
            }
        } else {
            Ok(expr)
        }
    }

    fn or(&mut self) -> Result<Expression, ParserError> {
        let mut expr = self.and()?;

        while self.match_single(TokenType::Or) {
            let operator = self.previous().clone();
            let right = self.and()?;
            expr = Expression::Logical {
                operator,
                left: Rc::new(expr),
                right: Rc::new(right),
            }
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expression, ParserError> {
        let mut expr = self.equality()?;

        while self.match_single(TokenType::And) {
            let operator = self.previous().clone();
            let right = self.equality()?;
            expr = Expression::Logical {
                operator,
                left: Rc::new(expr),
                right: Rc::new(right),
            }
        }

        Ok(expr)
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

        while self.match_any(&[TokenType::Minus, TokenType::Plus]) {
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

        while self.match_any(&[TokenType::Slash, TokenType::Star]) {
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
            self.update()
        }
    }

    fn update(&mut self) -> Result<Expression, ParserError> {
        if self.match_any(&[TokenType::MinusMinus, TokenType::PlusPlus]) {
            let operator = self.previous().clone();
            let expr = self.call()?;
            Ok(Expression::Update {
                operator,
                prefix: true,
                expression: Rc::new(expr),
            })
        } else {
            let expr = self.call()?;
            if self.match_any(&[TokenType::MinusMinus, TokenType::PlusPlus]) {
                let operator = self.previous().clone();
                Ok(Expression::Update {
                    operator,
                    prefix: false,
                    expression: Rc::new(expr),
                })
            } else {
                Ok(expr)
            }
        }
    }

    fn call(&mut self) -> Result<Expression, ParserError> {
        let mut expr = self.primary()?;

        loop {
            if self.match_single(TokenType::LeftParen) {
                expr = self.finish_call(expr)?;
            } else if self.match_single(TokenType::LeftSquare) {
                expr = self.finish_index(expr)?;
            } else if self.match_single(TokenType::Dot) {
                expr = self.finish_member(expr)?;
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_member(&mut self, callee: Expression) -> Result<Expression, ParserError> {
        let name = self
            .consume(TokenType::Identifier, "Expect property name after '.'")?
            .clone();
        Ok(Expression::Member {
            object: Rc::new(callee),
            name,
        })
    }

    fn finish_index(&mut self, callee: Expression) -> Result<Expression, ParserError> {
        let index = self.expression()?;
        let paren = self
            .consume(TokenType::RightSquare, "Expect ']' after index")?
            .clone();
        Ok(Expression::Index {
            index: Rc::new(index),
            object: Rc::new(callee),
            paren,
        })
    }

    fn finish_call(&mut self, callee: Expression) -> Result<Expression, ParserError> {
        let mut arguments: Vec<Rc<Expression>> = Vec::new();

        if !self.check(TokenType::RightParen) {
            loop {
                let argument = self.expression()?;

                arguments.push(Rc::new(argument));

                if !self.match_single(TokenType::Comma) {
                    break;
                }
            }
        }

        let paren = self
            .consume(TokenType::RightParen, "Expect ')' after arguments")?
            .clone();
        Ok(Expression::Call {
            callee: Rc::new(callee),
            arguments,
            paren,
        })
    }

    fn primary(&mut self) -> Result<Expression, ParserError> {
        // "false"
        if self.match_single(TokenType::False) {
            return Ok(Expression::Literal {
                value: Literal::Boolean(false),
            });
        }
        // "true"
        if self.match_single(TokenType::True) {
            return Ok(Expression::Literal {
                value: Literal::Boolean(true),
            });
        }
        // "null"
        if self.match_single(TokenType::Null) {
            return Ok(Expression::Literal {
                value: Literal::Null,
            });
        }

        // "this"
        if self.match_single(TokenType::This) {
            return Ok(Expression::This {
                keyword: self.previous().clone(),
            });
        }
        // "super"
        if self.match_single(TokenType::Super) {
            return Ok(Expression::Super {
                keyword: self.previous().clone(),
            });
        }

        // NUMBER | STRING
        if self.match_any(&[TokenType::Integer, TokenType::Float, TokenType::String]) {
            return Ok(Expression::Literal {
                value: self.previous().literal.as_ref().unwrap().clone(),
            });
        }

        // IDENTIFIER
        if self.match_single(TokenType::Identifier) {
            return Ok(Expression::Identifier {
                name: self.previous().clone(),
            });
        }

        // "(" expression ")"
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

    #[allow(dead_code)]
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
