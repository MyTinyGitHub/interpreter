use std::{fmt::format, result};

use crate::{
    ast::Node,
    error::MonkeyError,
    evaluator::{Object, eval},
    lexer::Lexer,
    parser::Parser,
};

#[test]
fn test_if_operator_eval() -> Result<(), MonkeyError> {
    let inputs = [
        "if (true) { 10 }",
        "if (false) { 10 }",
        "if (1) { 10 }",
        "if (1 < 2) { 10 }",
        "if (1 > 2) { 10 }",
        "if (1 > 2) { 10 } else { 20 }",
        "if (1 < 2) { 10 } else { 20 }",
    ];

    let expects = [Some(10), None, Some(10), Some(10), None, Some(20), Some(10)];

    for (input, expectation) in inputs.iter().zip(expects) {
        let object = test_eval(input)?;
        println!("object: {:?}, expect: {:?}", object, expectation);
        assert!(test_optional_integer_object(object, expectation));
    }

    Ok(())
}

#[test]
fn test_integer_eval() -> Result<(), MonkeyError> {
    let inputs = [
        "5",
        "10",
        "-5",
        "-10",
        "5 + 5 + 5 + 5 - 10",
        "2 * 2 * 2 * 2 * 2",
        "-50 + 100 + -50",
        "5 * 2 + 10",
        "5 + 2 * 10",
        "20 + 2 * -10",
        "50 / 2 * 2 + 10",
        "2 * (5 + 10)",
        "3 * 3 * 3 + 10",
        "3 * (3 * 3) + 10",
        "(5 + 10 * 2 + 15 / 3) * 2 + -10",
    ];

    let expects = [5, 10, -5, -10, 10, 32, 0, 20, 25, 0, 60, 30, 37, 37, 50];

    for (input, expectation) in inputs.iter().zip(expects) {
        let object = test_eval(input)?.unwrap();
        println!("object: {:?}, expect: {}", object, expectation);
        assert!(test_integer_object(object, expectation));
    }

    Ok(())
}

#[test]
fn test_bang_operation_eval() -> Result<(), MonkeyError> {
    let inputs = ["!true", "!false", "!5", "!!true", "!!false", "!!5"];
    let expects = [false, true, false, true, false, true];

    for (input, expectation) in inputs.iter().zip(expects) {
        let object = test_eval(input)?.unwrap();
        println!("object: {:?}, expect: {}", object, expectation);
        assert!(test_boolean_object(object, expectation));
    }

    Ok(())
}

#[test]
fn test_return() -> Result<(), MonkeyError> {
    let inputs = [
        "return 10;",
        "return 10; 9;",
        "return 2 * 5; 9;",
        "9; return 2 * 5; 9;",
    ];

    let expected = [10, 10, 10, 10];

    for (input, expectation) in inputs.iter().zip(expected) {
        let object = test_eval(input)?.unwrap();

        println!(
            "input: {} object: {:?}, expect: {}",
            input, object, expectation
        );

        assert!(test_integer_object(object, expectation));
    }

    Ok(())
}

#[test]
fn error_handling_test() {
    let inputs = [
        "5 + true;",
        "5 + true; 5;",
        "-true",
        "true + false;",
        "5; true + false; 5",
        "if (10 > 1) { true + false; }",
        r#"if (10 > 1) {
            if (10 > 1) {
                return true + false;
            }
        return 1;
        }"#,
    ];

    let expectations = [
        "type mismatch: INTEGER + BOOLEAN",
        "type mismatch: INTEGER + BOOLEAN",
        "unknown operator: -BOOLEAN",
        "unknown operator: BOOLEAN + BOOLEAN",
        "unknown operator: BOOLEAN + BOOLEAN",
        "unknown operator: BOOLEAN + BOOLEAN",
    ];

    for (input, expect) in inputs.iter().zip(expectations) {
        let result = test_eval(input);

        let error = format!("{}", result.err().unwrap());

        assert_eq!(error, expect);
    }
}

#[test]
fn test_boolean_eval() -> Result<(), MonkeyError> {
    let inputs = [
        "true",
        "false",
        "1 < 2",
        "1 > 2",
        "1 < 1",
        "1 > 1",
        "1 == 1",
        "1 != 1",
        "1 == 2",
        "1 != 2",
        "true==true",
        "false==false",
        "true==false",
        "true!=false",
        "false!=true",
        "(1 < 2) == true",
        "(1 < 2) == false",
        "(1 > 2) == true",
        "(1 > 2) == false",
        "(5 > 5) == false",
    ];

    let expects = [
        true, false, true, false, false, false, true, false, false, true, true, true, false, true,
        true, true, false, false, true, true,
    ];

    for (input, expectation) in inputs.iter().zip(expects) {
        let object = test_eval(input)?.unwrap();

        println!(
            "input: {} object: {:?}, expect: {}",
            input, object, expectation
        );

        assert!(test_boolean_object(object, expectation));
    }

    Ok(())
}

fn test_eval(input: &str) -> Result<Option<Object>, MonkeyError> {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program()?;

    eval(&Node::Program(program))
}

fn test_integer_object(object: Object, expect: i64) -> bool {
    match object {
        Object::Integer(val) => val == expect,
        _ => false,
    }
}

fn test_optional_integer_object(object: Option<Object>, expect: Option<i64>) -> bool {
    object.map(|v| match v {
        Object::Integer(val) => val,
        _ => panic!("not an integer"),
    }) == expect
}

fn test_boolean_object(object: Object, expect: bool) -> bool {
    match object {
        Object::Boolean(val) => val == expect,
        _ => false,
    }
}
