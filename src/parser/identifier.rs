use crate::{
    exprs::{AssignExpression, VariableExpression},
    lexer::{Lexer, TokenInfo, TokenType},
};

use super::{parse_expression, ParserResult};

pub fn parse(lexer: &mut Lexer, name: String, info: TokenInfo) -> ParserResult {
    if let Some(token) = lexer.peek() {
        match token.ttype() {
            TokenType::Equal => {
                lexer.next();
                return Ok(AssignExpression::new(parse_expression(lexer)?, name, info));
            }
            _ => (),
        }
    }
    Ok(VariableExpression::new(name, info))
}
