use crate::{
    exprs::{Expression, ForExpression},
    lexer::{Lexer, TokenInfo, TokenType},
};

use super::{
    block::{self, BlockGuard},
    parse_expression,
    result::{self, unexpected, unexpected_eof},
    Parser, ParserResult,
};

pub fn parse(lexer: &mut Lexer, parser: &mut Parser, info: TokenInfo, require: bool) -> ParserResult {
    let mut guard = BlockGuard::new(parser);

    let variable = if let Some(token) = lexer.next(true) {
        let info = token.info();
        match token.take_type() {
            TokenType::Identifier(name) => name,
            _ => return unexpected(info),
        }
    } else {
        return unexpected_eof();
    };

    if !guard
        .parser_mut()
        .stack
        .last_mut()
        .unwrap()
        .push_variable(variable.clone())
    {
        return result::already_exist(info);
    }

    if let Some(token) = lexer.next(true) {
        let info = token.info();
        match token.take_type() {
            TokenType::In => (),
            _ => return unexpected(info),
        }
    } else {
        return unexpected_eof();
    }

    let condition = parse_expression(lexer, guard.parser_mut(), true)?;

    let block = match block::parse(lexer, guard.parser_mut(), require)? {
        Expression::Block(block) => block,
        _ => panic!("Expected block"),
    };

    Ok(ForExpression::new(variable, condition, block, info))
}
