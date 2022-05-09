use super::TokenType;

pub fn lex(c1: char, c2: char) -> Option<TokenType> {
    match (c1, c2) {
        ('=', '=') => Some(TokenType::EqualEqual),
        ('!', '=') => Some(TokenType::ExclamationEqual),
        ('>', '=') => Some(TokenType::GreaterEqual),
        ('<', '=') => Some(TokenType::LessEqual),
        ('<', '<') => Some(TokenType::LessLess),
        ('>', '>') => Some(TokenType::GreaterGreater),
        _ => None,
    }
}
