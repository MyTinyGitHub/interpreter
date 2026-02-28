use std::collections::HashMap;

use crate::token::{Token, TokenLiteral};

#[cfg(test)]
pub mod tests;

pub struct Lexer {
    input: Vec<u8>,
    position: usize,
    read_position: usize,
    ch: Option<u8>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut result = Self {
            input: input.as_bytes().to_owned(),
            position: 0,
            read_position: 0,
            ch: None,
        };

        result.read_char();
        result
    }

    pub fn next_token(&mut self) -> TokenLiteral {
        let token_map = HashMap::from([
            (
                "let".to_owned(),
                TokenLiteral::new(Token::Let, Some("let".to_owned())),
            ),
            (
                "fn".to_owned(),
                TokenLiteral::new(Token::Function, Some("fn".to_owned())),
            ),
            (
                "if".to_owned(),
                TokenLiteral::new(Token::If, Some("if".to_owned())),
            ),
            (
                "else".to_owned(),
                TokenLiteral::new(Token::Else, Some("else".to_owned())),
            ),
            (
                "return".to_owned(),
                TokenLiteral::new(Token::Return, Some("return".to_owned())),
            ),
            (
                "true".to_owned(),
                TokenLiteral::new(Token::True, Some("true".to_owned())),
            ),
            (
                "false".to_owned(),
                TokenLiteral::new(Token::False, Some("false".to_owned())),
            ),
        ]);

        self.skip_white_space();

        if self.ch.is_none() {
            return TokenLiteral::new(Token::Eof, None);
        }

        let result = match self.ch.unwrap() as char {
            '=' => {
                self.peek_char();
                if self.ch == Some(b'=') {
                    self.read_char();
                    TokenLiteral::new(Token::Equal, Some("==".to_owned()))
                } else {
                    TokenLiteral::new(Token::Assign, Some("=".to_owned()))
                }
            }
            ';' => TokenLiteral::new(Token::Semicolon, Some(";".to_owned())),
            ',' => TokenLiteral::new(Token::Comma, Some(",".to_owned())),
            '(' => TokenLiteral::new(Token::Lparen, Some("(".to_owned())),
            ')' => TokenLiteral::new(Token::Rparen, Some(")".to_owned())),
            '!' => {
                self.peek_char();

                if self.ch == Some(b'=') {
                    self.read_char();
                    TokenLiteral::new(Token::Notequal, Some("!=".to_owned()))
                } else {
                    TokenLiteral::new(Token::Bang, Some("!".to_owned()))
                }
            }
            '+' => TokenLiteral::new(Token::Plus, Some("+".to_owned())),
            '-' => TokenLiteral::new(Token::Minus, Some("-".to_owned())),
            '*' => TokenLiteral::new(Token::Asterisk, Some("*".to_owned())),
            '/' => TokenLiteral::new(Token::Slash, Some("/".to_owned())),
            '{' => TokenLiteral::new(Token::Lbrace, Some("{".to_owned())),
            '}' => TokenLiteral::new(Token::Rbrace, Some("}".to_owned())),
            '>' => TokenLiteral::new(Token::Gt, Some(">".to_owned())),
            '<' => TokenLiteral::new(Token::Lt, Some("<".to_owned())),
            _ => {
                if self.ch.unwrap().is_ascii_alphabetic() {
                    let identifier = self.read_identifier();
                    if let Some(token) = token_map.get(&identifier).cloned() {
                        return token;
                    }

                    return TokenLiteral::new(Token::Ident, Some(identifier));
                }

                if self.ch.is_some() {
                    return TokenLiteral::new(Token::Int, Some(self.read_number()));
                }

                TokenLiteral::new(Token::Illegal, None)
            }
        };

        self.read_char();
        result
    }

    fn skip_white_space(&mut self) {
        while let Some(ch) = self.ch {
            if !ch.is_ascii_whitespace() {
                break;
            }

            self.read_char();
        }
    }

    fn read_identifier(&mut self) -> String {
        let position = self.read_position - 1;

        while let Some(ch) = self.ch {
            if ch.is_ascii_alphabetic() {
                self.read_char();
            } else {
                break;
            }
        }

        String::from_utf8(self.input[position..self.read_position - 1].to_owned()).unwrap()
    }

    fn read_number(&mut self) -> String {
        let position = self.read_position - 1;

        while let Some(ch) = self.ch {
            if ch.is_ascii_digit() {
                self.read_char();
            } else {
                break;
            }
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
        "#;

    let expectation = [
        TokenLiteral::new(Token::Let, Some("let".to_owned())),
        TokenLiteral::new(Token::Ident, Some("five".to_owned())),
        TokenLiteral::new(Token::Assign, Some("=".to_owned())),
        TokenLiteral::new(Token::Int, Some("5".to_owned())),
        TokenLiteral::new(Token::Semicolon, Some(";".to_owned())),
        TokenLiteral::new(Token::Let, Some("let".to_owned())),
        TokenLiteral::new(Token::Ident, Some("ten".to_owned())),
        TokenLiteral::new(Token::Assign, Some("=".to_owned())),
        TokenLiteral::new(Token::Int, Some("10".to_owned())),
        TokenLiteral::new(Token::Semicolon, Some(";".to_owned())),
        TokenLiteral::new(Token::Let, Some("let".to_owned())),
        TokenLiteral::new(Token::Ident, Some("add".to_owned())),
        TokenLiteral::new(Token::Assign, Some("=".to_owned())),
        TokenLiteral::new(Token::Function, Some("fn".to_owned())),
        TokenLiteral::new(Token::Lparen, Some("(".to_owned())),
        TokenLiteral::new(Token::Ident, Some("x".to_owned())),
        TokenLiteral::new(Token::Comma, Some(",".to_owned())),
        TokenLiteral::new(Token::Ident, Some("y".to_owned())),
        TokenLiteral::new(Token::Rparen, Some(")".to_owned())),
        TokenLiteral::new(Token::Lbrace, Some("{".to_owned())),
        TokenLiteral::new(Token::Ident, Some("x".to_owned())),
        TokenLiteral::new(Token::Plus, Some("+".to_owned())),
        TokenLiteral::new(Token::Ident, Some("y".to_owned())),
        TokenLiteral::new(Token::Semicolon, Some(";".to_owned())),
        TokenLiteral::new(Token::Rbrace, Some("}".to_owned())),
        TokenLiteral::new(Token::Semicolon, Some(";".to_owned())),
        TokenLiteral::new(Token::Let, Some("let".to_owned())),
        TokenLiteral::new(Token::Ident, Some("result".to_owned())),
        TokenLiteral::new(Token::Assign, Some("=".to_owned())),
        TokenLiteral::new(Token::Ident, Some("add".to_owned())),
        TokenLiteral::new(Token::Lparen, Some("(".to_owned())),
        TokenLiteral::new(Token::Ident, Some("five".to_owned())),
        TokenLiteral::new(Token::Comma, Some(",".to_owned())),
        TokenLiteral::new(Token::Ident, Some("ten".to_owned())),
        TokenLiteral::new(Token::Rparen, Some(")".to_owned())),
        TokenLiteral::new(Token::Semicolon, Some(";".to_owned())),
        TokenLiteral::new(Token::Bang, Some("!".to_owned())),
        TokenLiteral::new(Token::Minus, Some("-".to_owned())),
        TokenLiteral::new(Token::Slash, Some("/".to_owned())),
        TokenLiteral::new(Token::Asterisk, Some("*".to_owned())),
        TokenLiteral::new(Token::Int, Some("5".to_owned())),
        TokenLiteral::new(Token::Semicolon, Some(";".to_owned())),
        TokenLiteral::new(Token::Int, Some("5".to_owned())),
        TokenLiteral::new(Token::Lt, Some("<".to_owned())),
        TokenLiteral::new(Token::Int, Some("10".to_owned())),
        TokenLiteral::new(Token::Gt, Some(">".to_owned())),
        TokenLiteral::new(Token::Int, Some("5".to_owned())),
        TokenLiteral::new(Token::Semicolon, Some(";".to_owned())),
        TokenLiteral::new(Token::If, Some("if".to_owned())),
        TokenLiteral::new(Token::Lparen, Some("(".to_owned())),
        TokenLiteral::new(Token::Int, Some("5".to_owned())),
        TokenLiteral::new(Token::Lt, Some("<".to_owned())),
        TokenLiteral::new(Token::Int, Some("10".to_owned())),
        TokenLiteral::new(Token::Rparen, Some(")".to_owned())),
        TokenLiteral::new(Token::Lbrace, Some("{".to_owned())),
        TokenLiteral::new(Token::Return, Some("return".to_owned())),
        TokenLiteral::new(Token::True, Some("true".to_owned())),
        TokenLiteral::new(Token::Semicolon, Some(";".to_owned())),
        TokenLiteral::new(Token::Rbrace, Some("}".to_owned())),
        TokenLiteral::new(Token::Else, Some("else".to_owned())),
        TokenLiteral::new(Token::Lbrace, Some("{".to_owned())),
        TokenLiteral::new(Token::Return, Some("return".to_owned())),
        TokenLiteral::new(Token::False, Some("false".to_owned())),
        TokenLiteral::new(Token::Semicolon, Some(";".to_owned())),
        TokenLiteral::new(Token::Rbrace, Some("}".to_owned())),
        TokenLiteral::new(Token::Int, Some("10".to_owned())),
        TokenLiteral::new(Token::Equal, Some("==".to_owned())),
        TokenLiteral::new(Token::Int, Some("10".to_owned())),
        TokenLiteral::new(Token::Semicolon, Some(";".to_owned())),
        TokenLiteral::new(Token::Int, Some("1".to_owned())),
        TokenLiteral::new(Token::Notequal, Some("!=".to_owned())),
        TokenLiteral::new(Token::Int, Some("4".to_owned())),
        TokenLiteral::new(Token::Semicolon, Some(";".to_owned())),
    ];

    let mut token_processor = Lexer::new(input);

    for token in expectation {
        assert_eq!(&token, &mut token_processor.next_token());
    }
}
