use std::io::Write;

use anna_rs::{exprs, lexer::Lexer, parser::{self, ParserErrorType}};

fn main() {
    loop {
        print!("-> ");

        let mut code = String::new();

        loop {
            std::io::stdout().flush().unwrap();

            let mut line = String::new();

            std::io::stdin().read_line(&mut line).unwrap();

            code.push_str(line.as_str());

            let mut lexer = Lexer::new(code.as_bytes());

            match parser::parse(&mut lexer) {
                Ok(expression) => match exprs::eval(&expression) {
                    Ok(value) => println!("{:?}", value),
                    Err(_error) => panic!("RuntimeError"),
                },
                Err(error) => {
                    if error.etype() == ParserErrorType::UnexpectedEndOfFile {
                        print!("-| ");
                        continue;
                    }
                    panic!("ParserError");
                },
            }
            
            break;
        }
    }
}
