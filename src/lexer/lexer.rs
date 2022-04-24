use super::{reader::Reader, Token, single, number};

fn lex(reader: &mut Reader) -> Option<Token> {
    let mut c = reader.peek()?;
    while c == b' ' || c == b'\t' ||
        c == b'\r' || c == b'\n' {
        reader.next()?;
        c = reader.peek()?;
    }

    if c.is_ascii_digit() {
        match number::lex(reader) {
            Some(token) => return Some(token),
            None => (),
        }
    }

    single::lex(reader)
}

pub struct Lexer<'a> {
    reader: Reader<'a>,
    token: Option<Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(data: &'a[u8]) -> Self {
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
