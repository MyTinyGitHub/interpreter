use crate::{
    ast::Node,
    evaluator::{Object, eval},
    lexer::Lexer,
    parser::Parser,
};

#[test]
fn test_integer_eval() {
    let inputs = ["5", "10", "-5", "-10"];
    let expects = [5, 10, -5, -10];

    for (input, expectation) in inputs.iter().zip(expects) {
        let object = test_eval(input);
        assert!(test_integer_object(object, expectation));
    }
}

#[test]
fn test_bang_operation_eval() {
    let inputs = ["!true", "!false", "!5", "!!true", "!!false", "!!5"];
    let expects = [false, true, false, true, false, true];

    for (input, expectation) in inputs.iter().zip(expects) {
        println!("testing {}", input);
        let object = test_eval(input);
        assert!(test_boolean_object(object, expectation));
    }
}

#[test]
fn test_boolean_eval() {
    let inputs = ["true", "false"];
    let expects = [true, false];

    for (input, expectation) in inputs.iter().zip(expects) {
        let object = test_eval(input);
        assert!(test_boolean_object(object, expectation));
    }
}

fn test_eval(input: &str) -> Object {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    eval(&Node::Program(parser.parse_program()))
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
