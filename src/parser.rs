use std::collections::HashMap;

use crate::{
    ast::{
        Expression, ExpressionStatement, Identifier, IntegerLiteral, LetStatement,
        PrefixExpression, Program, ReturnStatement, Statement,
    },
    lexer::Lexer,
    token::{Token, TokenLiteral},
};

type PrefixParserFn = fn(&mut Parser) -> Expression;
type InfixParserFn = fn(&mut Parser, &Expression) -> Expression;

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

        result
            .prefix_fns
            .insert(Token::Bang, Parser::parse_prefix_expression);

        result
            .prefix_fns
            .insert(Token::Minus, Parser::parse_prefix_expression);

        result.next_token();
        result.next_token();

        result
    }

    fn parse_identifier(&mut self) -> Expression {
        Expression::Identifier(Identifier {
            token: self.current_token.clone(),
            value: self.current_token.value.clone().unwrap(),
        })
    }

    fn parse_prefix_expression(&mut self) -> Expression {
        let token = self.current_token.clone();
        let operator = self.current_token.value.clone().unwrap();

        self.next_token();

        Expression::Prefix(PrefixExpression {
            token,
            operator,
            right: Box::new(self.parse_integer_literal()),
        })
    }

    fn parse_integer_literal(&mut self) -> Expression {
        Expression::IntegerLiteral(IntegerLiteral {
            token: self.current_token.clone(),
            value: self.current_token.value.clone().unwrap().parse().unwrap(),
        })
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
            Token::Let => self.parse_let_statement(),
            Token::Return => self.parse_return_statement(),
            _ => self.parse_expresion_statement(),
        }
    }

    fn parse_expresion_statement(&mut self) -> Statement {
        let mut statement = ExpressionStatement {
            token: self.current_token.clone(),
            value: None,
        };

        statement.value = self.parse_expresion(Token::Lowest);

        if self.peek_token.token == Token::Semicolon {
            self.next_token();
        }

        Statement::Expression(statement)
    }

    fn parse_expresion(&mut self, token: Token) -> Option<Expression> {
        let cur_token = self.current_token.token.clone();

        if !self.prefix_fns.contains_key(&cur_token) {
            self.no_prefix_operator_error(&cur_token);
            return None;
        }

        let prefix = self.prefix_fns[&cur_token];

        Some(prefix(self))
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

        let statement = ReturnStatement::new(&token);

        while !matches!(self.current_token.token, Token::Semicolon) {
            self.next_token();
        }

        Statement::Return(statement)
    }

    fn parse_let_statement(&mut self) -> Statement {
        self.expect_peek(Token::Ident);

        let token = self.current_token.clone();

        self.next_token();

        let statement = LetStatement::new(&token, &self.current_token);

        while !matches!(self.current_token.token, Token::Semicolon) {
            self.next_token();
        }

        Statement::Let(statement)
    }

    fn parse_program(&mut self) -> Program {
        let mut program = Program::default();

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
    use core::panic;

    use crate::{
        ast::{Expression, Statement},
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

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_errors(&parser);

        let expected = ["x", "y", "foobar"];

        assert_eq!(program.statements.len(), expected.len());

        for (statement, name) in program.statements.iter().zip(expected.iter()) {
            test_statement(statement, name);
        }
    }

    #[test]
    fn test_identifier_expression() {
        let input = "foobar;";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_errors(&parser);

        assert_eq!(program.statements.len(), 1);

        let statement = &program.statements[0];

        match statement {
            Statement::Expression(s) => match s.value.as_ref().unwrap() {
                Expression::Identifier(expr) => assert_eq!(expr.value, "foobar"),
                _ => panic!(),
            },
            _ => panic!(),
        }
    }

    #[test]
    fn test_integer_expression() {
        let input = "5;";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_errors(&parser);

        assert_eq!(program.statements.len(), 1);

        let statement = &program.statements[0];

        match statement {
            Statement::Expression(s) => {
                test_integer_literal_expression(s.value.as_ref().unwrap(), 5)
            }
            _ => panic!(),
        }
    }

    fn test_integer_literal_expression(expression: &Expression, value: i64) {
        match expression {
            Expression::IntegerLiteral(expr) => assert_eq!(expr.value, value),
            _ => panic!(),
        }
    }

    #[test]
    fn test_return_statement() {
        let input = r#"
            return 5;
            return 10;
            return 993322;
        "#;

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_errors(&parser);

        for statement in program.statements.iter() {
            return_statement(statement);
        }
    }

    #[test]
    fn test_prefix_opertor() {
        let inputs = ["!5;", "-15;"];
        let expected = [("!", 5), ("-", 15)];

        for (input, (expected_prefix, expected_value)) in inputs.iter().zip(expected) {
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);

            let program = parser.parse_program();
            check_errors(&parser);

            let statement = &program.statements[0];

            match statement {
                Statement::Expression(s) => match s.value.as_ref().unwrap() {
                    Expression::Prefix(expr) => {
                        assert_eq!(expr.operator, expected_prefix);
                        test_integer_literal_expression(&expr.right, expected_value);
                    }
                    _ => panic!(),
                },
                _ => panic!(),
            }
        }
    }

    fn test_statement(statement: &Statement, name: &str) {
        assert_eq!(statement.token_literal().as_str(), "let");
    }

    fn return_statement(statement: &Statement) {
        assert_eq!(statement.token_literal().as_str(), "return");
    }

    fn check_errors(parser: &Parser) {
        for error in parser.errors() {
            println!("parser error: {}", error);
        }

        assert!(parser.errors.is_empty());
    }
}
