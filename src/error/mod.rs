//! Error types for the Monkey interpreter.
//!
//! All errors in the pipeline flow through this enum. Each variant
//! represents an error from a specific stage:
//!
//! - `Lexer`: Tokenization failures
//! - `Parser`: Syntactic errors (unexpected tokens, etc.)
//! - `Evaluator`: Runtime errors (unknown operators, undefined identifiers)

use thiserror::Error;

#[derive(Error, Debug)]
pub enum MonkeyError {
    #[error("lexer: {0}")]
    Lexer(String),
    #[error("parser: {0}")]
    Parser(String),
    #[error("{0}")]
    Evaluator(String),
}
