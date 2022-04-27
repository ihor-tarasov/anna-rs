use crate::{lexer::{Lexer, TokenType}, types::Value, exprs::CachingExpression};

use super::{ParserResult, parse_expression};

pub fn parse(lexer: &mut Lexer, value: Value) -> ParserResult {
    let mut expr = None;

    if let Some(token) = lexer.peek() {
        match token.ttype() {
            TokenType::RightSquareBracket => (),
            TokenType::Comma => (),
            TokenType::RightParenthesis => (),
            _ => {
                expr = Some(parse_expression(lexer)?);
            },
        }
    }

    Ok(CachingExpression::new(expr, value))
}
