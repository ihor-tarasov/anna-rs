use crate::{lexer::TokenInfo, types::Value};

pub fn print_info(code: &[u8], info: TokenInfo) {
    let (line, offset) = code
        .iter()
        .enumerate()
        .take_while(|(index, _)| *index != info.begin())
        .fold((0usize, 0usize), |(line, offset), (index, element)| {
            if *element == b'\n' {
                (line + 1, index + 1)
            } else {
                (line, offset)
            }
        });
    println!("In line: {}", line + 1);
    code.iter()
        .skip(offset)
        .cloned()
        .take_while(|e| *e != b'\r' && *e != b'\n')
        .for_each(|e| print!("{}", e as char));
    println!();
    (offset..info.begin()).for_each(|_| print!(" "));
    (0..info.length()).for_each(|_| print!("^"));
    println!();
}

pub fn print_value(value: Value) {
    match value {
        Value::Void => print!("#void"),
        Value::Break => print!("#break"),
        Value::Continue => print!("#continue"),
        Value::Return => print!("#return"),
        Value::Boolean(value) => print!("{}", value),
        Value::Integer(value) => print!("{}", value),
        Value::Real(value) => print!("{}", value),
        Value::ObjectId(value) => print!("#[{}]", value),
        Value::NativeFunctionId(value) => print!("#NFN[{}]", value),
    }
}

pub fn println_value(value: Value) {
    print_value(value);
    println!();
}
