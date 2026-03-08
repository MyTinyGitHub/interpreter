use core::panic;

use crate::{
    ast::{Expression, Statement},
    error::MonkeyError,
    lexer::Lexer,
    parser::Parser,
    token::Token,
};

struct InfixTestValue {
    left: Box<TestValue>,
    operator: String,
    right: Box<TestValue>,
}

struct PrefixTestValue {
    operator: String,
    right: Box<TestValue>,
}

enum TestValue {
    String(String),
    Infix(InfixTestValue),
    Prefix(PrefixTestValue),
}

#[test]
fn test_parser() -> Result<(), MonkeyError> {
    let input = r#"
            let x = 5;
            let y = 10;
            let foobar = 838383;
        "#;

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program()?;

    let expected = ["x", "y", "foobar"];

    assert_eq!(program.statements.len(), expected.len());

    for (statement, name) in program.statements.iter().zip(expected.iter()) {
        test_let_statement(statement, name);
    }

    Ok(())
}

#[test]
fn test_let_statements() -> Result<(), MonkeyError> {
    let inputs = ["let x = 5;", "let y = true;", "let foobar = y;"];
    let expectations = [vec!["x", "5"], vec!["y", "true"], vec!["foobar", "y"]];

    for (input, expectation) in inputs.iter().zip(expectations) {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program()?;

        assert_eq!(program.statements.len(), 1);

        let statement = &program.statements[0];

        match statement {
            Statement::Let(s) => {
                let idf = &s.name;
                assert_eq!(idf.token.literal(), expectation[0]);

                let value = &s.value;
                test_expression(value, TestValue::String(expectation[1].to_string()));
            }
            _ => panic!(),
        }
    }

    Ok(())
}

#[test]
fn test_identifier_expression() -> Result<(), MonkeyError> {
    let input = "foobar;";

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program()?;

    assert_eq!(program.statements.len(), 1);

    let statement = &program.statements[0];

    match statement {
        Statement::Expression(s) => {
            test_expression(s, TestValue::String("foobar".to_string()));
        }
        _ => panic!(),
    }

    Ok(())
}

#[test]
fn test_integer_expression() -> Result<(), MonkeyError> {
    let input = "5;";

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program()?;

    assert_eq!(program.statements.len(), 1);

    let statement = &program.statements[0];

    match statement {
        Statement::Expression(s) => test_expression(s, TestValue::String(5.to_string())),
        _ => panic!(),
    }

    Ok(())
}

#[test]
fn test_boolean_expression() -> Result<(), MonkeyError> {
    let inputs = ["true;", "false"];
    let results = [true, false];

    for (input, result) in inputs.iter().zip(results) {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program()?;

        assert_eq!(program.statements.len(), 1);

        let statement = &program.statements[0];

        match statement {
            Statement::Expression(s) => test_expression(s, TestValue::String(result.to_string())),
            _ => panic!(),
        }
    }

    Ok(())
}

fn test_expression(expression: &Expression, value: TestValue) {
    match value {
        TestValue::String(val) => match expression {
            Expression::IntegerLiteral(expr) => assert_eq!(expr.value.to_string(), val),
            Expression::Identifier(expr) => assert_eq!(expr.token.literal().to_string(), val),
            Expression::Boolean(expr) => assert_eq!(expr.value.to_string(), val),
            _ => panic!(),
        },
        TestValue::Infix(value) => match expression {
            Expression::Infix(expr) => {
                test_expression(expr.left.as_ref(), *value.left);
                assert_eq!(expr.operator, value.operator);
                test_expression(expr.right.as_ref(), *value.right);
            }
            _ => panic!(),
        },
        TestValue::Prefix(value) => match expression {
            Expression::Prefix(expr) => {
                assert_eq!(expr.operator, value.operator);
                test_expression(expr.right.as_ref(), *value.right);
            }
            _ => panic!(),
        },
    }
}

#[test]
fn test_return() -> Result<(), MonkeyError> {
    let input = r#"
            return 5;
            return 10;
            return 993322;
        "#;

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program()?;

    for statement in program.statements.iter() {
        test_return_statement(statement);
    }

    Ok(())
}

#[test]
fn test_function_params() -> Result<(), MonkeyError> {
    let inputs = ["fn(){}", "fn(x){}", "fn(x, y){}"];
    let exp_params = [vec![], vec!["x"], vec!["x", "y"]];

    for (input, expected) in inputs.iter().zip(exp_params) {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program()?;

        let statement = &program.statements[0];

        match statement {
            Statement::Expression(Expression::Function(fun)) => {
                assert_eq!(fun.token, Token::Function);
                for (param, expected) in fun.parameters.iter().zip(expected) {
                    assert_eq!(param.token.literal(), expected);
                }
            }
            _ => panic!(),
        }
    }

    Ok(())
}

#[test]
fn test_call_expresion() -> Result<(), MonkeyError> {
    let input = "add(1, 2 * 3, 4 + 5)";

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program()?;

    let statement = &program.statements[0];

    match statement {
        Statement::Expression(Expression::Call(call)) => {
            test_expression(&call.function, TestValue::String("add".to_string()));

            assert_eq!(call.arguments.len(), 3);

            let argument = &call.arguments[0];
            test_expression(argument, TestValue::String("1".to_string()));

            let argument = &call.arguments[1];
            test_expression(
                argument,
                TestValue::Infix(InfixTestValue {
                    left: Box::new(TestValue::String("2".to_string())),
                    operator: "*".to_string(),
                    right: Box::new(TestValue::String("3".to_string())),
                }),
            );

            let argument = &call.arguments[2];
            test_expression(
                argument,
                TestValue::Infix(InfixTestValue {
                    left: Box::new(TestValue::String("4".to_string())),
                    operator: "+".to_string(),
                    right: Box::new(TestValue::String("5".to_string())),
                }),
            );
        }
        _ => panic!(),
    }

    Ok(())
}

