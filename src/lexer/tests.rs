use crate::{
    lexer::Lexer,
    token::{Token, TokenLiteral},
};

#[test]
fn test_token() {
    let input = " \n =+(){}";
    let expectation = [
        TokenLiteral::new(Token::Assign, "=".to_owned()),
        TokenLiteral::new(Token::Plus, "+".to_owned()),
        TokenLiteral::new(Token::Lparen, "(".to_owned()),
        TokenLiteral::new(Token::Rparen, ")".to_owned()),
        TokenLiteral::new(Token::Lbrace, "{".to_owned()),
        TokenLiteral::new(Token::Rbrace, "}".to_owned()),
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
        TokenLiteral::new(Token::Let, "let".to_owned()),
        TokenLiteral::new(Token::Ident, "five".to_owned()),
        TokenLiteral::new(Token::Assign, "=".to_owned()),
        TokenLiteral::new(Token::Int, "5".to_owned()),
        TokenLiteral::new(Token::Semicolon, ";".to_owned()),
        TokenLiteral::new(Token::Let, "let".to_owned()),
        TokenLiteral::new(Token::Ident, "ten".to_owned()),
        TokenLiteral::new(Token::Assign, "=".to_owned()),
        TokenLiteral::new(Token::Int, "10".to_owned()),
        TokenLiteral::new(Token::Semicolon, ";".to_owned()),
        TokenLiteral::new(Token::Let, "let".to_owned()),
        TokenLiteral::new(Token::Ident, "add".to_owned()),
        TokenLiteral::new(Token::Assign, "=".to_owned()),
        TokenLiteral::new(Token::Function, "fn".to_owned()),
        TokenLiteral::new(Token::Lparen, "(".to_owned()),
        TokenLiteral::new(Token::Ident, "x".to_owned()),
        TokenLiteral::new(Token::Comma, ",".to_owned()),
        TokenLiteral::new(Token::Ident, "y".to_owned()),
        TokenLiteral::new(Token::Rparen, ")".to_owned()),
        TokenLiteral::new(Token::Lbrace, "{".to_owned()),
        TokenLiteral::new(Token::Ident, "x".to_owned()),
        TokenLiteral::new(Token::Plus, "+".to_owned()),
        TokenLiteral::new(Token::Ident, "y".to_owned()),
        TokenLiteral::new(Token::Semicolon, ";".to_owned()),
        TokenLiteral::new(Token::Rbrace, "}".to_owned()),
        TokenLiteral::new(Token::Semicolon, ";".to_owned()),
        TokenLiteral::new(Token::Let, "let".to_owned()),
        TokenLiteral::new(Token::Ident, "result".to_owned()),
        TokenLiteral::new(Token::Assign, "=".to_owned()),
        TokenLiteral::new(Token::Ident, "add".to_owned()),
        TokenLiteral::new(Token::Lparen, "(".to_owned()),
        TokenLiteral::new(Token::Ident, "five".to_owned()),
        TokenLiteral::new(Token::Comma, ",".to_owned()),
        TokenLiteral::new(Token::Ident, "ten".to_owned()),
        TokenLiteral::new(Token::Rparen, ")".to_owned()),
        TokenLiteral::new(Token::Semicolon, ";".to_owned()),
        TokenLiteral::new(Token::Bang, "!".to_owned()),
        TokenLiteral::new(Token::Minus, "-".to_owned()),
        TokenLiteral::new(Token::Slash, "/".to_owned()),
        TokenLiteral::new(Token::Asterisk, "*".to_owned()),
        TokenLiteral::new(Token::Int, "5".to_owned()),
        TokenLiteral::new(Token::Semicolon, ";".to_owned()),
        TokenLiteral::new(Token::Int, "5".to_owned()),
        TokenLiteral::new(Token::Lt, "<".to_owned()),
        TokenLiteral::new(Token::Int, "10".to_owned()),
        TokenLiteral::new(Token::Gt, ">".to_owned()),
        TokenLiteral::new(Token::Int, "5".to_owned()),
        TokenLiteral::new(Token::Semicolon, ";".to_owned()),
        TokenLiteral::new(Token::If, "if".to_owned()),
        TokenLiteral::new(Token::Lparen, "(".to_owned()),
        TokenLiteral::new(Token::Int, "5".to_owned()),
        TokenLiteral::new(Token::Lt, "<".to_owned()),
        TokenLiteral::new(Token::Int, "10".to_owned()),
        TokenLiteral::new(Token::Rparen, ")".to_owned()),
        TokenLiteral::new(Token::Lbrace, "{".to_owned()),
        TokenLiteral::new(Token::Return, "return".to_owned()),
        TokenLiteral::new(Token::True, "true".to_owned()),
        TokenLiteral::new(Token::Semicolon, ";".to_owned()),
        TokenLiteral::new(Token::Rbrace, "}".to_owned()),
        TokenLiteral::new(Token::Else, "else".to_owned()),
        TokenLiteral::new(Token::Lbrace, "{".to_owned()),
        TokenLiteral::new(Token::Return, "return".to_owned()),
        TokenLiteral::new(Token::False, "false".to_owned()),
        TokenLiteral::new(Token::Semicolon, ";".to_owned()),
        TokenLiteral::new(Token::Rbrace, "}".to_owned()),
        TokenLiteral::new(Token::Int, "10".to_owned()),
        TokenLiteral::new(Token::Equal, "==".to_owned()),
        TokenLiteral::new(Token::Int, "10".to_owned()),
        TokenLiteral::new(Token::Semicolon, ";".to_owned()),
        TokenLiteral::new(Token::Int, "1".to_owned()),
        TokenLiteral::new(Token::Notequal, "!=".to_owned()),
        TokenLiteral::new(Token::Int, "4".to_owned()),
        TokenLiteral::new(Token::Semicolon, ";".to_owned()),
    ];

    let mut token_processor = Lexer::new(input);

    for token in expectation {
        assert_eq!(&token, &mut token_processor.next_token());
    }
}
