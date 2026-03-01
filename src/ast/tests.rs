use crate::{
    ast::{Expression, Identifier, LetStatement, Program, Statement},
    token::{Token, TokenLiteral},
};

#[test]
fn test_string() {
    let input = Program {
        statements: vec![Statement::Let(LetStatement {
            token: TokenLiteral {
                token: Token::Let,
                value: Some("let".to_owned()),
            },
            name: Identifier {
                token: TokenLiteral {
                    token: Token::Ident,
                    value: Some("myVar".to_owned()),
                },
                value: "myVar".to_owned(),
            },
            value: Expression::Identifier(Identifier {
                token: TokenLiteral {
                    token: Token::Ident,
                    value: Some("anotherVar".to_owned()),
                },
                value: "anotherVar".to_owned(),
            }),
        })],
    };

    assert_eq!(input.string(), "let myVar = anotherVar;")
}
