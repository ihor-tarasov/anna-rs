use crate::{
    exprs::LiteralExpression,
    lexer::{Lexer, TokenType},
    types::Value, Functions,
};

use super::{
    array, caching, function, identifier, if_parser, var,
    while_parser, ParserResult, result,
};

pub fn parse(lexer: &mut Lexer, functions: &mut Functions) -> ParserResult {
    let token = match lexer.next() {
        Some(token) => token,
        None => return result::unexpected_eof(),
    };
    let info = token.info();
    match token.take_type() {
        TokenType::Integer(value) => Ok(LiteralExpression::new(Value::Integer(value))),
        TokenType::Real(value) => Ok(LiteralExpression::new(Value::Real(value))),
        TokenType::Identifier(name) => identifier::parse(lexer, functions, name, info),
        TokenType::Var => var::parse(lexer, functions),
        TokenType::If => if_parser::parse(lexer, functions, info),
        TokenType::While => while_parser::parse(lexer, functions, info),
        TokenType::LeftSquareBracket => array::parse(lexer, functions),
        TokenType::Break => caching::parse(lexer, functions, Value::Break),
        TokenType::Return => caching::parse(lexer, functions, Value::Return),
        TokenType::Continue => Ok(LiteralExpression::new(Value::Continue)),
        TokenType::VerticalBar => function::parse(lexer, functions, info),
        TokenType::Unknown => result::unknown(info),
        _ => result::unexpected(info),
    }
}
