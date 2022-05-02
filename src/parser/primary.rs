use crate::{
    exprs::LiteralExpression,
    lexer::{Lexer, TokenType},
    types::Value, State,
};

use super::{
    array, caching, function, identifier, if_parser, unexpected, unexpected_eof, unknown, var,
    while_parser, ParserResult,
};

pub fn parse(lexer: &mut Lexer, state: &mut State) -> ParserResult {
    let token = match lexer.next() {
        Some(token) => token,
        None => return unexpected_eof(),
    };
    let info = token.info();
    match token.take_type() {
        TokenType::Integer(value) => Ok(LiteralExpression::new(Value::Integer(value))),
        TokenType::Real(value) => Ok(LiteralExpression::new(Value::Real(value))),
        TokenType::Identifier(name) => identifier::parse(lexer, state, name, info),
        TokenType::Var => var::parse(lexer, state),
        TokenType::If => if_parser::parse(lexer, state, info),
        TokenType::While => while_parser::parse(lexer, state, info),
        TokenType::LeftSquareBracket => array::parse(lexer, state),
        TokenType::Break => caching::parse(lexer, state, Value::Break),
        TokenType::Return => caching::parse(lexer, state, Value::Return),
        TokenType::Continue => Ok(LiteralExpression::new(Value::Continue)),
        TokenType::VerticalBar => function::parse(lexer, state, info),
        TokenType::Unknown => unknown(info),
        _ => unexpected(info),
    }
}
