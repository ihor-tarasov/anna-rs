use super::{reader::Reader, Token, TokenType, TokenInfo};

pub fn lex(reader: &mut Reader) -> Option<Token> {
    let mut accumulator = String::new();
    let mut dot = false;
    let begin = reader.position();
    while let Some(c) = reader.peek() {
        match c {
            b'0'..=b'9' => accumulator.push(c as char),
            b'.' => {
                if dot { break } else { dot = true }
                accumulator.push('.');
            },
            _ => break,
        }
        reader.next();
    }

    if accumulator.is_empty() { return None }

    let ttype = if dot {
        TokenType::Real(accumulator.parse::<f64>().unwrap())
    } else {
        TokenType::Integer(accumulator.parse::<i64>().unwrap())
    };

    let info = TokenInfo::new(begin, accumulator.len());

    Some(Token::new(ttype, info))
}
