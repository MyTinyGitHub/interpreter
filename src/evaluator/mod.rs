use crate::ast::{Expression, ExpressionStatement, Node, Statement};

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

pub fn eval(node: &Node) -> Object {
    match node {
        Node::Program(prog) => eval_statements(&prog.statements),
        Node::Statement(stmt) => match stmt {
            Statement::Expression(expr) => match expr {
                Expression::IntegerLiteral(integer) => return Object::Integer(integer.value),
                Expression::Boolean(boolean) => return Object::Boolean(boolean.value),
                Expression::Prefix(prefix) => {
                    let expression = *prefix.right.clone();
                    let right = eval(&Node::Statement(Statement::Expression(expression)));
                    eval_prefix(&prefix.operator, right)
                }
                _ => Object::Null,
            },
            _ => Object::Null,
        },
    }
}

pub fn eval_prefix(operator: &str, object: Object) -> Object {
    match operator {
        "!" => eval_bang_operator_expresion(object),
        "-" => eval_minus_operator_expresion(object),
        _ => Object::Null,
    }
}

pub fn eval_minus_operator_expresion(object: Object) -> Object {
    match object {
        Object::Integer(value) => Object::Integer(-value),
        _ => Object::Null,
    }
}

pub fn eval_bang_operator_expresion(object: Object) -> Object {
    match object {
        Object::Boolean(boolean) => Object::Boolean(!boolean),
        _ => Object::Boolean(false),
    }
}

pub fn eval_statements(statements: &Vec<Statement>) -> Object {
    let mut result: Object = Object::Null;

    for statement in statements {
        result = eval(&Node::Statement(statement.clone()));
    }

    result
}

