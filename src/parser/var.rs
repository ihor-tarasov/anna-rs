use crate::{
    exprs::VarExpression,
    lexer::{Lexer, TokenType},
};

use super::{result, ParserResult, Parser};

pub fn parse(lexer: &mut Lexer, parser: &mut Parser, require: bool) -> ParserResult {
    let token = match lexer.next(true) {
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

    match lexer.next(true) {
        Some(token) => match token.ttype() {
            TokenType::Equal => (),
            _ => return result::unexpected(token.info()),
        },
        None => return result::unexpected_eof(),
    }

    let expr = super::parse_expression(lexer, parser, require)?;

    Ok(VarExpression::new(expr, name, info))
}
