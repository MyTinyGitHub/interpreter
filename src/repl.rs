//! Read-Eval-Print Loop for the Monkey interpreter.
//!
//! Provides an interactive REPL that reads user input, passes it through
//! the lexer→parser→evaluator pipeline, and prints the result. The environment
//! is shared across REPL sessions, so variables persist between commands.
//!
//! # Flow
//!
//! 1. Read a line of input from stdin
//! 2. Lex the input into tokens
//! 3. Parse tokens into an AST
//! 4. Evaluate the AST in the shared environment
//! 5. Print the resulting object (or error)
//!
//! # Exit
//!
//! Typing "exit" terminates the REPL.

use std::io;

use crate::{
    ast::Node,
    evaluator::{environment::Environment, eval},
    lexer::Lexer,
    parser::Parser,
};

pub fn repl_loop() {
    let mut env = Environment::default();

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

        let obj = match eval(&Node::Program(program), &mut env) {
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
