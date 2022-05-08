use super::{reader::Reader, Token, TokenType, TokenInfo};

fn invalid_string(begin: usize, length: usize) -> Token {
    return Token::new(TokenType::InvalidString, TokenInfo::new(begin, length));
}

pub fn lex(reader: &mut Reader) -> Token {
    if reader.next().unwrap() != b'"' {
        panic!("Expected \"");
    }

    let mut acc = String::new();
    let start = reader.position();

    loop {
        let c = match reader.peek() {
            Some(c) => c,
            None => return invalid_string(start, acc.len()),
        };

        if c == b'\n' || c == b'\r' {
            return invalid_string(start, acc.len());
        }

        if c == b'"' {
            reader.next().unwrap();
            break;
        }

        acc.push(c as char);
        reader.next().unwrap();
    }

    let length = acc.len();
    Token::new(TokenType::String(acc), TokenInfo::new(start, length))
}
