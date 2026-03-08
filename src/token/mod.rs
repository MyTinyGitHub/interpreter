use std::hash::{Hash, Hasher};

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

#[derive(Debug, Eq, Clone)]
pub enum Token {
    Start,
    Sum,
    Product,
    Prefix,
    Call,

    Illegal,
    Eof,
    Ident(String),
    If,
    Else,
    Return,
    True,
    False,
    Int(String),

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

impl Token {
    pub fn literal(&self) -> &str {
        match self {
            Self::Start => "",
            Self::Sum => "+",
            Self::Product => "*",
            Self::Prefix => "",
            Self::Call => "",
            Self::Illegal => "",
            Self::Eof => "",
            Self::Ident(ident) => ident,
            Self::If => "if",
            Self::Else => "else",
            Self::Return => "return",
            Self::True => "true",
            Self::False => "false",
            Self::Int(int) => int,

            Self::Assign => "=",
            Self::Equal => "==",
            Self::Notequal => "!=",

            Self::Plus => "+",
            Self::Minus => "-",
            Self::Bang => "!",
            Self::Asterisk => "*",
            Self::Slash => "/",

            Self::Gt => ">",
            Self::Lt => "<",

            Self::Comma => ",",
            Self::Semicolon => ";",
            Self::Lparen => "(",
            Self::Rparen => ")",
            Self::Lbrace => "{",
            Self::Rbrace => "}",
            Self::Function => "fn",
            Self::Let => "let",
        }
    }

    pub fn precedence(&self) -> Precedence {
        match self {
            Token::Equal | Token::Notequal => Precedence::Equals,
            Token::Lt | Token::Gt => Precedence::Lessgreater,
            Token::Plus | Token::Minus => Precedence::Sum,
            Token::Asterisk | Token::Slash => Precedence::Product,
            Token::Lparen => Precedence::Call,
            _ => Precedence::Lowest,
        }
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

impl Hash for Token {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
    }
}
