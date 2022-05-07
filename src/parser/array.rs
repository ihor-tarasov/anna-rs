use crate::{
    exprs::ArrayExpression,
    lexer::{Lexer, TokenType},
};

use super::{ParserResult, result, Parser};

pub fn parse(lexer: &mut Lexer, parser: &mut Parser) -> ParserResult {
    let mut exprs = Vec::new();

    match lexer.peek() {
        Some(token) => match token.ttype() {
            TokenType::RightSquareBracket => {
                lexer.next();
                return Ok(ArrayExpression::new(exprs));
            }
            _ => (),
        },
        None => return result::unexpected_eof(),
    }

    loop {
        exprs.push(super::parse_expression(lexer, parser)?);

        match lexer.peek() {
            Some(token) => match token.ttype() {
                TokenType::RightSquareBracket => {
                    lexer.next();
                    break;
                }
                TokenType::Comma => {
                    lexer.next();
                }
                _ => return result::unexpected(token.info()),
            },
            None => return result::unexpected_eof(),
        }
    }

    Ok(ArrayExpression::new(exprs))
}
