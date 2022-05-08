use crate::{
    exprs::{Expression, IndexExpression},
    lexer::{Lexer, TokenInfo, TokenType},
};

use super::{call, ParserResult, result, Parser};

pub fn parse(
    lexer: &mut Lexer,
    parser: &mut Parser,
    from: Expression,
    info: TokenInfo,
) -> ParserResult {
    let index = super::parse_expression(lexer, parser)?;
    if let Some(token) = lexer.next() {
        match token.take_type() {
            TokenType::RightSquareBracket => (),
            _ => return result::unexpected(info),
        }
    }

    if let Some(token) = lexer.peek() {
        match token.ttype() {
            TokenType::LeftSquareBracket => {
                lexer.next();
                return parse(
                    lexer,
                    parser,
                    IndexExpression::new(from, index, info.clone()),
                    info,
                );
            }
            TokenType::LeftParenthesis => {
                lexer.next();
                return call::parse(
                    lexer,
                    parser,
                    IndexExpression::new(from, index, info.clone()),
                    info,
                    false,
                );
            }
            _ => (),
        }
    }

    Ok(IndexExpression::new(from, index, info))
}
