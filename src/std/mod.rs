use std::sync::Arc;

use crate::{
    debug, native,
    parser::ParserStack,
    state::StorageRc,
    types::{Value, Object},
    Functions, State,
};

fn print_value(args: Vec<Value>, storage: StorageRc) {
    for arg in args {
        debug::print_value(arg, Arc::clone(&storage));
    }
}

pub fn register(state: &mut State, stack: &mut ParserStack, functions: &mut Functions) {
    stack.last_mut().unwrap().push_variable("print".to_string());
    native(state, functions, "print".to_string(), |storage, args| {
        print_value(args, storage);
        Value::Void
    });

    stack
        .last_mut()
        .unwrap()
        .push_variable("println".to_string());
    native(state, functions, "println".to_string(), |storage, args| {
        print_value(args, storage);
        println!();
        Value::Void
    });

    stack.last_mut().unwrap().push_variable("exit".to_string());
    native(state, functions, "exit".to_string(), |_storage, _args| {
        std::process::exit(0);
    });

    stack.last_mut().unwrap().push_variable("sleep".to_string());
    native(state, functions, "sleep".to_string(), |_storage, args| {
        let mut millis = 1000;
        if args.len() == 1 {
            if let Value::Integer(i) = args[0] {
                if i > 0 {
                    millis = i as u64;
                }
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(millis));
        Value::Void
    });

    stack.last_mut().unwrap().push_variable("size".to_string());
    native(state, functions, "size".to_string(), |storage, args| {
        match args.first() {
            Some(arg) => match arg {
                Value::ObjectId(id) => {
                    match storage.lock().unwrap().get(*id) {
                        Object::String(string) => Value::Integer(string.len() as i64),
                        Object::Array(array) => Value::Integer(array.len() as i64),
                        Object::Range(range) => Value::Integer(range.1 - range.0),
                        _ => Value::Integer(1),
                    }
                },
                _ => Value::Integer(1)
            },
            None => Value::Integer(0),
        }
    });

    stack.last_mut().unwrap().push_variable("range".to_string());
    native(state, functions, "range".to_string(), |storage, args| {
        match args.first() {
            Some(start) => match start {
                Value::Integer(start) => match args.get(1) {
                    Some(end) => match end {
                        Value::Integer(end) => {
                            storage.lock().unwrap().push(Object::Range((*start, *end)))
                        },
                        _ => Value::Void,
                    },
                    None => Value::Void,
                },
                _ => Value::Void,
            },
            None => Value::Void,
        }
    });

    stack.last_mut().unwrap().push_variable("push".to_string());
    native(state, functions, "push".to_string(), |storage, args| {
        match args.first() {
            Some(target) => match *target {
                Value::ObjectId(id) => match storage.lock().unwrap().get_mut(id) {
                    Object::String(string) => match args.get(1) {
                        Some(value) => match *value {
                            Value::Integer(value) => {
                                string.push(char::from_u32(value as u32).unwrap());
                                Value::Void
                            },
                            _ => Value::Void,
                        },
                        None => Value::Void,
                    },
                    Object::Array(array) => match args.get(1) {
                        Some(value) => {
                            array.push(value.clone());
                            Value::Void
                        },
                        None => Value::Void,
                    },
                    _ => Value::Void,
                },
                _ => Value::Void
            },
            None => Value::Void,
        }
    });
}
