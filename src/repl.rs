use std::io;

use crate::{lexer::Lexer, token::Token};

pub fn repl_loop() {
    loop {
        let mut input: String = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        input = input.trim_end().to_string();
        if input.as_str() == "exit" {
            break;
        }

        let mut token_processor = Lexer::new(&input);

        loop {
            let token = token_processor.next_token();
            if token.token == Token::Eof {
                break;
            }

            println!("{:?}", token);
        }
    }
}
