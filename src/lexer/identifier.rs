use super::{reader::Reader, Token, TokenType, TokenInfo};

pub fn lex(reader: &mut Reader, require: bool) -> Option<Token> {
    let mut accumulator = String::new();
    let begin = reader.position();
    while let Some(c) = reader.peek(require) {
        if c.is_ascii_alphanumeric() || c == '_' {
            accumulator.push(c as char);
        } else {
            break;
        }
        reader.next(require);
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
        "while" => TokenType::While,
        "for" => TokenType::For,
        "in" => TokenType::In,
        _ => TokenType::Identifier(accumulator),
    };

    let info = TokenInfo::new(begin, length);

    Some(Token::new(ttype, info))
}
