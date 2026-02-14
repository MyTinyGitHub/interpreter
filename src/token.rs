#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Start,

    Illegal,
    Eof,
    Ident,
    If,
    Else,
    Return,
    True,
    False,
    Int,

    Assign,
    Equal,
    Notequal,

    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    Gt,
    Lt,

    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Function,
    Let,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TokenLiteral {
    pub token: Token,
    pub value: Option<String>,
}

impl TokenLiteral {
    pub fn new(token: Token, value: Option<String>) -> Self {
        Self { token, value }
    }
}
