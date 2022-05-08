use crate::{
    exprs::VarExpression,
    lexer::{Lexer, TokenType},
};

use super::{result, ParserResult, Parser};

pub fn parse(lexer: &mut Lexer, parser: &mut Parser) -> ParserResult {
    let token = match lexer.next() {
        Some(token) => token,
        None => return result::unexpected_eof(),
    };

    let info = token.info();

    let name = match token.take_type() {
        TokenType::Identifier(name) => name,
        _ => return result::unexpected(info),
    };

    if !parser.stack_mut().last_mut().unwrap().push_variable(name.clone()) {
        return result::already_exist(info);
    }

    match lexer.next() {
        Some(token) => match token.ttype() {
            TokenType::Equal => (),
            _ => return result::unexpected(token.info()),
        },
        None => return result::unexpected_eof(),
    }

    let expr = super::parse_expression(lexer, parser)?;

    Ok(VarExpression::new(expr, name, info))
}
