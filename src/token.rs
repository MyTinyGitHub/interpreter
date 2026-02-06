#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Illegal,
    Eof,
    Ident(String),
    If(String),
    Else(String),
    Return(String),
    True(String),
    False(String),
    Int(String),

    Assign(String),
    Equal(String),
    Notequal(String),

    Plus(String),
    Minus(String),
    Bang(String),
    Asterisk(String),
    Slash(String),

    Gt(String),
    Lt(String),

    Comma(String),
    Semicolon(String),
    Lparen(String),
    Rparen(String),
    Lbrace(String),
    Rbrace(String),
    Function(String),
    Let(String),
}
