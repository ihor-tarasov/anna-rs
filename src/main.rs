use std::io::Write;

use anna_rs::{
    debug, exprs,
    lexer::Lexer,
    parser::{self, ParserErrorType},
    types::Value,
    State,
};

fn aria_print(args: Vec<Value>) {
    let mut it = args.iter();

    if let Some(value) = it.next() {
        debug::print_value(value.clone());
        while let Some(value) = it.next() {
            print!(", ");
            debug::print_value(value.clone());
        }
    }
}

fn main() {
    let mut state = State::new();

    state.native("print".to_string(), |_state, args| {
        aria_print(args);
        Value::Void
    });

    state.native("println".to_string(), |_state, args| {
        aria_print(args);
        println!();
        Value::Void
    });

    state.native("exit".to_string(), |_state, _args| {
        std::process::exit(0);
    });

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

            match parser::parse(&mut lexer, &mut state) {
                Ok(expression) => match exprs::eval(&expression, &mut state) {
                    Ok(value) => match value {
                        Value::Void => (),
                        _ => debug::println_value(value),
                    },
                    Err(error) => {
                        debug::print_info(code.as_bytes(), error.info());
                        println!("Runtime error: {:?}", error.etype());
                    }
                },
                Err(error) => {
                    match error.etype() {
                        ParserErrorType::UnexpectedEndOfFile => {
                            print!("-| ");
                            continue;
                        }
                        ParserErrorType::Empty => break,
                        _ => (),
                    }
                    debug::print_info(code.as_bytes(), error.info());
                    println!("Parser error: {:?}", error.etype());
                }
            }

            break;
        }
    }
}
