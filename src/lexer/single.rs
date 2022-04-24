use super::{reader::Reader, Token, TokenType, TokenInfo};

pub fn lex(reader: &mut Reader) -> Option<Token> {
    let ttype = match reader.next()? {
        b'+' => TokenType::Plus,
        b'*' => TokenType::Asterisk,
        _ => TokenType::Unknown,
    };

    let info = TokenInfo::new(reader.position() - 1, 1);

    Some(Token::new(ttype, info))
}
