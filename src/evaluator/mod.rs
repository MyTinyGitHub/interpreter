use core::error;
use std::collections::HashMap;

use crate::{
    ast::{BlockStatement, Expression, Identifier, IfExpression, Node, Program, Statement},
    error::MonkeyError,
};

type ObjectType = String;

#[cfg(test)]
pub mod tests;

#[derive(Debug, Clone)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    Return(Box<Object>),
    Null,
}

#[derive(Debug, Default)]
pub struct Environment {
    store: HashMap<String, Object>,
}

impl Environment {
    pub fn get(&self, value: &str) -> Option<&Object> {
        self.store.get(value)
    }

    pub fn set(&mut self, value: &str, object: Object) {
        self.store.insert(value.to_string(), object);
    }
}

impl Object {
    pub fn obj_type(&self) -> ObjectType {
        match self {
            Self::Integer(_) => "INTEGER".to_string(),
            Self::Boolean(_) => "BOOLEAN".to_string(),
            Self::Return(_) => "RETURN_VALUE".to_string(),
            Self::Null => "NULL".to_string(),
        }
    }

    pub fn inspect(&self) -> String {
        match self {
            Self::Integer(val) => val.to_string(),
            Self::Boolean(val) => val.to_string(),
            Self::Return(val) => val.inspect(),
            Self::Null => "null".to_string(),
        }
    }
}

pub fn eval(node: &Node, env: &mut Environment) -> Result<Option<Object>, MonkeyError> {
    let result: Option<Object> = match node {
        Node::Program(prog) => eval_program(prog, env)?,
        Node::Statement(stmt) => match stmt {
            Statement::Expression(expr) => match expr {
                Expression::Identifier(ident) => eval_identifier(ident, env)?,
                Expression::IntegerLiteral(integer) => Some(Object::Integer(integer.value)),
                Expression::Boolean(boolean) => Some(Object::Boolean(boolean.value)),
                Expression::Prefix(prefix) => {
                    let expression = *prefix.right.clone();
                    let right = eval_unwrap(expression, env)?;

                    Some(eval_prefix(&prefix.operator, right)?)
                }
                Expression::Infix(infix) => {
                    let right = eval_unwrap(*infix.right.clone(), env)?;
                    let left = eval_unwrap(*infix.left.clone(), env)?;

                    Some(eval_infix(&infix.operator, left, right)?)
                }
                Expression::If(if_expr) => eval_if(if_expr, env)?,
                _ => {
                    return Err(MonkeyError::Evaluator(
                        "expression not covered yet".to_owned(),
                    ));
                }
            },
            Statement::Block(block) => eval_block_statements(block, env)?,
            Statement::Return(retrun_stmt) => {
                let expr = eval_unwrap(retrun_stmt.value.clone(), env)?;
                Some(Object::Return(Box::new(expr)))
            }
            Statement::Let(let_stmt) => {
                let value = eval(&Node::statement(let_stmt.value.clone()), env)?;
                env.set(&let_stmt.name.value, value.clone().unwrap());
                value
            }
        },
    };

    Ok(result)
}
pub fn eval_identifier(
    ident: &Identifier,
    env: &Environment,
) -> Result<Option<Object>, MonkeyError> {
    let val = env.get(&ident.value);

    match val {
        Some(val) => Ok(Some(val.clone())),
        None => {
            let error_msg = format!("identifier not found: {}", ident.value);
            Err(MonkeyError::Evaluator(error_msg))
        }
    }
}

pub fn eval_if(expr: &IfExpression, env: &mut Environment) -> Result<Option<Object>, MonkeyError> {
    let cond = eval_unwrap(*expr.condition.clone(), env)?;

    if is_truthy(cond) {
        eval(&Node::block(expr.consequence.clone()), env)
    } else {
        match &expr.alternative {
            Some(alt) => eval(&Node::block(alt.clone()), env),
            None => Ok(None),
        }
    }
}

pub fn is_truthy(obj: Object) -> bool {
    match obj {
        Object::Boolean(b) => b,
        _ => true,
    }
}

pub fn eval_unwrap(expr: Expression, env: &mut Environment) -> Result<Object, MonkeyError> {
    eval(&Node::statement(expr), env)?
        .ok_or_else(|| MonkeyError::Evaluator("value expected but not found".to_owned()))
}

pub fn eval_infix(operator: &str, left: Object, right: Object) -> Result<Object, MonkeyError> {
    let result = match (&left, &right) {
        (Object::Integer(l), Object::Integer(r)) => eval_integer_infix(operator, *l, *r)?,
        (Object::Boolean(l), Object::Boolean(r)) => eval_boolean_infix(operator, *l, *r)?,
        _ => {
            let error_msg = format!("type mismatch: {} + {}", left.obj_type(), right.obj_type());
            return Err(MonkeyError::Evaluator(error_msg));
        }
    };

    Ok(result)
}

pub fn eval_boolean_infix(operator: &str, left: bool, right: bool) -> Result<Object, MonkeyError> {
    let result = match operator {
        "==" => Object::Boolean(left == right),
        "!=" => Object::Boolean(left != right),
        _ => {
            let error_msg = format!("unknown operator: BOOLEAN {} BOOLEAN", operator);
            return Err(MonkeyError::Evaluator(error_msg));
        }
    };

    Ok(result)
}

pub fn eval_integer_infix(operator: &str, left: i64, right: i64) -> Result<Object, MonkeyError> {
    let result = match operator {
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
        _ => {
            return Err(MonkeyError::Evaluator(
                "invalid integer operator".to_owned(),
            ));
        }
    };

    Ok(result)
}

pub fn eval_prefix(operator: &str, object: Object) -> Result<Object, MonkeyError> {
    let result = match operator {
        "!" => eval_bang_operator_expresion(object)?,
        "-" => eval_minus_operator_expresion(object)?,
        _ => return Err(MonkeyError::Evaluator("invalid prefix operator".to_owned())),
    };

    Ok(result)
}

pub fn eval_minus_operator_expresion(object: Object) -> Result<Object, MonkeyError> {
    match object {
        Object::Integer(value) => Ok(Object::Integer(-value)),
        _ => {
            let error_msg = format!("unknown operator: -{}", object.obj_type());
            Err(MonkeyError::Evaluator(error_msg))
        }
    }
}

pub fn eval_bang_operator_expresion(object: Object) -> Result<Object, MonkeyError> {
    match object {
        Object::Boolean(boolean) => Ok(Object::Boolean(!boolean)),
        _ => Ok(Object::Boolean(false)),
    }
}

pub fn eval_program(
    program: &Program,
    env: &mut Environment,
) -> Result<Option<Object>, MonkeyError> {
    let mut result: Option<Object> = None;

    for statement in program.statements.iter() {
        result = eval(&Node::Statement(statement.clone()), env)?;

        if let Some(Object::Return(res)) = result {
            return Ok(Some(*res));
        }
    }

    Ok(result)
}

pub fn eval_block_statements(
    block_statement: &BlockStatement,
    env: &mut Environment,
) -> Result<Option<Object>, MonkeyError> {
    let mut result: Option<Object> = None;

    for statement in block_statement.statements.iter() {
        result = eval(&Node::Statement(statement.clone()), env)?;

        println!("{:?}", result);

        if let Some(Object::Return(_)) = result {
            return Ok(result);
        }
    }

    Ok(result)
}
