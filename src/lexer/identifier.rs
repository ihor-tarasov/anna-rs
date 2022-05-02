use super::{reader::Reader, Token, TokenType, TokenInfo};

pub fn lex(reader: &mut Reader) -> Option<Token> {
    let mut accumulator = String::new();
    let begin = reader.position();
    while let Some(c) = reader.peek() {
        if c.is_ascii_alphanumeric() || c == b'_' {
            accumulator.push(c as char);
        } else {
            break;
        }
        reader.next();
    }

    if accumulator.is_empty() { return None }

    let length = accumulator.len();

    let ttype = match accumulator.as_str() {
        "var" => TokenType::Var,
        "break" => TokenType::Break,
        "continue" => TokenType::Continue,
        "return" => TokenType::Return,
        "if" => TokenType::If,
        "else" => TokenType::Else,
        _ => TokenType::Identifier(accumulator),
    };

    let info = TokenInfo::new(begin, length);

    Some(Token::new(ttype, info))
}
