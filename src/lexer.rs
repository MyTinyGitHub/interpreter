use std::collections::HashMap;

use crate::token::Token;

pub struct Lexer {
    input: Vec<u8>,
    position: usize,
    read_position: usize,
    ch: Option<u8>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut result = Self {
            input: input.as_bytes().to_owned(),
            position: 0,
            read_position: 0,
            ch: None,
        };

        result.read_char();
        result
    }

    pub fn next_token(&mut self) -> Token {
        let token_map = HashMap::from([
            ("let".to_owned(), Token::Let("let".to_owned())),
            ("fn".to_owned(), Token::Function("fn".to_owned())),
            ("if".to_owned(), Token::If("if".to_owned())),
            ("else".to_owned(), Token::Else("else".to_owned())),
            ("return".to_owned(), Token::Return("return".to_owned())),
            ("true".to_owned(), Token::True("true".to_owned())),
            ("false".to_owned(), Token::False("false".to_owned())),
        ]);

        self.skip_white_space();

        if self.ch.is_none() {
            return Token::Eof;
        }

        let result = match self.ch.unwrap() as char {
            '=' => {
                self.peek_char();
                if self.ch == Some(b'=') {
                    self.read_char();
                    Token::Equal("==".to_owned())
                } else {
                    Token::Assign("=".to_owned())
                }
            }
            ';' => Token::Semicolon(";".to_owned()),
            ',' => Token::Comma(",".to_owned()),
            '(' => Token::Lparen("(".to_owned()),
            ')' => Token::Rparen(")".to_owned()),
            '!' => {
                self.peek_char();

                if self.ch == Some(b'=') {
                    self.read_char();
                    Token::Notequal("!=".to_owned())
                } else {
                    Token::Bang("!".to_owned())
                }
            }
            '+' => Token::Plus("+".to_owned()),
            '-' => Token::Minus("-".to_owned()),
            '*' => Token::Asterisk("*".to_owned()),
            '/' => Token::Slash("/".to_owned()),
            '{' => Token::Lbrace("{".to_owned()),
            '}' => Token::Rbrace("}".to_owned()),
            '>' => Token::Gt(">".to_owned()),
            '<' => Token::Lt("<".to_owned()),
            _ => {
                if self.ch.unwrap().is_ascii_alphabetic() {
                    let identifier = self.read_identifier();
                    if let Some(token) = token_map.get(&identifier).cloned() {
                        return token;
                    }

                    return Token::Ident(identifier);
                }

                if self.ch.unwrap().is_ascii_digit() {
                    return Token::Int(self.read_number());
                }

                Token::Illegal
            }
        };

        self.read_char();
        result
    }

    fn skip_white_space(&mut self) {
        loop {
            if self.ch.is_none() {
                break;
            }
            if self.ch.unwrap().is_ascii_whitespace() {
                self.read_char();
                continue;
            }

            break;
        }
    }

    fn read_identifier(&mut self) -> String {
        let position = self.read_position - 1;
        loop {
            if !self.ch.unwrap().is_ascii_alphabetic() {
                break;
            }

            self.read_char();
        }

        String::from_utf8(self.input[position..self.read_position - 1].to_owned()).unwrap()
    }

    fn read_number(&mut self) -> String {
        let position = self.read_position - 1;
        loop {
            if !self.ch.unwrap().is_ascii_digit() {
                break;
            }

            self.read_char();
        }

        String::from_utf8(self.input[position..self.read_position - 1].to_owned()).unwrap()
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = self.input.get(self.read_position).copied();
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = self.input.get(self.read_position).copied();
        }
    }
}

