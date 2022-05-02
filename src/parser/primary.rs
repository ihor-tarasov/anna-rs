use crate::{
    exprs::LiteralExpression,
    lexer::{Lexer, TokenType},
    types::Value,
};

use super::{array, identifier, unexpected, unexpected_eof, unknown, var, ParserResult, caching, if_parser, while_parser};

pub fn parse(lexer: &mut Lexer) -> ParserResult {
    let token = match lexer.next() {
        Some(token) => token,
        None => return unexpected_eof(),
    };
    let info = token.info();
    match token.take_type() {
        TokenType::Integer(value) => Ok(LiteralExpression::new(Value::Integer(value))),
        TokenType::Real(value) => Ok(LiteralExpression::new(Value::Real(value))),
        TokenType::Identifier(name) => identifier::parse(lexer, name, info),
        TokenType::Var => var::parse(lexer),
        TokenType::If => if_parser::parse(lexer, info),
        TokenType::While => while_parser::parse(lexer, info),
        TokenType::LeftSquareBracket => array::parse(lexer),
        TokenType::Break => caching::parse(lexer, Value::Break),
        TokenType::Return => caching::parse(lexer, Value::Return),
        TokenType::Continue => Ok(LiteralExpression::new(Value::Continue)),
        TokenType::Unknown => unknown(info),
        _ => unexpected(info),
    }
}
