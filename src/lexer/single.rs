use super::TokenType;

pub fn lex(c: char) -> TokenType {
    match c {
        '+' => TokenType::Plus,
        '-' => TokenType::Minus,
        '*' => TokenType::Asterisk,
        '/' => TokenType::Slash,
        '<' => TokenType::Less,
        '>' => TokenType::Greater,
        '!' => TokenType::Exclamation,
        '&' => TokenType::Ampersand,
        '|' => TokenType::VerticalBar,
        '^' => TokenType::Circumflex,
        '=' => TokenType::Equal,
        '[' => TokenType::LeftSquareBracket,
        ']' => TokenType::RightSquareBracket,
        ',' => TokenType::Comma,
        '(' => TokenType::LeftParenthesis,
        ')' => TokenType::RightParenthesis,
        '{' => TokenType::LeftBrace,
        '}' => TokenType::RightBrace,
        ';' => TokenType::Semicolon,
        _ => TokenType::Unknown,
    }
}
