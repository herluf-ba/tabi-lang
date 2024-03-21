use super::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct Root {
    pub declarations: Vec<Declaration>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Declaration {
    Fn {
        name: Ident,
        parameters: Vec<Parameter>,
        return_type: Type,
        body: Expression,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: Ident,
    pub ttype: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Ident(Ident),
    Literal(Value),
    Block { expressions: Vec<Expression> },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Ident(Ident),
}

#[derive(Debug, Clone, Hash, PartialEq)]
pub struct Ident(pub String);
