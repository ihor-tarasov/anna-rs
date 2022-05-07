use crate::{State, types::Value, debug, native, Functions};

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

pub fn register(state: &mut State, functions: &mut Functions) {
    native(state, functions, "print".to_string(),  |args| {
        print_value(args);
        Value::Void
    });

    native(state, functions, "println".to_string(), |args| {
        print_value(args);
        println!();
        Value::Void
    });

    native(state, functions, "exit".to_string(), |_args| {
        std::process::exit(0);
    });

    native(state, functions, "sleep".to_string(), |args| {
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
}
