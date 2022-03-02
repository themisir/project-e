use crate::scanner::token::Token;
use std::rc::Rc;

#[derive(Debug)]
pub struct TypeReference {
    pub type_name: TypeName,
}

#[derive(Debug, Clone)]
pub enum TypeName {
    Identifier { name: Token },
    QualifiedName { left: Rc<TypeName>, right: Token },
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
