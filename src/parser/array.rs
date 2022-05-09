use crate::{
    exprs::ArrayExpression,
    lexer::{Lexer, TokenType},
};

use super::{ParserResult, result, Parser};

pub fn parse(lexer: &mut Lexer, parser: &mut Parser) -> ParserResult {
    let mut exprs = Vec::new();

    match lexer.peek(true) {
        Some(token) => match token.ttype() {
            TokenType::RightSquareBracket => {
                lexer.next(true);
                return Ok(ArrayExpression::new(exprs));
            }
            _ => (),
        },
        None => return result::unexpected_eof(),
    }

    loop {
        exprs.push(super::parse_expression(lexer, parser, true)?);

        match lexer.peek(true) {
            Some(token) => match token.ttype() {
                TokenType::RightSquareBracket => {
                    lexer.next(true);
                    break;
                }
                TokenType::Comma => {
                    lexer.next(true);
                }
                _ => return result::unexpected(token.info()),
            },
            None => return result::unexpected_eof(),
        }
    }

    Ok(ArrayExpression::new(exprs))
}
