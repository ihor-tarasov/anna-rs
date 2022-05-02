use crate::{
    exprs::{BinaryExpression, Expression},
    lexer::{Lexer, TokenType},
    State,
};

use super::{bitwise, ParserResult};

pub fn parse(lexer: &mut Lexer, state: &mut State) -> ParserResult {
    let mut lhs = bitwise::parse(lexer, state)?;
    while let Some(token) = lexer.peek() {
        match token.ttype() {
            TokenType::Asterisk => {
                let info = token.info();
                lexer.next();
                let rhs = bitwise::parse(lexer, state)?;
                lhs = Expression::Multiply(Box::new(BinaryExpression::new(lhs, rhs, info)));
            }
            TokenType::Slash => {
                let info = token.info();
                lexer.next();
                let rhs = bitwise::parse(lexer, state)?;
                lhs = Expression::Divide(Box::new(BinaryExpression::new(lhs, rhs, info)));
            }
            _ => break,
        }
    }
    Ok(lhs)
}
