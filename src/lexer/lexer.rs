use super::{double, identifier, number, reader::Reader, single, Token, TokenInfo, string};

fn lex(reader: &mut Reader, require: bool) -> Option<Token> {
    let mut c = reader.peek(require)?;
    while c == ' ' || c == '\t' || c == '\r' || c == '\n' {
        reader.next(require)?;
        c = reader.peek(require)?;
    }

    if c.is_ascii_alphabetic() {
        match identifier::lex(reader, require) {
            Some(token) => return Some(token),
            None => (),
        }
    }

    if c.is_ascii_digit() {
        match number::lex(reader, require) {
            Some(token) => return Some(token),
            None => (),
        }
    }

    if c == '"' {
        return Some(string::lex(reader, require));
    }

    if c == '\0' {
        return None;
    }

    c = reader.next(require)?;

    if let Some(c2) = reader.peek(require) {
        if let Some(ttype) = double::lex(c, c2) {
            reader.next(require)?;
            return Some(Token::new(ttype, TokenInfo::new(reader.position() - 2, 2)));
        }
    }

    Some(Token::new(
        single::lex(c),
        TokenInfo::new(reader.position() - 1, 1),
    ))
}

pub struct Lexer {
    reader: Reader,
    token: Option<Token>,
}

impl Lexer {
    pub fn new<F: Fn() -> Option<String> + 'static>(callback: F) -> Self {
        let mut reader = Reader::new(Box::new(callback));
        Self {
            token: lex(&mut reader, true),
            reader,
        }
    }

    fn lexer(&mut self, require: bool) -> Option<Token> {
        std::mem::replace(&mut self.token, lex(&mut self.reader, require))
    }

    pub fn peek(&mut self, require: bool) -> Option<&Token> {
        if self.token.is_some() {
            self.token.as_ref()
        } else {
            if require {
                self.lexer(true);
                self.token.as_ref()
            } else {
                None
            }
        }
    }

    pub fn next(&mut self, require: bool) -> Option<Token> {
        if self.token.is_some() {
            self.lexer(false)
        } else {
            if require {
                self.lexer(true);
                if self.token.is_some() {
                    self.lexer(false)
                } else {
                    None
                }
            } else {
                None
            }
        }
    }
}
