use crate::{
    ast::Node,
    error::MonkeyError,
    evaluator::{Environment, Object, eval},
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

        assert!(test_integer_object(object, expectation));
    }

    Ok(())
}

#[test]
fn environment_test() -> Result<(), MonkeyError> {
    let inputs = [
        "let a = 6; a;",
        "let a = 5 * 5; a;",
        "let a = 5; let b = a; b;",
        "let a = 5; let b = a; let c = a + b + 5; c;",
    ];

    let expectation = [6, 25, 5, 15];

    for (input, expect) in inputs.iter().zip(expectation) {
        let result = test_eval(input)?.unwrap();

        assert!(test_integer_object(result, expect));
    }

    Ok(())
}

#[test]
fn test_closure() -> Result<(), MonkeyError> {
    let input = r#"
        let newAdder = fn(x) {
            fn(y) { x + y };
        };
        let addTwo = newAdder(2);
        addTwo(2);
    "#;

    assert!(test_integer_object(test_eval(input)?.unwrap(), 4));
    Ok(())
}

#[test]
fn test_function() -> Result<(), MonkeyError> {
    let inputs = [
        "let identity = fn(x) { x; }; identity(5);",
        "let identity = fn(x) { return x; }; identity(5);",
        "let double = fn(x) { x * 2; }; double(5);",
        "let add = fn(x, y) { x + y; }; add(5, 5);",
        "let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5));",
        "fn(x) { x; }(5)",
    ];

    let expectations = [5, 5, 10, 10, 20, 5];

    for (input, expect) in inputs.iter().zip(expectations) {
        let result = test_eval(input)?;

        assert!(test_integer_object(result.unwrap(), expect));
    }

    Ok(())
}

#[test]
fn test_function_object() -> Result<(), MonkeyError> {
    let input = "fn(x) { x + 2; }";
    let eval = test_eval(input)?;

    match eval {
        Some(Object::Function(func)) => {
            assert_eq!(func.parameters.len(), 1);
            assert_eq!(func.parameters[0].token.literal(), "x");
            assert_eq!(func.body.string(), "(x + 2)");
            Ok(())
        }
        _ => panic!("Expected a function"),
    }
}

#[test]
fn error_handling_test() {
    let inputs = [
        "5 + true;",
        "foobar",
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
        "identifier not found: foobar",
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

        assert!(test_boolean_object(object, expectation));
    }

    Ok(())
}

fn test_eval(input: &str) -> Result<Option<Object>, MonkeyError> {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program()?;
    let mut env = Environment::default();

    eval(&Node::Program(program), &mut env)
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