#[test]
fn test_next_token() {
    let input = r#"
        let five = 5;
        let ten = 10;
        
        let add = fn(x, y) {
            x + y;
        };

        let result = add(five, ten); 
        !-/*5;
        5 < 10 > 5;

        if (5 < 10) {
            return true;
        } else {
            return false;
        }

        10 == 10;
        1 != 4;
        "#
    .to_owned();

    let expectation = [
        Token::Let("let".to_owned()),
        Token::Ident("five".to_owned()),
        Token::Assign("=".to_owned()),
        Token::Int("5".to_owned()),
        Token::Semicolon(";".to_owned()),
        Token::Let("let".to_owned()),
        Token::Ident("ten".to_owned()),
        Token::Assign("=".to_owned()),
        Token::Int("10".to_owned()),
        Token::Semicolon(";".to_owned()),
        Token::Let("let".to_owned()),
        Token::Ident("add".to_owned()),
        Token::Assign("=".to_owned()),
        Token::Function("fn".to_owned()),
        Token::Lparen("(".to_owned()),
        Token::Ident("x".to_owned()),
        Token::Comma(",".to_owned()),
        Token::Ident("y".to_owned()),
        Token::Rparen(")".to_owned()),
        Token::Lbrace("{".to_owned()),
        Token::Ident("x".to_owned()),
        Token::Plus("+".to_owned()),
        Token::Ident("y".to_owned()),
        Token::Semicolon(";".to_owned()),
        Token::Rbrace("}".to_owned()),
        Token::Semicolon(";".to_owned()),
        Token::Let("let".to_owned()),
        Token::Ident("result".to_owned()),
        Token::Assign("=".to_owned()),
        Token::Ident("add".to_owned()),
        Token::Lparen("(".to_owned()),
        Token::Ident("five".to_owned()),
        Token::Comma(",".to_owned()),
        Token::Ident("ten".to_owned()),
        Token::Rparen(")".to_owned()),
        Token::Semicolon(";".to_owned()),
        Token::Bang("!".to_owned()),
        Token::Minus("-".to_owned()),
        Token::Slash("/".to_owned()),
        Token::Asterisk("*".to_owned()),
        Token::Int("5".to_owned()),
        Token::Semicolon(";".to_owned()),
        Token::Int("5".to_owned()),
        Token::Lt("<".to_owned()),
        Token::Int("10".to_owned()),
        Token::Gt(">".to_owned()),
        Token::Int("5".to_owned()),
        Token::Semicolon(";".to_owned()),
        // if (5 < 10) {
        // return true;
        // } else {
        // return false;
        // }
        Token::If("if".to_owned()),
        Token::Lparen("(".to_owned()),
        Token::Int("5".to_owned()),
        Token::Lt("<".to_owned()),
        Token::Int("10".to_owned()),
        Token::Rparen(")".to_owned()),
        Token::Lbrace("{".to_owned()),
        Token::Return("return".to_owned()),
        Token::True("true".to_owned()),
        Token::Semicolon(";".to_owned()),
        Token::Rbrace("}".to_owned()),
        Token::Else("else".to_owned()),
        Token::Lbrace("{".to_owned()),
        Token::Return("return".to_owned()),
        Token::False("false".to_owned()),
        Token::Semicolon(";".to_owned()),
        Token::Rbrace("}".to_owned()),
        Token::Int("10".to_owned()),
        Token::Equal("==".to_owned()),
        Token::Int("10".to_owned()),
        Token::Semicolon(";".to_owned()),
        Token::Int("1".to_owned()),
        Token::Notequal("!=".to_owned()),
        Token::Int("4".to_owned()),
        Token::Semicolon(";".to_owned()),
    ];

    let mut token_processor = Lexer::new(input);

    for token in expectation {
        assert_eq!(&token, &mut token_processor.next_token());
    }
}

#[test]
fn test_token() {
    let input = " \n =+(){}".to_owned();
    let expectation = [
        Token::Assign("=".to_owned()),
        Token::Plus("+".to_owned()),
        Token::Lparen("(".to_owned()),
        Token::Rparen(")".to_owned()),
        Token::Lbrace("{".to_owned()),
        Token::Rbrace("}".to_owned()),
    ];

    let mut token_processor = Lexer::new(input);

    for token in expectation {
        assert_eq!(&token, &mut token_processor.next_token());
    }
}
