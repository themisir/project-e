use crate::scanner::token::{Literal, Token};
use std::rc::Rc;

#[derive(Debug)]
pub enum Expression {
    Assignment {
        left: Rc<Expression>,
        operator: Token,
        right: Rc<Expression>,
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

#[derive(Debug)]
#[allow(dead_code)]
pub enum Statement {
    Block {
        declarations: Vec<Declaration>,
    },
    Expression {
        expression: Expression,
    },
    For {
        init: Option<Expression>,
        condition: Option<Expression>,
        update: Option<Expression>,
    },
    If {
        condition: Expression,
        then_branch: Rc<Statement>,
        else_branch: Option<Rc<Statement>>,
    },
    Return {
        value: Expression,
    },
    While {
        condition: Expression,
        body: Rc<Statement>,
    },
    Break {
        keyword: Token,
    },
    Continue {
        keyword: Token,
    },
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Declaration {
    Class {
        name: Token,
        extends_type: Option<TypeName>,
        fields: Vec<ClassField>,
        methods: Vec<ClassMethod>,
    },
    Function {
        name: Token,
        parameters: Vec<FunctionParameter>,
        return_type: Option<TypeName>,
        body: Statement,
    },
    Var {
        name: Token,
        value: Expression,
        value_type: TypeName,
    },
    Statement {
        statement: Statement,
    },
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct TypeName(Vec<Token>);

#[derive(Debug)]
#[allow(dead_code)]
pub struct FunctionParameter {
    name: Token,
    value_type: TypeName,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct ClassField {
    name: Token,
    value_type: TypeName,
    value: Option<Expression>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct ClassMethod {
    name: Token,
    parameters: Vec<FunctionParameter>,
    return_type: Option<TypeName>,
    body: Statement,
}
