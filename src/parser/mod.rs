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

pub struct Parser<'a> {
    functions: &'a mut Functions,
}

impl<'a> Parser<'a> {
    pub fn new(functions: &'a mut Functions) -> Self {
        Self {
            functions,
        }
    }

    pub fn functions(&self) -> &Functions {
        self.functions
    }

    pub fn functions_mut(&mut self) -> &mut Functions {
        self.functions
    }
}

pub fn parse_expression(lexer: &mut Lexer, parser: &mut Parser) -> ParserResult {
    equality::parse(lexer, parser)
}

pub fn parse(lexer: &mut Lexer, parser: &mut Parser) -> ParserResult {
    let result = match lexer.peek() {
        Some(_) => parse_expression(lexer, parser)?,
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
