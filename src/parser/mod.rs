use crate::{
    lexer::{Lexer, TokenInfo},
    Functions,
};

mod array;
mod bitwise;
mod block;
mod caching;
mod call;
mod comparison;
mod equality;
mod factor;
mod function;
mod identifier;
mod if_parser;
mod index;
mod primary;
mod result;
mod term;
mod unary;
mod var;
mod while_parser;

pub use result::ParserError;
pub use result::ParserErrorType;
pub use result::ParserResult;

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
        result::unexpected(token.info())
    } else {
        Ok(result)
    }
}
