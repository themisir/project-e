use crate::scanner::token::{Literal, Token};
use std::rc::Rc;

#[derive(Debug)]
pub enum Expression {
    Binary {
        left: Rc<Expression>,
        operator: Token,
        right: Rc<Expression>,
    },
    Grouping {
        expression: Rc<Expression>,
    },
    Literal {
        value: Literal,
    },
    Unary {
        operator: Token,
        right: Rc<Expression>,
    },
    Update {
        operator: Token,
        prefix: bool,
        expression: Rc<Expression>,
    },
}
