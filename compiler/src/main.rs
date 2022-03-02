use crate::parser::Parser;
use crate::scanner::{token::TokenType, Scanner};

use std::fs;

mod parser;
mod scanner;

fn main() {
    let source = fs::read_to_string("script.txt").unwrap();
    let mut scanner = Scanner::new(source.clone());

    match scanner.scan_all() {
        Ok(tokens) => {
            let mut parser = Parser::new(tokens);

            match parser.parse() {
                Ok(expr) => println!("{:#?}", expr),
                Err(e) => {
                    eprintln!("parse error: {}", e);
                    e.token.range().print_source(source.as_str());
                }
            }
        }
        Err(e) => {
            eprintln!("scan error: {}", e);
            e.range().print_source(source.as_str());
        }
    }
}
