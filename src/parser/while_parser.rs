use crate::{lexer::{Lexer, TokenInfo}, exprs::{Expression, WhileExpression}, State};

use super::{ParserResult, parse_expression, block};

pub fn parse(lexer: &mut Lexer, state: &mut State, info: TokenInfo) -> ParserResult {
    let condition = parse_expression(lexer, state)?;
    let block = match block::parse(lexer, state)? {
        Expression::Block(block) => block,
        _ => panic!("Expected block"),
    };
    Ok(WhileExpression::new(condition, block, info))
}
