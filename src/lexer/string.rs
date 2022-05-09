use super::{reader::Reader, Token, TokenType, TokenInfo};

fn invalid_string(begin: usize, length: usize) -> Token {
    return Token::new(TokenType::InvalidString, TokenInfo::new(begin, length));
}

pub fn lex(reader: &mut Reader, require: bool) -> Token {
    if reader.next(require).unwrap() != '"' {
        panic!("Expected \"");
    }

    let mut acc = String::new();
    let start = reader.position();

    loop {
        let c = match reader.peek(require) {
            Some(c) => c,
            None => return invalid_string(start, acc.len()),
        };

        if c == '\n' || c == '\r' {
            return invalid_string(start, acc.len());
        }

        if c == '"' {
            reader.next(require).unwrap();
            break;
        }

        acc.push(c as char);
        reader.next(require).unwrap();
    }

    let length = acc.len();
    Token::new(TokenType::String(acc), TokenInfo::new(start, length))
}
