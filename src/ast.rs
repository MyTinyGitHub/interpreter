use crate::token::TokenLiteral;

pub trait Node {
    fn token_literal(&self) -> String;
}

pub trait Statement: Node {
    fn statement_node(&self);
    fn name(&self) -> &Identifier;
    //fn value(&self) -> &dyn Expression;
}

pub trait Expression: Node {
    fn expression_node(&self);
}

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

pub struct Identifier {
    pub token: TokenLiteral,
    pub value: String,
}

pub struct LetStatement {
    token: TokenLiteral,
    pub name: Identifier,
    //pub value: dyn Expression,
}

pub struct ReturnStatement {
    token: TokenLiteral,
    pub name: Identifier,
    //returnValue: dyn Expression,
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
                value: identifier.value.to_owned().unwrap(),
            },
        }
    }
}

impl ReturnStatement {
    pub fn new(token: &TokenLiteral, identifier: &TokenLiteral) -> Self {
        Self {
            token: token.clone(),
            name: Identifier {
                token: identifier.clone(),
                value: identifier.value.to_owned().unwrap(),
            },
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
}

impl Statement for LetStatement {
    fn statement_node(&self) {}
    fn name(&self) -> &Identifier {
        &self.name
    }
    //fn value(&self) -> &dyn Expression {
    //   &self.value
    //}
}

impl Statement for ReturnStatement {
    fn statement_node(&self) {}
    fn name(&self) -> &Identifier {
        &self.name
    }
    //fn value(&self) -> &dyn Expression {
    //   &self.value
    //}
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.value.clone().unwrap().to_string()
    }
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.value.clone().unwrap().to_string()
    }
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.value.clone().unwrap().to_string()
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {}
}
