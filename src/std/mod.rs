use crate::{State, types::{Value, Object}, debug, native, Functions, parser::ParserStack};

fn print_value(args: Vec<Value>) {
    let mut it = args.iter();

    if let Some(value) = it.next() {
        debug::print_value(value.clone());
        while let Some(value) = it.next() {
            print!(", ");
            debug::print_value(value.clone());
        }
    }
}

pub fn register(state: &mut State, stack: &mut ParserStack, functions: &mut Functions) {
    stack.last_mut().unwrap().push_variable("print".to_string());
    native(state, functions, "print".to_string(),  |_storage, args| {
        print_value(args);
        Value::Void
    });

    stack.last_mut().unwrap().push_variable("println".to_string());
    native(state, functions, "println".to_string(), |_storage, args| {
        print_value(args);
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

    stack.last_mut().unwrap().push_variable("join".to_string());
    native(state, functions, "join".to_string(), |storage, args| {
        let mut result = Value::Void;
        for arg in args {
            match arg {
                Value::ObjectId(id) => {
                    match storage.lock().unwrap().get_mut(id) {
                        Object::Thread(handle) => {
                            if let Some(handle) = handle.take() {
                                result = handle.join().unwrap();
                            } else {
                                result = Value::Void;
                            }
                        },
                        _ => result = arg,
                    }
                },
                _ => result = arg,
            }
        }
        result
    });
}
