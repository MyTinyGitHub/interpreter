//! Lexical analyzer for the Monkey language.
//!
//! Scans source code character-by-character and produces a stream of tokens.
//! Uses a two-pointer technique for one-character lookahead to distinguish
//! operators like `=` from `==`.
//!
//! # Key Types
//!
//! - `Lexer`: Main lexer struct with input buffer and position tracking
//! - `TOKEN_MAP`: Static map of keywords to their token variants
//!
//! # Design Decisions
//!
//! - Uses `Vec<u8>` instead of `&str` for O(1) indexing (Monkey is ASCII-only)
//! - Static keyword map for O(1) keyword lookup
//! - Returns `Token::Illegal` for unrecognized characters

use std::collections::HashMap;
use std::sync::LazyLock;

use crate::token::Token;

#[cfg(test)]
pub mod tests;

/// Lexical analyzer that tokenizes Monkey source code.
///
/// Uses two positions (`position` and `read_position`) to implement lookahead.
/// The `ch` field holds the current character being processed.
pub struct Lexer {
    input: Vec<u8>,
    position: usize,
    read_position: usize,
    ch: Option<u8>,
}

static TOKEN_MAP: LazyLock<HashMap<&'static str, Token>> = LazyLock::new(|| {
    HashMap::from([
        ("let", Token::Let),
        ("fn", Token::Function),
        ("if", Token::If),
        ("else", Token::Else),
        ("return", Token::Return),
        ("true", Token::True),
        ("false", Token::False),
    ])
});

impl Lexer {
    /// Creates a new Lexer from source code.
    ///
    /// Initializes position to 0 and reads the first character.
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

    pub fn next_token(&mut self) -> Token {
        self.skip_white_space();

        if self.ch.is_none() {
            return Token::Eof;
        }

        let result = match self.ch.unwrap() as char {
            '=' => {
                self.peek_char();
                if self.ch == Some(b'=') {
                    self.read_char();
                    Token::Equal
                } else {
                    Token::Assign
                }
            }
            ';' => Token::Semicolon,
            ',' => Token::Comma,
            '(' => Token::Lparen,
            ')' => Token::Rparen,
            '!' => {
                self.peek_char();

                if self.ch == Some(b'=') {
                    self.read_char();
                    Token::Notequal
                } else {
                    Token::Bang
                }
            }
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Asterisk,
            '/' => Token::Slash,
            '{' => Token::Lbrace,
            '}' => Token::Rbrace,
            '>' => Token::Gt,
            '<' => Token::Lt,
            _ => {
                if self.ch.unwrap().is_ascii_alphabetic() {
                    let identifier = self.read_identifier();
                    if let Some(token) = TOKEN_MAP.get(identifier.as_str()) {
                        return token.to_owned();
                    }

                    return Token::Ident(identifier);
                }

                if self.ch.is_some() {
                    return Token::Int(self.read_number());
                }

                Token::Illegal
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
