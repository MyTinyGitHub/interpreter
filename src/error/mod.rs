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
