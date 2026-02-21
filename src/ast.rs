use crate::token::TokenLiteral;
use std::any::Any;

pub trait Node {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
}

pub trait Statement: Node {
    fn statement_node(&self);
    fn as_any(&self) -> &dyn Any;
    //fn value(&self) -> &dyn Expression;
}

pub trait Expression: Node {
    fn expression_node(&self);
    fn as_any(&self) -> &dyn Any;
}

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

#[derive(Debug)]
pub struct Identifier {
    pub token: TokenLiteral,
    pub value: String,
}

#[derive(Debug)]
pub struct IntegerLiteral {
    pub token: TokenLiteral,
    pub value: i64,
}

pub struct LetStatement {
    pub token: TokenLiteral,
    pub name: Identifier,
    pub value: Option<Box<dyn Expression>>,
}

pub struct ReturnStatement {
    pub token: TokenLiteral,
    pub value: Option<Box<dyn Expression>>,
}

pub struct ExpressionStatement {
    pub token: TokenLiteral,
    pub value: Option<Box<dyn Expression>>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }
}

impl LetStatement {
    pub fn new(token: &TokenLiteral, identifier: &TokenLiteral) -> Self {
        Self {
            token: token.clone(),
            name: Identifier {
                token: identifier.clone(),
                value: identifier.value.clone().unwrap(),
            },
            value: None,
        }
    }
}

impl ExpressionStatement {
    pub fn new(token: &TokenLiteral) -> Self {
        Self {
            token: token.clone(),
            value: None,
        }
    }
}

impl IntegerLiteral {
    pub fn new(token: &TokenLiteral, value: i64) -> Self {
        Self {
            token: token.clone(),
            value,
        }
    }
}

impl ReturnStatement {
    pub fn new(token: &TokenLiteral) -> Self {
        Self {
            token: token.clone(),
            value: None,
        }
    }
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.is_empty() {
            "".to_owned()
        } else {
            self.statements[0].token_literal()
        }
    }

    fn string(&self) -> String {
        self.statements
            .iter()
            .map(|s| s.string())
            .collect::<Vec<_>>()
            .join("")
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) {}
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Statement for ReturnStatement {
    fn statement_node(&self) {}
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Statement for ExpressionStatement {
    fn statement_node(&self) {}
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Statement for IntegerLiteral {
    fn statement_node(&self) {}
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.value.clone().unwrap().to_string()
    }
    fn string(&self) -> String {
        format!(
            "{} {};",
            self.token_literal(),
            self.value.as_ref().map_or(String::new(), |v| v.string())
        )
    }
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.value.clone().unwrap().to_string()
    }

    fn string(&self) -> String {
        format!(
            "{} {} = {};",
            self.token_literal(),
            self.name.string(),
            self.value.as_ref().map_or(String::new(), |v| v.string())
        )
    }
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.value.clone().unwrap().to_string()
    }

    fn string(&self) -> String {
        self.value.as_ref().map_or(String::new(), |v| v.string())
    }
}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> String {
        self.token.value.clone().unwrap()
    }

    fn string(&self) -> String {
        self.token.value.clone().unwrap()
        //self.value.map_or(String::new(), |v| v.string())
    }
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.value.clone().unwrap().to_string()
    }

    fn string(&self) -> String {
        self.value.clone()
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {}
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for IntegerLiteral {
    fn expression_node(&self) {}
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod test {
    use crate::{
        ast::{Identifier, LetStatement, Node, Program},
        token::{Token, TokenLiteral},
    };

    #[test]
    fn test_string() {
        let input = Program {
            statements: vec![Box::new(LetStatement {
                token: TokenLiteral {
                    token: Token::Let,
                    value: Some("let".to_owned()),
                },
                name: Identifier {
                    token: TokenLiteral {
                        token: Token::Ident,
                        value: Some("myVar".to_owned()),
                    },
                    value: "myVar".to_owned(),
                },
                value: Some(Box::new(Identifier {
                    token: TokenLiteral {
                        token: Token::Ident,
                        value: Some("anotherVar".to_owned()),
                    },
                    value: "anotherVar".to_owned(),
                })),
            })],
        };

        assert_eq!(input.string(), "let myVar = anotherVar;")
    }
}
