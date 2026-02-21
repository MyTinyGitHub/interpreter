use std::any::Any;
use std::collections::HashMap;

use crate::{
    ast::{
        Expression, ExpressionStatement, Identifier, IntegerLiteral, LetStatement, Program,
        ReturnStatement, Statement,
    },
    lexer::Lexer,
    token::{Token, TokenLiteral},
};

type PrefixParserFn = fn(&mut Parser) -> Option<Box<dyn Expression>>;
type InfixParserFn = fn(&mut Parser, dyn Expression) -> Option<Box<dyn Expression>>;

struct Parser {
    lexer: Lexer,
    current_token: TokenLiteral,
    peek_token: TokenLiteral,
    errors: Vec<String>,
    prefix_fns: HashMap<Token, PrefixParserFn>,
    infix_fns: HashMap<Token, InfixParserFn>,
}

impl Parser {
    fn new(lexer: Lexer) -> Self {
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

        result.next_token();
        result.next_token();

        result
    }

    fn parse_identifier(&mut self) -> Option<Box<dyn Expression>> {
        Some(Box::new(Identifier {
            token: self.current_token.clone(),
            value: self.current_token.value.clone().unwrap(),
        }))
    }

    fn parse_integer_literal(&mut self) -> Option<Box<dyn Expression>> {
        Some(Box::new(IntegerLiteral {
            token: self.current_token.clone(),
            value: self.current_token.value.clone().unwrap().parse().unwrap(),
        }))
    }

    pub fn errors(&self) -> &[String] {
        &self.errors
    }

    pub fn peek_error(&mut self, token: &Token) {
        let msg = format!(
            "expected next toekn to be {:?}, got {:?} instead",
            token, self.peek_token
        );

        self.errors.push(msg);
    }

    fn next_token(&mut self) {
        self.current_token = std::mem::replace(&mut self.peek_token, self.lexer.next_token());
    }

    fn parse_statement(&mut self) -> Box<dyn Statement> {
        match self.current_token.token {
            Token::Let => self.parse_let_statement(),
            Token::Return => self.parse_return_statement(),
            _ => self.parse_expresion_statement(),
        }
    }

    fn parse_expresion_statement(&mut self) -> Box<dyn Statement> {
        let mut statement = ExpressionStatement {
            token: self.current_token.clone(),
            value: None,
        };

        statement.value = self.parse_expresion(Token::Lowest);

        if self.peek_token.token == Token::Semicolon {
            self.next_token();
        }

        Box::new(statement)
    }

    fn parse_expresion(&mut self, token: Token) -> Option<Box<dyn Expression>> {
        if !self.prefix_fns.contains_key(&self.current_token.token) {
            return None;
        }

        let prefix = self.prefix_fns[&self.current_token.token];
        prefix(self)
    }

    fn expect_peek(&mut self, token: Token) -> bool {
        if self.peek_token.clone().token != token {
            self.peek_error(&token);
            return false;
        }
        true
    }

    fn parse_return_statement(&mut self) -> Box<dyn Statement> {
        let token = self.current_token.clone();

        self.next_token();

        let statement = ReturnStatement::new(&token);

        while !matches!(self.current_token.token, Token::Semicolon) {
            self.next_token();
        }

        Box::new(statement)
    }

    fn parse_let_statement(&mut self) -> Box<dyn Statement> {
        self.expect_peek(Token::Ident);

        let token = self.current_token.clone();

        self.next_token();

        let statement = LetStatement::new(&token, &self.current_token);

        while !matches!(self.current_token.token, Token::Semicolon) {
            self.next_token();
        }

        Box::new(statement)
    }

    fn parse_program(&mut self) -> Program {
        let mut program = Program::new();

        while self.current_token.token != Token::Eof {
            let statement = self.parse_statement();

            program.statements.push(statement);

            self.next_token();
        }

        program
    }
}

#[cfg(test)]
mod test {
    use crate::{
        ast::{ExpressionStatement, Identifier, IntegerLiteral, Statement},
        lexer::Lexer,
        parser::Parser,
    };

    #[test]
    fn test_parser() {
        let input = r#"
            let x = 5;
            let y = 10;
            let foobar = 838383;
        "#;

        let lexer = Lexer::new(input.to_owned());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_errors(&parser);

        let expected = ["x", "y", "foobar"];

        assert_eq!(program.statements.len(), expected.len());

        for (statement, name) in program.statements.iter().zip(expected.iter()) {
            test_statement(statement.as_ref(), name);
        }
    }

    #[test]
    fn test_identifier_expression() {
        let input = "foobar;";

        let lexer = Lexer::new(input.to_owned());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_errors(&parser);

        assert_eq!(program.statements.len(), 1);

        let statement = &program.statements[0];
        let stmt = statement
            .as_any()
            .downcast_ref::<ExpressionStatement>()
            .and_then(|s| s.value.as_ref())
            .and_then(|v| v.as_any().downcast_ref::<Identifier>())
            .expect("expected Identifier");

        assert!(stmt.value == "foobar");
    }

    #[test]
    fn test_integer_expression() {
        let input = "5;";

        let lexer = Lexer::new(input.to_owned());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_errors(&parser);

        assert_eq!(program.statements.len(), 1);

        let statement = program.statements[0].as_ref();

        let stmt = statement
            .as_any()
            .downcast_ref::<ExpressionStatement>()
            .and_then(|s| s.value.as_ref())
            .and_then(|v| v.as_any().downcast_ref::<IntegerLiteral>())
            .expect("expected IntegerLiteral");

        assert!(stmt.value == 5);
    }

    #[test]
    fn test_return_statement() {
        let input = r#"
            return 5;
            return 10;
            return 993322;
        "#;

        let lexer = Lexer::new(input.to_owned());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_errors(&parser);

        for statement in program.statements.iter() {
            return_statement(statement.as_ref());
        }
    }

    fn test_statement(statement: &dyn Statement, name: &str) {
        assert_eq!(statement.token_literal().as_str(), "let");
    }

    fn return_statement(statement: &dyn Statement) {
        assert_eq!(statement.token_literal().as_str(), "return");
    }

    fn check_errors(parser: &Parser) {
        for error in parser.errors() {
            println!("parser error: {}", error);
        }

        assert!(parser.errors.is_empty());
    }
}
