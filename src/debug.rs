use crate::{lexer::TokenInfo, types::{Value, Object}, state::StorageRc};

pub fn print_info(code: &str, info: TokenInfo) {
    let (line, offset) = code
        .chars()
        .enumerate()
        .take_while(|(index, _)| *index != info.begin())
        .fold((0usize, 0usize), |(line, offset), (index, element)| {
            if element == '\n' {
                (line + 1, index + 1)
            } else {
                (line, offset)
            }
        });
    println!("In line: {}", line + 1);
    code.chars()
        .skip(offset)
        .take_while(|e| *e != '\r' && *e != '\n')
        .for_each(|e| print!("{}", e));
    println!();
    (offset..info.begin()).for_each(|_| print!(" "));
    (0..info.length()).for_each(|_| print!("^"));
    println!();
}

pub fn print_value(value: Value, storage: StorageRc) {
    match value {
        Value::Void => print!("#void"),
        Value::Break => print!("#break"),
        Value::Continue => print!("#continue"),
        Value::Return => print!("#return"),
        Value::Boolean(value) => print!("{}", value),
        Value::Integer(value) => print!("{}", value),
        Value::Real(value) => print!("{}", value),
        Value::ObjectId(value) => match storage.lock().unwrap().get(value) {
            Object::String(string) => print!("{}", string),
            _ => print!("#[{}]", value),
        },
        Value::NativeFunctionId(value) => print!("#NFN[{}]", value),
    }
}

pub fn println_value(value: Value, storage: StorageRc) {
    print_value(value, storage);
    println!();
}
