use crate::token::Token;

pub mod token;
pub mod token_processor;

fn main() {
    println!("{:?}", Token::Illegal);
}
