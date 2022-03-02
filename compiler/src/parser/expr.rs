use crate::parser::stmt::{FunctionParameter, Statement};
use crate::parser::types::TypeReference;
use crate::scanner::token::{Literal, Token};
use std::rc::Rc;

#[derive(Debug)]
pub struct AssignmentExpression {
    pub left: Rc<Expression>,
    pub operator: Token,
    pub right: Rc<Expression>,
}

#[derive(Debug)]
pub struct BinaryExpression {
    pub left: Rc<Expression>,
    pub operator: Token,
    pub right: Rc<Expression>,
}

#[derive(Debug)]
pub struct CallExpression {
    pub callee: Rc<Expression>,
    pub arguments: Vec<Rc<Expression>>,
    pub paren: Token,
}

#[derive(Debug)]
pub struct GroupingExpression {
    pub expression: Rc<Expression>,
}

#[derive(Debug)]
pub struct IdentifierExpression {
    pub name: Token,
}

#[derive(Debug)]
pub struct IndexExpression {
    pub object: Rc<Expression>,
    pub index: Rc<Expression>,
    pub paren: Token,
}

#[derive(Debug)]
pub struct LambdaFunctionExpression {
    pub keyword: Token,
    pub parameters: Vec<FunctionParameter>,
    pub return_type: Option<TypeReference>,
    pub body: Vec<Rc<Statement>>,
}

#[derive(Debug)]
pub struct LiteralExpression {
    pub value: Literal,
}

#[derive(Debug)]
pub struct LogicalExpression {
    pub left: Rc<Expression>,
    pub operator: Token,
    pub right: Rc<Expression>,
}

#[derive(Debug)]
pub struct MemberExpression {
    pub object: Rc<Expression>,
    pub name: Token,
}

#[derive(Debug)]
pub struct SuperExpression {
    pub keyword: Token,
}

#[derive(Debug)]
pub struct ThisExpression {
    pub keyword: Token,
}

#[derive(Debug)]
pub struct UnaryExpression {
    pub operator: Token,
    pub right: Rc<Expression>,
}

#[derive(Debug)]
pub struct UpdateExpression {
    pub operator: Token,
    pub prefix: bool,
    pub expression: Rc<Expression>,
}

#[derive(Debug)]
pub enum Expression {
    Assignment(AssignmentExpression),
    Binary(BinaryExpression),
    Call(CallExpression),
    Grouping(GroupingExpression),
    Identifier(IdentifierExpression),
    Index(IndexExpression),
    LambdaFunction(LambdaFunctionExpression),
    Literal(LiteralExpression),
    Logical(LogicalExpression),
    Member(MemberExpression),
    Super(SuperExpression),
    This(ThisExpression),
    Unary(UnaryExpression),
    Update(UpdateExpression),
}
