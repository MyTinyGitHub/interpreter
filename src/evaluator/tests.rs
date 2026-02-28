use crate::{
    ast::{Node, Program},
    evaluator::{Object, eval},
    lexer::Lexer,
    parser::Parser,
};

#[test]
fn test_integer_eval() {
    let inputs = ["5", "10"];
    let expects = [5, 10];

    for (input, expectation) in inputs.iter().zip(expects) {
        let object = test_eval(input);
        assert!(test_integer_object(object.unwrap(), expectation));
    }
}

#[test]
fn test_boolean_eval() {
    let inputs = ["true", "false"];
    let expects = [true, false];

    for (input, expectation) in inputs.iter().zip(expects) {
        let object = test_eval(input);
        assert!(test_boolean_object(object.unwrap(), expectation));
    }
}

fn test_eval(input: &str) -> Option<Object> {
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
