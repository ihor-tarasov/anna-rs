use std::{
    collections::HashMap,
    io::Write,
    sync::{Arc, Mutex},
};

use anna_rs::{
    debug,
    exprs::{self, EvalArgs},
    lexer::Lexer,
    parser::{self, ParserErrorType, Parser},
    types::{Storage, Value},
    Functions, State,
};

fn main() {
    let storage = Arc::new(Mutex::new(Storage::new()));
    let mut state = State::new();
    state.stack_mut().push(HashMap::new());

    let mut functions = Arc::new(Functions::new());
    anna_rs::std::register(&mut state, Arc::get_mut(&mut functions).unwrap());

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

            let mut parser = Parser::new(Arc::get_mut(&mut functions).unwrap());
            
            match parser::parse(&mut lexer, &mut parser) {
                Ok(expression) => {
                    let functions = Arc::clone(&functions);
                    let mut eval_args = EvalArgs {
                        state: &mut state,
                        storage: storage.clone(),
                        functions,
                    };
                    match exprs::eval(&expression, &mut eval_args) {
                        Ok(value) => match value {
                            Value::Void => (),
                            _ => debug::println_value(value),
                        },
                        Err(error) => {
                            debug::print_info(code.as_bytes(), error.info());
                            println!("Runtime error: {:?}", error.etype());
                        }
                    }
                }
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
