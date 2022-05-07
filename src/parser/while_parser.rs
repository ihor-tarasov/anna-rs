use crate::{
    exprs::{Expression, WhileExpression},
    lexer::{Lexer, TokenInfo},
    Functions,
};

use super::{block, parse_expression, ParserResult};

pub fn parse(lexer: &mut Lexer, functions: &mut Functions, info: TokenInfo) -> ParserResult {
    let condition = parse_expression(lexer, functions)?;
    let block = match block::parse(lexer, functions)? {
        Expression::Block(block) => block,
        _ => panic!("Expected block"),
    };
    Ok(WhileExpression::new(condition, block, info))
}
