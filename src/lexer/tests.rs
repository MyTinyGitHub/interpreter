use crate::{lexer::Lexer, token::Token};

#[test]
fn test_token() {
    let input = " \n =+(){}";
    let expectation = [
        Token::Assign,
        Token::Plus,
        Token::Lparen,
        Token::Rparen,
        Token::Lbrace,
        Token::Rbrace,
    ];

    let mut token_processor = Lexer::new(input);

    for token in expectation {
        assert_eq!(&token, &mut token_processor.next_token());
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
        Token::Let,
        Token::Ident("five".to_owned()),
        Token::Assign,
        Token::Int("5".to_owned()),
        Token::Semicolon,
        Token::Let,
        Token::Ident("ten".to_owned()),
        Token::Assign,
        Token::Int("10".to_owned()),
        Token::Semicolon,
        Token::Let,
        Token::Ident("add".to_owned()),
        Token::Assign,
        Token::Function,
        Token::Lparen,
        Token::Ident("x".to_owned()),
        Token::Comma,
        Token::Ident("y".to_owned()),
        Token::Rparen,
        Token::Lbrace,
        Token::Ident("x".to_owned()),
        Token::Plus,
        Token::Ident("y".to_owned()),
        Token::Semicolon,
        Token::Rbrace,
        Token::Semicolon,
        Token::Let,
        Token::Ident("result".to_owned()),
        Token::Assign,
        Token::Ident("add".to_owned()),
        Token::Lparen,
        Token::Ident("five".to_owned()),
        Token::Comma,
        Token::Ident("ten".to_owned()),
        Token::Rparen,
        Token::Semicolon,
        Token::Bang,
        Token::Minus,
        Token::Slash,
        Token::Asterisk,
        Token::Int("5".to_owned()),
        Token::Semicolon,
        Token::Int("5".to_owned()),
        Token::Lt,
        Token::Int("10".to_owned()),
        Token::Gt,
        Token::Int("5".to_owned()),
        Token::Semicolon,
        Token::If,
        Token::Lparen,
        Token::Int("5".to_owned()),
        Token::Lt,
        Token::Int("10".to_owned()),
        Token::Rparen,
        Token::Lbrace,
        Token::Return,
        Token::True,
        Token::Semicolon,
        Token::Rbrace,
        Token::Else,
        Token::Lbrace,
        Token::Return,
        Token::False,
        Token::Semicolon,
        Token::Rbrace,
        Token::Int("10".to_owned()),
        Token::Equal,
        Token::Int("10".to_owned()),
        Token::Semicolon,
        Token::Int("1".to_owned()),
        Token::Notequal,
        Token::Int("4".to_owned()),
        Token::Semicolon,
    ];

    let mut token_processor = Lexer::new(input);

    for token in expectation {
        assert_eq!(&token, &mut token_processor.next_token());
    }
}
