use crate::{
    exprs::ArrayExpression,
    lexer::{Lexer, TokenType}, State,
};

use super::{unexpected, unexpected_eof, ParserResult};

pub fn parse(lexer: &mut Lexer, state: &mut State) -> ParserResult {
    let mut exprs = Vec::new();

    match lexer.peek() {
        Some(token) => match token.ttype() {
            TokenType::RightSquareBracket => {
                lexer.next();
                return Ok(ArrayExpression::new(exprs));
            }
            _ => (),
        },
        None => return unexpected_eof(),
    }

    loop {
        exprs.push(super::parse_expression(lexer, state)?);

        match lexer.peek() {
            Some(token) => match token.ttype() {
                TokenType::RightSquareBracket => {
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

    Ok(ArrayExpression::new(exprs))
}