#[test]
fn test_function() -> Result<(), MonkeyError> {
    let input = "fn(a,b,c) { a }";
    let exp_params = vec!["a", "b", "c"];

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program()?;

    let statement = &program.statements[0];

    match statement {
        Statement::Expression(Expression::Function(fun)) => {
            assert_eq!(fun.token, Token::Function);
            for (param, expected) in fun.parameters.iter().zip(exp_params) {
                assert_eq!(param.token.literal(), expected);
            }

            let statement = &fun.body.statements[0];

            match statement {
                Statement::Expression(s) => {
                    test_expression(s, TestValue::String("a".to_string()));
                }
                _ => panic!(),
            }
        }
        _ => panic!(),
    }

    Ok(())
}

#[test]
fn test_prefix_opertor() -> Result<(), MonkeyError> {
    let inputs = ["!5;", "-15;"];
    let expected = [("!", 5), ("-", 15)];

    for (input, (expected_prefix, expected_value)) in inputs.iter().zip(expected) {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program()?;

        let statement = &program.statements[0];

        match statement {
            Statement::Expression(s) => {
                test_expression(
                    s,
                    TestValue::Prefix(PrefixTestValue {
                        operator: expected_prefix.to_string(),
                        right: Box::new(TestValue::String(expected_value.to_string())),
                    }),
                );
            }
            _ => panic!(),
        }
    }

    Ok(())
}

#[test]
fn test_infix_opertor() -> Result<(), MonkeyError> {
    let inputs = [
        "5 + 5;", "5 - 5;", "5 * 5;", "5 / 5;", "5 > 5;", "5 < 5;", "5 == 5;", "5 != 5;",
    ];

    let expected = [
        (5, "+", 5),
        (5, "-", 5),
        (5, "*", 5),
        (5, "/", 5),
        (5, ">", 5),
        (5, "<", 5),
        (5, "==", 5),
        (5, "!=", 5),
    ];

    for (input, (left, operator, right)) in inputs.iter().zip(expected) {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program()?;

        let statement = &program.statements[0];

        match statement {
            Statement::Expression(expression) => {
                test_expression(
                    expression,
                    TestValue::Infix(InfixTestValue {
                        operator: operator.to_string(),
                        left: Box::new(TestValue::String(left.to_string())),
                        right: Box::new(TestValue::String(right.to_string())),
                    }),
                );
            }
            _ => panic!(),
        }
    }

    Ok(())
}

#[test]
fn test_if_else_condition() -> Result<(), MonkeyError> {
    let input = "if (a == b) { y } else { x }";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program()?;

    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Statement::Expression(Expression::If(expr)) => {
            test_expression(
                expr.condition.as_ref(),
                TestValue::Infix(InfixTestValue {
                    operator: "==".to_string(),
                    left: Box::new(TestValue::String("a".to_string())),
                    right: Box::new(TestValue::String("b".to_string())),
                }),
            );
            let statement = &expr.consequence.statements[0];

            match statement {
                Statement::Expression(s) => {
                    test_expression(s, TestValue::String("y".to_string()));
                }
                _ => panic!(),
            }

            let statement = &expr
                .alternative
                .as_ref()
                .expect("Expected alternative value")
                .statements[0];

            match statement {
                Statement::Expression(s) => {
                    test_expression(s, TestValue::String("x".to_string()));
                }
                _ => panic!(),
            }
        }
        _ => panic!(),
    }

    Ok(())
}

#[test]
fn test_infix_opertor_more() -> Result<(), MonkeyError> {
    let inputs = [
        "-a * b",
        "!-a",
        "a + b + c",
        "a + b - c",
        "a * b * c",
        "a * b / c",
        "a + b / c",
        "a + b * c + d / e - f",
        "3 + 4; -5 * 5",
        "5 > 4 == 3 < 4",
        "5 < 4 != 3 > 4",
        "3 + 4 * 5 == 3 * 1 + 4 * 5",
        "3 < 5 == true",
        "3 > 5 == false",
        "1 + (2 + 3) + 4",
        "(5 + 5) * 2",
        "2 / (5 + 5)",
        "-(5 + 5)",
        "!(true == true)",
        "a + add(b * c) + d",
        "add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))",
        "add(a + b + c * d / f + g)",
    ];

    let expected = [
        "((-a) * b)",
        "(!(-a))",
        "((a + b) + c)",
        "((a + b) - c)",
        "((a * b) * c)",
        "((a * b) / c)",
        "(a + (b / c))",
        "(((a + (b * c)) + (d / e)) - f)",
        "(3 + 4)((-5) * 5)",
        "((5 > 4) == (3 < 4))",
        "((5 < 4) != (3 > 4))",
        "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
        "((3 < 5) == true)",
        "((3 > 5) == false)",
        "((1 + (2 + 3)) + 4)",
        "((5 + 5) * 2)",
        "(2 / (5 + 5))",
        "(-(5 + 5))",
        "(!(true == true))",
        "((a + add((b * c))) + d)",
        "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))",
        "add((((a + b) + ((c * d) / f)) + g))",
    ];

    for (input, expected) in inputs.iter().zip(expected) {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program()?;

        assert_eq!(program.string(), expected);
    }

    Ok(())
}

fn test_let_statement(statement: &Statement, name: &str) {
    assert_eq!(statement.token_literal(), "let");
    match statement {
        Statement::Let(stmt) => assert_eq!(stmt.name.token.literal(), name),
        _ => panic!(),
    }
}

fn test_return_statement(statement: &Statement) {
    assert_eq!(statement.token_literal(), "return");
    match statement {
        Statement::Return(_) => (),
        _ => panic!(),
    }
}
