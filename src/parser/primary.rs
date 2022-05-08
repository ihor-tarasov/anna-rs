use crate::{
    exprs::{LiteralExpression, StringLiteralExpression},
    lexer::{Lexer, TokenType},
    types::Value,
};

use super::{
    array, caching, function, identifier, if_parser, var,
    while_parser, ParserResult, result, Parser, block,
};

pub fn parse(lexer: &mut Lexer, parser: &mut Parser) -> ParserResult {
    if let Some(token) = lexer.peek() {
        match token.ttype() {
            TokenType::LeftBrace => {
                return block::parse(lexer, parser);
            }
            _ => (),
        }
    }

    let token = match lexer.next() {
        Some(token) => token,
        None => return result::unexpected_eof(),
    };
    let info = token.info();
    match token.take_type() {
        TokenType::Integer(value) => Ok(LiteralExpression::new(Value::Integer(value))),
        TokenType::Real(value) => Ok(LiteralExpression::new(Value::Real(value))),
        TokenType::Identifier(name) => identifier::parse(lexer, parser, name, info),
        TokenType::Var => var::parse(lexer, parser),
        TokenType::If => if_parser::parse(lexer, parser, info),
        TokenType::While => while_parser::parse(lexer, parser, info),
        TokenType::LeftSquareBracket => array::parse(lexer, parser),
        TokenType::Break => caching::parse(lexer, parser, Value::Break),
        TokenType::Return => caching::parse(lexer, parser, Value::Return),
        TokenType::Continue => Ok(LiteralExpression::new(Value::Continue)),
        TokenType::VerticalBar => function::parse(lexer, parser, info),
        TokenType::String(value) => Ok(StringLiteralExpression::new(value)),
        TokenType::Unknown => result::unknown(info),
        _ => result::unexpected(info),
    }
}
