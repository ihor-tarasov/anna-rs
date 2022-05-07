use crate::{lexer::TokenInfo, exprs::Expression};

#[derive(Debug, Clone, PartialEq)]
pub enum ParserErrorType {
    UnknownCharacter,
    UnexpectedToken,
    UnexpectedEndOfFile,
    Empty,
    ArgumentAlreadyExist,
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

pub fn unexpected(info: TokenInfo) -> ParserResult {
    Err(ParserError::new(ParserErrorType::UnexpectedToken, info))
}

pub fn unexpected_eof() -> ParserResult {
    Err(ParserError::new(
        ParserErrorType::UnexpectedEndOfFile,
        TokenInfo::new(0, 0),
    ))
}

pub fn unknown(info: TokenInfo) -> ParserResult {
    Err(ParserError::new(ParserErrorType::UnknownCharacter, info))
}
