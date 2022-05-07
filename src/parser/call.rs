use crate::{
    exprs::{CallExpression, Expression},
    lexer::{Lexer, TokenInfo, TokenType},
    Functions,
};

use super::{index, unexpected, unexpected_eof, ParserResult};

pub fn parse(
    lexer: &mut Lexer,
    functions: &mut Functions,
    from: Expression,
    info: TokenInfo,
    is_async: bool,
) -> ParserResult {
    let mut exprs = Vec::new();

    match lexer.peek() {
        Some(token) => match token.ttype() {
            TokenType::RightParenthesis => {
                lexer.next();
                return Ok(CallExpression::new(from, exprs, is_async, info));
            }
            _ => (),
        },
        None => return unexpected_eof(),
    }

    loop {
        exprs.push(super::parse_expression(lexer, functions)?);

        match lexer.peek() {
            Some(token) => match token.ttype() {
                TokenType::RightParenthesis => {
                    lexer.next();
                    break;
                }
                TokenType::Comma => {
                    lexer.next();
                }
                _ => return unexpected(token.info()),
            },
            None => return unexpected_eof(),
        }
    }

    if let Some(token) = lexer.peek() {
        match token.ttype() {
            TokenType::LeftSquareBracket => {
                lexer.next();
                return index::parse(
                    lexer,
                    functions,
                    CallExpression::new(from, exprs, is_async, info.clone()),
                    info,
                );
            }
            _ => (),
        }
    }

    Ok(CallExpression::new(from, exprs, is_async, info))
}
