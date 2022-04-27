use crate::{lexer::{Lexer, TokenInfo, TokenType}, exprs::{Expression, IndexExpression}};

use super::{ParserResult, unexpected};

pub fn parse(lexer: &mut Lexer, from: Expression, info: TokenInfo) -> ParserResult {
    let index = super::parse_expression(lexer)?;
    if let Some(token) = lexer.next() {
        match token.take_type() {
            TokenType::RightSquareBracket => (),
            _ => return unexpected(info),
        }
    }

    Ok(IndexExpression::new(from, index, info))
}
