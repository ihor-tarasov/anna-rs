use crate::{
    exprs::{AssignExpression, VariableExpression},
    lexer::{Lexer, TokenInfo, TokenType},
    State,
};

use super::{call, index, parse_expression, ParserResult};

pub fn parse(lexer: &mut Lexer, state: &mut State, name: String, info: TokenInfo) -> ParserResult {
    if let Some(token) = lexer.peek() {
        match token.ttype() {
            TokenType::Equal => {
                lexer.next();
                return Ok(AssignExpression::new(
                    parse_expression(lexer, state)?,
                    name,
                    info,
                ));
            }
            TokenType::LeftSquareBracket => {
                lexer.next();
                return index::parse(
                    lexer,
                    state,
                    VariableExpression::new(name, info.clone()),
                    info,
                );
            }
            TokenType::LeftParenthesis => {
                lexer.next();
                return call::parse(
                    lexer,
                    state,
                    VariableExpression::new(name, info.clone()),
                    info,
                );
            }
            _ => (),
        }
    }
    Ok(VariableExpression::new(name, info))
}
