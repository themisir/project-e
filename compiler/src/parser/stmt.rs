use crate::parser::expr::Expression;
use crate::parser::types::TypeReference;
use crate::scanner::token::Token;
use std::rc::Rc;

#[derive(Debug)]
pub struct BlockStatement {
    pub declarations: Vec<Rc<Statement>>,
}

#[derive(Debug)]
pub struct BreakStatement {
    pub keyword: Token,
}

#[derive(Debug)]
pub struct ClassStatement {
    pub name: Token,
    pub extends: Option<TypeReference>,
    pub members: Vec<ClassMember>,
}

#[derive(Debug)]
pub struct ContinueStatement {
    pub keyword: Token,
}

#[derive(Debug)]
pub struct ExpressionStatement {
    pub expression: Expression,
}

#[derive(Debug)]
pub struct ForStatement {
    pub initializer: Option<Rc<Statement>>,
    pub condition: Option<Expression>,
    pub update: Option<Expression>,
    pub body: Rc<Statement>,
}

#[derive(Debug)]
pub struct FunctionStatement {
    pub name: Token,
    pub parameters: Vec<FunctionParameter>,
    pub return_type: Option<TypeReference>,
    pub body: Vec<Rc<Statement>>,
}

#[derive(Debug)]
pub struct IfStatement {
    pub condition: Expression,
    pub then_branch: Rc<Statement>,
    pub else_branch: Option<Rc<Statement>>,
}

#[derive(Debug)]
pub struct ProgramStatement {
    pub declarations: Vec<Rc<Statement>>,
}

#[derive(Debug)]
pub struct ReturnStatement {
    pub keyword: Token,
    pub value: Option<Expression>,
}

#[derive(Debug)]
pub struct VarStatement {
    pub name: Token,
    pub value: Option<Expression>,
    pub value_type: TypeReference,
}

#[derive(Debug)]
pub struct WhileStatement {
    pub condition: Expression,
    pub body: Rc<Statement>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Statement {
    Block(BlockStatement),
    Break(BreakStatement),
    Class(ClassStatement),
    Continue(ContinueStatement),
    Expression(ExpressionStatement),
    For(ForStatement),
    Function(FunctionStatement),
    If(IfStatement),
    Program(ProgramStatement),
    Return(ReturnStatement),
    Var(VarStatement),
    While(WhileStatement),
}

#[derive(Debug)]
pub struct FunctionParameter {
    pub name: Token,
    pub value_type: TypeReference,
}

#[derive(Debug)]
pub enum ClassMember {
    Field {
        name: Token,
        value_type: TypeReference,
        value: Option<Expression>,
    },
    Method {
        name: Token,
        parameters: Vec<FunctionParameter>,
        return_type: Option<TypeReference>,
        body: Vec<Rc<Statement>>,
    },
}
