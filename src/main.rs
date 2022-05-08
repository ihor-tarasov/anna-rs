use std::{
    collections::HashMap,
    io::Write,
    sync::{Arc, Mutex},
};

use anna_rs::{
    debug,
    exprs::{self, EvalArgs},
    lexer::Lexer,
    parser::{self, Parser, ParserErrorType, ParserFrame, ParserStack},
    types::{Storage, Value},
    Functions, State,
};

fn read_file(path: String) {
    let code = std::fs::read_to_string(path).expect("Unable to open file");

    let storage = Arc::new(Mutex::new(Storage::new()));
    let mut state = State::new();
    state.stack_mut().push(HashMap::new());

    let mut functions = Arc::new(Functions::new());

    let mut stack = ParserStack::new();
    stack.push(ParserFrame::new());
    stack.last_mut().unwrap().push_block();

    anna_rs::std::register(
        &mut state,
        &mut stack,
        Arc::get_mut(&mut functions).unwrap(),
    );

    let mut lexer = Lexer::new(code.as_bytes());

    let mut parser = Parser::new(Arc::get_mut(&mut functions).unwrap(), &mut stack);

    match parser::parse(&mut lexer, &mut parser) {
        Ok(expression) => {
            let functions = Arc::clone(&functions);
            let mut eval_args = EvalArgs {
                state: &mut state,
                storage: Arc::clone(&storage),
                functions,
            };
            match exprs::eval(&expression, &mut eval_args) {
                Ok(value) => match value {
                    Value::Void => (),
                    _ => debug::println_value(value, storage),
                },
                Err(error) => {
                    debug::print_info(code.as_bytes(), error.info());
                    println!("Runtime error: {:?}", error.etype());
                }
            }
        }
        Err(error) => {
            match error.etype() {
                ParserErrorType::Empty => return,
                _ => (),
            }
            debug::print_info(code.as_bytes(), error.info());
            println!("Parser error: {:?}", error.etype());
        }
    }
}

fn main() {
    if std::env::args().len() == 2 {
        return read_file(std::env::args().nth(1).unwrap());
    }

    let storage = Arc::new(Mutex::new(Storage::new()));
    let mut state = State::new();
    state.stack_mut().push(HashMap::new());

    let mut functions = Arc::new(Functions::new());

    let mut stack = ParserStack::new();
    stack.push(ParserFrame::new());
    stack.last_mut().unwrap().push_block();

    anna_rs::std::register(
        &mut state,
        &mut stack,
        Arc::get_mut(&mut functions).unwrap(),
    );

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

            let mut parser = Parser::new(Arc::get_mut(&mut functions).unwrap(), &mut stack);

            match parser::parse(&mut lexer, &mut parser) {
                Ok(expression) => {
                    let functions = Arc::clone(&functions);
                    let mut eval_args = EvalArgs {
                        state: &mut state,
                        storage: Arc::clone(&storage),
                        functions,
                    };
                    match exprs::eval(&expression, &mut eval_args) {
                        Ok(value) => match value {
                            Value::Void => (),
                            _ => debug::println_value(value, Arc::clone(&storage)),
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
