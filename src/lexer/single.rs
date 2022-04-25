use super::TokenType;

pub fn lex(c: u8) -> TokenType {
    match c {
        b'+' => TokenType::Plus,
        b'-' => TokenType::Minus,
        b'*' => TokenType::Asterisk,
        b'/' => TokenType::Slash,
        b'<' => TokenType::Less,
        b'>' => TokenType::Greater,
        b'!' => TokenType::Exclamation,
        b'&' => TokenType::Ampersand,
        b'|' => TokenType::VerticalBar,
        b'^' => TokenType::Circumflex,
        b'=' => TokenType::Equal,
        _ => TokenType::Unknown,
    }
}
