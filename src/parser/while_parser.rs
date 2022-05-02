use crate::{lexer::{Lexer, TokenInfo}, exprs::{Expression, WhileExpression}};

use super::{ParserResult, parse_expression, block};

pub fn parse(lexer: &mut Lexer, info: TokenInfo) -> ParserResult {
    let condition = parse_expression(lexer)?;
    let block = match block::parse(lexer)? {
        Expression::Block(block) => block,
        _ => panic!("Expected block"),
    };
    Ok(WhileExpression::new(condition, block, info))
}
