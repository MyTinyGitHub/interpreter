use crate::ast::{Expression, Node, Statement};

type ObjectType = String;

#[cfg(test)]
pub mod tests;

pub enum Object {
    Integer(i64),
    Boolean(bool),
    Null,
}

impl Object {
    pub fn obj_type(&self) -> ObjectType {
        match self {
            Self::Integer(_) => "INTEGER".to_string(),
            Self::Boolean(_) => "BOOLEAN".to_string(),
            Self::Null => "NULL".to_string(),
        }
    }

    pub fn inspect(&self) -> String {
        match self {
            Self::Integer(val) => val.to_string(),
            Self::Boolean(val) => val.to_string(),
            Self::Null => "null".to_string(),
        }
    }
}

pub fn eval(node: &Node) -> Option<Object> {
    match node {
        Node::Program(prog) => return eval_statements(&prog.statements),
        Node::Statement(stmt) => match stmt {
            Statement::Expression(expr) => match expr.value.as_deref().expect("") {
                Expression::IntegerLiteral(integer) => return Some(Object::Integer(integer.value)),
                _ => None,
            },
            _ => None,
        },
        _ => None,
    }
}

pub fn eval_statements(statements: &Vec<Statement>) -> Option<Object> {
    let mut result: Option<Object> = None;

    for statement in statements {
        result = eval(&Node::Statement(statement.clone()));
    }

    result
}
