use crate::{
    exprs::Expression,
    lexer::{Lexer, TokenInfo}, Functions,
};

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

mod comparison;
mod equality;
mod factor;
mod primary;
mod term;
mod unary;
mod bitwise;
mod var;
mod identifier;
mod array;
mod index;
mod call;
mod caching;
mod block;
mod if_parser;
mod while_parser;
mod function;

pub fn parse_expression(lexer: &mut Lexer, functions: &mut Functions) -> ParserResult {
    equality::parse(lexer, functions)
}

pub fn parse(lexer: &mut Lexer, functions: &mut Functions) -> ParserResult {
    let result = match lexer.peek() {
        Some(_) => parse_expression(lexer, functions)?,
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
