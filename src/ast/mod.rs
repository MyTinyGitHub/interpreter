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
    fn token_literal(&self) -> &str {
        self.statements[0].token_literal()
    }

    pub fn string(&self) -> String {
        self.statements
            .iter()
            .map(|s| s.string())
            .collect::<String>()
    }
}

impl Node {
    fn token_literal(&self) -> &str {
        match self {
            Self::Statement(node) => node.token_literal(),
            Self::Program(node) => node.token_literal(),
        }
    }

    pub fn block(expr: BlockStatement) -> Self {
        Node::Statement(Statement::Block(expr))
    }

    pub fn statement(expr: Expression) -> Self {
        Node::Statement(Statement::Expression(expr))
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
            Self::If(expr) => expr.expression_node(),
            Self::Function(expr) => expr.expression_node(),
            Self::Call(expr) => expr.expression_node(),
        }
    }

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

    fn statement_node(&self) {}
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
        &self.token.literal()
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
        &self.token.literal()
    }

    fn string(&self) -> String {
        format!("{} {};", self.token_literal(), self.value.string())
    }
}

impl BlockStatement {
    fn token_literal(&self) -> &str {
        &self.token.literal()
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
        &self.token.literal()
    }

    fn string(&self) -> String {
        self.token_literal().to_owned()
    }

    fn expression_node(&self) {}
}

impl BooleanLiteral {
    fn token_literal(&self) -> &str {
        &self.token.literal()
    }

    fn string(&self) -> String {
        self.token_literal().to_owned()
    }

    fn expression_node(&self) {}
}

impl Identifier {
    fn token_literal(&self) -> &str {
        &self.token.literal()
    }

    fn string(&self) -> String {
        self.token_literal().to_owned()
    }

    fn expression_node(&self) {}
}

impl PrefixExpression {
    fn token_literal(&self) -> &str {
        &self.token.literal()
    }

    fn string(&self) -> String {
        format!("({}{})", self.operator, self.right.as_ref().string(),)
    }

    fn expression_node(&self) {}
}

impl InfixExpression {
    fn token_literal(&self) -> &str {
        &self.token.literal()
    }

    fn string(&self) -> String {
        format!(
            "({} {} {})",
            self.left.as_ref().string(),
            self.operator,
            self.right.as_ref().string(),
        )
    }

    fn expression_node(&self) {}
}

impl IfExpression {
    fn token_literal(&self) -> &str {
        &self.token.literal()
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

    fn expression_node(&self) {}
}

impl FunctionLiteral {
    fn token_literal(&self) -> &str {
        &self.token.literal()
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

    fn expression_node(&self) {}
}

impl CallExpression {
    fn token_literal(&self) -> &str {
        &self.token.literal()
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

    fn expression_node(&self) {}
}
