use crate::{
    ast::{Expression, IfExpression, Node, Statement},
    error::MonkeyError,
};

type ObjectType = String;

#[cfg(test)]
pub mod tests;

#[derive(Debug)]
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

pub fn eval(node: &Node) -> Result<Option<Object>, MonkeyError> {
    let result: Option<Object> = match node {
        Node::Program(prog) => eval_statements(&prog.statements)?,
        Node::Statement(stmt) => match stmt {
            Statement::Expression(expr) => match expr {
                Expression::IntegerLiteral(integer) => Some(Object::Integer(integer.value)),
                Expression::Boolean(boolean) => Some(Object::Boolean(boolean.value)),
                Expression::Prefix(prefix) => {
                    let expression = *prefix.right.clone();
                    let right = eval_unwrap(expression)?;

                    Some(eval_prefix(&prefix.operator, right)?)
                }
                Expression::Infix(infix) => {
                    let right = eval_unwrap(*infix.right.clone())?;
                    let left = eval_unwrap(*infix.left.clone())?;

                    Some(eval_infix(&infix.operator, left, right)?)
                }
                Expression::If(if_expr) => eval_if(if_expr)?,
                _ => {
                    return Err(MonkeyError::Evaluator(
                        "expression not covered yet".to_owned(),
                    ));
                }
            },
            Statement::Block(block) => eval_statements(&block.statements)?,
            _ => {
                return Err(MonkeyError::Evaluator(
                    "statement not covered yet".to_owned(),
                ));
            }
        },
    };

    Ok(result)
}

pub fn eval_if(expr: &IfExpression) -> Result<Option<Object>, MonkeyError> {
    let cond = eval_unwrap(*expr.condition.clone())?;

    if is_truthy(cond) {
        eval(&Node::block(expr.consequence.clone()))
    } else {
        match &expr.alternative {
            Some(alt) => eval(&Node::block(alt.clone())),
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

pub fn eval_unwrap(expr: Expression) -> Result<Object, MonkeyError> {
    eval(&Node::statement(expr))?
        .ok_or_else(|| MonkeyError::Evaluator("value expected but not found".to_owned()))
}

pub fn eval_infix(operator: &str, left: Object, right: Object) -> Result<Object, MonkeyError> {
    let result = match (left, right) {
        (Object::Integer(l), Object::Integer(r)) => eval_integer_infix(operator, l, r)?,
        (Object::Boolean(l), Object::Boolean(r)) => eval_boolean_infix(operator, l, r)?,
        _ => {
            return Err(MonkeyError::Evaluator(
                "incompatible infix objects".to_owned(),
            ));
        }
    };

    Ok(result)
}

pub fn eval_boolean_infix(operator: &str, left: bool, right: bool) -> Result<Object, MonkeyError> {
    let result = match operator {
        "==" => Object::Boolean(left == right),
        "!=" => Object::Boolean(left != right),
        _ => {
            return Err(MonkeyError::Evaluator(
                "invalid boolean infix operator".to_owned(),
            ));
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
        _ => Err(MonkeyError::Evaluator(
            "invalid minus prefix operator".to_owned(),
        )),
    }
}

pub fn eval_bang_operator_expresion(object: Object) -> Result<Object, MonkeyError> {
    match object {
        Object::Boolean(boolean) => Ok(Object::Boolean(!boolean)),
        _ => Ok(Object::Boolean(false)),
    }
}

pub fn eval_statements(statements: &Vec<Statement>) -> Result<Option<Object>, MonkeyError> {
    let mut result: Option<Object> = None;

    for statement in statements {
        result = eval(&Node::Statement(statement.clone()))?;
    }

    Ok(result)
}
