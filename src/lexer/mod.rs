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
                TokenLiteral::new(Token::Let, "let".to_owned()),
            ),
            (
                "fn".to_owned(),
                TokenLiteral::new(Token::Function, "fn".to_owned()),
            ),
            (
                "if".to_owned(),
                TokenLiteral::new(Token::If, "if".to_owned()),
            ),
            (
                "else".to_owned(),
                TokenLiteral::new(Token::Else, "else".to_owned()),
            ),
            (
                "return".to_owned(),
                TokenLiteral::new(Token::Return, "return".to_owned()),
            ),
            (
                "true".to_owned(),
                TokenLiteral::new(Token::True, "true".to_owned()),
            ),
            (
                "false".to_owned(),
                TokenLiteral::new(Token::False, "false".to_owned()),
            ),
        ]);

        self.skip_white_space();

        if self.ch.is_none() {
            return TokenLiteral::new(Token::Eof, "".to_owned());
        }

        let result = match self.ch.unwrap() as char {
            '=' => {
                self.peek_char();
                if self.ch == Some(b'=') {
                    self.read_char();
                    TokenLiteral::new(Token::Equal, "==".to_owned())
                } else {
                    TokenLiteral::new(Token::Assign, "=".to_owned())
                }
            }
            ';' => TokenLiteral::new(Token::Semicolon, ";".to_owned()),
            ',' => TokenLiteral::new(Token::Comma, ",".to_owned()),
            '(' => TokenLiteral::new(Token::Lparen, "(".to_owned()),
            ')' => TokenLiteral::new(Token::Rparen, ")".to_owned()),
            '!' => {
                self.peek_char();

                if self.ch == Some(b'=') {
                    self.read_char();
                    TokenLiteral::new(Token::Notequal, "!=".to_owned())
                } else {
                    TokenLiteral::new(Token::Bang, "!".to_owned())
                }
            }
            '+' => TokenLiteral::new(Token::Plus, "+".to_owned()),
            '-' => TokenLiteral::new(Token::Minus, "-".to_owned()),
            '*' => TokenLiteral::new(Token::Asterisk, "*".to_owned()),
            '/' => TokenLiteral::new(Token::Slash, "/".to_owned()),
            '{' => TokenLiteral::new(Token::Lbrace, "{".to_owned()),
            '}' => TokenLiteral::new(Token::Rbrace, "}".to_owned()),
            '>' => TokenLiteral::new(Token::Gt, ">".to_owned()),
            '<' => TokenLiteral::new(Token::Lt, "<".to_owned()),
            _ => {
                if self.ch.unwrap().is_ascii_alphabetic() {
                    let identifier = self.read_identifier();
                    if let Some(token) = token_map.get(&identifier).cloned() {
                        return token;
                    }

                    return TokenLiteral::new(Token::Ident, identifier);
                }

                if self.ch.is_some() {
                    return TokenLiteral::new(Token::Int, self.read_number());
                }

                TokenLiteral::new(Token::Illegal, "".to_owned())
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
