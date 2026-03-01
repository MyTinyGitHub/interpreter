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

pub fn eval(node: &Node) -> Object {
    match node {
        Node::Program(prog) => eval_statements(&prog.statements),
        Node::Statement(stmt) => match stmt {
            Statement::Expression(expr) => match expr {
                Expression::IntegerLiteral(integer) => Object::Integer(integer.value),
                Expression::Boolean(boolean) => Object::Boolean(boolean.value),
                Expression::Prefix(prefix) => {
                    let expression = *prefix.right.clone();
                    let right = eval(&Node::Statement(Statement::Expression(expression)));
                    eval_prefix(&prefix.operator, right)
                }
                Expression::Infix(infix) => {
                    let right = eval(&Node::Statement(Statement::Expression(
                        *infix.right.clone(),
                    )));
                    let left = eval(&Node::Statement(Statement::Expression(*infix.left.clone())));
                    eval_infix(&infix.operator, left, right)
                }
                _ => Object::Null,
            },
            _ => Object::Null,
        },
    }
}

pub fn eval_infix(operator: &str, left: Object, right: Object) -> Object {
    match (left, right) {
        (Object::Integer(l), Object::Integer(r)) => eval_integer_infix(operator, l, r),
        (Object::Boolean(l), Object::Boolean(r)) => eval_boolean_infix(operator, l, r),
        _ => panic!("Not compatible types"),
    }
}

pub fn eval_boolean_infix(operator: &str, left: bool, right: bool) -> Object {
    match operator {
        "==" => Object::Boolean(left == right),
        "!=" => Object::Boolean(left != right),
        _ => panic!("incompatible boolean operation"),
    }
}

pub fn eval_integer_infix(operator: &str, left: i64, right: i64) -> Object {
    match operator {
        "+" => Object::Integer(left + right),
        "*" => Object::Integer(left * right),
        "-" => Object::Integer(left - right),
        "/" => Object::Integer(left / right),
        "==" => Object::Boolean(left == right),
        "!=" => Object::Boolean(left != right),
        "<" => Object::Boolean(left < right),
        ">" => Object::Boolean(left > right),
        ">=" => Object::Boolean(left >= right),
        "<=" => Object::Boolean(left <= right),
        _ => panic!("incompatible integer operation"),
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
