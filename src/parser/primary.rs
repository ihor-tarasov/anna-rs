use crate::{
    exprs::LiteralExpression,
    lexer::{Lexer, TokenType},
    types::Value,
};

use super::{unexpected_eof, ParserResult, unknown, unexpected, var, identifier};

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
        TokenType::Unknown => unknown(info),
        _ => unexpected(info),
    }
}
