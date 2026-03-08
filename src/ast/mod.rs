//! Abstract Syntax Tree node definitions.
//!
//! Defines all AST nodes representing parsed Monkey code. The tree structure
//! captures grammatical relationships between language constructs.
//!
//! # Node Types
//!
//! - `Program`: Root node containing all statements
//! - `Statement`: Executable constructs (Let, Return, Block, Expression)
//! - `Expression`: Value-producing constructs (Identifier, Literal, Operation, etc.)
//!
//! Each node implements `string()` for debugging and REPL feedback.

use crate::token::Token;

#[cfg(test)]
pub mod tests;

#[derive(Debug)]
pub enum Node {
    Statement(Statement),
    Program(Program),
}

#[derive(Debug, Clone)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    Block(BlockStatement),
    Expression(Expression),
}

#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    Boolean(BooleanLiteral),
    Prefix(PrefixExpression),
    Infix(InfixExpression),
    If(IfExpression),
    Function(FunctionLiteral),
    Call(CallExpression),
}

#[derive(Debug, Default)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub token: Token,
}

#[derive(Debug, Clone)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

#[derive(Debug, Clone)]
pub struct BooleanLiteral {
    pub token: Token,
    pub value: bool,
}

#[derive(Debug, Clone)]
pub struct FunctionLiteral {
    pub token: Token,
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
}

#[derive(Debug, Clone)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Expression,
}

#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub token: Token,
    pub value: Expression,
}

#[derive(Debug, Clone)]
pub struct BlockStatement {
    pub token: Token,
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct PrefixExpression {
    pub token: Token,
    pub operator: String,
    pub right: Box<Expression>,
}

#[derive(Debug, Clone)]
pub struct InfixExpression {
    pub token: Token,
    pub operator: String,
    pub right: Box<Expression>,
    pub left: Box<Expression>,
}

#[derive(Debug, Clone)]
pub struct IfExpression {
    pub token: Token,
    pub condition: Box<Expression>,
    pub consequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
}

#[derive(Debug, Clone)]
pub struct CallExpression {
    pub token: Token,
    pub function: Box<Expression>,
    pub arguments: Vec<Expression>,
}

impl Program {
    pub fn string(&self) -> String {
        self.statements
            .iter()
            .map(|s| s.string())
            .collect::<String>()
    }
}

impl Node {
    pub fn block(expr: BlockStatement) -> Self {
        Node::Statement(Statement::Block(expr))
    }

    pub fn statement(expr: Expression) -> Self {
        Node::Statement(Statement::Expression(expr))
    }
}

impl Expression {
    pub fn string(&self) -> String {
        match self {
            Self::Identifier(expr) => expr.string(),
            Self::IntegerLiteral(expr) => expr.string(),
            Self::Prefix(expr) => expr.string(),
            Self::Infix(expr) => expr.string(),
            Self::Boolean(expr) => expr.string(),
            Self::If(expr) => expr.string(),
            Self::Function(expr) => expr.string(),
            Self::Call(expr) => expr.string(),
        }
    }

    fn token_literal(&self) -> &str {
        match self {
            Self::Identifier(expr) => expr.token_literal(),
            Self::IntegerLiteral(expr) => expr.token_literal(),
            Self::Prefix(expr) => expr.token_literal(),
            Self::Infix(expr) => expr.token_literal(),
            Self::Boolean(expr) => expr.token_literal(),
            Self::If(expr) => expr.token_literal(),
            Self::Function(expr) => expr.token_literal(),
            Self::Call(expr) => expr.token_literal(),
        }
    }
}

impl Statement {
    pub fn token_literal(&self) -> &str {
        match self {
            Self::Let(stmt) => stmt.token_literal(),
            Self::Return(stmt) => stmt.token_literal(),
            Self::Block(stmt) => stmt.token_literal(),
            Self::Expression(stmt) => stmt.token_literal(),
        }
    }

    pub fn string(&self) -> String {
        match self {
            Self::Let(stmt) => stmt.string(),
            Self::Return(stmt) => stmt.string(),
            Self::Block(stmt) => stmt.string(),
            Self::Expression(stmt) => stmt.string(),
        }
    }
}

impl LetStatement {
    pub fn new(token: &Token, identifier: &Token, value: Expression) -> Self {
        Self {
            token: token.clone(),
            name: Identifier {
                token: identifier.clone(),
            },
            value,
        }
    }

    fn token_literal(&self) -> &str {
        self.token.literal()
    }

    fn string(&self) -> String {
        format!(
            "{} {} = {};",
            self.token_literal(),
            self.name.string(),
            self.value.string(),
        )
    }
}

impl ReturnStatement {
    fn token_literal(&self) -> &str {
        self.token.literal()
    }

    fn string(&self) -> String {
        format!("{} {};", self.token_literal(), self.value.string())
    }
}

impl BlockStatement {
    fn token_literal(&self) -> &str {
        self.token.literal()
    }

    pub fn string(&self) -> String {
        self.statements
            .iter()
            .map(|v| v.string())
            .collect::<Vec<String>>()
            .join("")
    }
}

impl IntegerLiteral {
    fn token_literal(&self) -> &str {
        self.token.literal()
    }

    fn string(&self) -> String {
        self.token_literal().to_owned()
    }
}

impl BooleanLiteral {
    fn token_literal(&self) -> &str {
        self.token.literal()
    }

    fn string(&self) -> String {
        self.token_literal().to_owned()
    }
}

impl Identifier {
    fn token_literal(&self) -> &str {
        self.token.literal()
    }

    fn string(&self) -> String {
        self.token_literal().to_owned()
    }
}

impl PrefixExpression {
    fn token_literal(&self) -> &str {
        self.token.literal()
    }

    fn string(&self) -> String {
        format!("({}{})", self.operator, self.right.as_ref().string(),)
    }
}

impl InfixExpression {
    fn token_literal(&self) -> &str {
        self.token.literal()
    }

    fn string(&self) -> String {
        format!(
            "({} {} {})",
            self.left.as_ref().string(),
            self.operator,
            self.right.as_ref().string(),
        )
    }
}

impl IfExpression {
    fn token_literal(&self) -> &str {
        self.token.literal()
    }

    fn string(&self) -> String {
        format!(
            "if {} {} {}",
            self.condition.as_ref().string(),
            self.consequence.string(),
            self.alternative
                .as_ref()
                .map_or(String::new(), |v| format!("else {}", v.string()))
        )
    }
}

impl FunctionLiteral {
    fn token_literal(&self) -> &str {
        self.token.literal()
    }

    fn string(&self) -> String {
        format!(
            "{}({}){}",
            self.token_literal(),
            self.parameters
                .iter()
                .map(|v| v.string())
                .collect::<Vec<String>>()
                .join(", "),
            self.body.string(),
        )
    }
}

impl CallExpression {
    fn token_literal(&self) -> &str {
        self.token.literal()
    }

    fn string(&self) -> String {
        format!(
            "{}({})",
            self.function.as_ref().string(),
            self.arguments
                .iter()
                .map(|v| v.string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
