use crate::{
    exprs::{Expression, IndexExpression},
    lexer::{Lexer, TokenInfo, TokenType},
    State,
};

use super::{call, unexpected, ParserResult};

pub fn parse(
    lexer: &mut Lexer,
    state: &mut State,
    from: Expression,
    info: TokenInfo,
) -> ParserResult {
    let index = super::parse_expression(lexer, state)?;
    if let Some(token) = lexer.next() {
        match token.take_type() {
            TokenType::RightSquareBracket => (),
            _ => return unexpected(info),
        }
    }

    if let Some(token) = lexer.peek() {
        match token.ttype() {
            TokenType::LeftSquareBracket => {
                lexer.next();
                return parse(
                    lexer,
                    state,
                    IndexExpression::new(from, index, info.clone()),
                    info,
                );
            }
            TokenType::LeftParenthesis => {
                lexer.next();
                return call::parse(
                    lexer,
                    state,
                    IndexExpression::new(from, index, info.clone()),
                    info,
                );
            }
            _ => (),
        }
    }

    Ok(IndexExpression::new(from, index, info))
}
