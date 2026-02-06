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

impl Token {
    pub fn value(&self) -> Option<&str> {
        match self {
            Token::Illegal | Token::Eof => None,
            Token::Ident(s)
            | Token::If(s)
            | Token::Else(s)
            | Token::Return(s)
            | Token::True(s)
            | Token::False(s)
            | Token::Int(s)
            | Token::Assign(s)
            | Token::Equal(s)
            | Token::Notequal(s)
            | Token::Plus(s)
            | Token::Minus(s)
            | Token::Bang(s)
            | Token::Asterisk(s)
            | Token::Slash(s)
            | Token::Gt(s)
            | Token::Lt(s)
            | Token::Comma(s)
            | Token::Semicolon(s)
            | Token::Lparen(s)
            | Token::Rparen(s)
            | Token::Lbrace(s)
            | Token::Rbrace(s)
            | Token::Function(s)
            | Token::Let(s) => Some(s.as_str()),
        }
    }
}
