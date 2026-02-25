use std::{collections::HashMap, os::linux::raw::stat};

use crate::{
    ast::{
        BlockStatement, BooleanLiteral, Expression, ExpressionStatement, FunctionLiteral,
        Identifier, IfExpression, InfixExpression, IntegerLiteral, LetStatement, PrefixExpression,
        Program, ReturnStatement, Statement,
    },
    lexer::Lexer,
    token::{Precedence, Token, TokenLiteral},
};

type PrefixParserFn = fn(&mut Parser) -> Option<Expression>;
type InfixParserFn = fn(&mut Parser, Option<Expression>) -> Option<Expression>;

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

        result.next_token();
        result.next_token();

        result
    }

    fn parse_if(&mut self) -> Option<Expression> {
        let token = self.current_token.clone();

        if self.peek_token.token != Token::Lparen {
            return None;
        }

        self.next_token();
        self.next_token();

        let condition = self.parse_expresion(Precedence::Lowest);

        if self.peek_token.token != Token::Rparen {
            return None;
        }

        self.next_token();

        if self.peek_token.token != Token::Lbrace {
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
            condition: condition.map(Box::new),
            consequence: Some(consequence),
            alternative,
        }))
    }

    fn parse_function(&mut self) -> Option<Expression> {
        let token = self.current_token.clone();

        if self.peek_token.token != Token::Lparen {
            return None;
        }

        self.next_token();

        let parameters = self
            .parse_function_parameters()
            .expect("Unable to parse the function parameters");

        if self.peek_token.token != Token::Lbrace {
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

        if self.peek_token.token != Token::Rparen {
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

        if self.peek_token.token != Token::Rparen {
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
            right: self.parse_expresion(Precedence::Prefix).map(Box::new),
        }))
    }

    fn parse_infix(&mut self, expr: Option<Expression>) -> Option<Expression> {
        let token = self.current_token.clone();
        let operator = self.current_token.value.clone().unwrap();
        let precedence = self.current_token.precedence();

        self.next_token();

        Some(Expression::Infix(InfixExpression {
            token,
            operator,
            right: self.parse_expresion(precedence).map(Box::new),
            left: expr.map(Box::new),
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

        statement.value = self
            .parse_expresion(self.current_token.precedence())
            .map(Box::new);

        if self.peek_token.token == Token::Semicolon {
            self.next_token();
        }

        Statement::Expression(statement)
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

            left_expr = infix(self, left_expr);
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

#[cfg(test)]
mod test {
    use core::panic;

    use crate::{
        ast::{Expression, Statement},
        lexer::Lexer,
        parser::Parser,
        token::Token,
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
            test_let_statement(statement, name);
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
            Statement::Expression(s) => {
                test_expression(s.value.as_deref(), TestValue::String("foobar".to_string()));
            }
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
                test_expression(s.value.as_deref(), TestValue::String(5.to_string()))
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_boolean_expression() {
        let inputs = ["true;", "false"];
        let results = [true, false];

        for (input, result) in inputs.iter().zip(results) {
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);

            let program = parser.parse_program();
            check_errors(&parser);

            assert_eq!(program.statements.len(), 1);

            let statement = &program.statements[0];

            match statement {
                Statement::Expression(s) => {
                    test_expression(s.value.as_deref(), TestValue::String(result.to_string()))
                }
                _ => panic!(),
            }
        }
    }

    struct InfixTestValue {
        left: Box<TestValue>,
        operator: String,
        right: Box<TestValue>,
    }

    struct PrefixTestValue {
        operator: String,
        right: Box<TestValue>,
    }

    enum TestValue {
        String(String),
        Infix(InfixTestValue),
        Prefix(PrefixTestValue),
    }

    fn test_expression(expression: Option<&Expression>, value: TestValue) {
        match value {
            TestValue::String(val) => match expression.as_ref().unwrap() {
                Expression::IntegerLiteral(expr) => assert_eq!(expr.value.to_string(), val),
                Expression::Identifier(expr) => assert_eq!(expr.value.to_string(), val),
                Expression::Boolean(expr) => assert_eq!(expr.value.to_string(), val),
                _ => panic!(),
            },
            TestValue::Infix(value) => match expression.as_ref().unwrap() {
                Expression::Infix(expr) => {
                    test_expression(expr.left.as_deref(), *value.left);
                    assert_eq!(expr.operator, value.operator);
                    test_expression(expr.right.as_deref(), *value.right);
                }
                _ => panic!(),
            },
            TestValue::Prefix(value) => match expression.as_ref().unwrap() {
                Expression::Prefix(expr) => {
                    assert_eq!(expr.operator, value.operator);
                    test_expression(expr.right.as_deref(), *value.right);
                }
                _ => panic!(),
            },
        }
    }

    #[test]
    fn test_return() {
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
            test_return_statement(statement);
        }
    }

    #[test]
    fn test_function_params() {
        let inputs = ["fn(){}", "fn(x){}", "fn(x, y){}"];
        let exp_params = [vec![], vec!["x"], vec!["x", "y"]];

        for (input, expected) in inputs.iter().zip(exp_params) {
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);

            let program = parser.parse_program();
            check_errors(&parser);

            let statement = &program.statements[0];

            match statement {
                Statement::Expression(expr) => match expr.value.as_deref().unwrap() {
                    Expression::Function(fun) => {
                        assert_eq!(fun.token.token, Token::Function);
                        for (param, expected) in fun.parameters.iter().zip(expected) {
                            assert_eq!(param.value, expected);
                        }
                    }
                    _ => panic!(),
                },
                _ => panic!(),
            }
        }
    }

    #[test]
    fn test_function() {
        let input = "fn(a,b,c) { a }";
        let exp_params = vec!["a", "b", "c"];

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_errors(&parser);

        let statement = &program.statements[0];

        match statement {
            Statement::Expression(expr) => match expr.value.as_deref().unwrap() {
                Expression::Function(fun) => {
                    assert_eq!(fun.token.token, Token::Function);
                    for (param, expected) in fun.parameters.iter().zip(exp_params) {
                        assert_eq!(param.value, expected);
                    }

                    let statement = &fun.body.as_ref().unwrap().statements[0];

                    match statement {
                        Statement::Expression(s) => {
                            test_expression(s.value.as_deref(), TestValue::String("a".to_string()));
                        }
                        _ => panic!(),
                    }
                }
                _ => panic!(),
            },
            _ => panic!(),
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
                Statement::Expression(s) => {
                    test_expression(
                        s.value.as_deref(),
                        TestValue::Prefix(PrefixTestValue {
                            operator: expected_prefix.to_string(),
                            right: Box::new(TestValue::String(expected_value.to_string())),
                        }),
                    );
                }
                _ => panic!(),
            }
        }
    }

    #[test]
    fn test_infix_opertor() {
        let inputs = [
            "5 + 5;", "5 - 5;", "5 * 5;", "5 / 5;", "5 > 5;", "5 < 5;", "5 == 5;", "5 != 5;",
        ];

        let expected = [
            (5, "+", 5),
            (5, "-", 5),
            (5, "*", 5),
            (5, "/", 5),
            (5, ">", 5),
            (5, "<", 5),
            (5, "==", 5),
            (5, "!=", 5),
        ];

        for (input, (left, operator, right)) in inputs.iter().zip(expected) {
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);

            let program = parser.parse_program();
            check_errors(&parser);

            let statement = &program.statements[0];

            match statement {
                Statement::Expression(s) => {
                    test_expression(
                        s.value.as_deref(),
                        TestValue::Infix(InfixTestValue {
                            operator: operator.to_string(),
                            left: Box::new(TestValue::String(left.to_string())),
                            right: Box::new(TestValue::String(right.to_string())),
                        }),
                    );
                }
                _ => panic!(),
            }
        }
    }

    #[test]
    fn test_if_else_condition() {
        let input = "if (a == b) { y } else { x }";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_errors(&parser);

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Expression(stmt) => match stmt.value.as_deref().unwrap() {
                Expression::If(expr) => {
                    test_expression(
                        expr.condition.as_deref(),
                        TestValue::Infix(InfixTestValue {
                            operator: "==".to_string(),
                            left: Box::new(TestValue::String("a".to_string())),
                            right: Box::new(TestValue::String("b".to_string())),
                        }),
                    );
                    let statement = &expr.consequence.as_ref().unwrap().statements[0];

                    match statement {
                        Statement::Expression(s) => {
                            test_expression(s.value.as_deref(), TestValue::String("y".to_string()));
                        }
                        _ => panic!(),
                    }

                    let statement = &expr.alternative.as_ref().unwrap().statements[0];

                    match statement {
                        Statement::Expression(s) => {
                            test_expression(s.value.as_deref(), TestValue::String("x".to_string()));
                        }
                        _ => panic!(),
                    }
                }
                _ => panic!(),
            },
            _ => panic!(),
        }
    }

    #[test]
    fn test_infix_opertor_more() {
        let inputs = [
            "-a * b",
            "!-a",
            "a + b + c",
            "a + b - c",
            "a * b * c",
            "a * b / c",
            "a + b / c",
            "a + b * c + d / e - f",
            "3 + 4; -5 * 5",
            "5 > 4 == 3 < 4",
            "5 < 4 != 3 > 4",
            "3 + 4 * 5 == 3 * 1 + 4 * 5",
            "3 < 5 == true",
            "3 > 5 == false",
            "1 + (2 + 3) + 4",
            "(5 + 5) * 2",
            "2 / (5 + 5)",
            "-(5 + 5)",
            "!(true == true)",
        ];

        let expected = [
            "((-a) * b)",
            "(!(-a))",
            "((a + b) + c)",
            "((a + b) - c)",
            "((a * b) * c)",
            "((a * b) / c)",
            "(a + (b / c))",
            "(((a + (b * c)) + (d / e)) - f)",
            "(3 + 4)((-5) * 5)",
            "((5 > 4) == (3 < 4))",
            "((5 < 4) != (3 > 4))",
            "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
            "((3 < 5) == true)",
            "((3 > 5) == false)",
            "((1 + (2 + 3)) + 4)",
            "((5 + 5) * 2)",
            "(2 / (5 + 5))",
            "(-(5 + 5))",
            "(!(true == true))",
        ];

        for (input, expected) in inputs.iter().zip(expected) {
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);

            let program = parser.parse_program();
            check_errors(&parser);

            assert_eq!(program.string(), expected);
        }
    }

    fn test_let_statement(statement: &Statement, name: &str) {
        assert_eq!(statement.token_literal().as_str(), "let");
        match statement {
            Statement::Let(stmt) => assert_eq!(stmt.name.value, name),
            _ => panic!(),
        }
    }

    fn test_return_statement(statement: &Statement) {
        assert_eq!(statement.token_literal().as_str(), "return");
        match statement {
            Statement::Return(_) => (),
            _ => panic!(),
        }
    }

    fn check_errors(parser: &Parser) {
        for error in parser.errors() {
            println!("parser error: {}", error);
        }

        assert!(parser.errors.is_empty());
    }
}
