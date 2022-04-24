use std::io::Write;

use anna_rs::{exprs, lexer::Lexer, parser::{self, ParserErrorType}, State, debug::print_info};

fn main() {
    let mut state = State::new();

    loop {
        print!("-> ");

        let mut code = String::new();

        loop {
            std::io::stdout().flush().unwrap();

            let mut line = String::new();

            std::io::stdin().read_line(&mut line).unwrap();

            line = line.replace("\t", "    ");

            code.push_str(line.as_str());

            let mut lexer = Lexer::new(code.as_bytes());

            match parser::parse(&mut lexer) {
                Ok(expression) => match exprs::eval(&expression, &mut state) {
                    Ok(value) => println!("{:?}", value),
                    Err(error) => {
                        print_info(code.as_bytes(), error.info());
                        println!("Runtime error: {:?}", error.etype());
                    },
                },
                Err(error) => {
                    if error.etype() == ParserErrorType::UnexpectedEndOfFile {
                        print!("-| ");
                        continue;
                    }
                    print_info(code.as_bytes(), error.info());
                    println!("Parser error: {:?}", error.etype());
                },
            }

            break;
        }
    }
}
