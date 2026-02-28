use crate::{
    lexer::Lexer,
    token::{Token, TokenLiteral},
};

#[test]
fn test_token() {
    let input = " \n =+(){}";
    let expectation = [
        TokenLiteral::new(Token::Assign, Some("=".to_owned())),
        TokenLiteral::new(Token::Plus, Some("+".to_owned())),
        TokenLiteral::new(Token::Lparen, Some("(".to_owned())),
        TokenLiteral::new(Token::Rparen, Some(")".to_owned())),
        TokenLiteral::new(Token::Lbrace, Some("{".to_owned())),
        TokenLiteral::new(Token::Rbrace, Some("}".to_owned())),
    ];

    let mut token_processor = Lexer::new(input);

    for token in expectation {
        assert_eq!(&token, &mut token_processor.next_token());
    }
}
