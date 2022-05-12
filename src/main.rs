use std::{
    collections::HashMap,
    io::{Write, Read},
    sync::{Arc, Mutex, RwLock},
};

use anna_rs::{
    debug,
    exprs::{self, EvalArgs},
    lexer::Lexer,
    parser::{self, Parser, ParserErrorType, ParserFrame, ParserStack, ParserBlock},
    types::{Storage, Value},
    Functions, State,
};

fn read_file(path: &str) -> String {
    std::fs::read_to_string(path).unwrap()
}

fn run_file(path: &str) {
    let storage = Arc::new(Mutex::new(Storage::new()));
    let mut state = State::new();
    state.stack_mut().push(HashMap::new());

    let mut functions = Arc::new(Functions::new());

    let mut stack = ParserStack::new();
    stack.push(ParserFrame::new());
    stack.last_mut().unwrap().push_block(ParserBlock::new());

    anna_rs::std::register(
        &mut state,
        &mut stack,
        Arc::get_mut(&mut functions).unwrap(),
    );
    
    let file = Arc::new(RwLock::new(std::fs::File::open(path).unwrap()));

    let mut lexer = Lexer::new(
        move || {
        let mut buf = [0u8; 1000];
        let count = file.write().unwrap().read(&mut buf).unwrap();
        if count == 0 {
            None
        } else {
            Some(String::from_utf8_lossy(&buf).to_string())
        }
    });

    let mut parser = Parser::new(Arc::get_mut(&mut functions).unwrap(), &mut stack);

    match parser::parse_block(&mut lexer, &mut parser) {
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
                    debug::print_info(read_file(path).as_str(), error.info());
                    println!("Runtime error: {:?}", error.etype());
                }
            }
        }
        Err(error) => {
            debug::print_info(read_file(path).as_str(), error.info());
            println!("Parser error: {:?}", error.etype());
        }
    }
}

fn main() {
    let mut use_repl = true;
    std::env::args().skip(1).for_each(|arg| {
        run_file(arg.as_str());
        use_repl = false;
    });

    if !use_repl {
        return;
    }

    let storage = Arc::new(Mutex::new(Storage::new()));
    let mut state = State::new();
    state.stack_mut().push(HashMap::new());

    let mut functions = Arc::new(Functions::new());

    let mut stack = ParserStack::new();
    stack.push(ParserFrame::new());
    stack.last_mut().unwrap().push_block(ParserBlock::new());

    anna_rs::std::register(
        &mut state,
        &mut stack,
        Arc::get_mut(&mut functions).unwrap(),
    );

    loop {
        let code = Arc::new(RwLock::new(String::new()));
        let mut lexer = Lexer::new({
            let code = Arc::clone(&code);
            move || {
                print!("-> ");
                std::io::stdout().flush().unwrap();
                let mut line = String::new();
                std::io::stdin().read_line(&mut line).unwrap();
                line = line.replace("\t", "    ");
                code.write().unwrap().push_str(line.as_str());
                Some(line)
        }});

        stack.last_mut().unwrap().push_block(ParserBlock::new());
        let mut parser = Parser::new(Arc::get_mut(&mut functions).unwrap(), &mut stack);

        match parser::parse(&mut lexer, &mut parser) {
            Ok(expression) => {
                stack.last_mut().unwrap().merge_last();
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
                        debug::print_info(code.read().unwrap().as_str(), error.info());
                        println!("Runtime error: {:?}", error.etype());
                    }
                }
            }
            Err(error) => {
                stack.last_mut().unwrap().pop_block();
                match error.etype() {
                    ParserErrorType::Empty => break,
                    _ => (),
                }
                debug::print_info(code.read().unwrap().as_str(), error.info());
                println!("Parser error: {:?}", error.etype());
            }
        }
    }
}
