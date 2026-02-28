use crate::repl::repl_loop;

pub mod ast;
pub mod evaluator;
pub mod lexer;
pub mod parser;
pub mod repl;
pub mod token;

fn main() {
    repl_loop();
}
