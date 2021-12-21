use crate::scanner::token::{Literal, Token};
use std::rc::Rc;

#[derive(Debug)]
pub enum Expression {
    Assignment {
        name: Token,
        operator: Token,
        value: Rc<Expression>,
    },
    Logical {
        left: Rc<Expression>,
        operator: Token,
        right: Rc<Expression>,
    },
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
    Identifier {
        name: Token,
    },
    This {
        keyword: Token,
    },
    Super {
        keyword: Token,
    },
    Call {
        callee: Rc<Expression>,
        arguments: Vec<Rc<Expression>>,
        paren: Token,
    },
    Member {
        object: Rc<Expression>,
        name: Token,
    },
    Index {
        object: Rc<Expression>,
        index: Rc<Expression>,
        paren: Token,
    },
}
