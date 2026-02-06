use crate::{
    ast::{LetStatement, Program, Statement},
    lexer::Lexer,
    token::Token,
};

struct Parser {
    lexer: Lexer,
    current_token: Option<Token>,
    peek_token: Option<Token>,
}

impl Parser {
    fn new(lexer: Lexer) -> Self {
        let mut result = Self {
            lexer,
            current_token: None,
            peek_token: None,
        };

        result.next_token();

        result
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.take();
        self.peek_token = Some(self.lexer.next_token());
    }

    fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        match self.current_token.clone()? {
            Token::Let(_) => self.parse_let_statement(),
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<Box<dyn Statement>> {
        if !matches!(self.peek_token, Some(Token::Ident(_))) {
            return None;
        }

        let token = self.current_token.clone();
        println!("{:?}", self.current_token);

        self.next_token();

        println!("{:?}", self.current_token);

        let statement = LetStatement::new(
            token.as_ref().unwrap(),
            self.current_token.as_ref().unwrap(),
        );

        while !matches!(self.current_token, Some(Token::Semicolon(_))) {
            self.next_token();
        }

        Some(Box::new(statement))
    }

    fn parse_program(&mut self) -> Program {
        let mut program = Program::new();

        while self.current_token != Some(Token::Eof) {
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
        let expected = ["x", "y", "foobar"];

        assert_eq!(program.statements.len(), expected.len());

        for (statement, name) in program.statements.iter().zip(expected.iter()) {
            test_statement(statement, name);
        }
    }

    fn test_statement(statement: &Box<dyn Statement>, name: &str) {
        assert_eq!(statement.token_literal().as_str(), "let");
        assert_eq!(statement.name().value, name);
        assert_eq!(statement.name().token_literal(), name);
    }
}
