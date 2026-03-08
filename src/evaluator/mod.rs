//! Tree-walking evaluator for the Monkey language.
//!
//! Recursively walks the AST and computes values. This is a naive interpreter—
//! no bytecode, no JIT, no compilation. Each AST node has a corresponding
//! evaluation function.
//!
//! # Evaluation Flow
//!
//! 1. `eval()` dispatches on node type
//! 2. For expressions: evaluate sub-expressions, apply operations
//! 3. For statements: evaluate in sequence, handle return/let specially
//! 4. Environment provides variable bindings
//!
//! # Truthiness
//!
//! Only `false` is falsy. All other values (including `0` and `null`) are truthy.

use crate::{
    ast::{BlockStatement, Expression, Identifier, IfExpression, Node, Program, Statement},
    error::MonkeyError,
    evaluator::{
        environment::Environment,
        object::{Function, Object},
    },
};

pub mod environment;
pub mod object;

#[cfg(test)]
pub mod tests;

/// Evaluates an AST node and returns the resulting Object.
///
/// This is the main entry point for evaluation. It pattern matches on the
/// node type and dispatches to specialized evaluation functions:
/// - `Program` → `eval_program`
/// - `Statement` → statement-specific evaluation
/// - `Expression` → expression-specific evaluation
///
/// # Arguments
/// - `node`: The AST node to evaluate
/// - `env`: Mutable reference to the environment (for variable bindings)
///
/// # Returns
/// - `Ok(Some(Object))`: A value was produced
/// - `Ok(None)`: No value produced (e.g., empty program)
/// - `Err(MonkeyError)`: An error occurred (type mismatch, unknown operator, etc.)
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
                Expression::Function(func) => Some(Object::Function(Function {
                    parameters: func.parameters.clone(),
                    body: func.body.clone(),
                    env: env.clone(),
                })),
                Expression::Call(call) => {
                    let func = eval_unwrap(*call.function.clone(), env)?;

                    let args = eval_expressions(call.arguments.clone(), env)?;

                    apply_function(func, args)?
                }
            },
            Statement::Block(block) => eval_block_statements(block, env)?,
            Statement::Return(retrun_stmt) => {
                let expr = eval_unwrap(retrun_stmt.value.clone(), env)?;
                Some(Object::Return(Box::new(expr)))
            }
            Statement::Let(let_stmt) => {
                let value = eval(&Node::statement(let_stmt.value.clone()), env)?;
                if let Some(val) = &value {
                    env.set(let_stmt.name.token.literal(), val.clone());
                }
                value
            }
        },
    };

    Ok(result)
}

pub fn apply_function(object: Object, args: Vec<Object>) -> Result<Option<Object>, MonkeyError> {
    let func = match object {
        Object::Function(func) => func,
        _ => {
            return Err(MonkeyError::Evaluator(format!(
                "not a function {}",
                object.obj_type()
            )));
        }
    };

    let mut extended_env = func.env.extend_func_env(&func, args);
    let evaluated = eval(&Node::block(func.body.clone()), &mut extended_env)?;

    Ok(evaluated)
}

pub fn eval_expressions(
    exprs: Vec<Expression>,
    env: &mut Environment,
) -> Result<Vec<Object>, MonkeyError> {
    let mut result = vec![];

    for expr in exprs {
        let evaluation = eval(&Node::statement(expr.clone()), env)?;
        if let Some(v) = evaluation {
            result.push(v);
        }
    }

    Ok(result)
}

pub fn eval_identifier(
    ident: &Identifier,
    env: &Environment,
) -> Result<Option<Object>, MonkeyError> {
    let val = env.get(ident.token.literal());

    match val {
        Some(val) => Ok(Some(val.clone())),
        None => {
            let error_msg = format!("identifier not found: {}", ident.token.literal());
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

        if let Some(Object::Return(_)) = result {
            return Ok(result);
        }
    }

    Ok(result)
}
