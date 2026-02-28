use std::collections::HashMap;

use crate::{
    ast::{
        BlockStatement, BooleanLiteral, CallExpresion, Expression, FunctionLiteral, Identifier,
        IfExpression, InfixExpression, IntegerLiteral, LetStatement, PrefixExpression, Program,
        ReturnStatement, Statement,
    },
    lexer::Lexer,
    token::{Precedence, Token, TokenLiteral},
};

#[cfg(test)]
mod tests;

type PrefixParserFn = fn(&mut Parser) -> Option<Expression>;
type InfixParserFn = fn(&mut Parser, Expression) -> Option<Expression>;

pub struct Parser {
    lexer: Lexer,
    current_token: TokenLiteral,
    peek_token: TokenLiteral,
    errors: Vec<String>,
    prefix_fns: HashMap<Token, PrefixParserFn>,
    infix_fns: HashMap<Token, InfixParserFn>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut result = Self {
            lexer,
            current_token: TokenLiteral::new(Token::Start, None),
            peek_token: TokenLiteral::new(Token::Start, None),
            errors: Vec::new(),
            prefix_fns: HashMap::new(),
            infix_fns: HashMap::new(),
        };

        result
            .prefix_fns
            .insert(Token::Ident, Parser::parse_identifier);

        result
            .prefix_fns
            .insert(Token::Int, Parser::parse_integer_literal);

        result
            .prefix_fns
            .insert(Token::Bang, Parser::parse_prefix_expression);

        result
            .prefix_fns
            .insert(Token::Minus, Parser::parse_prefix_expression);

        result.prefix_fns.insert(Token::True, Parser::parse_boolean);

        result
            .prefix_fns
            .insert(Token::Lparen, Parser::parse_grouped);

        result
            .prefix_fns
            .insert(Token::False, Parser::parse_boolean);

        result.prefix_fns.insert(Token::If, Parser::parse_if);
        result
            .prefix_fns
            .insert(Token::Function, Parser::parse_function);

        result.infix_fns.insert(Token::Plus, Parser::parse_infix);

        result.infix_fns.insert(Token::Minus, Parser::parse_infix);

        result
            .infix_fns
            .insert(Token::Asterisk, Parser::parse_infix);

        result.infix_fns.insert(Token::Slash, Parser::parse_infix);

        result.infix_fns.insert(Token::Equal, Parser::parse_infix);

        result
            .infix_fns
            .insert(Token::Notequal, Parser::parse_infix);

        result.infix_fns.insert(Token::Lt, Parser::parse_infix);

        result.infix_fns.insert(Token::Gt, Parser::parse_infix);

        result.infix_fns.insert(Token::Lparen, Parser::parse_call);

        result.next_token();
        result.next_token();

