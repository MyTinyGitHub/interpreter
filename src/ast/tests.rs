use crate::{
    ast::{Expression, Identifier, LetStatement, Program, Statement},
    token::Token,
};

#[test]
fn test_string() {
    let input = Program {
        statements: vec![Statement::Let(LetStatement {
            token: Token::Let,
            name: Identifier {
                token: Token::Ident("myVar".to_owned()),
            },
            value: Expression::Identifier(Identifier {
                token: Token::Ident("anotherVar".to_owned()),
            }),
        })],
    };

    assert_eq!(input.string(), "let myVar = anotherVar;")
}
