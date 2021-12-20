use crate::scanner::scanner::Scanner;

mod scanner;

fn main() {
    let source = "/* dd*/+";
    let mut scanner = Scanner::new(String::from(source));
    let result = scanner.scan_token();

    match result {
        Ok(token) => println!("token: {:?}", token),
        Err(e) => println!("error: {}", e),
    }
}
