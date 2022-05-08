use super::{double, identifier, number, reader::Reader, single, Token, TokenInfo, string};

fn lex(reader: &mut Reader) -> Option<Token> {
    let mut c = reader.peek()?;
    while c == b' ' || c == b'\t' || c == b'\r' || c == b'\n' {
        reader.next()?;
        c = reader.peek()?;
    }

    if c.is_ascii_alphabetic() {
        match identifier::lex(reader) {
            Some(token) => return Some(token),
            None => (),
        }
    }

    if c.is_ascii_digit() {
        match number::lex(reader) {
            Some(token) => return Some(token),
            None => (),
        }
    }

    if c == b'"' {
        return Some(string::lex(reader));
    }

    c = reader.next()?;

    if let Some(c2) = reader.peek() {
        if let Some(ttype) = double::lex(c, c2) {
            reader.next()?;
            return Some(Token::new(ttype, TokenInfo::new(reader.position() - 2, 2)));
        }
    }

    Some(Token::new(
        single::lex(c),
        TokenInfo::new(reader.position() - 1, 1),
    ))
}

pub struct Lexer<'a> {
    reader: Reader<'a>,
    token: Option<Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        let mut reader = Reader::new(data);
        Self {
            token: lex(&mut reader),
            reader,
        }
    }

    pub fn peek(&self) -> Option<&Token> {
        self.token.as_ref()
    }

    pub fn next(&mut self) -> Option<Token> {
        std::mem::replace(&mut self.token, lex(&mut self.reader))
    }
}
