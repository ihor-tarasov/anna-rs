use crate::{
    exprs::Expression,
    lexer::{Lexer, TokenInfo},
};

#[derive(Clone, PartialEq)]
pub enum ParserErrorType {
    UnknownCharacter,
    UnexpectedToken,
    UnexpectedEndOfFile,
    Empty,
}

pub struct ParserError {
    etype: ParserErrorType,
    info: TokenInfo,
}

impl ParserError {
    pub fn new(etype: ParserErrorType, info: TokenInfo) -> Self {
        Self { etype, info }
    }

    pub fn etype(&self) -> ParserErrorType {
        self.etype.clone()
    }

    pub fn info(&self) -> TokenInfo {
        self.info.clone()
    }
}

pub type ParserResult = Result<Expression, ParserError>;

fn unexpected(info: TokenInfo) -> ParserResult {
    Err(ParserError::new(ParserErrorType::UnexpectedToken, info))
}

fn unexpected_eof() -> ParserResult {
    Err(ParserError::new(
        ParserErrorType::UnexpectedEndOfFile,
        TokenInfo::new(0, 0),
    ))
}

fn unknown(info: TokenInfo) -> ParserResult {
    Err(ParserError::new(ParserErrorType::UnknownCharacter, info))
}

pub fn parse(lexer: &mut Lexer) -> ParserResult {
    let result = match lexer.peek() {
        Some(_) => term::parse(lexer)?,
        None => {
            return Err(ParserError::new(
                ParserErrorType::Empty,
                TokenInfo::new(0, 0),
            ))
        }
    };
    if let Some(token) = lexer.peek() {
        unexpected(token.info())
    } else {
        Ok(result)
    }
}

mod factor;
mod primary;
mod term;
