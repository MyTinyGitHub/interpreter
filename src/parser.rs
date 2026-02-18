use crate::{
    ast::{LetStatement, Program, ReturnStatement, Statement},
    lexer::Lexer,
    token::{Token, TokenLiteral},
};

struct Parser {
    lexer: Lexer,
    current_token: TokenLiteral,
    peek_token: TokenLiteral,
    errors: Vec<String>,
}

impl Parser {
    fn new(lexer: Lexer) -> Self {
        let mut result = Self {
            lexer,
            current_token: TokenLiteral::new(Token::Start, None),
            peek_token: TokenLiteral::new(Token::Start, None),
            errors: Vec::new(),
        };

        result.next_token();

        result
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

    fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        match self.current_token.token {
            Token::Let => self.parse_let_statement(),
            Token::Return => self.parse_return_statement(),
            _ => None,
        }
    }

    fn expect_peek(&mut self, token: Token) -> bool {
        if self.peek_token.clone().token != token {
            self.peek_error(&token);
            return false;
        }
        true
    }

    fn parse_return_statement(&mut self) -> Option<Box<dyn Statement>> {
        let token = self.current_token.clone();

        self.next_token();

        let statement = ReturnStatement::new(&token);

        while !matches!(self.current_token.token, Token::Semicolon) {
            self.next_token();
        }

        Some(Box::new(statement))
    }

    fn parse_let_statement(&mut self) -> Option<Box<dyn Statement>> {
        self.expect_peek(Token::Ident);

        let token = self.current_token.clone();

        self.next_token();

        let statement = LetStatement::new(&token, &self.current_token);

        while !matches!(self.current_token.token, Token::Semicolon) {
            self.next_token();
        }

        Some(Box::new(statement))
    }

    fn parse_program(&mut self) -> Program {
        let mut program = Program::new();

        while self.current_token.token != Token::Eof {
            let statement = self.parse_statement();
            if let Some(value) = statement {
                program.statements.push(value);
            }
            self.next_token();
        }

        program
    }
}

#[cfg(test)]
mod test {
    use crate::{
        ast::{Node, Statement},
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