        result
    }

    fn parse_call(&mut self, function: Expression) -> Option<Expression> {
        Some(Expression::Call(CallExpresion {
            token: self.current_token.clone(),
            function: Box::new(function),
            arguments: self.parse_call_arguments(),
        }))
    }

    fn parse_call_arguments(&mut self) -> Vec<Expression> {
        let mut arguments: Vec<Expression> = vec![];

        if self.peek_token.token == Token::Rparen {
            self.next_token();
            return arguments;
        }

        self.next_token();

        arguments.push(
            self.parse_expresion(Precedence::Lowest)
                .expect("Unable to parse the expresion"),
        );

        while self.peek_token.token == Token::Comma {
            self.next_token();
            self.next_token();
            arguments.push(
                self.parse_expresion(Precedence::Lowest)
                    .expect("Unable to parse the expresion"),
            );
        }

        if self.peek_token.token != Token::Rparen {
            return vec![];
        }

        self.next_token();

        arguments
    }

    fn parse_if(&mut self) -> Option<Expression> {
        let token = self.current_token.clone();

        if !self.expect_peek(Token::Lparen) {
            return None;
        }

        self.next_token();
        self.next_token();

        let condition = self.parse_expresion(Precedence::Lowest).expect("tmp");

        if !self.expect_peek(Token::Rparen) {
            return None;
        }

        self.next_token();

        if !self.expect_peek(Token::Lbrace) {
            return None;
        }

        self.next_token();

        let consequence = self.parse_block_statement();

        let alternative = if self.peek_token.token == Token::Else {
            self.next_token();
            if self.peek_token.token != Token::Lbrace {
                return None;
            }
            self.next_token();
            Some(self.parse_block_statement())
        } else {
            None
        };

        Some(Expression::If(IfExpression {
            token,
            condition: Box::new(condition),
            consequence: Some(consequence),
            alternative,
        }))
    }

    fn parse_function(&mut self) -> Option<Expression> {
        let token = self.current_token.clone();

        if !self.expect_peek(Token::Lparen) {
            return None;
        }

        self.next_token();

        let parameters = self
            .parse_function_parameters()
            .expect("Unable to parse the function parameters");

        if !self.expect_peek(Token::Lbrace) {
            return None;
        }

        self.next_token();

        let body = Some(self.parse_block_statement());

        Some(Expression::Function(FunctionLiteral {
            token,
            parameters,
            body,
        }))
    }

    fn parse_function_parameters(&mut self) -> Option<Vec<Identifier>> {
        let mut identifiers: Vec<Identifier> = vec![];

        if self.peek_token.token == Token::Rparen {
            self.next_token();
            return Some(identifiers);
        }

        self.next_token();

        identifiers.push(Identifier {
            token: self.current_token.clone(),
            value: self.current_token.value.clone().unwrap(),
        });

        while self.peek_token.token == Token::Comma {
            self.next_token();
            self.next_token();

            identifiers.push(Identifier {
                token: self.current_token.clone(),
                value: self.current_token.value.clone().unwrap(),
            });
        }

        if !self.expect_peek(Token::Rparen) {
            return None;
        }

        self.next_token();

        Some(identifiers)
    }

    fn parse_block_statement(&mut self) -> BlockStatement {
        let mut block = BlockStatement {
            token: self.current_token.clone(),
            statements: Vec::new(),
        };

        self.next_token();

        while self.current_token.token != Token::Rbrace && self.current_token.token != Token::Eof {
            let stmt = self.parse_statement();
            block.statements.push(stmt);
            self.next_token();
        }

        block
    }

    fn parse_identifier(&mut self) -> Option<Expression> {
        Some(Expression::Identifier(Identifier {
            token: self.current_token.clone(),
            value: self.current_token.value.clone().unwrap(),
        }))
    }

    fn parse_boolean(&mut self) -> Option<Expression> {
        Some(Expression::Boolean(BooleanLiteral {
            token: self.current_token.clone(),
            value: match self.current_token.token {
                Token::True => true,
                Token::False => false,
                _ => panic!(
                    "Expected boolean token found {:?}",
                    self.current_token.token
                ),
            },
        }))
    }

    fn parse_grouped(&mut self) -> Option<Expression> {
        self.next_token();

        let exp = self.parse_expresion(Precedence::Lowest);

        if !self.expect_peek(Token::Rparen) {
            return None;
        }

        self.next_token();

        exp
    }

    fn parse_prefix_expression(&mut self) -> Option<Expression> {
        let token = self.current_token.clone();
        let operator = self.current_token.value.clone().unwrap();

        self.next_token();

        Some(Expression::Prefix(PrefixExpression {
            token,
            operator,
            right: self
                .parse_expresion(Precedence::Prefix)
                .map(Box::new)
                .expect("tmp"),
        }))
    }

    fn parse_infix(&mut self, expr: Expression) -> Option<Expression> {
        let token = self.current_token.clone();
        let operator = self.current_token.value.clone().unwrap();
        let precedence = self.current_token.precedence();

        self.next_token();

        Some(Expression::Infix(InfixExpression {
            token,
            operator,
            right: self.parse_expresion(precedence).map(Box::new).expect("tmp"),
            left: Box::new(expr),
        }))
    }

    fn parse_integer_literal(&mut self) -> Option<Expression> {
        Some(Expression::IntegerLiteral(IntegerLiteral {
            token: self.current_token.clone(),
            value: self.current_token.value.clone().unwrap().parse().unwrap(),
        }))
    }

    pub fn errors(&self) -> &[String] {
        &self.errors
    }

    fn no_prefix_operator_error(&mut self, token: &Token) {
        let msg = format!("no prefix operation found for token {:?}", token);

        self.errors.push(msg);
    }

    fn peek_error(&mut self, token: &Token) {
        let msg = format!(
            "expected next token to be {:?}, got {:?} instead",
            token, self.peek_token
        );

        self.errors.push(msg);
    }

    fn next_token(&mut self) {
        self.current_token = std::mem::replace(&mut self.peek_token, self.lexer.next_token());
    }

    fn parse_statement(&mut self) -> Statement {
        match self.current_token.token {
            Token::Let => self
                .parse_let_statement()
                .expect("unable to parse let statement"),
            Token::Return => self.parse_return_statement(),
            _ => self.parse_expresion_statement(),
        }
    }

    fn parse_expresion_statement(&mut self) -> Statement {
        let value = self.parse_expresion(Precedence::Lowest);

        if self.peek_token.token == Token::Semicolon {
            self.next_token();
        }

        Statement::Expression(value.expect("Expected value"))
    }

    fn parse_expresion(&mut self, precedenece: Precedence) -> Option<Expression> {
        let cur_token = self.current_token.token.clone();

        if !self.prefix_fns.contains_key(&cur_token) {
            self.no_prefix_operator_error(&cur_token);
            return None;
        }

        let prefix = self.prefix_fns[&cur_token];
        let mut left_expr = prefix(self);

        while self.peek_token.token != Token::Semicolon
            && precedenece < self.peek_token.precedence()
        {
            let peek_token = self.peek_token.token.clone();

            if !self.infix_fns.contains_key(&peek_token) {
                return left_expr;
            }

            let infix = self.infix_fns[&peek_token];

            self.next_token();

            left_expr = infix(self, left_expr.expect("tmp"));
        }

        left_expr
    }

    fn expect_peek(&mut self, token: Token) -> bool {
        if self.peek_token.clone().token != token {
            self.peek_error(&token);
            return false;
        }
        true
    }

    fn parse_return_statement(&mut self) -> Statement {
        let token = self.current_token.clone();

        self.next_token();

        let statement = self.parse_expresion(Precedence::Lowest);

        if self.peek_token.token == Token::Semicolon {
            self.next_token();
        }

        Statement::Return(ReturnStatement {
            token,
            value: statement,
        })
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        let token = self.current_token.clone();

        if !self.expect_peek(Token::Ident) {
            return None;
        }

        self.next_token();

        let name = Identifier {
            token: self.current_token.clone(),
            value: self.current_token.value.clone().unwrap(),
        };

        if !self.expect_peek(Token::Assign) {
            return None;
        }

        self.next_token();
        self.next_token();

        let value = self.parse_expresion(Precedence::Lowest);

        if self.peek_token.token == Token::Semicolon {
            self.next_token();
        }

        Some(Statement::Let(LetStatement { token, name, value }))
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program::default();

        while self.current_token.token != Token::Eof {
            let statement = self.parse_statement();

            program.statements.push(statement);

            self.next_token();
        }

        program
    }
}
