use crate::scanner::scanner::Scanner;
use crate::scanner::token::TokenType;
use std::fs;

mod scanner;

fn main() {
    let source = fs::read_to_string("script.txt").unwrap();
    let mut scanner = Scanner::new(String::from(source));

    loop {
        match scanner.scan_token() {
            Ok(token) => {
                println!("{:?}", token);

                if token.token_type == TokenType::EOF {
                    break;
                }
            }
            Err(e) => {
                println!("error: {}", e);
                break;
            }
        }
    }
}
