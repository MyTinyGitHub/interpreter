//! Monkey Interpreter — Entry Point
//!
//! This is a Rust implementation of the Monkey programming language interpreter,
//! following Thorsten Ball's "Writing an Interpreter in Go". The interpreter uses
//! a classic three-stage pipeline:
//!
//! ```text
//! Source Code → Lexer → Parser → Evaluator → Output
//! ```
//!
//! # Modules
//!
//! - `lexer`: Tokenizes source code into a stream of tokens
//! - `parser`: Builds an AST from tokens using Pratt parsing
//! - `evaluator`: Tree-walking interpreter that computes values
//! - `ast`: AST node definitions
//! - `token`: Token types and precedence levels
//! - `error`: Error types for all stages
//! - `repl`: Read-Eval-Print Loop

pub mod ast;
pub mod error;
pub mod evaluator;
pub mod lexer;
pub mod parser;
pub mod repl;
pub mod token;

fn main() {
    repl::repl_loop();
}
