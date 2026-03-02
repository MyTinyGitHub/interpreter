use std::io;

use crate::{ast::Node, evaluator::eval, lexer::Lexer, parser::Parser};

pub fn repl_loop() {
    loop {
        let mut input: String = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        input = input.trim_end().to_string();
        if input.as_str() == "exit" {
            break;
        }

        let token_processor = Lexer::new(&input);
        let mut parser = Parser::new(token_processor);
        let program = match parser.parse_program() {
            Ok(program) => program,
            Err(error) => {
                println!("{}", error);
                continue;
            }
        };

        let obj = match eval(&Node::Program(program)) {
            Ok(obj) => obj,
            Err(error) => {
                println!("{}", error);
                continue;
            }
        };

        match obj {
            Some(v) => println!("{}\n", v.inspect()),
            _ => continue,
        }
    }
}
