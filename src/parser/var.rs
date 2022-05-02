use crate::{
    exprs::VarExpression,
    lexer::{Lexer, TokenType}, State,
};

use super::{parse_expression, unexpected, unexpected_eof, ParserResult};

pub fn parse(lexer: &mut Lexer, state: &mut State) -> ParserResult {
    let token = match lexer.next() {
        Some(token) => token,
        None => return unexpected_eof(),
    };

    let info = token.info();

    let name = match token.take_type() {
        TokenType::Identifier(name) => name,
        _ => return unexpected(info),
    };

    match lexer.next() {
        Some(token) => match token.ttype() {
            TokenType::Equal => (),
            _ => return unexpected(token.info()),
        },
        None => return unexpected_eof(),
    }

    let expr = parse_expression(lexer, state)?;

    Ok(VarExpression::new(expr, name, info))
}
