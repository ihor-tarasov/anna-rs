use super::TokenType;

pub fn lex(c1: u8, c2: u8) -> Option<TokenType> {
    match (c1, c2) {
        (b'=', b'=') => Some(TokenType::EqualEqual),
        (b'!', b'=') => Some(TokenType::ExclamationEqual),
        (b'>', b'=') => Some(TokenType::GreaterEqual),
        (b'<', b'=') => Some(TokenType::LessEqual),
        (b'<', b'<') => Some(TokenType::LessLess),
        (b'>', b'>') => Some(TokenType::GreaterGreater),
        _ => None,
    }
}
