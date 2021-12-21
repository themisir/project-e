use crate::parser::parser::Parser;
use crate::scanner::scanner::Scanner;
use crate::scanner::token::TokenType;

use std::fs;

mod parser;
mod scanner;

fn main() {
    let source = fs::read_to_string("script.txt").unwrap();
    let mut scanner = Scanner::new(String::from(source));

    match scanner.scan_all() {
        Ok(tokens) => {
            let mut parser = Parser::new(tokens);

            match parser.parse() {
                Ok(expr) => println!("{:#?}", expr),
                Err(e) => println!("parse error: {}", e),
            }
        }
        Err(e) => {
            println!("scan error: {}", e);
        }
    }
}
