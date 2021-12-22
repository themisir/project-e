use crate::scanner::token::{Literal, Token};
use std::rc::Rc;

#[derive(Debug)]
pub enum Expression {
    Assignment {
        left: Rc<Expression>,
        operator: Token,
        right: Rc<Expression>,
    },
    Binary {
        left: Rc<Expression>,
        operator: Token,
        right: Rc<Expression>,
    },
    Call {
        callee: Rc<Expression>,
        arguments: Vec<Rc<Expression>>,
        paren: Token,
    },
    Grouping {
        expression: Rc<Expression>,
    },
    Identifier {
        name: Token,
    },
    Index {
        object: Rc<Expression>,
        index: Rc<Expression>,
        paren: Token,
    },
    LambdaFunction {
        keyword: Token,
        parameters: Vec<FunctionParameter>,
        return_type: Option<TypeReference>,
        body: Vec<Rc<Statement>>,
    },
    Literal {
        value: Literal,
    },
    Logical {
        left: Rc<Expression>,
        operator: Token,
        right: Rc<Expression>,
    },
    Member {
        object: Rc<Expression>,
        name: Token,
    },
    Super {
        keyword: Token,
    },
    This {
        keyword: Token,
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

#[derive(Debug)]
#[allow(dead_code)]
pub enum Statement {
    Block {
        declarations: Vec<Rc<Statement>>,
    },
    Break {
        keyword: Token,
    },
    Continue {
        keyword: Token,
    },
    Class {
        name: Token,
        extends_type: Option<TypeReference>,
        fields: Vec<ClassField>,
        methods: Vec<ClassMethod>,
    },
    Expression {
        expression: Expression,
    },
    For {
        initializer: Option<Rc<Statement>>,
        condition: Option<Expression>,
        update: Option<Expression>,
        body: Rc<Statement>,
    },
    Function {
        name: Token,
        parameters: Vec<FunctionParameter>,
        return_type: Option<TypeReference>,
        body: Vec<Rc<Statement>>,
    },
    If {
        condition: Expression,
        then_branch: Rc<Statement>,
        else_branch: Option<Rc<Statement>>,
    },
    Program {
        declarations: Vec<Rc<Statement>>,
    },
    Return {
        keyword: Token,
        value: Option<Expression>,
    },
    Var {
        name: Token,
        value: Option<Expression>,
        value_type: TypeReference,
    },
    While {
        condition: Expression,
        body: Rc<Statement>,
    },
}

#[derive(Debug, Clone)]
pub enum TypeName {
    Identifier { name: Token },
    QualifiedName { left: Rc<TypeName>, right: Token },
}

#[derive(Debug)]
pub struct TypeReference {
    pub type_name: TypeName,
}

impl TypeName {
    pub fn new(name: Token) -> Self {
        TypeName::Identifier { name }
    }

    pub fn push(&self, name: Token) -> Self {
        TypeName::QualifiedName {
            left: Rc::new(self.clone()),
            right: name,
        }
    }
}

#[derive(Debug)]
pub struct FunctionParameter {
    pub name: Token,
    pub value_type: TypeReference,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct ClassField {
    pub name: Token,
    pub value_type: TypeReference,
    pub value: Option<Expression>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct ClassMethod {
    pub name: Token,
    pub parameters: Vec<FunctionParameter>,
    pub return_type: Option<TypeReference>,
    pub body: Statement,
}
