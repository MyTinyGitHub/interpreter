use std::collections::HashMap;

use crate::{
    ast::{
        BlockStatement, BooleanLiteral, CallExpresion, Expression, FunctionLiteral, Identifier,
        IfExpression, InfixExpression, IntegerLiteral, LetStatement, PrefixExpression, Program,
        ReturnStatement, Statement,
    },
    error::MonkeyError,
    lexer::Lexer,
    token::{Precedence, Token, TokenLiteral},
};

#[cfg(test)]
mod tests;

type PrefixParserFn = fn(&mut Parser) -> Result<Expression, MonkeyError>;
type InfixParserFn = fn(&mut Parser, Expression) -> Result<Expression, MonkeyError>;

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

    fn parse_call(&mut self, function: Expression) -> Result<Expression, MonkeyError> {
        Ok(Expression::Call(CallExpresion {
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

    fn parse_if(&mut self) -> Result<Expression, MonkeyError> {
        let token = self.current_token.clone();

        self.expect_peek(Token::Lparen)?;

        self.next_token();
        self.next_token();

        let condition = self.parse_expresion(Precedence::Lowest).expect("tmp");

        self.expect_peek(Token::Rparen)?;

        self.next_token();

        self.expect_peek(Token::Lbrace)?;

        self.next_token();

        let consequence = self.parse_block_statement()?;

        let alternative = match self.expect_peek(Token::Else) {
            Ok(()) => {
                self.next_token();
                self.expect_peek(Token::Lbrace)?;
                self.next_token();
                Some(self.parse_block_statement()?)
            }
            Err(_) => None,
        };

        let if_statement = Expression::If(IfExpression {
            token,
            condition: Box::new(condition),
            consequence,
            alternative,
        });

        Ok(if_statement)
    }

    fn parse_function(&mut self) -> Result<Expression, MonkeyError> {
        let token = self.current_token.clone();

        self.expect_peek(Token::Lparen)?;

        self.next_token();

        let parameters = self
            .parse_function_parameters()
            .expect("Unable to parse the function parameters");

        self.expect_peek(Token::Lbrace)?;

        self.next_token();

        let body = self.parse_block_statement()?;

        Ok(Expression::Function(FunctionLiteral {
            token,
            parameters,
            body,
        }))
    }

    fn parse_function_parameters(&mut self) -> Result<Vec<Identifier>, MonkeyError> {
        let mut identifiers: Vec<Identifier> = vec![];

        if self.peek_token.token == Token::Rparen {
            self.next_token();
            return Ok(identifiers);
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

        self.expect_peek(Token::Rparen)?;

        self.next_token();

        Ok(identifiers)
    }

    fn parse_block_statement(&mut self) -> Result<BlockStatement, MonkeyError> {
        let mut block = BlockStatement {
            token: self.current_token.clone(),
            statements: Vec::new(),
        };

        self.next_token();

        while self.current_token.token != Token::Rbrace && self.current_token.token != Token::Eof {
            let stmt = self.parse_statement()?;
            block.statements.push(stmt);
            self.next_token();
        }

        Ok(block)
    }

    fn parse_identifier(&mut self) -> Result<Expression, MonkeyError> {
        Ok(Expression::Identifier(Identifier {
            token: self.current_token.clone(),
            value: self.current_token.value.clone().unwrap(),
        }))
    }

    fn parse_boolean(&mut self) -> Result<Expression, MonkeyError> {
        Ok(Expression::Boolean(BooleanLiteral {
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

    fn parse_grouped(&mut self) -> Result<Expression, MonkeyError> {
        self.next_token();

        let exp = self.parse_expresion(Precedence::Lowest)?;

        self.expect_peek(Token::Rparen)?;

        self.next_token();

        Ok(exp)
    }

    fn parse_prefix_expression(&mut self) -> Result<Expression, MonkeyError> {
        let token = self.current_token.clone();
        let operator = self.current_token.value.clone().unwrap();

        self.next_token();

        Ok(Expression::Prefix(PrefixExpression {
            token,
            operator,
            right: self
                .parse_expresion(Precedence::Prefix)
                .map(Box::new)
                .expect("tmp"),
        }))
    }

    fn parse_infix(&mut self, expr: Expression) -> Result<Expression, MonkeyError> {
        let token = self.current_token.clone();
        let operator = self.current_token.value.clone().unwrap();
        let precedence = self.current_token.precedence();

        self.next_token();

        Ok(Expression::Infix(InfixExpression {
            token,
            operator,
            right: self.parse_expresion(precedence).map(Box::new).expect("tmp"),
            left: Box::new(expr),
        }))
    }

    fn parse_integer_literal(&mut self) -> Result<Expression, MonkeyError> {
        Ok(Expression::IntegerLiteral(IntegerLiteral {
            token: self.current_token.clone(),
            value: self.current_token.value.clone().unwrap().parse().unwrap(),
        }))
    }

    pub fn errors(&self) -> &[String] {
        &self.errors
    }

    fn no_prefix_operator_error(&mut self, token: &Token) -> Result<(), MonkeyError> {
        let msg = format!("no prefix operation found for token {:?}", token);
        Err(MonkeyError::Parser(msg))
    }

    fn peek_error(&mut self, token: &Token) -> MonkeyError {
        let msg = format!(
            "expected next token to be {:?}, got {:?} instead",
            token, self.peek_token
        );

        MonkeyError::Parser(msg)
    }

    fn next_token(&mut self) {
        self.current_token = std::mem::replace(&mut self.peek_token, self.lexer.next_token());
    }

    fn parse_statement(&mut self) -> Result<Statement, MonkeyError> {
        match self.current_token.token {
            Token::Let => self.parse_let_statement(),
            Token::Return => self.parse_return_statement(),
            _ => self.parse_expresion_statement(),
        }
    }

    fn parse_expresion_statement(&mut self) -> Result<Statement, MonkeyError> {
        let value = self.parse_expresion(Precedence::Lowest)?;

        if self.peek_token.token == Token::Semicolon {
            self.next_token();
        }

        Ok(Statement::Expression(value))
    }

    fn prefix_fn(&mut self, token: &Token) -> Result<PrefixParserFn, MonkeyError> {
        if !self.prefix_fns.contains_key(token) {
            self.no_prefix_operator_error(token)?
        }

        Ok(self.prefix_fns[token])
    }

    fn parse_expresion(&mut self, precedenece: Precedence) -> Result<Expression, MonkeyError> {
        let cur_token = self.current_token.token.clone();

        let prefix = self.prefix_fn(&cur_token)?;
        let mut left_expr = prefix(self)?;

        while self.peek_token.token != Token::Semicolon
            && precedenece < self.peek_token.precedence()
        {
            let peek_token = self.peek_token.token.clone();

            if !self.infix_fns.contains_key(&peek_token) {
                return Ok(left_expr);
            }

            let infix = self.infix_fns[&peek_token];

            self.next_token();

            left_expr = infix(self, left_expr)?;
        }

        Ok(left_expr)
    }

    fn expect_peek(&mut self, token: Token) -> Result<(), MonkeyError> {
        if self.peek_token.clone().token != token {
            return Err(self.peek_error(&token));
        }

        Ok(())
    }

    fn parse_return_statement(&mut self) -> Result<Statement, MonkeyError> {
        let token = self.current_token.clone();

        self.next_token();

        let statement = self.parse_expresion(Precedence::Lowest)?;

        if self.peek_token.token == Token::Semicolon {
            self.next_token();
        }

        let return_statmenet = Statement::Return(ReturnStatement {
            token,
            value: statement,
        });

        Ok(return_statmenet)
    }

    fn parse_let_statement(&mut self) -> Result<Statement, MonkeyError> {
        let token = self.current_token.clone();

        self.expect_peek(Token::Ident)?;

        self.next_token();

        let name = Identifier {
            token: self.current_token.clone(),
            value: self.current_token.value.clone().unwrap(),
        };

        self.expect_peek(Token::Assign)?;

        self.next_token();
        self.next_token();

        let value = self.parse_expresion(Precedence::Lowest)?;

        if self.peek_token.token == Token::Semicolon {
            self.next_token();
        }

        Ok(Statement::Let(LetStatement { token, name, value }))
    }

    pub fn parse_program(&mut self) -> Result<Program, MonkeyError> {
        let mut program = Program::default();

        while self.current_token.token != Token::Eof {
            let statement = self.parse_statement()?;

            program.statements.push(statement);

            self.next_token();
        }

        Ok(program)
    }
}
