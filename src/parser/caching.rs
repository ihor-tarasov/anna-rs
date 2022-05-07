use crate::{lexer::{Lexer, TokenType}, types::Value, exprs::CachingExpression};

use super::{ParserResult, parse_expression, Parser};

pub fn parse(lexer: &mut Lexer, parser: &mut Parser, value: Value) -> ParserResult {
    let mut expr = None;

    if let Some(token) = lexer.peek() {
        match token.ttype() {
            TokenType::RightSquareBracket => (),
            TokenType::Comma => (),
            TokenType::RightParenthesis => (),
            _ => {
                expr = Some(parse_expression(lexer, parser)?);
            },
        }
    }

    Ok(CachingExpression::new(expr, value))
}
