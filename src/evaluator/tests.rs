use crate::{
    ast::Node,
    error::MonkeyError,
    evaluator::{Object, eval},
    lexer::Lexer,
    parser::Parser,
};

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
        let object = test_eval(input)?;
        assert!(test_integer_object(object, expectation));
    }

    Ok(())
}

#[test]
fn test_bang_operation_eval() -> Result<(), MonkeyError> {
    let inputs = ["!true", "!false", "!5", "!!true", "!!false", "!!5"];
    let expects = [false, true, false, true, false, true];

    for (input, expectation) in inputs.iter().zip(expects) {
        println!("testing {}", input);
        let object = test_eval(input)?;
        assert!(test_boolean_object(object, expectation));
    }

    Ok(())
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
    ];

    let expects = [
        true, false, true, false, false, false, true, false, false, true, true, true, false, true,
        true, true, false, false, true,
    ];

    for (input, expectation) in inputs.iter().zip(expects) {
        let object = test_eval(input)?;
        assert!(test_boolean_object(object, expectation));
    }

    Ok(())
}

fn test_eval(input: &str) -> Result<Object, MonkeyError> {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program()?;

    Ok(eval(&Node::Program(program)))
}

fn test_integer_object(object: Object, expect: i64) -> bool {
    match object {
        Object::Integer(val) => val == expect,
        _ => false,
    }
}

fn test_boolean_object(object: Object, expect: bool) -> bool {
    match object {
        Object::Boolean(val) => val == expect,
        _ => false,
    }
}
