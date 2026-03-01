#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i64)]
pub enum Precedence {
    Lowest = 1,
    Equals = 2,
    Lessgreater = 3,
    Sum = 4,
    Product = 5,
    Prefix = 6,
    Call = 7,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Token {
    Start,
    Lessgreater,
    Sum,
    Product,
    Prefix,
    Call,

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
    pub value: String,
}

impl TokenLiteral {
    pub fn new(token: Token, value: String) -> Self {
        Self { token, value }
    }

    pub fn precedence(&self) -> Precedence {
        match self.token {
            Token::Equal | Token::Notequal => Precedence::Equals,
            Token::Lt | Token::Gt => Precedence::Lessgreater,
            Token::Plus | Token::Minus => Precedence::Sum,
            Token::Asterisk | Token::Slash => Precedence::Product,
            Token::Lparen => Precedence::Call,
            _ => Precedence::Lowest,
        }
    }
}
