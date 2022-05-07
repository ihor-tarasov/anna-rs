use crate::{
    exprs::VarExpression,
    lexer::{Lexer, TokenType},
    Functions,
};

use super::{result, ParserResult};

pub fn parse(lexer: &mut Lexer, functions: &mut Functions) -> ParserResult {
    let token = match lexer.next() {
        Some(token) => token,
        None => return result::unexpected_eof(),
    };

    let info = token.info();

    let name = match token.take_type() {
        TokenType::Identifier(name) => name,
        _ => return result::unexpected(info),
    };

    match lexer.next() {
        Some(token) => match token.ttype() {
            TokenType::Equal => (),
            _ => return result::unexpected(token.info()),
        },
        None => return result::unexpected_eof(),
    }

    let expr = super::parse_expression(lexer, functions)?;

    Ok(VarExpression::new(expr, name, info))
}
