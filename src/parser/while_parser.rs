use crate::{
    exprs::{Expression, WhileExpression},
    lexer::{Lexer, TokenInfo},
};

use super::{block, parse_expression, ParserResult, Parser};

pub fn parse(lexer: &mut Lexer, parser: &mut Parser, info: TokenInfo) -> ParserResult {
    let condition = parse_expression(lexer, parser)?;
    let block = match block::parse(lexer, parser)? {
        Expression::Block(block) => block,
        _ => panic!("Expected block"),
    };
    Ok(WhileExpression::new(condition, block, info))
}
