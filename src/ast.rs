use crate::token::TokenLiteral;

#[derive(Debug)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(ExpressionStatement),
}

#[derive(Debug)]
pub enum Expression {
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    Boolean(BooleanLiteral),
    Prefix(PrefixExpression),
    Infix(InfixExpression),
}

#[derive(Default)]
pub struct Program {
    pub statements: Vec<Statement>,
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

#[derive(Debug)]
pub struct BooleanLiteral {
    pub token: TokenLiteral,
    pub value: bool,
}

#[derive(Debug)]
pub struct LetStatement {
    pub token: TokenLiteral,
    pub name: Identifier,
    pub value: Option<Expression>,
}

#[derive(Debug)]
pub struct ReturnStatement {
    pub token: TokenLiteral,
    pub value: Option<Expression>,
}

#[derive(Debug)]
pub struct PrefixExpression {
    pub token: TokenLiteral,
    pub operator: String,
    pub right: Option<Box<Expression>>,
}

#[derive(Debug)]
pub struct InfixExpression {
    pub token: TokenLiteral,
    pub operator: String,
    pub right: Option<Box<Expression>>,
    pub left: Option<Box<Expression>>,
}

#[derive(Debug)]
pub struct ExpressionStatement {
    pub token: TokenLiteral,
    pub value: Option<Box<Expression>>,
}

impl Program {
    fn token_literal(&self) -> String {
        if self.statements.is_empty() {
            "".to_owned()
        } else {
            self.statements[0].token_literal()
        }
    }

    pub fn string(&self) -> String {
        self.statements
            .iter()
            .map(|s| s.string())
            .collect::<Vec<_>>()
            .join("")
    }
}

impl Expression {
    fn expression_node(&self) {
        match self {
            Self::Identifier(expr) => expr.expression_node(),
            Self::IntegerLiteral(expr) => expr.expression_node(),
            Self::Prefix(expr) => expr.expression_node(),
            Self::Infix(expr) => expr.expression_node(),
            Self::Boolean(expr) => expr.expression_node(),
        }
    }

    pub fn string(&self) -> String {
        match self {
            Self::Identifier(expr) => expr.string(),
            Self::IntegerLiteral(expr) => expr.string(),
            Self::Prefix(expr) => expr.string(),
            Self::Infix(expr) => expr.string(),
            Self::Boolean(expr) => expr.string(),
        }
    }

    fn token_literal(&self) -> String {
        match self {
            Self::Identifier(expr) => expr.token_literal(),
            Self::IntegerLiteral(expr) => expr.token_literal(),
            Self::Prefix(expr) => expr.token_literal(),
            Self::Infix(expr) => expr.token_literal(),
            Self::Boolean(expr) => expr.token_literal(),
        }
    }
}

impl Statement {
    pub fn token_literal(&self) -> String {
        match self {
            Self::Let(stmt) => stmt.token_literal(),
            Self::Return(stmt) => stmt.token_literal(),
            Self::Expression(stmt) => stmt.token_literal(),
        }
    }

    pub fn string(&self) -> String {
        match self {
            Self::Let(stmt) => stmt.string(),
            Self::Return(stmt) => stmt.string(),
            Self::Expression(stmt) => stmt.string(),
        }
    }

    fn statement_node(&self) {}
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

impl ExpressionStatement {
    pub fn new(token: &TokenLiteral) -> Self {
        Self {
            token: token.clone(),
            value: None,
        }
    }

    fn token_literal(&self) -> String {
        self.token.value.clone().unwrap().to_string()
    }

    fn string(&self) -> String {
        self.value.as_ref().map_or(String::new(), |v| v.string())
    }
}

impl ReturnStatement {
    pub fn new(token: &TokenLiteral) -> Self {
        Self {
            token: token.clone(),
            value: None,
        }
    }

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

impl IntegerLiteral {
    fn token_literal(&self) -> String {
        self.token.value.clone().unwrap()
    }

    fn string(&self) -> String {
        self.token.value.clone().unwrap()
    }

    fn expression_node(&self) {}
}

impl BooleanLiteral {
    fn token_literal(&self) -> String {
        self.token.value.clone().unwrap()
    }

    fn string(&self) -> String {
        self.token.value.clone().unwrap()
    }

    fn expression_node(&self) {}
}

impl Identifier {
    fn token_literal(&self) -> String {
        self.token.value.clone().unwrap().to_string()
    }

    fn string(&self) -> String {
        self.value.clone()
    }

    fn expression_node(&self) {}
}

impl PrefixExpression {
    fn token_literal(&self) -> String {
        self.token.value.clone().unwrap()
    }

    fn string(&self) -> String {
        format!(
            "({}{})",
            self.operator,
            match self.right.as_ref() {
                None => String::new(),
                Some(v) => v.string(),
            }
        )
    }

    fn expression_node(&self) {}
}

impl InfixExpression {
    fn token_literal(&self) -> String {
        self.token.value.clone().unwrap()
    }

    fn string(&self) -> String {
        format!(
            "({} {} {})",
            self.left.as_ref().map_or(String::new(), |v| v.string()),
            self.operator,
            match self.right.as_ref() {
                None => String::new(),
                Some(v) => v.string(),
            }
        )
    }

    fn expression_node(&self) {}
}

#[cfg(test)]
mod test {
    use crate::{
        ast::{Identifier, LetStatement, Program, Statement},
        token::{Token, TokenLiteral},
    };

    #[test]
    fn test_string() {
        let input = Program {
            statements: vec![Statement::Let(LetStatement {
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
                value: Some(crate::ast::Expression::Identifier(Identifier {
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
