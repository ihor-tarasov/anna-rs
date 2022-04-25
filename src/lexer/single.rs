use super::TokenType;

pub fn lex(c: u8) -> TokenType {
    match c {
        b'+' => TokenType::Plus,
        b'-' => TokenType::Minus,
        b'*' => TokenType::Asterisk,
        b'/' => TokenType::Slash,
        b'<' => TokenType::Less,
        b'>' => TokenType::Greater,
        _ => TokenType::Unknown,
    }
}
